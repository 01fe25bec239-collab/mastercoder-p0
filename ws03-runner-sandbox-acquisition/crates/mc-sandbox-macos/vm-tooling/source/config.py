"""PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)."""
import hashlib
import json
import os
from pathlib import Path
import stat
import sys

LABEL = 'PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)'


def plain_path(path, directory=False, private=False):
    path = Path(path).absolute()
    for part in [*reversed(path.parents), path]:
        if part.is_symlink():
            raise ValueError('symlink component')
    s = path.stat()
    if not (stat.S_ISDIR(s.st_mode) if directory else stat.S_ISREG(s.st_mode)):
        raise ValueError('wrong file kind')
    if s.st_uid != os.getuid():
        raise ValueError('wrong owner')
    if private and stat.S_IMODE(s.st_mode) != (0o700 if directory else 0o600):
        raise ValueError('wrong private mode')
    return path


def digest(path):
    path = plain_path(path)
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def load(path):
    path = plain_path(path, private=True)
    c = json.loads(path.read_text())
    keys = {'prototype', 'interpreter', 'run_directory', 'deadline_seconds',
            'heartbeat_seconds', 'stale_seconds', 'ack_seconds', 'request_bytes',
            'output_bytes', 'run_bytes', 'free_bytes', 'request_count'}
    if set(c) != keys or c['prototype'] != LABEL:
        raise ValueError('config fields')
    if not Path(c['interpreter']).is_absolute() or not os.access(c['interpreter'], os.X_OK):
        raise ValueError('interpreter')
    if Path(c['interpreter']).resolve() != Path(sys.executable).resolve():
        raise ValueError('interpreter mismatch')
    if c['run_directory'] != Path(c['run_directory']).name or c['run_directory'] in ('', '.', '..'):
        raise ValueError('run directory')
    limits = {'deadline_seconds': (1, 2400), 'heartbeat_seconds': (.01, 5),
              'stale_seconds': (.05, 30), 'ack_seconds': (.05, 10),
              'request_bytes': (128, 4096), 'output_bytes': (512, 2**20),
              'run_bytes': (4096, 256 * 2**20), 'free_bytes': (20 * 2**30, 2**50),
              'request_count': (1, 1000)}
    for key, (low, high) in limits.items():
        if type(c[key]) not in (int, float) or not low <= c[key] <= high:
            raise ValueError('config limit: ' + key)
    for key in ('request_bytes', 'output_bytes', 'run_bytes', 'free_bytes', 'request_count'):
        if type(c[key]) is not int:
            raise ValueError('integer limit: ' + key)
    if c['stale_seconds'] <= c['heartbeat_seconds'] or c['output_bytes'] > c['run_bytes'] or c['deadline_seconds'] + c['ack_seconds'] * 8 > 2416:  # r7 owner-approved session bound
        raise ValueError('inconsistent limits')
    return c


def source_digest(root):
    # Fixed authored file set: runtime journals are never part of the source pin.
    names = ('config.py', 'proof.py', 'launch.sh', 'finish.sh', 'interpreter.txt')
    h = hashlib.sha256()
    for name in names:
        h.update(name.encode() + b'\0' + bytes.fromhex(digest(root / name)))
    return h.hexdigest()
