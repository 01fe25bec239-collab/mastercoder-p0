# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Historical catalog data. No extraction, endpoint access or operational migration."""
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import stat
from resource_ledger import LedgerError

SCHEMA = 'r37-fixture-archive-catalog-v1'
METADATA = ('device','inode','mode','uid','gid','nlink','size','blocks','mtimeNs','ctimeNs')
KINDS = {'regular':stat.S_IFREG,'directory':stat.S_IFDIR,'symlink':stat.S_IFLNK,
         'socket':stat.S_IFSOCK,'fifo':stat.S_IFIFO,'character':stat.S_IFCHR,'block':stat.S_IFBLK}


def fail(reason):
    raise LedgerError('archive_' + reason)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def encode(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=True, separators=(',',':'),
                      allow_nan=False).encode('ascii')


def pin(value):
    if type(value) is not str or len(value)!=64 or value.strip('0123456789abcdef'):
        fail('invalid_digest')
    return value


def canonical(value):
    if type(value) is not str or not value or '\0' in value or not value.startswith('/') or str(Path(os.path.normpath(value)))!=value:
        fail('noncanonical_path')
    return Path(value)


def member_name(value):
    if type(value) is not str or not value or '\0' in value or '\\' in value:
        fail('unsafe_member')
    p=PurePosixPath(value)
    if p.is_absolute() or str(p)!=value or any(x in ('','.','..') for x in value.split('/')):
        fail('unsafe_member')
    return value


def metadata(s):
    return dict(zip(METADATA,(s.st_dev,s.st_ino,s.st_mode,s.st_uid,s.st_gid,s.st_nlink,
                              s.st_size,s.st_blocks,s.st_mtime_ns,s.st_ctime_ns)))


def raw_metadata(row, uid, device):
    if type(row) is not dict or set(row)!=set(METADATA) or any(type(row[k]) is not int for k in METADATA):
        fail('metadata_shape')
    if row['uid']!=uid or row['device']!=device or row['inode']<=0 or row['nlink']<=0 or min(row[k] for k in ('device','uid','gid','size','blocks'))<0:
        fail('metadata_identity')
    if stat.S_IFMT(row['mode']) not in KINDS.values() or row['mode']<0:
        fail('metadata_type')


def no_duplicates(pairs):
    result={}
    for key,value in pairs:
        if key in result:fail('duplicate_key')
        result[key]=value
    return result


def parse(raw, *, uid, device, limits):
    if len(raw)>limits['catalogBytes']:fail('catalog_limit')
    try:value=json.loads(raw,object_pairs_hook=no_duplicates)
    except LedgerError:
        raise
    except (ValueError,UnicodeDecodeError,RecursionError) as exc:fail('catalog_encoding')
    if type(value) is not dict or set(value)!={'schema','roots','records','capsules','provenance'} or value['schema']!=SCHEMA:
        fail('catalog_shape')
    roots=value['roots']
    if type(roots) is not list or not roots or any(type(x) is not str for x in roots) or len(roots)!=len(set(roots)):
        fail('catalog_roots')
    roots=[canonical(x) for x in roots]
    if any(a!=b and a.is_relative_to(b) for a in roots for b in roots):fail('overlapping_roots')
    provenance=value['provenance']
    if type(provenance) is not dict or not provenance:fail('provenance')
    for key,digest in provenance.items():
        canonical(key);pin(digest)
    caps=value['capsules']
    if type(caps) is not list or not 1<=len(caps)<=limits['capsules']:fail('capsule_count')
    capnames=set()
    for cap in caps:
        if type(cap) is not dict or set(cap)!={'name','bytes','sha256'}:fail('capsule_shape')
        name=member_name(cap['name'])
        if '/' in name or name in capnames:fail('capsule_name')
        capnames.add(name);pin(cap['sha256'])
        if type(cap['bytes']) is not int or not 0<cap['bytes']<=limits['capsuleBytes']:fail('capsule_limit')
    records=value['records']
    if type(records) is not list or not records or len(records)>limits['livePaths']:fail('record_count')
    paths={};members={};groups={};identity_roots={};children_by_parent={};path_bytes=0;identities={}
    for row in records:
        keys={'path','root','kind','metadata','member','sha256','children','target'}
        if type(row) is not dict or set(row)!=keys:fail('record_shape')
        path,root=canonical(row['path']),canonical(row['root'])
        if root not in roots or not path.is_relative_to(root) or str(path) in paths:fail('record_path')
        path_bytes+=len(os.fsencode(str(path)))
        if path_bytes>limits['pathBytes'] or len(path.parts)-len(root.parts)>limits['depth']:fail('catalog_work_limit')
        raw_metadata(row['metadata'],uid,device)
        kind=row['kind'];m=row['metadata']
        if type(kind) is not str or kind not in KINDS or stat.S_IFMT(m['mode'])!=KINDS[kind]:fail('record_type')
        identity=(m['device'],m['inode'])
        if identity in identities and identities[identity]!=(kind,m):fail('hardlink_identity')
        identities[identity]=(kind,m)
        if len(identities)>limits['liveInodes']:fail('catalog_work_limit')
        if kind=='regular':
            member_name(row['member']);pin(row['sha256'])
            token=row['member']
            identity=(m['device'],m['inode'])
            if row['children'] is not None or row['target'] is not None:fail('record_fields')
            if identity in identity_roots and identity_roots[identity]!=root:fail('cross_boundary_hardlink')
            identity_roots[identity]=root
            if identity in groups and groups[identity]!=(token,row['sha256'],m):
                fail('hardlink_identity')
            if token in members and members[token]!=identity:fail('member_alias')
            members[token]=identity;groups[identity]=(token,row['sha256'],m)
        else:
            if row['member'] is not None or row['sha256'] is not None:fail('opaque_payload')
            if kind=='directory':
                names=row['children']
                if type(names) is not list or any(type(x) is not str or '/' in x or x in ('','.','..') or '\0' in x for x in names) or names!=sorted(set(names)):
                    fail('directory_membership')
                if row['target'] is not None:fail('record_fields')
            elif kind=='symlink':
                if type(row['target']) is not str or '\0' in row['target'] or row['children'] is not None:fail('opaque_target')
            elif row['children'] is not None or row['target'] is not None:fail('record_fields')
        paths[str(path)]=row
        if path!=root:children_by_parent.setdefault(str(path.parent),[]).append(path.name)
    for root in roots:
        if str(root) not in paths or paths[str(root)]['kind']!='directory':fail('root_missing')
    for name,row in paths.items():
        path=Path(name)
        if name!=row['root'] and (str(path.parent) not in paths or paths[str(path.parent)]['kind']!='directory'):
            fail('parent_missing')
        if row['kind']=='directory':
            actual=sorted(children_by_parent.get(name,[]))
            if row['children']!=actual:fail('directory_membership')
    counts={}
    for row in records:
        if row['kind']=='regular':
            key=(row['metadata']['device'],row['metadata']['inode'])
            counts[key]=counts.get(key,0)+1
    for row in records:
        if row['kind']=='regular' and row['metadata']['nlink']!=counts[(row['metadata']['device'],row['metadata']['inode'])]:
            fail('cross_boundary_hardlink')
    if any(key not in paths or paths[key]['kind']!='regular' or paths[key]['sha256']!=digest for key,digest in provenance.items()):
        fail('provenance')
    return value


def fixture_path(path, fixture_root):
    """Writer authority is confined to exact new private fixture trees, not arbitrary roots."""
    base=Path(__file__).resolve().parent/'r38'/'fixtures'
    root=canonical(str(fixture_root));path=canonical(str(path))
    if root==base or not root.is_relative_to(base) or not path.is_relative_to(root):
        fail('fixture_scope')
    if root.resolve(strict=True)!=root or path.parent.resolve(strict=True)!=path.parent:
        fail('fixture_redirect')
    s=root.lstat()
    if not stat.S_ISDIR(s.st_mode) or stat.S_IMODE(s.st_mode)!=0o700 or s.st_uid!=os.getuid():
        fail('fixture_private')
    return path


def migration_rows(rows, catalog_digest, roots):
    """Validate append-only document labels; this does not move or remove anything."""
    if type(rows) is not list:fail('migration_shape')
    state={}
    for row in rows:
        if type(row) is not dict or set(row)!={'root','stage','catalogSHA256','previousSHA256'}:
            fail('migration_shape')
        root=str(canonical(row['root']));stage=row['stage']
        if root not in roots or row['catalogSHA256']!=catalog_digest:fail('migration_binding')
        old=state.get(root)
        wanted=('PREPARED' if old is None else 'VERIFIED' if old['stage']=='PREPARED' else 'CUTOVER' if old['stage']=='VERIFIED' else None)
        if stage!=wanted or row['previousSHA256']!=(sha(encode(old)) if old else None):
            fail('migration_order')
        state[root]=row
    return state
