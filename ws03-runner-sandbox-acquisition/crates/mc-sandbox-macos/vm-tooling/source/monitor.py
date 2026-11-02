# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
import base64
import gc
import hashlib
import json
import os
import select
import secrets
import sys
import time
from pathlib import Path

sys.path.insert(0,str(Path(__file__).absolute().parent))
from live_ledger import collect, settlement_digest
from pilot_policy import evaluate_pilot, evaluate_pilot_pair
from native_config import limits
from native_config import validate_monitor_access


MAX_COMMAND_BYTES = 4096
# Every ledger/inventory frame, newline included, fits the controller's unchanged request_bytes (config.json):
# a 4097-byte read plus any partial line then stays within its 2 * request_bytes buffer.
MAX_EVENT_BYTES = MAX_COMMAND_BYTES


class MonitorProtocolError(ValueError):
    pass


def encode(row):
    # ensure_ascii (default) keeps the line ASCII, so len() is the byte count.
    return json.dumps(row, sort_keys=True, separators=(",", ":"), allow_nan=False)


def ledger_frames(head, seq, fields, sent):
    """Return (lines, inventory digest the controller then holds) for one complete ledger observation.

    A ledger that fits is the unchanged inline line. Otherwise the exact roots/framework JSON goes in
    digest-bound, sequenced inventory parts (only when the digest differs from `sent`), then a ledger
    referencing that digest. Nothing is dropped or truncated.
    """
    line = encode(dict(head, kind="ledger", seq=seq, **fields))
    if len(line) < MAX_EVENT_BYTES:
        return [line], sent
    fields = dict(fields)
    raw = encode({"framework": fields.pop("framework"), "roots": fields.pop("roots")}).encode()
    digest = hashlib.sha256(raw).hexdigest()
    lines = []
    if digest != sent:
        # Upper bounds for every numeric field, so each part's base64 data provably fits.
        empty = encode(dict(head, kind="inventory", seq=seq + len(raw), inventory=digest,
                            part=len(raw), parts=len(raw), bytes=len(raw), data=""))
        step = (MAX_EVENT_BYTES - 1 - len(empty)) // 4 * 3
        if step <= 0:
            raise ValueError("monitor_frame_limit")
        parts = -(-len(raw) // step)
        for index in range(parts):
            data = base64.b64encode(raw[index * step:(index + 1) * step]).decode()
            lines.append(encode(dict(head, kind="inventory", seq=seq + index, inventory=digest,
                                     part=index + 1, parts=parts, bytes=len(raw), data=data)))
    lines.append(encode(dict(head, kind="ledger", seq=seq + len(lines), inventory=digest, **fields)))
    if any(len(x) >= MAX_EVENT_BYTES for x in lines):  # GUARD_FRAME_LIMIT
        raise ValueError("monitor_frame_limit")
    return lines, digest


def main():
    c = json.loads(Path(sys.argv[1]).read_text())
    mode = sys.argv[2]
    from pilot_policy import ARCHIVE5_POLICY
    if c.get('policy')==ARCHIVE5_POLICY:
        return archive_main(c,mode)
    nonce = c.get("runNonce")
    if (type(nonce) is not str or len(nonce) < 32 or
            not nonce.isascii() or any(ch not in "0123456789abcdefABCDEF" for ch in nonce)):
        raise MonitorProtocolError("invalid_run_nonce")

    pid = os.getpid()
    started = time.monotonic()
    instance = secrets.token_hex(16)
    event_seq = 1
    command_seq = 0
    input_fd = sys.stdin.fileno()
    command_buffer = bytearray()
    head = {"nonce": nonce, "pid": pid, "startMonotonic": started, "instance": instance}
    sent_inventory = None
    cycle = 1  # the ledger this collection will produce; progress is bound to it and its start

    def emit(kind, **fields):
        nonlocal event_seq
        print(encode(dict(head, kind=kind, seq=event_seq, **fields)), flush=True)
        event_seq += 1

    # Consume only the start line, leaving any following command bytes for the reader below.
    start = bytearray()
    while len(start) <= 16 and not start.endswith(b"\n"):
        part = os.read(input_fd, 1)
        if not part:
            raise MonitorProtocolError("monitor_start_binding")
        start.extend(part)
    if start != b"start\n":
        raise MonitorProtocolError("monitor_start_binding")

    if mode == "native":
        try:
            validate_monitor_access(c)
        except ValueError as exc:
            emit("monitor_refused", reason=str(exc))
            return 3

    emit("ready", mode=mode)
    if mode == "native":
        # Back-to-back native cycles build only acyclic tuples/Entries, freed by refcounting. Cyclic GC
        # would just rescan the live snapshot inside the timed window; collect() already pauses it.
        gc.disable()

    def finish_if_requested(timeout=0):
        nonlocal command_seq
        readable, _, _ = select.select([input_fd], [], [], timeout)
        if readable:
            chunk = os.read(input_fd, MAX_COMMAND_BYTES + 1)
            if not chunk:
                raise MonitorProtocolError("monitor_control_eof")
            command_buffer.extend(chunk)
        if b"\n" not in command_buffer:
            if len(command_buffer) > MAX_COMMAND_BYTES:
                raise MonitorProtocolError("monitor_command_too_large")
            return False

        line, _, extra = command_buffer.partition(b"\n")
        command_buffer.clear()
        if extra or len(line) > MAX_COMMAND_BYTES:
            raise MonitorProtocolError("monitor_command_framing")
        try:
            command = json.loads(line.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as exc:
            raise MonitorProtocolError("monitor_command_encoding") from exc
        if (not isinstance(command, dict) or
                set(command) != {"command", "nonce", "pid", "instance", "commandSeq"} or
                command.get("command") != "finish" or command.get("nonce") != nonce or
                type(command.get("pid")) is not int or command["pid"] != pid or
                command.get("instance") != instance or
                type(command.get("commandSeq")) is not int or
                command["commandSeq"] != command_seq + 1):
            raise MonitorProtocolError("monitor_finish_binding")

        command_seq = command["commandSeq"]
        emit("finish_ack", command="finish", commandSeq=command_seq, outcome="finished")
        return True

    def checkpoint(phase, completed):
        # Liveness only, reported from genuine completed scan work (archive cadence: fastDeadline/2).
        # Never a timer: a blocked native pass stays silent and goes stale. Pending control input
        # interrupts like the native scan, so no progress follows a finish the monitor could read.
        if select.select([input_fd], [], [], 0)[0]:
            raise InterruptedError("collection_interrupted_by_control")
        now = time.monotonic()
        if completed > progress_state[0] and now - progress_state[1] >= c.get("fastDeadline", .5) / 2:
            progress_state[:] = completed, now
            emit("progress", cycle=cycle, collectionStarted=begin, phase=phase, completed=completed)

    try:
        while True:
            if finish_if_requested():
                return 0
            begin = time.monotonic()
            progress_state = [0, begin]
            faulted = False
            try:
                if mode == "native":
                    # Fail closed before any native inventory collection.
                    try:
                        validate_monitor_access(c)
                    except ValueError as exc:
                        emit("monitor_refused", reason=str(exc))
                        return 3
                    try:
                        # Pending control input interrupts the scan; the partial walk is discarded and
                        # the command is serviced now instead of after a full cycle (finish <= 2s).
                        s = None  # release the previous observation first: one snapshot at peak, not two
                        s = collect(c, control=input_fd, progress=checkpoint)
                    except InterruptedError:
                        continue
                    if finish_if_requested():
                        return 0
                    decision, start_decision = evaluate_pilot_pair(s, limits(c), c, now=time.monotonic())
                    payload = dict(
                        decision=decision,
                        startDecision=start_decision,
                        free=s.free_bytes,
                        scratch=s.scratch_allocated,
                        total=s.allocated_bytes,
                        disk=s.disk_allocated,
                        settlementFingerprint=settlement_digest(s.settlement_fingerprint),
                        roots=s.required_roots,
                        framework=s.discovered_framework_dirs,
                    )
                else:
                    if mode == "slow" and begin - started > c["fixtureFaultAfter"]:
                        if finish_if_requested(c["fixtureSlowSeconds"]):
                            return 0
                    if mode == "monitor-failure" and begin - started > c["fixtureFaultAfter"]:
                        return 7
                    payload = dict(
                        decision={"admitted": True, "reasons": []},
                        startDecision={"admitted": True, "reasons": []},
                        free=200 * 2**30,
                        scratch=0,
                        total=0,
                        disk=0,
                        settlementFingerprint=["fixture"],
                        roots=[],
                        framework=[],
                    )
                lines, digest = ledger_frames(head, event_seq, dict(
                    collectionStarted=begin,
                    collectionFinished=time.monotonic(),
                    **payload,
                ), sent_inventory)
                for line in lines:
                    print(line, flush=True)
                    event_seq += 1
                sent_inventory = digest
                cycle += 1
            except Exception as exc:
                emit(
                    "ledger_error",
                    collectionStarted=begin,
                    collectionFinished=time.monotonic(),
                    error=str(exc),
                )
                faulted = True
            if faulted and mode == "native":
                # Native GC is off: a handled fault's exception <-> traceback/frame cycles (chained causes
                # included) would keep the failed scan's data. Reclaim once, after the except block ends.
                gc.collect()

            # Native inventory remains back-to-back as before; fixture cadence stays bounded.
            if mode != "native" and finish_if_requested(c["fixtureMonitorInterval"]):
                return 0
    except MonitorProtocolError as exc:
        emit("monitor_protocol_error", reason=str(exc))
        return 2



def inventory_frames_controlled(head,seq,fields,sent,service):
    # Only the lossless logical roots/framework record crosses this pipe, never leaf diagnostics.
    from transport import json_chunks
    raw=b''.join(json_chunks(dict(framework=fields['framework'],roots=fields['roots']),
                             limit=2**20,service=service)).rstrip(b'\n')
    full=dict(head,kind='ledger',seq=seq,**fields)
    encoded=b''.join(json_chunks(full,limit=2**20,service=service)).rstrip(b'\n')
    if len(encoded)+1<=MAX_EVENT_BYTES:
        yield encoded.decode(),sent
        return
    digest=hashlib.sha256(raw).hexdigest()
    index=0
    if digest!=sent:
        parts=-(-len(raw)//2048)
        for index in range(parts):
            service()
            line=encode(dict(head,kind='inventory',seq=seq+index,inventory=digest,
                             part=index+1,parts=parts,bytes=len(raw),
                             data=base64.b64encode(raw[index*2048:(index+1)*2048]).decode()))
            if len(line)+1>MAX_EVENT_BYTES:raise ValueError('monitor_frame_limit')
            yield line,sent
        index=parts
    compact=dict(fields);compact.pop('roots');compact.pop('framework')
    line=encode(dict(head,kind='ledger',seq=seq+index,inventory=digest,**compact))
    if len(line)+1>MAX_EVENT_BYTES:raise ValueError('monitor_frame_limit')
    yield line,digest


def archive_main(c,mode):
    from archive_catalog import fixture_path
    from collection_control import bind_archive,collect_controlled,Control,Cancelled
    from transport import write_until
    from native_config import validate_archive_config
    validate_archive_config(c)
    if mode=='archive-fixture':
        # A harmless selected collector path is confined to this candidate's exact NEW private tree.
        fixture=Path(c['archiveAccounting']['currentSourceRoot']).parent
        for path in (*c['ownedRoots'].values(),c['archiveAccounting']['directory'],c['frameworkTempRoot'],
                     c['evidence'],c['archiveAccounting']['currentSourceRoot']):
            fixture_path(Path(path),fixture)
    elif mode=='native':validate_monitor_access(c)
    else:raise MonitorProtocolError('archive_monitor_mode')
    nonce=c.get('runNonce')
    if type(nonce) is not str or len(nonce)!=32 or nonce.strip('0123456789abcdef'):
        raise MonitorProtocolError('invalid_run_nonce')
    input_fd=sys.stdin.fileno();os.set_blocking(input_fd,False)
    start=bytearray();start_end=time.monotonic()+2
    while not start.endswith(b'\n'):
        if time.monotonic()>start_end:raise MonitorProtocolError('monitor_start_binding')
        if select.select([input_fd],[],[],max(0,start_end-time.monotonic()))[0]:
            part=os.read(input_fd,17)
            if not part:raise MonitorProtocolError('monitor_start_binding')
            start.extend(part)
            if len(start)>16:raise MonitorProtocolError('monitor_start_binding')
    if start!=b'start\n':raise MonitorProtocolError('monitor_start_binding')
    head=dict(nonce=nonce,pid=os.getpid(),startMonotonic=time.monotonic(),instance=secrets.token_hex(16))
    seq=1;cycle=0;total=0;buffer=bytearray();finishing=False;finish_end=None;busy=False
    control=None;last_work=-1;last_progress=time.monotonic()
    def poll():
        nonlocal finishing,finish_end
        if select.select([input_fd],[],[],0)[0]:
            part=os.read(input_fd,4097)
            if not part:raise MonitorProtocolError('monitor_control_eof')
            buffer.extend(part)
        if len(buffer)>4096:raise MonitorProtocolError('monitor_command_too_large')
        if b'\n' in buffer:
            if not buffer.endswith(b'\n') or buffer.count(b'\n')!=1:raise MonitorProtocolError('monitor_command_framing')
            row=json.loads(buffer);buffer.clear()
            expected=dict(command='finish',nonce=nonce,pid=head['pid'],instance=head['instance'],commandSeq=1)
            if row!=expected or type(row.get('pid')) is not int or type(row.get('commandSeq')) is not int or finishing:
                raise MonitorProtocolError('monitor_finish_binding')
            finishing=True;finish_end=time.monotonic()+2
    def service():
        nonlocal last_work,last_progress
        poll()
        if finish_end is not None and time.monotonic()>finish_end:raise TimeoutError('monitor_finish_deadline')
        if not busy and control is not None:
            if finishing:raise Cancelled('controlled_collection_cancelled')
            now=time.monotonic()
            if control.completed>last_work and now-last_progress>=c.get('fastDeadline',.5)/2:
                last_work=control.completed;last_progress=now
                emit('progress',cycle=cycle,collectionStarted=control.started,
                     phase=control.phase,completed=control.completed)
    def send(line):
        nonlocal total,busy
        raw=(line+'\n').encode()
        if len(raw)>4096:raise ValueError('monitor_frame_limit')
        total+=len(raw)
        if total>2**20:raise ValueError('monitor_output_limit')
        old=busy;busy=True
        try:
            end=finish_end if finishing else (control.deadline if control else time.monotonic()+2)
            write_until(sys.stdout.buffer,(raw,),deadline=end,service=service,quantum=.25)
        finally:busy=old
    def emit(kind,**fields):
        nonlocal seq
        send(encode(dict(head,kind=kind,seq=seq,**fields)));seq+=1
    sent=None
    # Full byte/member verification is pre-admission, never a recurring fast-loop hash.
    binding=bind_archive(c)
    emit('ready',mode=mode)
    try:
        while True:
            poll()
            if finishing:break
            cycle+=1;last_work=-1
            begin=time.monotonic()
            control=Control(service=lambda phase,completed:service(),clock=time.monotonic,
                            deadline=begin+5,fast=c.get('fastDeadline',.5),caps=c['archiveAccounting']['caps'])
            snapshot=collect_controlled(c,binding=binding,control=control)
            control.check('encode')
            def encoding_service():control.check('encode',work=1)
            decision=evaluate_pilot(snapshot,limits(c),c,now=time.monotonic(),preflight=False)
            start_decision=evaluate_pilot(snapshot,limits(c),c,now=time.monotonic(),preflight=True)
            from transport import json_chunks
            fingerprint=hashlib.sha256()
            for raw in json_chunks(snapshot.settlement_fingerprint,limit=None,service=encoding_service):
                # Same json.dumps default-space encoding as the inherited logical fingerprint.
                fingerprint.update(raw[:-1] if raw==b'\n' else raw)
            payload=dict(cycle=cycle,collectionStarted=snapshot.started,collectionFinished=snapshot.finished,
                decision=decision,startDecision=start_decision,free=snapshot.free_bytes,
                scratch=snapshot.scratch_allocated,total=snapshot.allocated_bytes,disk=snapshot.disk_allocated,
                settlementFingerprint=fingerprint.hexdigest(),roots=snapshot.required_roots,
                framework=snapshot.discovered_framework_dirs)
            first_seq=seq
            for line,digest in inventory_frames_controlled(head,first_seq,payload,sent,encoding_service):
                # Progress frames can advance seq between encoder checkpoints.
                row=json.loads(line);row['seq']=seq
                send(encode(row));seq+=1
                sent=digest
            control=None
    except Cancelled:
        if not finishing:raise
    except Exception as exc:
        emit('ledger_error',collectionStarted=control.started if control else time.monotonic(),
             collectionFinished=time.monotonic(),error=str(exc))
        return 2
    # All collection cursors have unwound; every started output frame finished. No background writer exists.
    control=None
    emit('finish_ack',command='finish',commandSeq=1,outcome='finished')
    return 0


if __name__ == "__main__":
    sys.exit(main())
