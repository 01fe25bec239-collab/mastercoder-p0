# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Trusted bounded host transport fixture. PLUMBING_ONLY; no VZ or hostile payload."""
import hashlib,json,os,select,sys,time,uuid
from pathlib import Path
c=json.loads(Path(sys.argv[1]).read_text());mode=sys.argv[2];seq=0;command_seq=0;start=None;cancel=False;callback=False;state='stopped';instance='trusted-fixture-object'
preparing=mode.startswith('preparing')
if preparing:instance=None
owner_start=str(uuid.uuid4());exact_stop=False;faulted=False
operational=c.get('nativeMode') in ('reinstall-existing','baseline');boot=c.get('nativeMode')=='baseline'
config_pin=hashlib.sha256(Path(sys.argv[1]).read_bytes()).hexdigest()
fd=os.open(c['ownerJournalPath'],os.O_APPEND|os.O_WRONLY|os.O_NOFOLLOW)
def emit(event,**fields):
    global seq
    seq+=1;v=dict(event=event,seq=seq,nonce=c['runNonce'],pid=os.getpid(),ownerStart=owner_start,instance=instance,stateName=state,**fields)
    raw=(json.dumps(v)+'\n').encode();os.write(fd,raw);os.fsync(fd);sys.stdout.buffer.write(raw);sys.stdout.buffer.flush()
emit('preparation_started' if preparing else 'ready');last_emit=time.monotonic();cancel_time=None;malformed=False
def fault(reason):
    global faulted,cancel,cancel_time
    if faulted:return
    faulted=True;emit('transport_fault',reason=reason,outcome='UNKNOWN')
    if start is not None and not callback and not cancel:
        cancel=True;cancel_time=time.monotonic();emit('cancel_requested',reason=reason)
while True:
    now=time.monotonic()
    if select.select([sys.stdin],[],[],c['controlInterval'])[0]:
        line=sys.stdin.readline()
        if not line:break
        try:
            cmd=json.loads(line)
            if (not isinstance(cmd,dict) or not set(cmd)<= ({'command','nonce','pid','instance','ownerStart','commandSeq','reason'} | ({'token','configSHA256','collectionStarted','recordSeq'} if operational else set()))
                    or cmd.get('nonce')!=c['runNonce'] or type(cmd.get('pid')) is not int or cmd['pid']!=os.getpid()
                    or cmd.get('ownerStart')!=owner_start or cmd.get('instance')!=(instance or '')
                    or type(cmd.get('commandSeq')) is not int or cmd['commandSeq']!=command_seq+1
                    or cmd.get('command') not in (('prepare_finish','heartbeat') if preparing else ('begin','cancel','finish','heartbeat'))):
                raise ValueError('fixture_command_binding')
        except (ValueError,TypeError):
            fault('malformed_stale_or_invalid_command');continue
        if operational and (hashlib.sha256(str(cmd.get('token','')).encode()).hexdigest()!=c['ownerTokenSHA256'] or cmd.get('configSHA256')!=config_pin):
            fault('fixture_capability_binding');continue
        command_seq=cmd['commandSeq'];name=cmd['command']
        if name=='prepare_finish':
            if mode=='preparing-missing':break
            emit('preparation_finish_ack',command='prepare_finish',commandSeq=command_seq+(1 if mode=='preparing-wrong' else 0),objectCreated=False,vmStarted=False);break
        if name=='begin':
            if start is not None or faulted or state!='stopped':fault('malformed_stale_or_invalid_command');continue
            if operational and (not 0<=time.monotonic()-cmd.get('collectionStarted',0)<=c['fullDeadline'] or not cmd.get('recordSeq')):
                fault('fixture_admission_binding');continue
            emit('command_ack',command=name,commandSeq=command_seq)
            start=now;state='running';emit('boot_begin' if boot else 'install_begin')
        elif name=='cancel':
            emit('command_ack',command=name,commandSeq=command_seq)
            if start is None:fault('cancel_before_begin')
            elif not callback and not cancel:
                cancel=True;cancel_time=now;emit('cancel_requested',reason=cmd.get('reason') or 'supervisor_cancel')
        elif name=='finish':
            if state!='stopped' or (start is not None and (not callback or not exact_stop)):
                fault('malformed_stale_or_invalid_command');continue
            if mode=='missing_ack':break
            if mode=='channel_loss':sys.exit(8)
            if mode=='wait_timeout':time.sleep(2);break
            emit('finish_ack',command='finish',commandSeq=command_seq+(1 if mode=='wrong_ack' else 0),
                 outcome=('BOOT_COMPLETED' if boot else 'INSTALLED') if start is not None and callback and not faulted and not cancel else 'UNKNOWN_OR_NOT_INSTALLED');break
        elif faulted:fault('malformed_stale_or_invalid_command')
    now=time.monotonic()
    if start is not None and mode=='owner-failure' and now-start>=c['fixtureFaultAfter']:sys.exit(8)
    if start is not None and mode=='malformed' and not malformed and now-start>=c['fixtureFaultAfter']:
        sys.stdout.buffer.write(b'{malformed\n');sys.stdout.buffer.flush();malformed=True
    if start is not None and not callback:
        if mode=='stopped-without-callback' and now-start>=c['fixtureCallbackAfter']:state='stopped'
        elif mode!='stopped-without-callback' and ((cancel_time is not None and now-cancel_time>=c['fixtureCancelDelay']) or now-start>=c['fixtureCallbackAfter']):
            state='stopped';callback=True;emit(('boot_callback_error' if boot else 'install_callback_error') if cancel or mode=='callback-error' else ('boot_callback_success' if boot else 'install_callback_success'));exact_stop=True;emit('exact_owner_stopped_after_callback')
    if now-last_emit>=c['progressInterval']:
        if not (mode=='lost-heartbeat' and start is not None and now-start>=c['fixtureFaultAfter'] and not callback):
            emit('owner_heartbeat',fraction=0.5 if start else 0,commandSeq=command_seq,active=start is not None and not callback,callbackReceived=callback,fault=faulted)
        last_emit=now
os.close(fd)
sys.exit(2 if mode=='unexpected_exit' else 1 if mode=='unfaulted_exit1' or faulted else 0)
