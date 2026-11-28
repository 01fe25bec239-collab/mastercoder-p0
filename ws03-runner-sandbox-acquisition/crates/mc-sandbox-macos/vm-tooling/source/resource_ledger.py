# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Metadata-only allocation collection; no native calls, content reads or cleanup."""
from dataclasses import asdict, dataclass
import math
import os
from pathlib import Path
import stat
from typing import Callable, Mapping


class LedgerError(ValueError):
    """An incomplete observation is never an admission."""


@dataclass(frozen=True)
class LedgerPaths:
    owned_roots: Mapping[str, Path]
    image: Path
    disk: Path
    temp_root: Path
    preserved_framework_dirs: tuple[Path, ...]
    expected_uid: int
    framework_prefix: str


@dataclass(frozen=True)
class ResourceLimits:
    disk_cap_bytes: int
    scratch_cap_bytes: int
    aggregate_cap_bytes: int
    preinstall_free_bytes: int
    reserve_bytes: int
    loss_rate_bytes_per_second: int
    max_response_seconds: float
    max_cancel_seconds: float
    max_sample_age_seconds: float
    settlement_seconds: float
    assumptions_verified: bool
    assumption_basis: str

    def validate(self):
        for key in ("disk_cap_bytes", "scratch_cap_bytes", "aggregate_cap_bytes",
                    "preinstall_free_bytes", "reserve_bytes", "loss_rate_bytes_per_second"):
            if type(getattr(self, key)) is not int or getattr(self, key) <= 0:
                raise LedgerError("malformed_limit:" + key)
        for key in ("max_response_seconds", "max_cancel_seconds", "max_sample_age_seconds",
                    "settlement_seconds"):
            value = getattr(self, key)
            if type(value) not in (float, int) or not math.isfinite(value) or value <= 0:
                raise LedgerError("malformed_limit:" + key)
        if self.max_sample_age_seconds > self.max_response_seconds:
            raise LedgerError("sample_age_exceeds_response_budget")
        if (type(self.assumptions_verified) is not bool or type(self.assumption_basis) is not str or
                not self.assumption_basis.strip()):
            raise LedgerError("missing_assumption_basis")

    @property
    def runway_bytes(self):
        self.validate()
        return math.ceil(self.loss_rate_bytes_per_second *
                         (self.max_response_seconds + self.max_cancel_seconds))


@dataclass(frozen=True)
class Entry:
    device: int
    inode: int
    allocated: int
    size: int
    category: str
    paths: tuple[str, ...]


@dataclass(frozen=True)
class Snapshot:
    started: float
    finished: float
    device: int
    free_bytes: int
    entries: tuple[Entry, ...]
    required_roots: tuple[str, ...]
    measured_roots: tuple[str, ...]
    discovered_framework_dirs: tuple[str, ...]
    measured_framework_dirs: tuple[str, ...]
    preserved_framework_dirs: tuple[str, ...]

    @property
    def allocated_bytes(self):
        return sum(e.allocated for e in self.entries)

    @property
    def disk_allocated(self):
        return sum(e.allocated for e in self.entries if e.category == "disk")

    @property
    def scratch_allocated(self):
        return sum(e.allocated for e in self.entries if e.category == "scratch")

    @property
    def allocation_fingerprint(self):
        return tuple(sorted((e.device, e.inode, e.allocated, e.category) for e in self.entries))

    def to_dict(self):
        return dict(asdict(self), allocatedBytes=self.allocated_bytes,
                    diskAllocated=self.disk_allocated, scratchAllocated=self.scratch_allocated)


@dataclass(frozen=True)
class Decision:
    admitted: bool
    reasons: tuple[str, ...]
    remaining_bytes: int
    runway_bytes: int
    projected_allocation_bytes: int
    projected_free_bytes: int
    free_gap_bytes: int
    aggregate_gap_bytes: int

    def to_dict(self):
        return asdict(self)


def _stamp(value):
    try:
        fields = (value.st_dev, value.st_ino, value.st_mode, value.st_uid,
                  value.st_size, value.st_blocks, value.st_mtime_ns, value.st_ctime_ns)
    except AttributeError as exc:
        raise LedgerError("missing_stat_field") from exc
    if (any(type(v) is not int for v in fields) or value.st_ino <= 0 or
            min(value.st_dev, value.st_uid, value.st_size, value.st_blocks) < 0):
        raise LedgerError("malformed_stat")
    return fields


def _checked_stat(path, uid):
    try:
        value = path.lstat()
        _stamp(value)
        if stat.S_ISLNK(value.st_mode):
            raise LedgerError("symlink:" + str(path))
        if not (stat.S_ISREG(value.st_mode) or stat.S_ISDIR(value.st_mode)):
            raise LedgerError("unsupported_file_type:" + str(path))
        if value.st_uid != uid:
            raise LedgerError("wrong_owner:" + str(path))
        # A symlink in any ancestor could redirect an otherwise ordinary leaf.
        if path.resolve(strict=True) != path:
            raise LedgerError("unresolved_or_redirected_path:" + str(path))
        return value
    except OSError as exc:
        raise LedgerError("missing_or_unreadable:" + str(path)) from exc


def collect_snapshot(paths: LedgerPaths, *, clock: Callable[[], float]) -> Snapshot:
    """Scan only admitted roots and matched installation dirs, inode-deduplicated.

    A second metadata pass catches replacement/growth/discovery races. Filesystem
    metadata is not an atomic/quota snapshot; that remaining limit is explicit.
    """
    started = clock()
    if not {"lab", "r4", "r5", "r6"}.issubset(paths.owned_roots):
        raise LedgerError("missing_or_ambiguous_owned_roots")
    if type(paths.expected_uid) is not int or paths.expected_uid < 0:
        raise LedgerError("invalid_uid")
    if not paths.framework_prefix or "/" in paths.framework_prefix:
        raise LedgerError("invalid_framework_prefix")
    roots = tuple(Path(paths.owned_roots[k]) for k in sorted(paths.owned_roots))
    image, disk, temp = Path(paths.image), Path(paths.disk), Path(paths.temp_root)
    declared = (*roots, image, disk, temp, *map(Path, paths.preserved_framework_dirs))
    if any(not p.is_absolute() or p != Path(os.path.normpath(p)) for p in declared):
        raise LedgerError("noncanonical_path")
    if any(not any(p.is_relative_to(r) for r in roots) for p in (image, disk)):
        raise LedgerError("image_or_disk_outside_owned_roots")
    initial = {str(p): _checked_stat(p, paths.expected_uid) for p in declared}
    if any(not stat.S_ISDIR(initial[str(p)].st_mode) for p in (*roots, temp)):
        raise LedgerError("root_not_directory")
    if any(not stat.S_ISREG(initial[str(p)].st_mode) for p in (image, disk)):
        raise LedgerError("image_or_disk_not_regular")
    device = initial[str(image)].st_dev
    if any(s.st_dev != device for s in initial.values()):
        raise LedgerError("unadmitted_separate_volume")
    image_key = (device, initial[str(image)].st_ino)
    disk_key = (device, initial[str(disk)].st_ino)
    if image_key == disk_key:
        raise LedgerError("ambiguous_image_disk_inode")
    preserved = tuple(sorted(str(p) for p in paths.preserved_framework_dirs))
    if not preserved or any(Path(p).parent != temp or
                            not Path(p).name.startswith(paths.framework_prefix) for p in preserved):
        raise LedgerError("missing_or_invalid_preserved_framework_directory")

    def discover():
        try:
            # Names of unrelated temp entries are used only to match the approved prefix.
            with os.scandir(temp) as entries:
                found = tuple(sorted(str(temp / e.name) for e in entries
                                     if e.name.startswith(paths.framework_prefix)))
        except OSError as exc:
            raise LedgerError("unreadable_temp_inventory") from exc
        for p in found:
            value = _checked_stat(Path(p), paths.expected_uid)
            if not stat.S_ISDIR(value.st_mode):
                raise LedgerError("ambiguous_framework_entry:" + p)
        if not set(preserved).issubset(found):
            raise LedgerError("missing_preserved_framework_directory")
        return found

    discovered = discover()
    seen = {}
    observations = {}

    def visit(path):
        value = _checked_stat(path, paths.expected_uid)
        if value.st_dev != device:
            raise LedgerError("unadmitted_separate_volume:" + str(path))
        key = (value.st_dev, value.st_ino)
        category = "image" if key == image_key else "disk" if key == disk_key else "scratch"
        observations[str(path)] = _stamp(value)
        if key in seen:
            old = seen[key]
            if old[0] != _stamp(value) or old[1] != category:
                raise LedgerError("ambiguous_inode_measurement:" + str(path))
            old[2].append(str(path))
            return
        seen[key] = [_stamp(value), category, [str(path)]]
        if stat.S_ISDIR(value.st_mode):
            try:
                with os.scandir(path) as children:
                    names = sorted(child.name for child in children)
                for name in names:
                    visit(path / name)
            except OSError as exc:
                raise LedgerError("unreadable_directory:" + str(path)) from exc
        if _stamp(_checked_stat(path, paths.expected_uid)) != observations[str(path)]:
            raise LedgerError("stale_metadata:" + str(path))

    for root in (*roots, *map(Path, discovered)):
        visit(root)
    if image_key not in seen or disk_key not in seen:
        raise LedgerError("missing_image_or_disk_measurement")
    for p, wanted in observations.items():
        if _stamp(_checked_stat(Path(p), paths.expected_uid)) != wanted:
            raise LedgerError("stale_metadata:" + p)
    if discover() != discovered:
        raise LedgerError("stale_framework_inventory")
    # Actual free is independent: shrinking owned files never credits unrelated loss.
    try:
        space = os.statvfs(image)
        free = space.f_bavail * space.f_frsize
    except OSError as exc:
        raise LedgerError("unreadable_free_space") from exc
    entries = tuple(Entry(k[0], k[1], v[0][5] * 512, v[0][4], v[1], tuple(sorted(set(v[2]))))
                    for k, v in sorted(seen.items()))
    return Snapshot(started, clock(), device, free, entries, tuple(sorted(map(str, roots))),
                    tuple(sorted(map(str, roots))), discovered, discovered, preserved)


def validate_snapshot(snapshot: Snapshot, limits: ResourceLimits, *, now: float):
    _validated_paths(snapshot, limits, now)


def _validated_paths(snapshot, limits, now):
    """validate_snapshot's checks, unchanged; returns the measured path set it built (callers reuse it)."""
    limits.validate()
    if any(type(t) not in (float, int) or not math.isfinite(t)
           for t in (snapshot.started, snapshot.finished, now)):
        raise LedgerError("malformed_clock")
    if not snapshot.started <= snapshot.finished <= now:
        raise LedgerError("invalid_sample_time")
    # Age includes collection, never just time since the last directory was scanned.
    if now - snapshot.started > limits.max_sample_age_seconds:
        raise LedgerError("stale_sample")
    if type(snapshot.free_bytes) is not int or snapshot.free_bytes < 0:
        raise LedgerError("malformed_free_space")
    if len(snapshot.required_roots) < 4 or not all(snapshot.required_roots) or \
            snapshot.required_roots != snapshot.measured_roots:
        raise LedgerError("incomplete_owned_roots")
    if (not snapshot.preserved_framework_dirs or
            snapshot.discovered_framework_dirs != snapshot.measured_framework_dirs or
            not set(snapshot.preserved_framework_dirs).issubset(snapshot.measured_framework_dirs)):
        raise LedgerError("incomplete_framework_inventory")
    # Every entry reaching the duplicate check is on snapshot.device (checked just before), so its
    # (device, inode) identity is a duplicate exactly when its inode is: no per-entry identity tuple.
    # Same checks and order; each attribute is read once, in the original first-read order.
    device, inodes, categories, measured_paths = snapshot.device, set(), [], set()
    for entry in snapshot.entries:
        if (type(d := entry.device) is not int or type(i := entry.inode) is not int or i <= 0 or
                type(a := entry.allocated) is not int or a < 0 or
                type(s := entry.size) is not int or s < 0 or not (paths := entry.paths)):
            raise LedgerError("malformed_entry")
        if d != device:
            raise LedgerError("unadmitted_separate_volume")
        if i in inodes:
            raise LedgerError("duplicate_inode")
        inodes.add(i)
        categories.append(entry.category)
        measured_paths.update(paths)
    if categories.count("image") != 1 or categories.count("disk") != 1 or \
            any(c not in ("image", "disk", "scratch") for c in categories):
        raise LedgerError("missing_or_ambiguous_categories")
    if not set(snapshot.measured_roots).issubset(measured_paths):
        raise LedgerError("unmeasured_owned_root")
    if not set(snapshot.measured_framework_dirs).issubset(measured_paths):
        raise LedgerError("unmeasured_framework_directory")
    return measured_paths


def evaluate(snapshot: Snapshot, limits: ResourceLimits, *, now: float, preflight: bool) -> Decision:
    """Equality satisfies a cap/reserve; continuous monitoring cancels at runway equality."""
    validate_snapshot(snapshot, limits, now=now)
    disk, scratch, total = snapshot.disk_allocated, snapshot.scratch_allocated, snapshot.allocated_bytes
    remaining = max(0, limits.disk_cap_bytes - disk) + max(0, limits.scratch_cap_bytes - scratch)
    runway = limits.runway_bytes
    projected_free = snapshot.free_bytes - remaining
    projected_allocation = total + remaining
    reasons = []
    if not limits.assumptions_verified:
        reasons.append("unverified_loss_or_latency_bound")
    if disk > limits.disk_cap_bytes:
        reasons.append("disk_cap_breach")
    if scratch > limits.scratch_cap_bytes:
        reasons.append("scratch_cap_breach")
    if total > limits.aggregate_cap_bytes or projected_allocation > limits.aggregate_cap_bytes:
        reasons.append("aggregate_plan_breach")
    if preflight and snapshot.free_bytes < limits.preinstall_free_bytes:
        reasons.append("preinstall_free_shortfall")
    if projected_free < limits.reserve_bytes + runway:
        reasons.append("projected_free_runway_shortfall")
    # All loss could land in any one allocation class: do not spend its cancellation headroom.
    for name, headroom in (("disk", limits.disk_cap_bytes - disk),
                           ("scratch", limits.scratch_cap_bytes - scratch),
                           ("aggregate", limits.aggregate_cap_bytes - total)):
        if headroom < runway or (not preflight and headroom == runway):
            reasons.append(name + "_cancellation_runway_shortfall")
    if not preflight and projected_free == limits.reserve_bytes + runway:
        reasons.append("projected_free_at_cancellation_trigger")
    return Decision(not reasons, tuple(reasons), remaining, runway, projected_allocation,
                    projected_free, max(0, limits.reserve_bytes + runway - projected_free),
                    max(0, projected_allocation - limits.aggregate_cap_bytes))
