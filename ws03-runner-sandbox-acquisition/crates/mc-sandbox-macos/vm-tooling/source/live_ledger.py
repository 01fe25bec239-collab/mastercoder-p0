# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Complete metadata ledger without per-leaf realpath traversal or immutable caches."""
from dataclasses import dataclass
import gc
from itertools import compress
from operator import is_not, itemgetter
import os
from pathlib import Path
import resource
import select
import stat
import time
import weakref
from resource_ledger import Entry, LedgerError, Snapshot as StrictSnapshot
from _metawalk import scan as _scan, restat as _restat, json_sha256 as _json_sha256  # compiled from _metawalk.c (build-command.txt)
from _metawalk import json_sha256_begin, json_sha256_feed, json_sha256_end, record as _record
from _metawalk import parts_entries as _parts_entries, parts_metadata as _parts_metadata  # r5

# Syscall worker threads inside _metawalk: owner-approved bounded four-thread collector (2026-10-23);
# _metawalk.c rejects anything outside 1..4. Workers only open/enumerate/lstat into C memory; every
# check, row and fault is decided serially in live_ledger order.
JOBS = 4
# Paths per fresh second-pass _restat call: bounds its transient native/Python buffers (~0.3 KiB/path,
# ~20 MiB per batch). r4: large enough that the workers keep lstat-ing through the longest overlapped
# snapshot step (sorting ~358k entries) instead of waiting for the next batch.
_RESTAT_BATCH = 65536

_INT_FIELDS = (int,) * 8
_KEY = itemgetter(0)
# R30 approved diagnostic order; st_blocks stays in raw512-byte units.
_STAMP_FIELDS = ("st_dev", "st_ino", "st_mode", "st_uid", "st_size", "st_blocks",
                 "st_mtime_ns", "st_ctime_ns")


def _stamp(value):
    # Same fields, order, predicate and errors as resource_ledger._stamp; the all-int check runs in C.
    try:
        fields = (value.st_dev, value.st_ino, value.st_mode, value.st_uid,
                  value.st_size, value.st_blocks, value.st_mtime_ns, value.st_ctime_ns)
    except AttributeError as exc:
        raise LedgerError("missing_stat_field") from exc
    if tuple(map(type, fields)) != _INT_FIELDS or fields[1] <= 0 or \
            min(fields[0], fields[3], fields[4], fields[5]) < 0:
        raise LedgerError("malformed_stat")
    return fields


@dataclass(frozen=True)
class Snapshot(StrictSnapshot):
    related_root_parent: str
    related_root_prefix: str
    related_root_additional_names: tuple[str, ...]
    discovered_related_roots: tuple[str, ...]
    known_writer_paths: tuple[str, ...]
    settlement_metadata: tuple[tuple, ...]

    @property
    def settlement_fingerprint(self):
        # The identical tuple collect() built from this very snapshot's entries/metadata during pass 2.
        latest = _LATEST
        if latest is not None and latest[0]() is self:
            return latest[1]
        # Only exact known writers are excluded, and only from settlement assessment.
        known = set(self.known_writer_paths)
        allocations = tuple(sorted((e.device, e.inode, e.allocated, e.size, e.category)
                                   for e in self.entries if not known.issuperset(e.paths)))
        return allocations + self.settlement_metadata

    def to_dict(self):
        return dict(super().to_dict(), settlementFingerprint=self.settlement_fingerprint,
                    accountingAtomic=False, immutableMetadataCache=False)


def settlement_digest(fingerprint):
    """Byte for byte hashlib.sha256(json.dumps(fingerprint, sort_keys=True).encode()).hexdigest().

    _metawalk streams the identical JSON bytes into SHA-256 without building the text. Any value
    outside exact tuple/list/str/int/None/bool raises TypeError there; the json module then decides.
    The fingerprint object collect() attached to its latest snapshot was already streamed, from these
    same immutable values, while pass 2 ran: that exact object's digest is returned (identity only).
    """
    latest = _LATEST
    if latest is not None and latest[1] is fingerprint and latest[2] is not None:
        return latest[2]
    try:
        return _json_sha256(fingerprint)
    except TypeError:
        import hashlib
        import json
        return hashlib.sha256(json.dumps(fingerprint, sort_keys=True).encode()).hexdigest()


def _path(value):
    if not isinstance(value, (str, Path)) or not str(value):
        raise LedgerError("malformed_path")
    path = Path(value)
    if not path.is_absolute() or path != Path(os.path.normpath(path)):
        raise LedgerError("noncanonical_path:" + str(path))
    return path


def _stat(path, uid, device=None):
    try:
        value = os.stat(path, follow_symlinks=False)
    except OSError as exc:
        raise LedgerError("missing_or_unreadable:" + str(path)) from exc
    fields = _stamp(value)
    mode = fields[2]
    if stat.S_ISLNK(mode):
        raise LedgerError("symlink:" + str(path))
    if not (stat.S_ISREG(mode) or stat.S_ISDIR(mode)):
        raise LedgerError("unsupported_file_type:" + str(path))
    if fields[3] != uid:
        raise LedgerError("wrong_owner:" + str(path))
    if device is not None and fields[0] != device:
        raise LedgerError("unadmitted_separate_volume:" + str(path))
    return value


def _anchor(path, uid):
    """Check root ancestors once; each visited descendant is checked without following links."""
    try:
        if path.resolve(strict=True) != path:
            raise LedgerError("unresolved_or_redirected_path:" + str(path))
    except OSError as exc:
        raise LedgerError("missing_or_unreadable:" + str(path)) from exc
    value = _stat(path, uid)
    if not stat.S_ISDIR(value.st_mode):
        raise LedgerError("root_not_directory:" + str(path))
    return value


def _discover(parent, prefix, additional, uid, device):
    try:
        with os.scandir(parent) as children:
            found = tuple(sorted(str(parent / child.name) for child in children
                                 if child.name.startswith(prefix) or child.name in additional))
    except OSError as exc:
        raise LedgerError("unreadable_root_inventory:" + str(parent)) from exc
    if not {str(parent / name) for name in additional}.issubset(found):
        raise LedgerError("missing_additional_related_root")
    for path in found:
        value = _stat(path, uid, device)
        if not stat.S_ISDIR(value.st_mode):
            raise LedgerError("ambiguous_discovered_root:" + path)
    return found


def _fixture_records(config, uid, device):
    records = config["fixtureSymlinkRecords"]
    if not isinstance(records, list):
        raise LedgerError("malformed_fixture_symlink_records")
    evidence = _path(config["evidence"])
    fixture_parent = evidence / "test-fixtures"
    result = {}
    fields = {"path", "fixtureRoot", "device", "inode", "uid", "mode", "size",
              "allocatedBytes", "mtimeNs", "ctimeNs", "target"}
    for record in records:
        if not isinstance(record, dict) or set(record) != fields:
            raise LedgerError("malformed_fixture_symlink_record")
        path, root, target = (_path(record[k]) for k in ("path", "fixtureRoot", "target"))
        if root.parent != fixture_parent or not root.name.startswith("live-ledger-") or \
                path == root or not path.is_relative_to(root) or target == root or not target.is_relative_to(root):
            raise LedgerError("fixture_symlink_outside_synthetic_scope")
        numeric = fields - {"path", "fixtureRoot", "target"}
        if any(type(record[k]) is not int or record[k] < 0 for k in numeric) or \
                record["allocatedBytes"] % 512 or record["inode"] == 0 or \
                record["uid"] != uid or record["device"] != device or not stat.S_ISLNK(record["mode"]):
            raise LedgerError("malformed_fixture_symlink_metadata")
        if str(path) in result:
            raise LedgerError("duplicate_fixture_symlink")
        result[str(path)] = record
    return result


def _pinned_fixture_link(path, record, uid, device):
    root, target = Path(record["fixtureRoot"]), Path(record["target"])
    _anchor(root, uid)
    _anchor(Path(path).parent, uid)
    try:
        if target.resolve(strict=True) != target or not stat.S_ISREG(_stat(target, uid, device).st_mode):
            raise LedgerError("fixture_symlink_target_not_canonical_regular")
        wanted = (record["device"], record["inode"], record["mode"], record["uid"],
                  record["size"], record["allocatedBytes"] // 512, record["mtimeNs"], record["ctimeNs"])
        initial = os.stat(path, follow_symlinks=False)
        if _stamp(initial) != wanted or os.readlink(path) != record["target"] or \
                _stamp(os.stat(path, follow_symlinks=False)) != wanted:
            raise LedgerError("fixture_symlink_pin_changed:" + str(path))
        return initial
    except OSError as exc:
        raise LedgerError("fixture_symlink_missing_or_unreadable:" + str(path)) from exc


# Prospective records authorize opaque accounting only, never endpoint or target access.
_SPECIAL_NUMBERS = ('device', 'inode', 'uid', 'gid', 'mode', 'nlink', 'size',
                    'allocatedBytes', 'mtimeNs', 'ctimeNs')


def _special_metadata(value):
    return dict(zip(_SPECIAL_NUMBERS, (value.st_dev, value.st_ino, value.st_uid,
                value.st_gid, value.st_mode, value.st_nlink, value.st_size,
                value.st_blocks * 512, value.st_mtime_ns, value.st_ctime_ns)))


def _special_values(value):
    return (value.st_dev, value.st_ino, value.st_uid, value.st_gid, value.st_mode, value.st_nlink, value.st_size,
            value.st_blocks * 512, value.st_mtime_ns, value.st_ctime_ns)


def _special_records(config, uid, device, related, absent=(), check=lambda:None):
    import hashlib
    import json
    keys = ('retainedMetadataSnapshotPath', 'retainedMetadataExpectedSHA256')
    if not any(key in config for key in keys):
        return {}
    if not all(key in config for key in keys):
        raise LedgerError('incomplete_special_metadata_opt_in')
    path = _path(config[keys[0]])
    pin = config[keys[1]]
    if type(pin) is not str or len(pin) != 64 or any(ch not in '0123456789abcdef' for ch in pin):
        raise LedgerError('invalid_special_metadata_sha256')
    _anchor(path.parent, uid)
    initial = _stat(path, uid, device)
    if not stat.S_ISREG(initial.st_mode) or initial.st_size > 4 * 2**20:
        raise LedgerError('invalid_special_metadata_file')
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(fd, 'rb') as stream:
            if _stamp(os.fstat(stream.fileno())) != _stamp(initial):
                raise LedgerError('special_metadata_file_changed')
            raw = stream.read(4 * 2**20 + 1)
            if _stamp(os.fstat(stream.fileno())) != _stamp(initial):
                raise LedgerError('special_metadata_file_changed')
        if _stamp(_stat(path, uid, device)) != _stamp(initial):
            raise LedgerError('special_metadata_file_changed')
    except OSError as exc:
        raise LedgerError('special_metadata_file_unreadable') from exc
    if hashlib.sha256(raw).hexdigest() != pin:
        raise LedgerError('special_metadata_sha256_mismatch')
    try:
        snapshot = json.loads(raw)
    except (ValueError, UnicodeDecodeError) as exc:
        raise LedgerError('malformed_special_metadata_snapshot') from exc
    if not isinstance(snapshot, dict) or snapshot.get('schema') != 'root-prospective-retained-metadata-v1' or \
            snapshot.get('authority') != 'ROOT_A1_CURRENT_OPAQUE_STORAGE_ACCOUNTING_ONLY' or \
            any(snapshot.get(key) is not False for key in ('historicalCreatorVerified',
                'endpointAuthorityGranted', 'followLinksAllowed', 'connectSocketsAllowed', 'contentReadsPerformed')):
        raise LedgerError('invalid_special_metadata_authority')
    roots = snapshot.get('roots')
    if snapshot.get('expected_uid') != uid or snapshot.get('expected_device') != device or \
            snapshot.get('root_parent') != config['relatedRootParent'] or \
            snapshot.get('root_prefix') != config['relatedRootPrefix'] or \
            snapshot.get('additional_names') != sorted(config['relatedRootAdditionalNames']) or \
            not isinstance(roots, list) or any(type(p) is not str for p in roots) or len(roots) != len(set(roots)) or not set(roots).issubset(set(related) | set(absent)) or \
            snapshot.get('unsupported') != []:
        raise LedgerError('special_metadata_inventory_mismatch')
    records = snapshot.get('records')
    if not isinstance(records, list):
        raise LedgerError('malformed_special_metadata_records')
    result = {}
    # Same checks and order; the root-only checks are pure functions of the root string, so each passing
    # root string is parsed/checked once per call, and relatedRootParent is parsed once, when first needed.
    roots_set, scope, root_parent = set(roots), {}, None
    for row in records:
        check()
        _validate_special_record(row, uid, device)
        path, text = _path(row['path']), row['root']
        checked = scope.get(text) if type(text) is str else None
        if checked is None:
            root = _path(text)
            if str(root) not in roots_set:
                raise LedgerError('special_metadata_outside_scope')
            if root_parent is None:
                root_parent = _path(config['relatedRootParent'])
            if root.parent != root_parent:
                raise LedgerError('special_metadata_outside_scope')
            checked = (root, root.parts)
            if type(text) is str:
                scope[text] = checked
        root, root_parts = checked
        # path.is_relative_to(root) for canonical paths (both from _path): root's parts prefix path's parts.
        if path == root or path.parts[:len(root_parts)] != root_parts:
            raise LedgerError('special_metadata_outside_scope')
        if str(path) in result:
            raise LedgerError('duplicate_special_metadata_path')
        if str(root) not in absent:
            result[str(path)] = row
    return result


def _validate_special_record(row, uid, device):
    if not isinstance(row, dict) or (row.get('kind') not in ('unix_socket', 'symlink') and
            not (row.get('kind') == 'fifo' and type(row.get('mode')) is int and stat.S_ISFIFO(row['mode']))):
        raise LedgerError('malformed_special_metadata_record')
    fields = {'path', 'root', 'kind', *_SPECIAL_NUMBERS}
    if row['kind'] == 'symlink':
        fields.add('target')
    if set(row) != fields or any(type(row[k]) is not int or row[k] < 0 for k in _SPECIAL_NUMBERS):
        raise LedgerError('malformed_special_metadata_record')
    if row['uid'] != uid or row['device'] != device or row['inode'] == 0 or row['nlink'] == 0 or \
            row['allocatedBytes'] % 512 or not (stat.S_ISSOCK(row['mode']) if row['kind'] == 'unix_socket'
                                                else stat.S_ISLNK(row['mode']) if row['kind'] == 'symlink'
                                                else stat.S_ISFIFO(row['mode'])) or \
            (row['kind'] == 'symlink' and (type(row['target']) is not str or '\0' in row['target'])):
        raise LedgerError('invalid_special_metadata_identity')


def _special_parent_chain(path, row):
    # Reuse only canonical lexical names, never filesystem metadata.
    return _chain(_path(path).parent.parts, _path(row['root']).parts)


def _chain(parts, root):
    if parts[:len(root)] != root:
        return ()
    # Same ancestors, order and strings as str(Path(*parts[:k])) for the in-root ancestors, without building
    # a Path per ancestor: a canonical posix anchor ('/' or '//') is followed by '/'-joined names.
    return tuple(parts[0] + '/'.join(parts[1:k]) for k in range(len(parts), len(root) - 1, -1))


def _observe_special(path, row, uid, device, parents, chain=None, check=lambda:None, call=None):
    real_os=globals()['os']
    if call is None:
        os=real_os
        invoke=lambda fn,*args,**kwargs:fn(*args,**kwargs)
    else:
        from types import SimpleNamespace
        invoke=call
        os=SimpleNamespace(stat=lambda *args,**kwargs:call(real_os.stat,*args,**kwargs),
                           readlink=lambda *args,**kwargs:call(real_os.readlink,*args,**kwargs))
    # Check every parent without resolving the opaque leaf's link text.
    if chain is None:
        chain = _special_parent_chain(path, row)
    for parent in reversed(chain):
        check()
        value = invoke(_stat,parent, uid, device)
        if not stat.S_ISDIR(value.st_mode):
            raise LedgerError('special_parent_not_directory:' + str(parent))
        identity = (value.st_dev, value.st_ino, value.st_mode, value.st_uid)  # = _stamp(value)[:4]; _stat validated it
        if str(parent) in parents and parents[str(parent)] != identity:
            raise LedgerError('special_parent_changed:' + str(parent))
        parents[str(parent)] = identity
    try:
        check()
        initial = os.stat(path, follow_symlinks=False)
        check()
        target = os.readlink(path) if row['kind'] == 'symlink' else None
        check()
        after = os.stat(path, follow_symlinks=False)
        check()
        # Same comparisons as the _special_metadata dicts (one key set): their values, in key order.
        first = _special_values(initial)
        if first != tuple(row[key] for key in _SPECIAL_NUMBERS) or _special_values(after) != first or \
                (row['kind'] == 'symlink' and (target != row['target'] or os.readlink(path) != target)):
            raise LedgerError('special_metadata_changed:' + str(path))
        return initial
    except OSError as exc:
        raise LedgerError('special_missing_or_unreadable:' + str(path)) from exc


def _control_context(config, uid, device):
    from pathlib import Path
    record = config.get('currentControlSocket')
    if record is None:
        return {}
    if not isinstance(record, dict) or set(record) != {'metadata', 'parentMetadata', 'runNonce', 'controllerIdentity'}:
        raise LedgerError('malformed_current_control_socket')
    row = record['metadata']
    _validate_special_record(row, uid, device)
    run = _path(config.get('runDir'))
    identity = record['controllerIdentity']
    if (run.parent != _path(config['evidence']) or row['root'] != str(run) or
            row['path'] != str(run / 'control.sock') or row['kind'] != 'unix_socket' or
            stat.S_IMODE(row['mode']) != 0o600 or row['nlink'] != 1 or
            record['runNonce'] != config.get('runNonce') or
            type(record['runNonce']) is not str or len(record['runNonce']) != 32 or
            any(ch not in '0123456789abcdef' for ch in record['runNonce']) or
            not isinstance(identity, dict) or set(identity) != {'pid', 'start_monotonic_ns', 'instance'} or
            identity != config.get('controllerIdentity') or type(identity['pid']) is not int or
            identity['pid'] != config.get('controllerPID') or identity['pid'] <= 0 or
            type(identity['start_monotonic_ns']) is not int or identity['start_monotonic_ns'] <= 0 or
            type(identity['instance']) is not str or len(identity['instance']) != 32):
        raise LedgerError('current_control_identity_mismatch')
    parent = _stat(run, uid, device)
    if stat.S_IMODE(parent.st_mode) != 0o700 or not stat.S_ISDIR(parent.st_mode) or \
            record['parentMetadata'] != {k: _special_metadata(parent)[k] for k in ('device','inode','uid','gid','mode','nlink')}:
        raise LedgerError('current_control_parent_mismatch')
    return {row['path']: row}


def capture_control_socket(server, config, run, nonce, identity):
    import socket
    run = _path(run)
    if server.family != socket.AF_UNIX or server.type != socket.SOCK_STREAM or \
            server.getsockname() != 'control.sock' or \
            Path.cwd() != run or identity.get('pid') != os.getpid():
        raise LedgerError('current_control_not_own_bind')
    uid = config['expectedUID']
    parent = _anchor(run, uid)
    leaf = os.stat(run / 'control.sock', follow_symlinks=False)
    row = dict(path=str(run / 'control.sock'), root=str(run), kind='unix_socket', **_special_metadata(leaf))
    record = dict(metadata=row, parentMetadata={k: _special_metadata(parent)[k] for k in
                  ('device','inode','uid','gid','mode','nlink')}, runNonce=nonce, controllerIdentity=identity)
    context = dict(config, runDir=str(run), runNonce=nonce, controllerPID=os.getpid(),
                   controllerIdentity=identity, currentControlSocket=record)
    _control_context(context, uid, parent.st_dev)
    _observe_special(row['path'], row, uid, parent.st_dev, {})
    return record


def _entry(key, value):
    # Entry is a frozen data-only dataclass: its generated initializer assigns these
    # six fields without validation. Populate the same dict in C, avoiding six
    # Python calls to object.__setattr__ per inode; the result is still exact Entry.
    entry = Entry.__new__(Entry)
    entry.__dict__.update(device=key[0], inode=key[1], allocated=value[0][5] * 512,
                          size=value[0][4], category=value[1],
                          paths=tuple(value[2]) if len(value[2]) == 1 else tuple(sorted(value[2])))
    return entry


def _observe_path(path, pinned, uid, device, specials, fixture_links, special_parents,
                  special_chains, check=lambda: None, call=None):
    check()
    invoke = call if call is not None else lambda fn, *args: fn(*args)
    if path not in pinned:
        return invoke(_stat, path, uid, device)
    special = specials.get(path)
    if special is not None:
        return _observe_special(path, special, uid, device, special_parents,
                                special_chains[path], check=check, call=call)
    return invoke(_pinned_fixture_link, path, fixture_links[path], uid, device)


def _special_coverage(specials, observations):
    if not set(specials).issubset(observations):
        raise LedgerError('unmeasured_special_metadata')


def _child_visits(path, names, visit):
    # One canonical join for ordinary and controlled traversal, including '/'.
    for name in names:
        yield name, visit(path.rstrip("/") + "/" + name)


# r6 installer phase (operational reinstall/baseline only; declared by the controller): the VZ installer's own expected writes.
_INSTALLER_TREE_LIMIT = 65536  # entries per non-preserved installer framework directory
# r11 continuation dispatch, F2: checked before path storage, descent or descriptor allocation.
_INSTALLER_DEPTH_LIMIT = 256
_INSTALLER_PATH_BYTES_LIMIT = 16 * 2**20
_INSTALLER_FD_LIMIT = _INSTALLER_DEPTH_LIMIT + 3  # parent, root + descendants, one scandir duplicate


def _installer_writers(config, uid, bundle):
    """The exact declared installer writers: only Guest.bundle/Disk.img and Guest.bundle/Auxiliary.bin may ever be declared."""
    writers = config.get("installerWriterPaths", [])
    if not isinstance(writers, (list, tuple)) or any(type(p) is not str for p in writers) or len(set(writers)) != len(writers):
        raise LedgerError("malformed_installer_writers")
    allowed = {str(bundle / "Disk.img"), str(bundle / "Auxiliary.bin")}
    if not set(writers) <= allowed:
        raise LedgerError("installer_writer_not_admitted")
    for path in writers:
        if not stat.S_ISREG(_stat(_path(path), uid).st_mode):
            raise LedgerError("installer_writer_not_regular:" + path)
    return tuple(sorted(writers))


def _discover_installer_frameworks(temp, prefix, preserved_paths, uid, device):
    """_discover for the framework temp root while installer writers are declared: preserved directories are checked exactly as
    before; any other prefixed directory (one the installer creates and removes) that vanished meanwhile is skipped, while one that
    exists must still be a same-device directory of the expected owner."""
    try:
        with os.scandir(temp) as children:
            found = tuple(sorted(str(temp / child.name) for child in children if child.name.startswith(prefix)))
    except OSError as exc:
        raise LedgerError("unreadable_root_inventory:" + str(temp)) from exc
    kept = []
    for path in found:
        if path in preserved_paths:
            value = _stat(path, uid, device)
        else:
            try:
                value = os.stat(path, follow_symlinks=False)
            except FileNotFoundError:
                continue
            except OSError as exc:
                raise LedgerError("missing_or_unreadable:" + path) from exc
            fields = _stamp(value)
            if fields[3] != uid:
                raise LedgerError("wrong_owner:" + path)
            if fields[0] != device:
                raise LedgerError("unadmitted_separate_volume:" + path)
        if not stat.S_ISDIR(value.st_mode):
            raise LedgerError("ambiguous_discovered_root:" + path)
        kept.append(path)
    return tuple(kept)


_DIRECTORY_FLAGS = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW


def _installer_checkpoint(state, work=0):
    control = state['control']
    if control is not None and select.select([control], [], [], 0)[0]:
        raise InterruptedError('collection_interrupted_by_control')
    if work:
        state['completed'] += work
        if state['progress'] is not None:
            state['progress']('validate', state['completed'])


def _installer_fd_check(state):
    _installer_checkpoint(state)
    if state['fds'] >= _INSTALLER_FD_LIMIT or state['baseline_fds'] + state['fds'] >= state['fd_limit']:
        raise LedgerError('installer_tree_fd_limit:' + state['root'])


def _installer_tree(root, uid, device, parent_identity=None, *, control=None, progress=None, strict=False):
    """Bounded no-follow accounting of one non-preserved installer framework directory as it is now. Entries the installer removes
    while this runs are skipped; every existing entry must be same-device and expected-owner; nothing is followed or opened except
    directories for listing. Returns [(path, stamp)] in pre-order (the root first), or [] if the root itself is gone.
    r10 F2: every lstat/open/list is relative to an already verified directory descriptor (single-component, O_NOFOLLOW), each
    opened directory must be the inode just lstat-ed, and after listing the name must still refer to it; a substitution fails closed.
    parent_identity, when given, is the (device, inode) the root's parent must have (the collector's identity-checked anchor).
    Strict retained catalogues also require stable full metadata and a second identical walk; active installers retain their
    disappearance tolerance. All traversals revalidate the configured parent and its no-link ancestor association at the end."""
    parent, name = os.path.split(root.rstrip("/"))
    # FULL5 supplies its existing integer control FD, never the archive Control object.
    state = dict(root=root, control=control, progress=progress, completed=0, entries=1,
                 path_bytes=len(os.fsencode(root)), fds=0, baseline_fds=0, fd_limit=0, strict=strict)
    _installer_checkpoint(state)
    if state['path_bytes'] > _INSTALLER_PATH_BYTES_LIMIT or _INSTALLER_TREE_LIMIT < 1:
        raise LedgerError('installer_tree_limit:' + root)
    try:
        soft, _ = resource.getrlimit(resource.RLIMIT_NOFILE)
        state['fd_limit'] = soft
        # This one bounded inventory counts only this process's already-open descriptors.
        with os.scandir('/dev/fd') as descriptors:
            for _ in descriptors:
                _installer_checkpoint(state, 1)
                state['baseline_fds'] += 1
                if soft != resource.RLIM_INFINITY and state['baseline_fds'] > soft:
                    raise LedgerError('installer_tree_fd_limit:' + root)
        state['baseline_fds'] -= 1  # the inventory's descriptor has closed
        if soft == resource.RLIM_INFINITY:
            state['fd_limit'] = state['baseline_fds'] + _INSTALLER_FD_LIMIT
        try:
            initial_parent = _anchor(Path(parent), uid)
        except LedgerError as exc:
            raise LedgerError('installer_tree_substituted:' + parent) from exc
        _installer_fd_check(state)
    except InterruptedError:
        raise
    except OSError as exc:
        raise LedgerError('installer_tree_unreadable:' + parent) from exc
    try:
        parent_fd = os.open(parent, _DIRECTORY_FLAGS)
    except OSError as exc:
        raise LedgerError("installer_tree_unreadable:" + parent) from exc
    out = []
    state['fds'] += 1
    try:
        initial = _stamp(os.fstat(parent_fd))
        if initial[:4] != _stamp(initial_parent)[:4] or (parent_identity is not None and initial[:2] != tuple(parent_identity)):
            raise LedgerError("installer_tree_substituted:" + parent)
        _installer_walk(parent_fd, name, root, uid, device, out, state, 0)
        if strict and out:
            # A later branch can mutate an earlier directory without changing its ancestors' stamps.
            expected = iter(out)
            state['entries'] = 1
            _installer_walk(parent_fd, name, root, uid, device, None, state, 0, expected)
            if next(expected, None) is not None:
                raise LedgerError('retained_installer_tree_stale:' + root)
        _installer_checkpoint(state)
        try:
            final_parent = _anchor(Path(parent), uid)
        except LedgerError as exc:
            raise LedgerError('installer_tree_substituted:' + parent) from exc
        if _stamp(final_parent)[:4] != initial[:4] or _stamp(os.fstat(parent_fd))[:4] != initial[:4]:
            raise LedgerError('installer_tree_substituted:' + parent)
    finally:
        os.close(parent_fd)
        state['fds'] -= 1
    return out


def _installer_walk(dir_fd, name, path, uid, device, out, state, depth, expected=None):
    _installer_checkpoint(state)
    root, strict = state['root'], state['strict']
    try:
        value = os.stat(name, dir_fd=dir_fd, follow_symlinks=False)
    except FileNotFoundError:
        if strict and (depth or expected is not None):
            raise LedgerError('retained_installer_tree_stale:' + path)
        return
    except OSError as exc:
        raise LedgerError("installer_tree_unreadable:" + path) from exc
    fields = _stamp(value)
    if fields[3] != uid:
        raise LedgerError("wrong_owner:" + path)
    if fields[0] != device:
        raise LedgerError("unadmitted_separate_volume:" + path)
    _installer_checkpoint(state, 1)
    if expected is not None:
        if next(expected, None) != (path, fields):
            raise LedgerError('retained_installer_tree_stale:' + path)
    else:
        out.append((path, fields))
    if not stat.S_ISDIR(fields[2]):
        return
    try:
        _installer_fd_check(state)
        fd = os.open(name, _DIRECTORY_FLAGS, dir_fd=dir_fd)
    except FileNotFoundError:
        if strict:
            raise LedgerError('retained_installer_tree_stale:' + path)
        return
    except InterruptedError:
        raise
    except OSError as exc:  # ELOOP/ENOTDIR: the name now refers to a link or a non-directory
        raise LedgerError("installer_tree_substituted:" + path) from exc
    state['fds'] += 1
    names, name_bytes = [], 0
    try:
        opened = _stamp(os.fstat(fd))
        if opened[:4] != fields[:4]:
            raise LedgerError("installer_tree_substituted:" + path)
        if strict and opened != fields:
            raise LedgerError('retained_installer_tree_stale:' + path)
        try:
            _installer_fd_check(state)
            with os.scandir(fd) as children:
                state['fds'] += 1
                try:
                    prefix_bytes = len(os.fsencode(path.rstrip('/'))) + 1
                    for child in children:
                        _installer_checkpoint(state, 1)
                        size = prefix_bytes + len(os.fsencode(child.name))
                        if (state['entries'] >= _INSTALLER_TREE_LIMIT or depth >= _INSTALLER_DEPTH_LIMIT
                                or state['path_bytes'] + size > _INSTALLER_PATH_BYTES_LIMIT):
                            raise LedgerError('installer_tree_limit:' + root)
                        state['entries'] += 1
                        state['path_bytes'] += size
                        name_bytes += size
                        names.append('/'.join((path.rstrip('/'), child.name)))
                finally:
                    state['fds'] -= 1
        except InterruptedError:
            raise
        except OSError as exc:
            raise LedgerError("installer_tree_unreadable:" + path) from exc
        names.sort()
        for child in names:
            _installer_walk(fd, os.path.basename(child), child, uid, device, out, state, depth + 1, expected)
        try:
            _installer_checkpoint(state)
            after = os.stat(name, dir_fd=dir_fd, follow_symlinks=False)
        except FileNotFoundError:
            if strict:
                raise LedgerError('retained_installer_tree_stale:' + path)
            return
        except InterruptedError:
            raise
        except OSError as exc:
            raise LedgerError("installer_tree_unreadable:" + path) from exc
        if _stamp(after)[:4] != fields[:4]:
            raise LedgerError("installer_tree_substituted:" + path)
        if strict and (_stamp(after) != fields or _stamp(os.fstat(fd)) != fields):
            raise LedgerError('retained_installer_tree_stale:' + path)
        _installer_checkpoint(state, 1)
    finally:
        os.close(fd)
        state['fds'] -= 1
        if expected is not None:
            state['path_bytes'] -= name_bytes


def _retained_installer_trees(config, temp, temp_identity, framework_prefix, uid, device, *, control=None, progress=None):
    """r10 F3: the owner-selected retained installer tree left under the framework temp root is charged as scratch. It must exist
    exactly as catalogued (descriptor-relative no-follow walk; per entry: relative path, inode, mode, size, mtime). A missing, stale,
    replaced or extended tree fails closed. Nothing is followed and no content is read. Returns its [(path, stamp)] rows."""
    trees = config.get("retainedInstallerTrees", [])
    if not isinstance(trees, (list, tuple)) or len(trees) > 1:
        raise LedgerError("malformed_retained_installer_trees")
    out = []
    for tree in trees:
        if (not isinstance(tree, dict) or set(tree) != {"name", "catalogueSHA256", "entries"} or type(tree["name"]) is not str
                or not tree["name"] or "/" in tree["name"] or tree["name"] in (".", "..") or tree["name"].startswith(framework_prefix)
                or not isinstance(tree["entries"], (list, tuple)) or not tree["entries"]
                or any(not isinstance(e, (list, tuple)) or len(e) != 5 or type(e[0]) is not str
                       or any(type(v) is not int for v in e[1:]) for e in tree["entries"])):
            raise LedgerError("malformed_retained_installer_tree")
        root = str(temp / tree["name"])
        rows = _installer_tree(root, uid, device, temp_identity, control=control, progress=progress, strict=True)
        if not rows:
            raise LedgerError("retained_installer_tree_missing:" + root)
        observed = sorted((os.path.relpath(path, root), f[1], f[2], f[4], f[6]) for path, f in rows)
        if observed != sorted(map(tuple, tree["entries"])):
            raise LedgerError("retained_installer_tree_stale:" + root)
        out.extend(rows)
    return out


def _validate_observation(path, initial, current, known, seen, image_key, disk_key, installer=()):
    if path in installer:
        # r6: a declared installer writer keeps its exact identity/type/owner; allocation and size are max(first, last).
        # The installation image is never an installer writer.
        if current[:4] != initial[:4]:
            raise LedgerError("installer_writer_identity_changed:" + path)
        key = current[:2]
        if key == image_key:
            raise LedgerError("installer_writer_is_image")
        old = seen[key][0]
        seen[key][0] = (*current[:4], max(old[4], current[4]), max(old[5], current[5]),
                        current[6], current[7])
    elif path in known:
        if current[:4] != initial[:4]:
            raise LedgerError("known_writer_identity_changed:" + path)
        key = current[:2]
        if key in (image_key, disk_key):
            raise LedgerError("known_writer_is_vm_asset")
        old = seen[key][0]
        seen[key][0] = (*current[:4], max(old[4], current[4]), max(old[5], current[5]),
                        current[6], current[7])
    else:
        # Both collection modes use the archive function's second-pass mutation seam.
        for _ in _collect_archive(None, None, None, None, validation=(path, initial, current)):
            pass


def collect(config, clock=time.monotonic, control=None, progress=None):
    """No content reads, cleanup, native calls or cached allocations.

    Unknown metadata and inventories must agree across the scan. Known writer
    identity/type/mode/owner must agree; allocation is max(first,last), never zero.
    This detects observed races, not writes after validation or an atomic quota.
    Optional progress(phase, completed) is called on this thread with strictly increasing genuine
    work counts between native passes; it services liveness only and may raise to abort the scan.
    """
    global _LATEST
    _LATEST = None  # never keep an earlier snapshot's fingerprint alive into this scan
    # Cyclic GC is paused only while the scan builds its acyclic tuples; refcounting still frees.
    enabled = gc.isenabled()
    gc.disable()
    try:
        return _collect(config, clock, control, progress)
    finally:
        if enabled:
            gc.enable()


def _collect(config, clock, control=None, progress=None):
    started = clock()
    required = ("ownedRoots", "imagePath", "bundlePath", "frameworkTempRoot",
                "frameworkPrefix", "preservedFrameworkNames", "expectedUID",
                "relatedRootParent", "relatedRootPrefix", "relatedRootAdditionalNames",
                "knownWriterPaths", "fixtureSymlinkRecords", "evidence")
    if any(key not in config for key in required):
        raise LedgerError("missing_ledger_config")
    uid = config["expectedUID"]
    if type(uid) is not int or uid < 0:
        raise LedgerError("invalid_uid")
    owned = config["ownedRoots"]
    if not isinstance(owned, dict) or not {"lab", "r4", "r5", "r6", "r7"}.issubset(owned):
        raise LedgerError("missing_or_ambiguous_owned_roots")
    roots = tuple(sorted(set(map(_path, owned.values()))))
    image = _path(config["imagePath"])
    disk = _path(config["bundlePath"]) / "Disk.img"
    parent, temp = _path(config["relatedRootParent"]), _path(config["frameworkTempRoot"])
    prefix, framework_prefix = config["relatedRootPrefix"], config["frameworkPrefix"]
    for value in (prefix, framework_prefix):
        if type(value) is not str or not value or "/" in value:
            raise LedgerError("invalid_discovery_prefix")
    additional, preserved = config["relatedRootAdditionalNames"], config["preservedFrameworkNames"]
    for values in (additional, preserved):
        if not isinstance(values, (list, tuple)) or any(type(v) is not str or not v or
                "/" in v or v in (".", "..") for v in values) or len(values) != len(set(values)):
            raise LedgerError("malformed_discovery_names")
    if not preserved or any(not name.startswith(framework_prefix) for name in preserved):
        raise LedgerError("missing_or_invalid_preserved_framework_directory")
    writers = config["knownWriterPaths"]
    if not isinstance(writers, (list, tuple)):
        raise LedgerError("malformed_known_writers")
    known = tuple(sorted(set(map(str, map(_path, writers)))))
    if len(known) != len(writers):
        raise LedgerError("duplicate_known_writer")
    if any(not any(p.is_relative_to(root) for root in roots) for p in (image, disk)):
        raise LedgerError("image_or_disk_outside_owned_roots")
    anchors = {str(p): _anchor(p, uid) for p in (*roots, parent, temp)}
    image_stat, disk_stat = _stat(image, uid), _stat(disk, uid)
    if not all(stat.S_ISREG(s.st_mode) for s in (image_stat, disk_stat)):
        raise LedgerError("image_or_disk_not_regular")
    device = image_stat.st_dev
    if any(s.st_dev != device for s in (*anchors.values(), disk_stat)):
        raise LedgerError("unadmitted_separate_volume")
    image_key, disk_key = (device, image_stat.st_ino), (device, disk_stat.st_ino)
    if image_key == disk_key:
        raise LedgerError("ambiguous_image_disk_inode")
    fixture_links = _fixture_records(config, uid, device)
    related = _discover(parent, prefix, additional, uid, device)
    preserved_paths = tuple(sorted(str(temp / name) for name in preserved))
    # r6: declared installer writers (operational reinstall/baseline only) enable installer-phase accounting; otherwise unchanged.
    installer = _installer_writers(config, uid, _path(config["bundlePath"]))
    if installer:
        frameworks = _discover_installer_frameworks(temp, framework_prefix, preserved_paths, uid, device)
        strict_frameworks = tuple(p for p in frameworks if p in preserved_paths)
        active_frameworks = tuple(p for p in frameworks if p not in preserved_paths)
    else:
        frameworks = _discover(temp, framework_prefix, (), uid, device)
        strict_frameworks, active_frameworks = frameworks, ()
    if not set(preserved_paths).issubset(frameworks):
        raise LedgerError("missing_preserved_framework_directory")
    all_roots = tuple(sorted(set(map(str, roots)) | set(related)))
    for path in related:
        anchors[path] = _anchor(Path(path), uid)
    # Path(p).is_relative_to(Path(root)) for canonical paths: some parts prefix of p is a root's parts.
    root_parts = frozenset(Path(root).parts for root in all_roots)
    if any(not any(parts[:k] in root_parts for k in range(1, len(parts) + 1))
           for parts in (Path(p).parts for p in known)):
        raise LedgerError("known_writer_outside_owned_roots")
    for path in known:
        if not stat.S_ISREG(_stat(path, uid, device).st_mode):
            raise LedgerError("known_writer_not_regular:" + path)
        if path in (str(image), str(disk)):
            raise LedgerError("known_writer_is_vm_asset")
    specials = _special_records(config, uid, device, related)
    current_control = _control_context(config, uid, device)
    if set(specials) & (set(fixture_links) | set(current_control)) or set(fixture_links) & set(current_control):
        raise LedgerError('ambiguous_special_admission')
    specials.update(current_control)
    # A root's parts are a prefix of `parts` exactly when some parts[:k] (k <= len(parts)) is a root's parts.
    scope_roots = frozenset(Path(root).parts for root in all_roots)
    special_chains, root_parts = {}, {}
    for path, row in specials.items():
        parts = _path(path).parts
        if not any(parts[:k] in scope_roots for k in range(1, len(parts) + 1)):
            raise LedgerError('special_metadata_outside_owned_roots')
        # _special_parent_chain(path, row) from the parts parsed above (each root string parsed once).
        text = row['root']
        rp = root_parts.get(text) if type(text) is str else None
        if rp is None:
            rp = _path(text).parts
            if type(text) is str:
                root_parts[text] = rp
        special_chains[path] = _chain(parts[:-1] if len(parts) > 1 else parts, rp)
    observations, seen, special_parents = {}, {}, {}

    pinned = frozenset(specials) | frozenset(fixture_links)  # disjoint, checked above

    def observe(path):
        return _observe_path(path, pinned, uid, device, specials, fixture_links,
                             special_parents, special_chains)

    # Native first pass (_metawalk.c): same pre-order coverage and _stat checks/messages. Roots and
    # known writers use lstat; other entries use their parent's no-follow bulk enumeration, and the
    # second pass below requires each to equal a fresh lstat, so accounting stays exact lstat values.
    # Pinned (opaque special / fixture-link) paths come back as None; their unchanged observation is below;
    # pinned records are never directories (validated mode), so the walk never descends them.
    # One call covers every root in this order; a root (or child) already covered by an earlier
    # root's tree is skipped exactly like live_ledger.visit's `path in observations`.
    exact = frozenset(known) | frozenset(installer)
    writers = tuple(sorted(set(known) | set(installer)))  # excluded from settlement metadata/fingerprint; rebuilt after pass 2
    # r4: the pass-1 observation of every pinned path runs here, before the scan workers start, so this
    # thread makes no file-system call while they walk (at most JOBS threads ever do). Each result, a
    # stamp or that path's own LedgerError, is used or raised only when consume() reaches its row, so
    # recorded order and the first reported fault are unchanged. A pinned path the walk never reaches
    # stays unrecorded and fails _special_coverage / unmeasured_fixture_symlink exactly as before.
    preobserved = {}
    for path in sorted(pinned):
        try:
            preobserved[path] = _stamp(observe(path))
        except LedgerError as exc:
            preobserved[path] = exc
    index, consumed = 0, None

    def consume(rows):
        # Pass-1 record step for every row emitted so far, resuming at `index`. r4: _scan calls this
        # while its workers are still walking (instead of idling), then once more below; rows only
        # ever grow at the end, so each row is recorded exactly once, in row order. No syscalls.
        # Every call must see the one list _scan builds and returns; any other list fails closed.
        nonlocal index, consumed
        if consumed is None:
            consumed = rows
        elif rows is not consumed:
            raise LedgerError("pass1_rows_mismatch")
        # _record performs exactly the per-row record step below (same order, checks, message and progress
        # calls) for each run of ordinary rows, stopping at the first other row, which is handled here.
        while (index := _record(rows, index, observations, seen, image_key, disk_key, LedgerError, progress)) < len(rows):
            path, stamp = rows[index]
            index += 1
            if stamp is None:
                stamp = preobserved.pop(path, None)  # only pinned paths come back as None
                if stamp is None:
                    raise LedgerError("unobserved_pinned_path:" + path)
                if isinstance(stamp, LedgerError):
                    raise stamp
            elif isinstance(stamp, LedgerError):
                raise stamp  # the walk ended at this path's own fault, in visit order
            # Record the observation (inlined per-path body: same order, checks and message).
            observations[path] = stamp
            key = stamp[:2]
            known_entry = seen.get(key)
            if known_entry is None:
                seen[key] = [stamp, "image" if key == image_key else "disk" if key == disk_key else "scratch", [path]]
            elif known_entry[0] != stamp:
                raise LedgerError("ambiguous_inode_measurement:" + path)
            else:
                known_entry[2].append(path)
            if progress is not None and not len(observations) & 4095:
                progress("collect", len(observations))

    rows = _scan((*all_roots, *strict_frameworks), pinned, exact, uid, device, LedgerError, control, JOBS, consume)
    consume(rows)
    rows = None
    if progress is not None:
        progress("collect", len(observations))  # pass 1 complete; repeats of a count are not new work
    if image_key not in seen or disk_key not in seen:
        raise LedgerError("missing_image_or_disk_measurement")
    if not set(fixture_links).issubset(observations):
        raise LedgerError("unmeasured_fixture_symlink")
    _special_coverage(specials, observations)
    # Fresh second pass starts only after the complete first pass: one new lstat per path, in
    # observation order. A path's own fault ends the list there and is raised at that position.
    # Given the pass-1 stamps, _restat returns a checked fresh stamp that EQUALS its pass-1 stamp as
    # that same object; for an unknown path _validate_observation is then a no-op, so it is skipped.
    # Any unequal stamp, every known writer and every pinned row still go through the checks below.
    # Bounded batches keep the native encoded-path/stat/result buffers small instead of inventory-sized;
    # each batch is validated, in order, before the next is restatted (the first fault still ends the pass).
    # Pass 2 can only confirm or fault the pass-1 stamps, except that it updates seen[key] of known-writer
    # paths; the snapshot's pure-CPU pieces are built on this thread while the pass-2 workers lstat (no
    # syscalls), and only those known-writer entries are rebuilt afterwards.
    parts = {}
    work = _snapshot_parts(seen, observations, specials, writers, parts)
    step = lambda: next(work, False)
    index, size = len(observations), _RESTAT_BATCH
    all_paths, all_initials = tuple(observations), tuple(observations.values())
    for start in range(0, len(all_paths), size):
        paths, initials = all_paths[start:start + size], all_initials[start:start + size]
        if len(paths) >= _OVERLAP_MIN:
            currents = _restat(paths, pinned, uid, device, LedgerError, control, JOBS, initials, step)
        else:
            currents = _restat(paths, pinned, uid, device, LedgerError, control, JOBS, initials)
        if len(currents) == len(paths) and exact.isdisjoint(paths):
            # No known writer here: the per-path loop below only `continue`s where the fresh stamp IS its
            # pass-1 object, so visit just the other positions, with its progress calls in the same order.
            reached = index
            for j in compress(range(len(paths)), map(is_not, currents, initials)):
                if progress is not None:
                    for done in _marks(reached, index + j + 1 - reached):
                        progress("validate", done)
                reached = index + j + 1
                path, initial, current = paths[j], initials[j], currents[j]
                if current is None:
                    current = _stamp(observe(path))
                elif isinstance(current, LedgerError):
                    raise current
                _validate_observation(path, initial, current, known, seen, image_key, disk_key, installer)
            if progress is not None:
                for done in _marks(reached, index + len(paths) - reached):
                    progress("validate", done)
            index += len(paths)
            continue
        for (path, initial), current in zip(zip(paths, initials), currents):
            index += 1
            if progress is not None and not index & 4095:
                progress("validate", index)
            if current is initial and path not in exact:
                continue
            if current is None:
                current = _stamp(observe(path))
            elif isinstance(current, LedgerError):
                raise current
            _validate_observation(path, initial, current, known, seen, image_key, disk_key, installer)
        if len(currents) != len(paths):
            raise LedgerError("incomplete_second_pass")
    if progress is not None:
        progress("validate", 2 * len(observations) + 1)
    if _discover(parent, prefix, additional, uid, device) != related:
        raise LedgerError("stale_related_root_inventory")
    if installer:
        # Preserved directories must remain exactly; installer directories may appear (measured by the next scan) or vanish.
        if not set(strict_frameworks).issubset(_discover_installer_frameworks(temp, framework_prefix, preserved_paths, uid, device)):
            raise LedgerError("stale_framework_inventory")
    elif _discover(temp, framework_prefix, (), uid, device) != frameworks:
        raise LedgerError("stale_framework_inventory")
    # Ancestor replacement/redirection is checked again after all leaf validation.
    for path, initial in anchors.items():
        current = _anchor(Path(path), uid)
        if _stamp(current)[:4] != _stamp(initial)[:4]:
            raise LedgerError("stale_anchor:" + path)
    _control_context(config, uid, device)
    for path, identity in special_parents.items():
        if _stamp(_stat(path, uid, device))[:4] != identity:
            raise LedgerError('special_parent_changed:' + path)
    try:
        space = os.statvfs(image)
        free = space.f_bavail * space.f_frsize
    except OSError as exc:
        raise LedgerError("unreadable_free_space") from exc
    installer_rows = {}
    temp_identity = _stamp(anchors[str(temp)])[:2]
    installer_completed = 2 * len(observations) + 1

    def installer_progress(phase, completed):
        nonlocal installer_completed
        installer_completed += 1
        if progress is not None:
            progress(phase, installer_completed)

    for root in active_frameworks:
        for path, fields in _installer_tree(root, uid, device, temp_identity, control=control, progress=installer_progress):
            installer_rows.setdefault(root, []).append((path, fields))
    retained_rows = _retained_installer_trees(config, temp, temp_identity, framework_prefix, uid, device,
                                               control=control, progress=installer_progress)
    for _ in work:  # any steps not overlapped (small batches, or workers finished first)
        pass
    if writers:
        # Exactly the after-pass-2 _entry() of each writer inode (its seen value may have been updated).
        entries, index, exact_known = list(parts["entries"]), parts["index"], frozenset(writers)
        stale = False
        for key in sorted({observations[path][:2] for path in writers if path in observations}):
            fresh = _entry(key, seen[key])
            if fresh != entries[index[key]]:
                entries[index[key]] = fresh
                stale = stale or not exact_known.issuperset(fresh.paths)  # it is part of the fingerprint
        parts["entries"] = tuple(entries)
        if stale:  # the property / settlement_digest then compute both from the final entries
            parts["fingerprint"] = parts["digest"] = None
    measured_frameworks = strict_frameworks
    if installer_rows or retained_rows:
        # Installer directories: allocation accounted as scratch, writer-like (outside the settlement fingerprint). An inode already
        # measured elsewhere is not counted twice; one inode seen at several installer paths becomes one entry.
        found = {}
        for root, rows_ in installer_rows.items():
            for path, fields in rows_:
                key = fields[:2]
                if key in seen:
                    continue
                if key in (image_key, disk_key):
                    raise LedgerError("installer_tree_vm_asset:" + path)
                found.setdefault(key, [fields, "scratch", []])[2].append(path)
        for path, fields in retained_rows:  # r10 F3: same deduplication; an image/disk alias is refused, never deduplicated away
            key = fields[:2]
            if key in (image_key, disk_key):
                raise LedgerError("retained_installer_tree_vm_asset:" + path)
            if key in seen:
                continue
            found.setdefault(key, [fields, "scratch", []])[2].append(path)
        added = [_entry(key, value) for key, value in found.items()]
        merged = sorted((*parts["entries"], *added), key=lambda e: (e.device, e.inode))
        parts["entries"] = tuple(merged)
        measured_frameworks = tuple(sorted((*strict_frameworks, *installer_rows)))
    snapshot = Snapshot(started, clock(), device, free, parts["entries"], all_roots, all_roots,
                        measured_frameworks, measured_frameworks, preserved_paths, str(parent), prefix,
                        tuple(additional), related, known, parts["metadata"])
    global _LATEST
    if parts["fingerprint"] is not None:
        _LATEST = (weakref.ref(snapshot), parts["fingerprint"], parts["digest"])
    return snapshot


def _marks(index, count):
    """The per-path loop's progress counts for paths index+1 .. index+count: each multiple of 4096."""
    return range((index // 4096 + 1) * 4096, index + count + 1, 4096)


# Batches smaller than this restat too quickly for overlapped work to pay for the callbacks.
_OVERLAP_MIN = 1024
# (weakref to the latest collect() snapshot, its settlement fingerprint, that fingerprint's digest or None).
# Nothing is stored on the snapshot itself; cleared when the next collect() starts.
_LATEST = None


def _snapshot_parts(seen, observations, specials, known, out):
    """Snapshot pieces from the COMPLETE pass 1, one bounded step per yield; pure CPU, no syscalls.

    Exactly the after-pass-2 expressions (and settlement_fingerprint's), on data pass 2 never changes:
    only known-writer keys of `seen` can be updated there, and _collect rebuilds those entries (out
    "index" locates them). The snapshot is still assembled only after every pass-2 and post check.
    """
    known = frozenset(known)
    items = sorted(seen.items(), key=_KEY)
    known_keys = {observations[path][:2] for path in known if path in observations}
    out["index"] = {key: i for i, (key, _) in enumerate(items) if key in known_keys} if known_keys else {}
    yield True
    # r5: _parts_entries / _parts_metadata perform exactly the Python steps below, in the same order, for each
    # run of exactly-shaped items, returning where they stopped; any other item takes the Python step here.
    entries, allocations = [], []
    for start in range(0, len(items), 4096):
        stop = min(start + 4096, len(items))
        index = start
        while (index := _parts_entries(items, index, stop, Entry, known, entries, allocations)) < stop:
            e = _entry(*items[index])
            entries.append(e)
            if not known.issuperset(e.paths):
                allocations.append((e.device, e.inode, e.allocated, e.size, e.category))
            index += 1
        yield True
    entries = tuple(entries)
    items = None
    # Unique keys: sorting the paths orders the (path, stamp) items identically.
    keys = sorted(observations)
    yield True
    metadata = []
    for start in range(0, len(keys), 4096):
        stop = min(start + 4096, len(keys))
        index = start
        while (index := _parts_metadata(keys, index, stop, observations, known, "metadata", metadata)) < stop:
            path = keys[index]
            if path not in known:
                metadata.append(("metadata", path) + observations[path])
            index += 1
        yield True
    keys = None
    metadata = tuple(metadata) + tuple(('special', path, *tuple(row[k] for k in _SPECIAL_NUMBERS),
                                        row['kind'], row.get('target')) for path, row in sorted(specials.items()))
    yield True
    fingerprint = tuple(sorted(allocations)) + metadata
    allocations = None
    yield True
    state = json_sha256_begin()
    for start in range(0, len(fingerprint), 4096):
        try:
            json_sha256_feed(state, fingerprint, start, start + 4096)
        except TypeError:  # outside the streamed types: settlement_digest's json fallback decides later
            state = None
            break
        yield True
    out.update(entries=entries, metadata=metadata, fingerprint=fingerprint,
               digest=None if state is None else json_sha256_end(state))


@dataclass(frozen=True)
class ArchiveSnapshot(Snapshot):
    archive_catalog_digest: str
    archive_absent_roots: tuple
    archive_historical_roots: tuple
    archive_capsule_paths: tuple
    archive_binding: object

def _collect_archive(config, clock, binding, control, *, validation=None):
    if validation is not None:
        path, initial, current = validation
        if current == initial:
            return
        elif current != initial:
            error = LedgerError("stale_metadata:" + path)
            error.stale_metadata_diagnostics = (path, _STAMP_FIELDS, initial, current,
                tuple(name for name, before, after in zip(_STAMP_FIELDS, initial, current) if before != after))
            raise error
        return
    started = control.started
    from collection_control import discover
    def checkpoint():
        control.check()
    def checked(fn):
        def invoke(*args,**kwargs):return control.call(fn,*args,**kwargs)
        return invoke
    # Only this cursor wraps operations; inherited collect/_stat hooks remain unchanged.
    _stat=checked(globals()['_stat']);_anchor=checked(globals()['_anchor'])

    required = ("ownedRoots", "imagePath", "bundlePath", "frameworkTempRoot",
                "frameworkPrefix", "preservedFrameworkNames", "expectedUID",
                "relatedRootParent", "relatedRootPrefix", "relatedRootAdditionalNames",
                "knownWriterPaths", "fixtureSymlinkRecords", "evidence")
    if any(key not in config for key in required):
        raise LedgerError("missing_ledger_config")
    uid = config["expectedUID"]
    if type(uid) is not int or uid < 0:
        raise LedgerError("invalid_uid")
    owned = config["ownedRoots"]
    if not isinstance(owned, dict) or not {"lab", "r4", "r5", "r6", "r7"}.issubset(owned):
        raise LedgerError("missing_or_ambiguous_owned_roots")
    roots = tuple(sorted(set(map(_path, owned.values())) - set(map(Path,binding.absent)) | {Path(binding.directory)}))
    image = _path(config["imagePath"])
    disk = _path(config["bundlePath"]) / "Disk.img"
    parent, temp = _path(config["relatedRootParent"]), _path(config["frameworkTempRoot"])
    prefix, framework_prefix = config["relatedRootPrefix"], config["frameworkPrefix"]
    for value in (prefix, framework_prefix):
        if type(value) is not str or not value or "/" in value:
            raise LedgerError("invalid_discovery_prefix")
    additional, preserved = config["relatedRootAdditionalNames"], config["preservedFrameworkNames"]
    for values in (additional, preserved):
        if not isinstance(values, (list, tuple)) or any(type(v) is not str or not v or
                "/" in v or v in (".", "..") for v in values) or len(values) != len(set(values)):
            raise LedgerError("malformed_discovery_names")
    if not preserved or any(not name.startswith(framework_prefix) for name in preserved):
        raise LedgerError("missing_or_invalid_preserved_framework_directory")
    writers = config["knownWriterPaths"]
    if not isinstance(writers, (list, tuple)):
        raise LedgerError("malformed_known_writers")
    known = tuple(sorted(set(map(str, map(_path, writers)))))
    if len(known) != len(writers):
        raise LedgerError("duplicate_known_writer")
    if any(not any(p.is_relative_to(root) for root in roots) for p in (image, disk)):
        raise LedgerError("image_or_disk_outside_owned_roots")
    anchors = {str(p): _anchor(p, uid) for p in (*roots, parent, temp)}
    image_stat, disk_stat = _stat(image, uid), _stat(disk, uid)
    if not all(stat.S_ISREG(s.st_mode) for s in (image_stat, disk_stat)):
        raise LedgerError("image_or_disk_not_regular")
    device = image_stat.st_dev
    if any(s.st_dev != device for s in (*anchors.values(), disk_stat)):
        raise LedgerError("unadmitted_separate_volume")
    image_key, disk_key = (device, image_stat.st_ino), (device, disk_stat.st_ino)
    if image_key == disk_key:
        raise LedgerError("ambiguous_image_disk_inode")
    fixture_links = _fixture_records(config, uid, device)
    related = discover(parent, prefix, additional, uid, device, binding, control)
    frameworks = discover(temp, framework_prefix, (), uid, device, binding, control)
    preserved_paths = tuple(sorted(str(temp / name) for name in preserved))
    if not set(preserved_paths).issubset(frameworks):
        raise LedgerError("missing_preserved_framework_directory")
    all_roots = tuple(sorted(set(map(str, roots)) | set(related)))
    for path in related:
        anchors[path] = _anchor(Path(path), uid)
    if any(not any(Path(p).is_relative_to(Path(root)) for root in all_roots) for p in known):
        raise LedgerError("known_writer_outside_owned_roots")
    for path in known:
        if not stat.S_ISREG(_stat(path, uid, device).st_mode):
            raise LedgerError("known_writer_not_regular:" + path)
        if path in (str(image), str(disk)):
            raise LedgerError("known_writer_is_vm_asset")
    specials = _special_records(config, uid, device, related, binding.absent, check=checkpoint)
    current_control = control.call(_control_context,config, uid, device)
    if set(specials) & (set(fixture_links) | set(current_control)) or set(fixture_links) & set(current_control):
        raise LedgerError('ambiguous_special_admission')
    specials.update(current_control)
    scope_roots = tuple(Path(root).parts for root in all_roots)
    special_chains = {}
    for path, row in specials.items():
        parts = _path(path).parts
        if not any(parts[:len(root)] == root for root in scope_roots):
            raise LedgerError('special_metadata_outside_owned_roots')
        special_chains[path] = _special_parent_chain(path, row)
    observations, seen, special_parents = {}, {}, {}

    pinned = frozenset(specials) | frozenset(fixture_links)  # disjoint, checked above

    def observe(path):
        return _observe_path(path, pinned, uid, device, specials, fixture_links,
                             special_parents, special_chains, checkpoint, control.call)

    def visit(path):
        if path in observations:
            return
        yield 'collect'
        value = observe(path)
        stamp = _stamp(value)
        control.insert(path, min(len(Path(r).parts) for r in (*all_roots,*frameworks) if Path(path).is_relative_to(r)), len(seen)+int(stamp[:2] not in seen))
        observations[path] = stamp
        key = stamp[:2]
        known_entry = seen.get(key)
        if known_entry is None:
            seen[key] = [stamp, "image" if key == image_key else "disk" if key == disk_key else "scratch", [path]]
        elif known_entry[0] != stamp:
            raise LedgerError("ambiguous_inode_measurement:" + path)
        else:
            known_entry[2].append(path)
        if stat.S_ISDIR(stamp[2]):
            try:
                with os.scandir(path) as children:
                    names = []
                    name_bytes=0
                    for child in children:
                        checkpoint()
                        if len(names)>=control.caps['livePaths']:
                            raise LedgerError('controlled_live_limit')
                        name_bytes+=len(os.fsencode(child.name))
                        if name_bytes>control.caps['pathBytes']:
                            raise LedgerError('controlled_live_limit')
                        control.pending_names+=1;control.pending_bytes+=len(os.fsencode(child.name))
                        if control.pending_names>control.caps['livePaths'] or control.pending_bytes>control.caps['pathBytes']:
                            raise LedgerError('controlled_live_limit')
                        names.append(child.name)
                        control.check(work=1)
                        yield 'collect'
            except OSError as exc:
                raise LedgerError("unreadable_directory:" + path) from exc
            for name, child in _child_visits(path, names, visit):
                control.pending_names-=1;control.pending_bytes-=len(os.fsencode(name))
                yield from child

    for root in (*all_roots, *frameworks):
        yield from visit(root)
    if image_key not in seen or disk_key not in seen:
        raise LedgerError("missing_image_or_disk_measurement")
    if not set(fixture_links).issubset(observations):
        raise LedgerError("unmeasured_fixture_symlink")
    _special_coverage(specials, observations)
    control.check('validate')
    for path, initial in observations.items():
        yield 'validate'
        current = _stamp(observe(path))
        _validate_observation(path, initial, current, known, seen, image_key, disk_key)
    if discover(parent, prefix, additional, uid, device, binding, control) != related:
        raise LedgerError("stale_related_root_inventory")
    if discover(temp, framework_prefix, (), uid, device, binding, control) != frameworks:
        raise LedgerError("stale_framework_inventory")
    # Ancestor replacement/redirection is checked again after all leaf validation.
    for path, initial in anchors.items():
        yield 'validate'
        current = _anchor(Path(path), uid)
        if _stamp(current)[:4] != _stamp(initial)[:4]:
            raise LedgerError("stale_anchor:" + path)
    control.call(_control_context,config, uid, device)
    for path, identity in special_parents.items():
        yield 'validate'
        if _stamp(_stat(path, uid, device))[:4] != identity:
            raise LedgerError('special_parent_changed:' + path)
    try:
        space = control.call(os.statvfs,image)
        free = space.f_bavail * space.f_frsize
    except OSError as exc:
        raise LedgerError("unreadable_free_space") from exc
    archive_names={path for path,_ in binding.stamps}
    allocated=0
    for key,value in seen.items():
        in_archive=set(value[2]) & archive_names
        if in_archive:
            if len(in_archive)!=1 or set(value[2])!=in_archive:
                raise LedgerError('archive_cross_boundary_alias')
            if value[1]!='scratch':raise LedgerError('archive_asset_alias')
            allocated+=value[0][5]*512
        yield 'validate'
    if allocated>control.caps['archiveAllocatedBytes']:
        raise LedgerError('archive_allocation_limit')
    binding.check(config,control=control)
    entries_list=[]
    for key,value in sorted(seen.items()):
        checkpoint();entries_list.append(_entry(key,value));control.check(work=1)
    entries=tuple(entries_list)
    snapshot = Snapshot(started, clock(), device, free, entries, all_roots, all_roots,
                    frameworks, frameworks, preserved_paths, str(parent), prefix,
                    tuple(additional), related, known,
                    tuple(("metadata", path) + stamp for path, stamp in sorted(observations.items())
                          if path not in known) +
                    tuple(('special', path, *tuple(row[k] for k in _SPECIAL_NUMBERS),
                           row['kind'], row.get('target')) for path, row in sorted(specials.items())))

    return ArchiveSnapshot(**snapshot.__dict__,archive_catalog_digest=binding.catalog_digest,
                           archive_absent_roots=binding.absent,archive_historical_roots=binding.historical_roots,
                           archive_capsule_paths=binding.capsules,archive_binding=binding)
