"""PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).
PLUMBING_ONLY: foreground supervisor and two harmless local journaling workers.
"""
import argparse
import hmac
import json
import os
from pathlib import Path
import secrets
import select
import shutil
import signal
import socket
import stat
import subprocess
import sys
import threading
import time

sys.path.insert(0, str(Path(__file__).absolute().parent))
from config import LABEL, digest, load, plain_path, source_digest


def atomic(path, value, limit=None):
    # r10 F4: a bounded caller's receipt/status/config serialization is checked before any byte is written (no partial temp file).
    raw = json.dumps(value, sort_keys=True) + '\n'
    if limit is not None and len(raw.encode()) > limit:
        raise ValueError('atomic_record_limit')
    temp = path.with_suffix('.pending')
    fd = os.open(temp, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    with os.fdopen(fd, 'w') as f:
        f.write(raw)
        f.flush()
        os.fsync(f.fileno())
    os.replace(temp, path)
    directory = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


class Gate:
    """One lock makes the ready/no-begin fence and test-only begin exclusive."""
    def __init__(self, nonce, identity, token):
        self.nonce, self.identity, self.token = nonce, identity, token
        self.seq, self.state, self.unknown = 0, 'READY_NO_BEGIN', []
        self.lock = threading.Lock()

    def fault(self, reason):
        if reason not in self.unknown:
            self.unknown.append(reason)

    def request(self, request, *, operational=False, owner=None):
        with self.lock:
            fields={'command','token','nonce','identity','sequence'}
            if operational and request.get('command') in ('begin','cancel'):fields.add('owner')
            if set(request) != fields:
                return 'fields'
            if type(request['token']) is not str or not hmac.compare_digest(request['token'], self.token):
                return 'token'
            if request['nonce'] != self.nonce or request['identity'] != self.identity:  # GUARD_IDENTITY
                return 'identity'
            if type(request['sequence']) is not int or request['sequence'] != self.seq + 1:
                return 'sequence'
            if operational and request['command'] in ('begin','cancel'):
                if owner is None or request['owner']!=owner:return 'owner_identity'
                if request['command']=='begin':
                    if self.state!='READY_NO_BEGIN' or self.unknown:return 'not_ready_no_begin'
                    self.state='BEGIN_FENCED'
                elif self.state!='BEGIN_FENCED':return 'not_begun'
                self.seq+=1
                return 'ok'
            if request['command'] not in ('status', 'finish', 'abandon'):
                return 'command'
            if request['command'] != 'status':
                if self.state not in (('READY_NO_BEGIN','BEGIN_FENCED') if operational else ('READY_NO_BEGIN',)):  # GUARD_FENCE
                    return 'not_ready_no_begin'
                self.state = 'ABANDON_FENCED'
            self.seq += 1
            return 'ok'

    def fixture_begin(self):
        # Test API only; no IPC/CLI/client route can call this.
        with self.lock:
            if self.state != 'READY_NO_BEGIN':
                return 'fenced'
            self.state = 'FIXTURE_BEGUN'
            return 'begun'


def worker(role, c, nonce, fault):
    # Terminal Ctrl-C reaches the foreground group; supervisor performs the handshake.
    signal.signal(signal.SIGINT, signal.SIG_IGN)
    identity = dict(pid=os.getpid(), start_monotonic_ns=time.monotonic_ns(),
                    instance=secrets.token_hex(16), role=role)
    sequence = 0
    journal = Path(role + '.jsonl')
    stream = journal.open('x', encoding='utf-8')
    os.chmod(journal, 0o600)

    def emit(event, command=0):
        nonlocal sequence
        sequence += 1
        row = dict(prototype=LABEL, label='PLUMBING_ONLY', nonce=nonce,
                   identity=identity, sequence=sequence, monotonic_ns=time.monotonic_ns(),
                   event=event, command_sequence=command, no_begin=True)
        encoded = json.dumps(row) + '\n'
        stream.write(encoded)
        stream.flush()
        if fault == 'fsync_failure' and event == 'finish_ack':
            raise OSError('fixture fsync failure')
        os.fsync(stream.fileno())
        print(encoded, end='', flush=True)

    try:
        emit('ready')
        if fault == 'early_exit':
            return 17
        deadline = time.monotonic() + c['deadline_seconds'] + c['ack_seconds'] * 3
        while time.monotonic() < deadline:
            readable, _, _ = select.select([sys.stdin], [], [], c['heartbeat_seconds'])
            if readable:
                line = sys.stdin.readline(c['request_bytes'] + 1)
                if not line:
                    return 18
                request = json.loads(line)
                if set(request) != {'command', 'nonce', 'sequence'} or request['nonce'] != nonce or request['command'] != 'finish':
                    return 19
                if fault == 'missing_ack':
                    return 20
                emit('finish_ack', request['sequence'] + (1 if fault == 'wrong_ack' else 0))
                return 0
            emit('heartbeat')
        return 21
    finally:
        stream.close()


class Supervisor:
    def __init__(self, config_path, fixture_faults=None):
        self.config_path = Path(config_path).absolute()
        self.root = plain_path(self.config_path.parent, directory=True, private=True)
        self.c = load(self.config_path)
        self.source = source_digest(self.root)
        self.config_digest = digest(self.config_path)
        expected = json.loads(plain_path(self.root / 'launcher.expected.json', private=True).read_text())
        if expected['source_sha256'] != self.source or expected['config_sha256'] != self.config_digest:
            raise ValueError('source_config_drift')
        self.run = self.root / self.c['run_directory']
        self.run.mkdir(mode=0o700)  # Never reuse an existing run root.
        self.nonce, self.token = secrets.token_hex(16), secrets.token_hex(32)
        self.identity = dict(pid=os.getpid(), start_monotonic_ns=time.monotonic_ns(), instance=secrets.token_hex(16))
        self.gate = Gate(self.nonce, self.identity, self.token)
        self.children, self.rows, self.buffers = {}, {}, {}
        self.output, self.events = 0, 0
        self.stop_reason = None
        self.faults = fixture_faults or {}
        self.server = None
        self.started = time.monotonic()
        self.old_cwd = Path.cwd()

    def snapshot(self, phase):
        return dict(prototype=LABEL, label='PLUMBING_ONLY', phase=phase,
                    nonce=self.nonce, identity=self.identity, command_sequence=self.gate.seq,
                    state=self.gate.state, unknown=list(self.gate.unknown), no_begin=True,
                    source_sha256=self.source, config_sha256=self.config_digest,
                    monotonic_ns=time.monotonic_ns(), children=self.rows)

    def save_status(self, phase):
        atomic(self.run / 'status.json', self.snapshot(phase), limit=self.c['output_bytes'])

    def spawn(self):
        os.chdir(self.run)
        for role in ('owner', 'recorder'):
            p = subprocess.Popen([self.c['interpreter'], '-I', '-B', str(self.root / 'proof.py'),
                                  'worker', str(self.config_path), role, self.nonce,
                                  self.faults.get(role, 'none')],
                                 stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                 stderr=subprocess.DEVNULL, env={}, close_fds=True)
            os.set_blocking(p.stdout.fileno(), False)
            self.children[role] = p
            self.buffers[role] = b''
        self.server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.server.bind('control.sock')  # Relative address also supports long workspace paths.
        os.chmod('control.sock', 0o600)
        self.server.listen(2)
        self.server.setblocking(False)
        atomic(self.run / 'client.json', dict(prototype=LABEL, token=self.token, nonce=self.nonce, identity=self.identity))

    def pump(self, duration):
        handles = [p.stdout for p in self.children.values()]
        ready, _, _ = select.select(handles, [], [], max(0, duration))
        for role, p in self.children.items():
            if p.stdout not in ready:
                continue
            chunk = os.read(p.stdout.fileno(), self.c['request_bytes'] + 1)
            self.output += len(chunk)
            if self.output > self.c['output_bytes']:
                self.gate.fault('output_limit')
                self.stop_reason = 'output_limit'
            self.buffers[role] += chunk
            if len(self.buffers[role]) > self.c['request_bytes'] and b'\n' not in self.buffers[role]:
                self.gate.fault(role + '_frame_limit')
                self.stop_reason = 'worker_frame_limit'
                self.buffers[role] = b''
            while b'\n' in self.buffers[role]:
                line, self.buffers[role] = self.buffers[role].split(b'\n', 1)
                try:
                    row = json.loads(line)
                    old = self.rows.get(role)
                    if row['nonce'] != self.nonce or row['identity']['pid'] != p.pid or row['identity']['role'] != role or row['no_begin'] is not True:
                        raise ValueError('worker identity')
                    if row['sequence'] != (old['sequence'] + 1 if old else 1) or (old and (row['identity'] != old['identity'] or row['monotonic_ns'] <= old['monotonic_ns'])):
                        raise ValueError('worker sequence')
                    if row['event'] not in ('ready', 'heartbeat', 'finish_ack'):
                        raise ValueError('worker event')
                    self.rows[role] = row
                except (ValueError, KeyError, TypeError):
                    self.gate.fault(role + '_invalid_frame')
                    self.stop_reason = 'invalid_frame'

    def readiness(self):
        end = time.monotonic() + self.c['ack_seconds']
        while len(self.rows) != 2 and time.monotonic() < end:
            self.pump(min(self.c['heartbeat_seconds'], max(0, end - time.monotonic())))
        if len(self.rows) != 2:
            self.gate.fault('ready_ack_missing')
            return False
        return True

    def health(self):
        if source_digest(self.root) != self.source or digest(self.config_path) != self.config_digest:
            self.gate.fault('source_config_drift')
        now = time.monotonic_ns()
        for role, p in self.children.items():
            if p.poll() is not None:
                self.gate.fault(role + '_early_exit')
            row = self.rows.get(role)
            if not row or now - row['monotonic_ns'] > self.c['stale_seconds'] * 1e9:
                self.gate.fault(role + '_stale')
        if shutil.disk_usage(self.run).free < self.c['free_bytes']:
            self.gate.fault('free_space_limit')
        size = sum(p.lstat().st_size for p in self.run.iterdir() if stat.S_ISREG(p.lstat().st_mode))
        if size >= self.c['run_bytes']:
            self.gate.fault('run_size_limit')
        if self.gate.unknown:
            self.stop_reason = self.stop_reason or 'UNKNOWN'

    def handle_request(self,request):
        if self.events>self.c['request_count']:
            self.gate.fault('request_count_limit');self.stop_reason='request_count_limit'
            return 'request_count_limit'
        reason=self.gate.request(request)
        if reason=='ok' and request['command']!='status':
            self.stop_reason='client_'+request['command']
            if getattr(self,'archive_mode',lambda:False)():
                self.finish_end=min(getattr(self,'finish_end',None) or float('inf'),time.monotonic()+self.c['ack_seconds'])
        return reason

    def serve_cooperative(self):
        # One nonblocking client with ONE deadline, also serviced during recorder/status waits.
        pending=getattr(self,'client_pending',None)
        if pending is None:
            try:conn,_=self.server.accept()
            except BlockingIOError:return
            conn.setblocking(False)
            pending=dict(conn=conn,deadline=time.monotonic()+self.c['ack_seconds'],raw=bytearray(),out=None)
            self.client_pending=pending;self.events+=1
        conn=pending['conn']
        try:
            if time.monotonic()>pending['deadline']:raise TimeoutError('request_deadline')
            if pending['out'] is None:
                try:chunk=conn.recv(self.c['request_bytes']+1)
                except BlockingIOError:return
                if not chunk:raise EOFError('incomplete_request')
                pending['raw'].extend(chunk)
                data=pending['raw']
                if len(data)>self.c['request_bytes']:raise ValueError('request_limit')
                if b'\n' not in data:return
                if not data.endswith(b'\n') or data.count(b'\n')!=1:raise ValueError('request_limit')
                request=json.loads(data)
                if type(request) is not dict:raise ValueError('fields')
                reason=self.handle_request(request)
                from transport import json_chunks
                pending['out']=b''.join(json_chunks(dict(reason=reason,status=self.snapshot('LIVE')),limit=self.c['output_bytes']))
            try:n=conn.send(pending['out'])
            except BlockingIOError:return
            if n<=0:raise OSError('client_response_lost')
            pending['out']=pending['out'][n:]
            if not pending['out']:
                conn.close();self.client_pending=None
        except (ValueError,TypeError,OSError,EOFError,TimeoutError) as exc:
            self.gate.fault('client_response_lost:'+str(exc));conn.close();self.client_pending=None

    def serve_one(self):
        if getattr(self,'archive_mode',lambda:False)():return self.serve_cooperative()
        try:
            conn, _ = self.server.accept()
        except BlockingIOError:
            return
        with conn:
            conn.settimeout(self.c['ack_seconds'])
            request_deadline = min(time.monotonic() + self.c['ack_seconds'],
                                   self.started + self.c['deadline_seconds'])
            self.events += 1
            try:
                data = bytearray()
                while b'\n' not in data and len(data) <= self.c['request_bytes']:
                    self.pump(0)
                    self.health()
                    self.save_status('LIVE')
                    now = time.monotonic()
                    if now >= self.started + self.c['deadline_seconds']:
                        self.stop_reason = self.stop_reason or 'deadline'
                    if self.stop_reason:
                        raise ValueError('request_interrupted')
                    remaining = request_deadline - now
                    if remaining <= 0:  # GUARD_REQUEST_DEADLINE
                        raise ValueError('request_deadline')
                    readable, _, _ = select.select([conn], [], [], min(self.c['heartbeat_seconds'], max(0, remaining)))
                    if not readable:
                        continue
                    chunk = conn.recv(min(512, self.c['request_bytes'] + 1 - len(data)))
                    if not chunk:
                        raise ValueError('incomplete_request')
                    data.extend(chunk)
                if len(data) > self.c['request_bytes'] or not data.endswith(b'\n') or data.count(b'\n') != 1:
                    raise ValueError('request_limit')
                request = json.loads(data)
                if not isinstance(request, dict):
                    raise ValueError('fields')
                self.health()
                reason = self.handle_request(request)
                self.save_status('LIVE')
                response = dict(reason=reason, status=self.snapshot('LIVE'))
            except (ValueError, TypeError, socket.timeout) as error:
                response = dict(reason=str(error), label='PLUMBING_ONLY')
            try:
                conn.sendall((json.dumps(response) + '\n').encode())
            except (BrokenPipeError, ConnectionResetError, socket.timeout):
                # A vanished client cannot un-fence a finish already admitted.
                self.gate.fault('client_response_lost')
                self.stop_reason = 'client_response_lost'

    def close(self, reason):
        self.gate.state = 'ABANDON_FENCED'
        command = self.gate.seq + 1
        for role, p in self.children.items():
            try:
                p.stdin.write((json.dumps(dict(command='finish', nonce=self.nonce, sequence=command)) + '\n').encode())
                p.stdin.flush()
                p.stdin.close()
            except (BrokenPipeError, OSError):
                self.gate.fault(role + '_finish_write')
                try:
                    p.stdin.close()
                except (BrokenPipeError, OSError):
                    self.gate.fault(role + '_finish_write')
        end = time.monotonic() + self.c['ack_seconds']
        while time.monotonic() < end:
            self.pump(min(self.c['heartbeat_seconds'], max(0, end - time.monotonic())))
            if all(p.poll() is not None for p in self.children.values()):
                self.pump(0)
                break
        waits = {}
        for role, p in self.children.items():
            try:
                waits[role] = p.wait(timeout=self.c['ack_seconds'])
            except subprocess.TimeoutExpired:
                self.gate.fault(role + '_wait_timeout')
                p.terminate()  # Only this supervisor's currently owned waited child handle.
                try:
                    waits[role] = p.wait(timeout=self.c['ack_seconds'])
                except subprocess.TimeoutExpired:
                    p.kill()
                    waits[role] = p.wait(timeout=self.c['ack_seconds'])
            p.stdout.close()
            row = self.rows.get(role, {})
            if row.get('event') != 'finish_ack' or row.get('command_sequence') != command or waits[role] != 0:  # GUARD_ACKS
                self.gate.fault(role + '_finish_ack_missing_or_wrong')
        clean = not any(x.endswith(('finish_write', 'wait_timeout', 'finish_ack_missing_or_wrong')) for x in self.gate.unknown)
        self.gate.state = 'HELPERS_CLOSED'
        receipt = self.snapshot('FINAL')
        receipt.update(reason=reason, child_waits=waits, helpers_clean_shutdown=clean,
                       outcome='UNKNOWN' if self.gate.unknown else 'PLUMBING_ONLY_CLEAN',  # GUARD_UNKNOWN
                       native_state='NOT_RUN', manual_survival='NOT_PROVEN_BY_FIXTURE_TESTS')
        atomic(self.run / 'shutdown.json', receipt)
        self.save_status('FINAL')
        if self.server:
            self.server.close()
        os.chdir(self.old_cwd)
        return receipt

    def run_foreground(self):
        previous = signal.getsignal(signal.SIGINT)
        signal.signal(signal.SIGINT, lambda _s, _f: setattr(self, 'stop_reason', 'Ctrl-C'))
        try:
            self.spawn()
            if not self.readiness():
                self.stop_reason = 'ready_failed'
            self.health()
            if not self.stop_reason:
                self.save_status('LIVE')
                print('READY PLUMBING_ONLY READY_NO_BEGIN', flush=True)
                print('Receipt location: ' + str(self.run), flush=True)
            while not self.stop_reason:
                self.pump(self.c['heartbeat_seconds'])
                self.health()
                self.serve_one()
                self.save_status('LIVE')
                if time.monotonic() - self.started >= self.c['deadline_seconds']:
                    self.stop_reason = 'deadline'
            receipt = self.close(self.stop_reason)
            print('FINAL ' + receipt['outcome'] + ': ' + str(self.run / 'shutdown.json'), flush=True)
            return 0 if receipt['outcome'] == 'PLUMBING_ONLY_CLEAN' else 1
        except (OSError, ValueError, KeyError) as error:
            self.gate.fault('supervisor_error:' + type(error).__name__)
            self.close('supervisor_error')
            raise
        finally:
            signal.signal(signal.SIGINT, previous)


def client(config_path, command='status'):
    c = load(config_path)
    root = plain_path(Path(config_path).absolute().parent, directory=True, private=True)
    run = plain_path(root / c['run_directory'], directory=True, private=True)
    auth = json.loads(plain_path(run / 'client.json', private=True).read_text())
    status = json.loads(plain_path(run / 'status.json', private=True).read_text())
    if status['phase'] != 'LIVE' or time.monotonic_ns() - status['monotonic_ns'] > c['stale_seconds'] * 1e9:
        raise ValueError('dead_or_stale_run')
    if source_digest(root) != status['source_sha256'] or digest(config_path) != status['config_sha256']:
        raise ValueError('source_config_drift')
    if status['identity'] != auth['identity'] or status['nonce'] != auth['nonce']:
        raise ValueError('identity')
    endpoint = run / 'control.sock'
    s = endpoint.lstat()
    if not stat.S_ISSOCK(s.st_mode) or s.st_uid != os.getuid() or stat.S_IMODE(s.st_mode) != 0o600:
        raise ValueError('endpoint_mode')
    request = dict(command=command, token=auth['token'], nonce=auth['nonce'],
                   identity=auth['identity'], sequence=status['command_sequence'] + 1)
    if command in ('begin','cancel'):
        owner=status.get('owner') or {}
        request['owner']={k:owner.get(k) for k in ('pid','ownerStart','instance')}
    old = Path.cwd()
    try:
        os.chdir(run)
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
            end=time.monotonic()+c['ack_seconds']
            def remaining():
                left=end-time.monotonic()
                if left<=0:raise TimeoutError('client_transaction_deadline')
                return left
            sock.settimeout(remaining())
            sock.connect('control.sock')
            sock.settimeout(remaining())
            sock.sendall((json.dumps(request) + '\n').encode())
            data = bytearray()
            while b'\n' not in data:
                sock.settimeout(remaining())
                part = sock.recv(min(512, c['output_bytes'] + 1 - len(data)))
                if not part or len(data) + len(part) > c['output_bytes']:
                    raise ValueError('response_limit_or_missing')
                data.extend(part)
            if data.count(b'\n')!=1 or not data.endswith(b'\n'):raise ValueError('response_framing')
            return json.loads(data)
    finally:
        os.chdir(old)


def main():
    parser = argparse.ArgumentParser(description='PLUMBING_ONLY harmless manual fixture')
    commands = parser.add_subparsers(dest='mode', required=True)
    for mode in ('host', 'client', 'worker'):
        sub = commands.add_parser(mode)
        sub.add_argument('config', type=Path)
        if mode == 'client':
            sub.add_argument('command', choices=('status', 'finish', 'abandon'), nargs='?', default='status')
        if mode == 'worker':
            sub.add_argument('role', choices=('owner', 'recorder'))
            sub.add_argument('nonce')
            sub.add_argument('fault', choices=('none', 'early_exit', 'missing_ack', 'wrong_ack', 'fsync_failure'))
    a = parser.parse_args()
    if a.mode == 'host':
        return Supervisor(a.config).run_foreground()
    if a.mode == 'worker':
        return worker(a.role, load(a.config), a.nonce, a.fault)
    response = client(a.config, a.command)
    print(json.dumps(response, indent=2))
    return 0 if response['reason'] == 'ok' else 1


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError) as error:
        sys.exit('PLUMBING_ONLY UNKNOWN/refused: ' + str(error))
