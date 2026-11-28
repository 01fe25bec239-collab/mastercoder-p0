# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""One r10 foreground host, r7 owner/monitor/recorder protocol; native dispatch disabled."""
import argparse,base64,binascii,hashlib,json,os,select,signal,socket,stat,subprocess,sys,time
from pathlib import Path
if __name__=='__main__':sys.path.insert(0,str(Path(__file__).absolute().parent))
from config import digest
from proof import Supervisor,atomic,client
from live_protocol import Outcome,ProtocolError
from native_config import load_native,native_argv,validate_operational
from transport import write_bounded,read_ack
from pilot_policy import FULL5_POLICY

SOURCES=('config.py','proof.py','native_config.py','native_control.py','live_protocol.py','monitor.py','recorder.py','fixture_owner.py','transport.py','live_ledger.py','resource_ledger.py','resource_check.py','pilot_policy.py','VMOwner.swift','VMOwner','native-config.json','NativePolicy.swift','AuxiliaryCopy.swift','prebegin-native-config.json','prebegin-launch.sh','prebegin-finish.sh','prebegin-status.sh','archive_catalog.py','archive_capsule.py','collection_control.py','AuxiliaryPrestage.swift','AuxiliaryPrestage','_metawalk.c','_metawalk.cpython-314-darwin.so')  # compiled collector is pinned like VMOwner

INVENTORY_FIELDS={'kind','nonce','pid','startMonotonic','instance','seq','inventory','part','parts','bytes','data'}
INVENTORY_FAILED=dict(inventory=None,parts=0,bytes=0,part=-1,raw=b'')  # never matches a next part

def integration_digest(root):
    h=hashlib.sha256()
    for name in SOURCES:h.update(name.encode()+b'\0'+bytes.fromhex(digest(root/name)))
    profile=root/'operational-native-config.json'
    h.update(b'operational-native-config.json\0'+(bytes.fromhex(digest(profile)) if profile.exists() else b'ABSENT'))
    return h.hexdigest()

# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
_STAGE_ID_KEYS={'device','inode','uid','gid','mode','nlink','size','mtimeNs','ctimeNs'}
_STAGE_RESULT_KEYS={'version','nonce','manifestPath','manifestSHA256','sourcePath','sourceSHA256','directoryPath','leafPath','directoryIdentity','leafIdentity','sourceIdentity'}

def writer_plan(c,auxiliary_bytes):
    """r10 F4: one disjoint byte plan for every durable writer inside the configured run_bytes. Fixed files (live status, receipts,
    run config, client; each atomic write is bounded to output_bytes and may coexist with its .pending copy) get 8*output_bytes;
    the prebegin private auxiliary copy is charged as staged; the rest is split between the native owner journal (1/8) and the
    recorder event journal. Each journal keeps a closure reserve that optional telemetry never uses. Too small fails closed."""
    fixed=8*c['output_bytes'];available=c['run_bytes']-fixed-auxiliary_bytes
    journal=available//8;events=available-journal
    plan=dict(fixed=fixed,auxiliary=auxiliary_bytes,journal=journal,journalReserve=min(journal//4,64*c['request_bytes']),
              events=events,eventsReserve=min(events//4,4*c['output_bytes']))
    if plan['journalReserve']<16*c['request_bytes'] or plan['eventsReserve']<c['output_bytes']:raise ValueError('run_budget_too_small')
    assert fixed+auxiliary_bytes+journal+events<=c['run_bytes']
    return plan

def _stage_identity(s):
    return dict(device=s.st_dev,inode=s.st_ino,uid=s.st_uid,gid=s.st_gid,mode=s.st_mode,nlink=s.st_nlink,size=s.st_size,mtimeNs=s.st_mtime_ns,ctimeNs=s.st_ctime_ns)

def _stage_regular(path,limit,*,service=None,hash_only=False):
    import stat
    path=Path(path)
    if not path.is_absolute() or str(path.resolve())!=str(path):raise ValueError('stage_noncanonical')
    fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
    try:
        before=os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_uid!=os.getuid() or before.st_nlink!=1 or before.st_mode&0o022 or not 0<before.st_size<=limit:raise ValueError('stage_regular')
        raw=bytearray();hasher=hashlib.sha256();size=0
        while size<=limit:
            if service:service()
            part=os.read(fd,min(65536,limit+1-size))
            if not part:break
            size+=len(part)
            if hash_only:hasher.update(part)
            else:raw.extend(part)
        if service:service()
        if size>limit or _stage_identity(before)!=_stage_identity(os.fstat(fd)) or _stage_identity(before)!=_stage_identity(path.lstat()):raise ValueError('stage_read_race')
        return (hasher.hexdigest() if hash_only else bytes(raw)),before
    finally:os.close(fd)

def _stage_checkpoint(*,expires,free_floor,directory,cancel,cancel_error='stage_cancelled'):
    import shutil
    last=time.monotonic()
    def service():
        nonlocal last
        now=time.monotonic()
        if now-last>.5:raise ValueError('stage_service_gap')
        last=now
        if cancel():raise ValueError(cancel_error)
        if now>=expires:raise ValueError('stage_admission_expired')
        if shutil.disk_usage(directory).free<free_floor:raise ValueError('stage_free_floor')
        now=time.monotonic()
        if now-last>.5:raise ValueError('stage_service_gap')
        if now>=expires:raise ValueError('stage_admission_expired')
    return service

def _stage_capacity_child():
    from live_ledger import collect
    from pilot_policy import evaluate_pilot
    from native_config import limits
    raw=sys.stdin.buffer.readline(4097)
    if len(raw)>4096 or not raw.endswith(b'\n'):raise ValueError('stage_capacity_request_limit')
    q=json.loads(raw)
    if type(q) is not dict or any(type(v) is not str for v in q.values()) or len(q.get('nonce',''))!=32 or any(ch not in '0123456789abcdef' for ch in q.get('nonce','')) or set(q)!={'version','nonce','configPath','configSHA256'} or q['version']!='w03-aux-capacity-request-v1':raise ValueError('stage_capacity_request')
    raw,_=_stage_regular(q['configPath'],1024*1024)
    if hashlib.sha256(raw).hexdigest()!=q['configSHA256']:raise ValueError('stage_config_drift')
    c=load_native(Path(q['configPath']),prebegin=True)
    sequence=0;last_emit=0.0;completed_by_phase={}
    def progress(phase,completed):
        nonlocal sequence,last_emit
        if phase not in ('collect','validate') or type(completed) is not int or completed<=completed_by_phase.get(phase,0):return
        completed_by_phase[phase]=completed
        now=time.monotonic()
        if now-last_emit<.25:return
        sequence+=1;last_emit=now
        frame=dict(version='w03-aux-prestage-progress-v1',nonce=q['nonce'],sequence=sequence)
        sys.stdout.buffer.write(json.dumps(frame,separators=(',',':')).encode()+b'\n');sys.stdout.buffer.flush()
    sample=collect(c,progress=progress);decision=evaluate_pilot(sample,limits(c),c,now=time.monotonic(),preflight=True)
    row=dict(version='w03-aux-capacity-result-v1',nonce=q['nonce'],configSHA256=q['configSHA256'],pid=os.getpid(),started=sample.started,finished=sample.finished,free=sample.free_bytes,scratch=sample.scratch_allocated,total=sample.allocated_bytes,decision=decision)
    raw=json.dumps(row,allow_nan=False).encode()+b'\n'
    if len(raw)>4096:raise ValueError('stage_capacity_frame')
    sys.stdout.buffer.write(raw);sys.stdout.buffer.flush()

def _stage_run(argv,request,*,expires,free_floor,pass_fds=(),cancel):
    import shutil
    raw=json.dumps(request,separators=(',',':'),allow_nan=False).encode()+b'\n'
    if len(raw)>4096:raise ValueError('stage_request_limit')
    p=subprocess.Popen(argv,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,env={},close_fds=True,pass_fds=pass_fds)
    buffer=b'';result=None;last=time.monotonic();service=last;output=0;sequence=0;close_deadline=None
    try:
        write_bounded(p.stdin,raw,.5);os.set_blocking(p.stdout.fileno(),False)
        while True:
            now=time.monotonic()
            if now-service>.5:raise ValueError('stage_service_gap')
            service=now
            if cancel():raise ValueError('stage_cancelled')
            if now>=expires:raise ValueError('stage_admission_expired')
            if now-last>3:raise ValueError('stage_helper_stale')
            if shutil.disk_usage(Path(request['configPath']).parent if request['version']=='w03-aux-capacity-request-v1' else request['directoryPath']).free<free_floor:raise ValueError('stage_free_floor')
            ready,_,_=select.select([p.stdout],[],[],min(.05,max(0,expires-now)))
            if ready:
                chunk=os.read(p.stdout.fileno(),4097);output+=len(chunk)
                if output>1024*1024 or len(buffer)+len(chunk)>8192:raise ValueError('stage_output_limit')
                if not chunk:
                    close_deadline=time.monotonic()+2
                    if buffer or result is None:raise ValueError('stage_helper_failed')
                    while True:
                        now=time.monotonic()
                        if now>=close_deadline:raise ValueError('stage_unreaped_UNKNOWN')
                        try:status=p.wait(timeout=min(.05,close_deadline-now))
                        except subprocess.TimeoutExpired:status=None
                        now=time.monotonic()
                        if now-service>.5:raise ValueError('stage_service_gap')
                        service=now
                        if cancel():raise ValueError('stage_cancelled')
                        now=time.monotonic()
                        if now-service>.5:raise ValueError('stage_service_gap')
                        if now>=expires:raise ValueError('stage_admission_expired')
                        if now-last>3:raise ValueError('stage_helper_stale')
                        if status is not None:
                            if status!=0:raise ValueError('stage_helper_failed')
                            return result
                        if shutil.disk_usage(Path(request['configPath']).parent if request['version']=='w03-aux-capacity-request-v1' else request['directoryPath']).free<free_floor:raise ValueError('stage_free_floor')
                buffer+=chunk
                while b'\n' in buffer:
                    line,buffer=buffer.split(b'\n',1)
                    if len(line)+1>4096 or result is not None:raise ValueError('stage_frame_limit')
                    row=json.loads(line)
                    if type(row) is not dict or row.get('nonce')!=request['nonce']:raise ValueError('stage_nonce')
                    last=time.monotonic()
                    if row.get('version')=='w03-aux-prestage-progress-v1':
                        if set(row)!={'version','nonce','sequence'} or type(row['sequence']) is not int or row['sequence']!=sequence+1:raise ValueError('stage_progress')
                        sequence+=1
                    elif row.get('version')=='w03-aux-capacity-result-v1':
                        if set(row)!={'version','nonce','configSHA256','pid','started','finished','free','scratch','total','decision'} or row['pid']!=p.pid or row['configSHA256']!=request['configSHA256']:raise ValueError('stage_capacity_binding')
                        if any(type(row[k]) is not int or row[k]<0 for k in ('free','scratch','total')) or any(type(row[k]) not in (int,float) for k in ('started','finished')) or not 0<row['started']<=row['finished']<=last or last-row['started']>5:raise ValueError('stage_capacity_time')
                        result=row
                    elif row.get('version')=='w03-aux-prestage-result-v1':result=row
                    else:raise ValueError('stage_result_version')
    finally:
        try:
            if p.poll() is None:
                close_deadline=close_deadline or time.monotonic()+2
                p.terminate()
                while True:
                    now=time.monotonic()
                    if now>=close_deadline:raise ValueError('stage_unreaped_UNKNOWN')
                    try:
                        p.wait(timeout=min(.05,close_deadline-now));break
                    except subprocess.TimeoutExpired:
                        # Reap remains bounded even after a guard has already refused.
                        cancel()
                        shutil.disk_usage(Path(request['configPath']).parent if request['version']=='w03-aux-capacity-request-v1' else request['directoryPath'])
        finally:
            for stream in (p.stdin,p.stdout):stream.close()

def _stage_check(result,request,*,service=None):
    import stat
    if type(result) is not dict or set(result)!=_STAGE_RESULT_KEYS or result['version']!='w03-aux-prestage-result-v1':raise ValueError('stage_result_shape')
    for key in ('nonce','manifestPath','manifestSHA256','sourcePath','sourceSHA256','directoryPath','sourceIdentity'):
        if result[key]!=request[key]:raise ValueError('stage_result_binding')
    if result['leafPath']!=result['directoryPath']+'/Auxiliary.private.bin':raise ValueError('stage_leaf_path')
    for key in ('directoryIdentity','leafIdentity','sourceIdentity'):
        if type(result[key]) is not dict or set(result[key])!=_STAGE_ID_KEYS or any(type(v) is not int or v<0 for v in result[key].values()):raise ValueError('stage_identity_shape')
    parent=Path(result['directoryPath']);leaf=Path(result['leafPath'])
    if str(parent.resolve())!=str(parent):raise ValueError('stage_parent_path')
    for key,path in (('directoryIdentity',parent),('leafIdentity',leaf),('sourceIdentity',Path(result['sourcePath']))):
        if result[key]!=_stage_identity(path.lstat()):raise ValueError('stage_identity_changed')
    ds=parent.lstat();ls=leaf.lstat()
    if not stat.S_ISDIR(ds.st_mode) or ds.st_uid!=os.getuid() or ds.st_mode&0o7777!=0o700 or sorted(p.name for p in parent.iterdir())!=['Auxiliary.private.bin']:raise ValueError('stage_directory')
    if ls.st_dev!=ds.st_dev or not stat.S_ISREG(ls.st_mode) or ls.st_uid!=os.getuid() or ls.st_mode&0o7777!=0o600 or ls.st_nlink!=1 or ls.st_size!=request['expectedBytes']:raise ValueError('stage_leaf')
    options=dict(service=service,hash_only=True) if service else {}
    raw,_=_stage_regular(leaf,64*2**20,**options)
    if (raw if service else hashlib.sha256(raw).hexdigest())!=result['sourceSHA256']:raise ValueError('stage_copy_hash')
    raw,_=_stage_regular(result['manifestPath'],2**20,**options)
    if (raw if service else hashlib.sha256(raw).hexdigest())!=result['manifestSHA256']:raise ValueError('stage_manifest_drift')
    if result['directoryIdentity']!=_stage_identity(parent.lstat()):raise ValueError('stage_parent_race')

def prepare_auxiliary(config_path,n,c,*,cancel):
    import secrets,tempfile,shutil,stat
    from native_config import limits
    root=Path(config_path).parent
    print('PREPARING PRIVATE AUXILIARY: no operational authority',flush=True)
    manifest,ms=_stage_regular(n['expectedManifestPath'],2**20)
    if hashlib.sha256(manifest).hexdigest()!=n['expectedManifestSHA256']:raise ValueError('stage_manifest_hash')
    m=json.loads(manifest)
    if type(m) is not dict or type(m.get('schema')) is not int or m['schema']!=1 or m.get('lab')!=n['lab'] or m.get('bundlePath')!=n['bundlePath'] or m.get('imagePath')!=n['imagePath'] or set(m.get('files',{}))!={'Restore.ipsw','Guest.bundle/Hardware.bin','Guest.bundle/Machine.bin','Guest.bundle/Auxiliary.bin','Guest.bundle/Disk.img'}:raise ValueError('stage_manifest_scope')
    record=m['files']['Guest.bundle/Auxiliary.bin'];source=Path(n['bundlePath'])/'Auxiliary.bin'
    if record['path']!=str(source) or type(record['bytes']) is not int or not 0<record['bytes']<=64*2**20 or type(record['sha256']) is not str or len(record['sha256'])!=64 or any(ch not in '0123456789abcdef' for ch in record['sha256']):raise ValueError('stage_source_record')
    ss=source.lstat();identity=_stage_identity(ss)
    if not stat.S_ISREG(ss.st_mode) or ss.st_nlink!=1 or ss.st_uid!=os.getuid() or ss.st_mode&0o022 or ss.st_size!=record['bytes']:raise ValueError('stage_source_identity')
    reserve=((record['bytes']+4095)//4096)*4096+2**20+4*4096
    if record['bytes']+2**20+4*4096>=c['run_bytes']:raise ValueError('stage_run_size_limit')
    stage_started=time.monotonic();nonce=secrets.token_hex(16);profile=root/'prebegin-native-config.json'
    capacity=_stage_run([c['interpreter'],'-I','-B','-c','import sys;sys.path.insert(0,sys.argv[1]);from native_control import _stage_capacity_child;_stage_capacity_child()',str(root)],dict(version='w03-aux-capacity-request-v1',nonce=nonce,configPath=str(profile),configSHA256=digest(profile)),expires=stage_started+60,free_floor=n['freeFloor'],cancel=cancel)
    decision=capacity['decision']
    fields={'policy','admitted','reasons','remaining_bytes','projected_allocation_bytes','projected_free_bytes','assumptionsVerified','hardQuotaOrConfinementClaim','verifiedMaximumBoundClaim','free_gap_bytes','aggregate_gap_bytes','runway_bytes'}
    if (type(decision) is not dict or set(decision)!=fields or decision['policy']!=n['policy'] or decision['admitted'] is not True or decision['reasons']!=[] or decision['assumptionsVerified'] is not False or decision['hardQuotaOrConfinementClaim'] is not False or decision['verifiedMaximumBoundClaim'] is not False or decision['runway_bytes'] is not None or any(type(decision[k]) is not int or decision[k]<0 for k in ('remaining_bytes','projected_allocation_bytes','projected_free_bytes','free_gap_bytes','aggregate_gap_bytes')) or capacity['scratch']+reserve>=n['scratch'] or capacity['total']+reserve>=n['aggregate'] or capacity['free']-reserve<n['freeFloor'] or decision['projected_allocation_bytes']+reserve>=n['aggregate'] or decision['projected_free_bytes']-reserve<n['freeFloor']):raise ValueError('stage_capacity_refused')
    expires=min(stage_started+60,capacity['started']+n['fullDeadline'])
    if time.monotonic()>=expires or cancel():raise ValueError('stage_admission_expired')
    stage=Path(tempfile.mkdtemp(prefix='.aux-prestage-',dir=root));stage.chmod(0o700)
    fd=os.open(stage,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
    request=dict(version='w03-aux-prestage-request-v1',nonce=nonce,manifestPath=n['expectedManifestPath'],manifestSHA256=n['expectedManifestSHA256'],sourcePath=str(source),sourceSHA256=record['sha256'],sourceIdentity=identity,directoryPath=str(stage),directoryIdentity=_stage_identity(os.fstat(fd)),directoryFD=fd,expectedBytes=record['bytes'],maxBytes=n['prebeginAuxiliaryMaxBytes'])
    try:
        result=_stage_run([str(root/'AuxiliaryPrestage')],request,expires=expires,free_floor=n['freeFloor'],pass_fds=(fd,),cancel=cancel)
        service=_stage_checkpoint(expires=expires,free_floor=n['freeFloor'],directory=stage,cancel=cancel)
        _stage_check(result,request,service=service);service()
        if cancel() or time.monotonic()>=expires or _stage_identity(os.fstat(fd))!=result['directoryIdentity']:raise ValueError('stage_completion_expired')
        return dict(result=result,request=request,fd=fd,cancel=cancel,expires=expires,free_floor=n['freeFloor'])
    except BaseException:os.close(fd);raise

def transfer_auxiliary(stage,run):
    import stat
    result=stage['result'];request=stage['request'];fd=stage['fd']
    transfer_started=time.monotonic()
    if transfer_started>=stage['expires']:raise ValueError('stage_transfer_expired')
    service=_stage_checkpoint(expires=stage['expires'],free_floor=stage.get('free_floor',0),directory=Path(result['directoryPath']).parent,cancel=stage['cancel'],cancel_error='stage_transfer_cancelled')
    _stage_check(result,request,service=service);service()
    if stage['cancel']() or _stage_identity(os.fstat(fd))!=result['directoryIdentity']:raise ValueError('stage_transfer_cancelled')
    destination=run/'auxiliary'
    if destination.exists() or destination.is_symlink():raise ValueError('stage_destination_exists')
    if run.lstat().st_dev!=os.fstat(fd).st_dev:raise ValueError('stage_transfer_device')
    # renamex_np(RENAME_EXCL) refuses a concurrently created destination on macOS.
    import ctypes
    rename=ctypes.CDLL(None,use_errno=True).renamex_np
    rename.argtypes=[ctypes.c_char_p,ctypes.c_char_p,ctypes.c_uint];rename.restype=ctypes.c_int
    if rename(os.fsencode(result['directoryPath']),os.fsencode(destination),4)!=0:raise OSError(ctypes.get_errno(),'stage_exclusive_rename')
    changed=dict(result,directoryPath=str(destination),leafPath=str(destination/'Auxiliary.private.bin'),directoryIdentity=_stage_identity(os.fstat(fd)))
    before=result['directoryIdentity'];after=changed['directoryIdentity']
    if any(before[k]!=after[k] for k in ('device','inode','uid','gid','mode','nlink','size','mtimeNs')):raise ValueError('stage_transfer_parent_drift')
    updated=dict(request,directoryPath=str(destination))
    _stage_check(changed,updated,service=service);service()
    if stage['cancel']() or time.monotonic()>=stage['expires']:raise ValueError('stage_transfer_cancelled')
    service()
    os.close(fd);stage['fd']=None
    return changed


class NativeSupervisor(Supervisor):
    monitor_finish_sent=monitor_done=retry_spent=record_halted=False;records_dropped=0  # class defaults: hosts built without __init__ stay valid
    budget=owner_journal_identity=None;event_bytes=0  # r10 F4/F5: bound at spawn; unbound means fail closed
    @property
    def preparation(self):return self.prebegin or getattr(self,'operational',False)

    def __init__(self,config_path,*,fixture=False,fixture_faults=None,prebegin=False,operational=False):
        if prebegin and operational:raise ValueError('conflicting_profiles')
        self.prebegin=prebegin;self.operational=operational
        self.n=(load_native(Path(config_path).parent/'prebegin-native-config.json',prebegin=True) if prebegin
                else load_native(Path(config_path).parent/('operational-native-config.json' if operational else 'native-config.json'),operational=operational))
        self.fixture=fixture
        # Refuse before run-root creation, owner spawn, preflight or VZ metadata calls.
        if not fixture and not prebegin and self.n['nativeInstallationAllowed'] is not True:
            raise ValueError('native_dispatch_disabled')
        expected=json.loads((Path(config_path).parent/'integration.expected.json').read_text())
        if expected['integration_sha256']!=integration_digest(Path(config_path).parent):raise ValueError('integration_source_drift')
        if operational:
            from config import load
            control=load(config_path)
            if control['ack_seconds']>2 or control['stale_seconds']>3 or control['heartbeat_seconds']>.5:
                raise ValueError('operational_control_limits')
        self.auxiliary_stage=None
        if prebegin and not fixture:
            from config import load,source_digest
            root=Path(config_path).parent;c=load(config_path)
            launcher=json.loads((root/'launcher.expected.json').read_text())
            if launcher['source_sha256']!=source_digest(root) or launcher['config_sha256']!=digest(config_path):raise ValueError('source_config_drift')
            cancelled=[False];previous=signal.getsignal(signal.SIGINT)
            signal.signal(signal.SIGINT,lambda *_:cancelled.__setitem__(0,True))
            try:
                self.auxiliary_stage=prepare_auxiliary(config_path,self.n,c,cancel=lambda:cancelled[0] or bool(getattr(self,'stop_reason',None)))
                if cancelled[0]:raise ValueError('stage_cancelled')
            finally:signal.signal(signal.SIGINT,previous)
        try:super().__init__(config_path,fixture_faults)
        except BaseException:
            if self.auxiliary_stage is not None:os.close(self.auxiliary_stage['fd']);self.auxiliary_stage['fd']=None
            raise
        self.c['heartbeat_seconds']=min(self.c['heartbeat_seconds'],self.n['controlInterval'])
        self.integration=integration_digest(self.root)
        self.owner_state=None;self.owner_command=0;self.record_seq=0;self.monitor_seq=0
        self.monitor_identity=None;self.monitor_ack=None;self.ledger=None;self.last_full=None
        self.inventory=None;self.inventory_part=None;self.admitted=None;self.monitor_poison=None
        self.waits={};self.finish_sequence=None;self.owner_is_native=not fixture
        self.finish_attempted=False;self.control_clean=False;self.cancel_sent=False
        self.preparation_finish=None;self.preparation_ack=None
        self.owner_handoff_requested=False;self.owner_handoff_sent=False;self.owner_handoff_aborted=False
        self.record_busy=False;self.deferred_events=[];self.deferred_bytes=0;self.progress_state=None;self.completed_cycle=0;self.finish_end=None;self.next_owner_heartbeat=0;self.last_io_service=time.monotonic()
        self.settlement_at=None;self.settlement_fp=None;self.completion_settled=False
        self.begin_authorization=None;self.finish_requested=False

    def snapshot(self,phase):
        value=super().snapshot(phase)
        value.update(label='NATIVE_SOURCE_ONLY' if self.owner_is_native else 'NATIVE_ADAPTER_PLUMBING_ONLY',
                     integration_sha256=self.integration,installation='NOT_RUN',assumptionsVerified=False,
                     owner=self.owner_state.record() if self.owner_state else None,
                     monitor_identity=self.monitor_identity,monitor_sequence=self.monitor_seq,
                     child_waits=self.waits,monitor_failure=getattr(self,'monitor_failure',None),records_dropped=self.records_dropped)
        if getattr(self,'operational',False):
            value.update(operation=self.n['nativeMode'],no_begin=not (self.owner_state and self.owner_state.started),
                         installation='NOT_RUN' if not self.owner_state or not self.owner_state.started else self.owner_state.callback or 'ACTIVE')
        if self.preparation:
            value['preparation_stage']='PREPARING' if not self.owner_state or not self.owner_state.instance else 'READY_NO_BEGIN'
        return value

    def spawn(self):
        os.chdir(self.run)
        self.server=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM)
        self.server.bind('control.sock');os.chmod('control.sock',0o600)
        self.server.listen(2);self.server.setblocking(False)
        atomic(self.run/'client.json',dict(prototype=self.c['prototype'],token=self.token,nonce=self.nonce,identity=self.identity),limit=self.c['output_bytes'])
        c=dict(self.n,runNonce=self.nonce,ownerJournalPath=str(self.run/'owner.jsonl'),runDir=str(self.run),
               controllerPID=os.getpid(),controllerIdentity=self.identity)
        from live_ledger import capture_control_socket
        c['currentControlSocket']=capture_control_socket(self.server,c,self.run,self.nonce,self.identity)
        # Exact writers are declared; immutable assets and unknown files stay in settlement.
        files=('owner.jsonl','events.jsonl','status.json','shutdown.json','emergency.json')
        for name in files:(self.run/name).touch(mode=0o600)
        c['knownWriterPaths']=[str(self.run/name) for name in files if (self.run/name).exists()]
        # r6: an operational install/boot writes exactly these two bundle files; the monitor accounts them as identity-checked growing
        # writers and tolerates the installer's own framework directories (live_ledger installer phase). Prebegin/fixture: none.
        if getattr(self,'operational',False) and not self.prebegin and not self.fixture and self.n.get('nativeMode') in ('reinstall-existing','baseline'):
            bundle=Path(self.n['bundlePath']);c['installerWriterPaths']=[str(bundle/'Disk.img'),str(bundle/'Auxiliary.bin')]
        if self.prebegin:
            auxiliary=self.run/'auxiliary'
            if self.fixture:auxiliary.mkdir(mode=0o700)
            else:
                if self.auxiliary_stage is None:raise ValueError('stage_missing')
                c['prebeginAuxiliaryStage']=transfer_auxiliary(self.auxiliary_stage,self.run)
            self.auxiliary_parent=auxiliary.lstat()
            self.auxiliary_leaf=(_stage_identity((auxiliary/'Auxiliary.private.bin').lstat()) if not self.fixture else None)
            if self.auxiliary_leaf is not None:
                value=(auxiliary/'Auxiliary.private.bin').lstat();self.auxiliary_leaf=(value.st_dev,value.st_ino,value.st_uid,value.st_gid,value.st_mode)
            c['prebeginAuxiliaryCopyPath']=str(auxiliary/'Auxiliary.private.bin')
        if getattr(self,'operational',False):c['ownerTokenSHA256']=hashlib.sha256(self.token.encode()).hexdigest()
        c['fixtureMonitorInterval']=self.c['heartbeat_seconds']
        # r10 F4: every durable writer gets its disjoint portion of run_bytes; F5: the owner journal is the inode created above, and
        # VMOwner must open exactly that inode (the run config is digest-pinned like every other owner input).
        auxiliary=self.run/'auxiliary'
        self.budget=writer_plan(self.c,sum(p.lstat().st_size for p in auxiliary.iterdir()) if self.prebegin else 0)
        journal=(self.run/'owner.jsonl').lstat();self.owner_journal_identity=(journal.st_dev,journal.st_ino)
        c.update(ownerJournalDevice=journal.st_dev,ownerJournalInode=journal.st_ino,
                 ownerJournalMaxBytes=self.budget['journal'],ownerJournalReserveBytes=self.budget['journalReserve'])
        self.run_config=self.run/'run-config.json';atomic(self.run_config,c,limit=self.c['output_bytes'])
        # Bind the parent after every fixed startup name exists in both modes. APFS counts
        # directory entries in st_nlink; the earlier capture cannot authorize a changed parent.
        c['currentControlSocket']=capture_control_socket(self.server,c,self.run,self.nonce,self.identity)
        atomic(self.run_config,c,limit=self.c['output_bytes'])

        commands={'recorder':[self.c['interpreter'],'-I','-B',str(self.root/'recorder.py'),str(self.run/'events.jsonl')]+(['fragmented'] if self.archive_mode() else ['ordinary',str(self.budget['events'])]),
                  'monitor':[self.c['interpreter'],'-I','-B',str(self.root/'monitor.py'),str(self.run_config),self.faults.get('monitor','archive-fixture' if self.archive_mode() else 'fixture') if self.fixture else 'native'],'owner':None}
        for role,command in commands.items():
            if role=='owner':
                c.update(controllerPID=os.getpid(),monitorPID=self.children['monitor'].pid)
                atomic(self.run_config,c,limit=self.c['output_bytes'])
                command=([self.c['interpreter'],'-I','-B',str(self.root/'fixture_owner.py'),str(self.run_config),self.faults.get('owner','success')]
                         if self.fixture else native_argv(c,self.run_config))  # GUARD_FINAL_RUN_CONFIG
            p=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,env={},close_fds=True)
            self.children[role]=p;self.buffers[role]=b''
            if role!='recorder':os.set_blocking(p.stdout.fileno(),False)
        self.owner_state=Outcome(self.nonce,self.children['owner'].pid,operation=self.n['nativeMode'] if getattr(self,'operational',False) else 'reinstall-existing')
        self.run_config_digest=digest(self.run_config)
        write_bounded(self.children['monitor'].stdin,b'start\n',self.c['ack_seconds'])
        self.record('binding',children={role:p.pid for role,p in self.children.items()},native=self.owner_is_native)

    def archive_mode(self):
        from pilot_policy import ARCHIVE5_POLICY
        return self.n.get('policy')==ARCHIVE5_POLICY

    def save_status(self,phase):
        if phase in ('FINAL','RETAINED_UNKNOWN') and getattr(self,'final_receipt',None) is not None:
            return atomic(self.run/'status.json',self.final_receipt,limit=self.c['output_bytes'])
        if self.n.get('policy')==FULL5_POLICY and phase=='LIVE':
            import stat
            from recorder import append
            path=self.run/'status.json'
            raw=(json.dumps(self.snapshot(phase),sort_keys=True,allow_nan=False)+'\n').encode()
            if len(raw)>self.c['output_bytes']:raise ValueError('live_status_size')
            before=path.lstat()
            fd=os.open(path,os.O_WRONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
            try:
                current=os.fstat(fd)
                if (not stat.S_ISREG(current.st_mode) or current.st_uid!=os.getuid()
                        or stat.S_IMODE(current.st_mode)!=0o600 or current.st_nlink!=1
                        or (current.st_dev,current.st_ino)!=(before.st_dev,before.st_ino)):
                    raise ValueError('live_status_identity')
                # Declared live writer: partial readers get invalid/UNKNOWN JSON; success follows fsync.
                os.ftruncate(fd,0);append(fd,raw);os.fsync(fd)
            finally:os.close(fd)
            return
        if not self.archive_mode() or ('recorder' in self.waits):return super().save_status(phase)
        return self.record_archive(self.snapshot(phase),status='status.json')

    def record_archive(self,value,*,status=None):
        from transport import transaction
        if self.record_busy:raise ValueError('recorder_transaction_pending')
        self.record_seq+=1
        if status is None:value=dict(value,recordSeq=self.record_seq)
        end=min(time.monotonic()+self.c['ack_seconds'],self.finish_end or float('inf'))
        self.record_busy=True
        self.candidate_receipt_pending=(status is None and value.get('kind')=='monitor_event' and value.get('value',{}).get('kind')=='ledger')
        try:
            p=self.children['recorder']
            ack=transaction(p.stdin,p.stdout,value,sequence=self.record_seq,deadline=end,
                            max_bytes=self.c['output_bytes'],service=self.io_service,status=status)
            if self.finish_end is not None and time.monotonic()>self.finish_end:raise TimeoutError('finish_deadline')
            return ack
        except (ValueError,OSError,EOFError,TimeoutError) as exc:
            self.monitor_fault('monitor','recorder_transaction_failure:'+str(exc))
            raise
        finally:self.record_busy=False;self.candidate_receipt_pending=False

    def io_service(self):
        # No recursive recorder calls. Owner faults are applied before any future authority.
        now=time.monotonic()
        if now-self.last_io_service>.5:raise TimeoutError('controller_fast_service_gap')
        self.last_io_service=now
        self.drain_archive(0)
        if time.monotonic()>self.next_owner_heartbeat and not self.finish_attempted and not getattr(self,'owner_write_pending',False) and (self.owner_state.instance or (self.preparation and self.owner_state.owner_start and not self.owner_handoff_sent)):
            self.owner_command+=1
            state=self.owner_state
            name='handoff' if self.owner_handoff_requested and not state.instance else 'heartbeat'
            if name=='handoff':self.owner_handoff_sent=True
            cmd=dict(command=name,nonce=self.nonce,pid=state.pid,instance=state.instance or '',
                     ownerStart=state.owner_start,commandSeq=self.owner_command)
            self.authorize_owner_command(cmd)
            fd=self.children['owner'].stdin.fileno();os.set_blocking(fd,False)
            raw=(json.dumps(cmd)+'\n').encode()
            if len(raw)>self.c['request_bytes'] or os.write(fd,raw)!=len(raw):
                raise ValueError('owner_control_backpressure')
            self.defer_archive('owner_command',dict(command={k:v for k,v in cmd.items() if k!='token'}),None)
            self.next_owner_heartbeat=time.monotonic()+min(.25,self.n['controlInterval'])
        import shutil
        if shutil.disk_usage(self.run).free<self.c['free_bytes']:self.gate.fault('free_space_limit')
        self.serve_cooperative()

    def defer_archive(self,kind,value,received):
        size=len(json.dumps(value).encode())
        candidate=(kind=='monitor_event' and value.get('value',{}).get('kind')=='ledger')
        if size>self.c['output_bytes']:raise ValueError('recorder_logical_record_limit')
        charge=0 if candidate else size
        if self.deferred_bytes+charge>self.c['request_bytes']*2:raise ValueError('recorder_event_backpressure')
        if kind=='monitor_event' and value.get('value',{}).get('kind')=='ledger':
            if any(k=='monitor_event' and v.get('value',{}).get('kind')=='ledger' for k,v,_ in self.deferred_events):
                raise ValueError('recorder_candidate_pending')
        self.deferred_events.append((kind,value,received));self.deferred_bytes+=charge

    def drain_archive(self,duration):
        handles=[self.children[r].stdout for r in ('owner','monitor')]
        ready,_,_=select.select(handles,[],[],max(0,duration))
        for role in ('owner','monitor'):
            p=self.children[role]
            if p.stdout not in ready and b'\n' not in self.buffers[role]:continue
            # Backpressure the monitor while the single candidate receipt is pending.
            if role=='monitor' and (getattr(self,'candidate_receipt_pending',False) or any(k=='monitor_event' and v.get('value',{}).get('kind')=='ledger' for k,v,_ in self.deferred_events)):continue
            if p.stdout in ready:
                chunk=os.read(p.stdout.fileno(),self.c['request_bytes']+1);self.output+=len(chunk)
                if not chunk:
                    if not self.finish_attempted:self.monitor_fault(role,role+'_channel_lost')
                    if not self.buffers[role]:continue
                self.buffers[role]+=chunk
            if self.output>self.c['output_bytes'] or len(self.buffers[role])>self.c['request_bytes']*2:
                self.monitor_fault(role,'event_backpressure_limit');self.buffers[role]=b'';continue
            while b'\n' in self.buffers[role]:
                line,self.buffers[role]=self.buffers[role].split(b'\n',1)
                try:
                    e=self.decode_frame(line);received=time.monotonic()
                    if role=='owner':
                        if getattr(self,'operational',False) and e.get('event') in ('install_begin','boot_begin') and self.owner_state.begin_command is None:raise ValueError('unsolicited_begin')
                        self.owner_state.event(e)
                        self.owner_handoff_event(e)
                        for reason in self.owner_state.unknown:self.gate.fault(reason)
                        if self.preparation and e.get('event')=='preparation_finish_ack':
                            if (self.preparation_finish is None or e.get('command')!='prepare_finish'
                                    or e.get('commandSeq')!=self.preparation_finish or e.get('instance') is not None
                                    or e.get('objectCreated') is not False or e.get('vmStarted') is not False):
                                raise ValueError('preparation_finish_ack_binding')
                            self.preparation_ack=e
                    else:e=self.monitor_event(e,p.pid,received)
                    self.rows[role]=dict(monotonic_ns=time.monotonic_ns(),value=e)
                    # Liveness-only progress has no logical ledger or recorder admission.
                    if role!='monitor' or e['kind']!='progress':self.defer_archive(role+'_event',dict(value=e),received)
                    if role=='monitor' and e['kind']=='ledger':break
                except (ValueError,KeyError,TypeError,OSError,EOFError,TimeoutError) as exc:
                    self.monitor_fault(role,role+'_invalid_event:'+str(exc))

    def pump_archive(self,duration):
        self.drain_archive(duration)
        until=time.monotonic()+min(.25,self.n['controlInterval'])
        while self.deferred_events and time.monotonic()<=until:
            kind,value,received=self.deferred_events.pop(0)
            if kind!='monitor_event' or value.get('value',{}).get('kind')!='ledger':self.deferred_bytes-=len(json.dumps(value).encode())
            try:
                sequence=self.record_seq+1;ack=self.record(kind,**value)
                if kind=='monitor_event' and value['value']['kind']=='ledger':self.admit(value['value'],received,ack,sequence)
            except (ValueError,OSError,EOFError,TimeoutError) as exc:self.monitor_fault('monitor','recorder_failure:'+str(exc))
        self.io_service()

    def record(self,kind,**fields):
        # FIX3-D4: past run_size_limit only the high-volume liveness/ledger traffic is dropped; control/finish/binding records stay durable.
        optional=(kind=='owner_command' and fields.get('command',{}).get('command')=='heartbeat'
                  or kind=='owner_event' and fields.get('value',{}).get('event') in ('owner_heartbeat','install_progress')
                  or kind=='monitor_event' and fields.get('value',{}).get('kind') in ('ledger','inventory','progress'))
        if self.record_halted and optional:
            self.records_dropped+=1;return {'ack':None}
        if self.archive_mode():
            return self.record_archive(dict(kind=kind,nonce=self.nonce,**fields))
        # r10 F4: admission before any write, inside the event journal's portion. Optional traffic stops at the regular bound and the
        # closure reserve stays for control/finish records; beyond the hard bound nothing is sent (the recorder enforces it itself).
        if self.budget is None:raise ValueError('writer_budget_unbound')
        row=dict(recordSeq=self.record_seq+1,kind=kind,nonce=self.nonce,**fields)
        raw=(json.dumps(row)+'\n').encode()
        if optional and self.event_bytes+len(raw)>self.budget['events']-self.budget['eventsReserve']:
            self.record_halted=True;self.records_dropped+=1;return {'ack':None}
        if self.event_bytes+len(raw)>self.budget['events']:raise ValueError('recorder_budget_exhausted')
        self.record_seq+=1
        p=self.children['recorder']
        write_bounded(p.stdin,raw,self.c['ack_seconds'])
        ack=read_ack(p.stdout,self.c['ack_seconds'],self.c['request_bytes'])
        if ack!={'ack':self.record_seq}:raise ValueError('recorder_ack_sequence')
        self.event_bytes+=len(raw)
        return ack

    def monitor_event(self,e,pid,received):
        if not isinstance(e,dict):raise ValueError('monitor_event_shape')
        identity={k:e.get(k) for k in ('nonce','pid','startMonotonic','instance')}
        if identity['nonce']!=self.nonce or type(identity['pid']) is not int or identity['pid']!=pid:
            raise ValueError('monitor_identity')
        if type(identity['startMonotonic']) not in (int,float) or not 0<identity['startMonotonic']<=received or not isinstance(identity['instance'],str) or not identity['instance']:
            raise ValueError('monitor_start_identity')
        if self.monitor_identity is None:
            if e.get('kind')!='ready':raise ValueError('monitor_ready_missing')
            self.monitor_identity=identity
        elif identity!=self.monitor_identity:raise ValueError('monitor_start_identity')
        if type(e.get('seq')) is not int or e['seq']!=self.monitor_seq+1:
            raise ValueError('monitor_sequence')
        self.monitor_seq=e['seq']
        if self.archive_mode():
            common={'nonce','pid','startMonotonic','instance','seq','kind'}
            kind=e['kind']
            if kind=='ready' and (set(e)!=common|{'mode'} or e['mode'] not in ('native','archive-fixture')):
                raise ValueError('monitor_ready_fields')
            if kind=='finish_ack' and (set(e)!=common|{'command','commandSeq','outcome'} or e.get('command')!='finish' or e.get('outcome')!='finished' or not self.finish_attempted):
                raise ValueError('monitor_finish_binding')
            if kind=='ledger':
                expected=common|{'cycle','collectionStarted','collectionFinished','decision','startDecision','free','scratch','total','disk','settlementFingerprint'}
                expected|=({'inventory'} if 'inventory' in e else {'roots','framework'})
                if set(e)!=expected:raise ValueError('monitor_ledger_fields')
                if any(type(e.get(key)) is not int or e[key]<0 for key in ('free','scratch','total','disk')):
                    raise ValueError('monitor_ledger_numbers')
                for key in ('decision','startDecision'):
                    d=e.get(key)
                    if type(d) is not dict or type(d.get('admitted')) is not bool or type(d.get('reasons')) is not list or any(type(x) is not str for x in d['reasons']):
                        raise ValueError('monitor_decision_shape')
                if type(e.get('settlementFingerprint')) is not str or len(e['settlementFingerprint'])!=64 or e['settlementFingerprint'].strip('0123456789abcdef'):
                    raise ValueError('monitor_fingerprint')
        if e['kind']=='progress':
            self.monitor_progress(e,received)  # liveness only: never a ledger, admission, freshness or begin input
            return e
        if e['kind']=='ledger':
            if self.archive_mode():
                cycle=e.get('cycle')
                if type(cycle) is not int or cycle!=self.completed_cycle+1:raise ValueError('monitor_cycle')
                if self.progress_state is not None and (cycle,e.get('collectionStarted'))!=(self.progress_state['cycle'],self.progress_state['collectionStarted']):
                    raise ValueError('monitor_cycle_start')
                self.completed_cycle=cycle
                self.progress_state=None
            else:
                # Native ledgers carry no cycle field: bind any progress to this ledger's original start.
                if self.progress_state is not None and e.get('collectionStarted')!=self.progress_state['collectionStarted']:
                    raise ValueError('monitor_cycle_start')
                self.completed_cycle+=1;self.progress_state=None

            if self.inventory_part is not None:raise ValueError('monitor_inventory_incomplete')  # GUARD_INVENTORY_COMPLETE
            if 'inventory' in e:
                if 'roots' in e or 'framework' in e or self.inventory is None or e['inventory']!=self.inventory[0]:  # GUARD_INVENTORY_REFERENCE
                    raise ValueError('monitor_inventory_reference')
                e=dict(e,roots=list(self.inventory[1]),framework=list(self.inventory[2]))
            if type(e.get('collectionStarted')) not in (int,float) or type(e.get('collectionFinished')) not in (int,float) or not 0<e['collectionStarted']<=e['collectionFinished']<=received:
                raise ValueError('invalid_ledger_time')
            if received-e['collectionStarted']>self.n['fullDeadline']:  # GUARD_FRESHNESS
                raise ValueError('late_full_observation')
            if self.last_full is not None and received-self.last_full>self.n['fullDeadline']:
                raise ValueError('missing_full_observation')
            self.ledger=e  # diagnostic candidate only; admit() alone confers authority after the durable receipt
        elif e['kind']=='finish_ack':
            if e.get('commandSeq')!=1:raise ValueError('monitor_finish_ack_sequence')
            self.monitor_ack=e
        elif e['kind']=='inventory':
            try:self.monitor_inventory(e)
            except ValueError:self.inventory_part=INVENTORY_FAILED;raise  # GUARD_INVENTORY_STICKY: no ledger after a bad transfer
        elif e['kind']!='ready':
            # Retain only bounded error/phase diagnostics after identity and sequence validation.
            self.monitor_failure=dict(error=str(e.get('error',''))[:512],phase=str(e.get('phase',''))[:128])
            raise ValueError('monitor_collection_failure:'+json.dumps(self.monitor_failure,sort_keys=True))
        return e

    def monitor_progress(self,e,received):
        fields={'nonce','pid','startMonotonic','instance','seq','kind','cycle','collectionStarted','phase','completed'}
        if set(e)!=fields:raise ValueError('monitor_progress_fields')
        phases=('collect','validate','encode','write')
        cycle,start,phase,completed=(e.get(k) for k in ('cycle','collectionStarted','phase','completed'))
        if type(cycle) is not int or cycle!=self.completed_cycle+1 or type(start) not in (int,float) or not 0<start<=received or type(completed) is not int or completed<1 or phase not in phases:
            raise ValueError('monitor_progress_shape')
        old=self.progress_state
        if old is not None and (cycle!=old['cycle'] or start!=old['collectionStarted'] or completed<=old['completed'] or phases.index(phase)<phases.index(old['phase'])):
            raise ValueError('monitor_progress_order')
        self.progress_state=e

    def monitor_inventory(self,e):
        # Bounded, digest-bound, strictly ordered parts; a ledger may reference only a completed inventory.
        if set(e)!=INVENTORY_FIELDS:raise ValueError('monitor_inventory_fields')
        digest,part,parts,size,data=e['inventory'],e['part'],e['parts'],e['bytes'],e['data']
        if (not isinstance(digest,str) or len(digest)!=64 or digest.strip('0123456789abcdef') or not isinstance(data,str) or not data
                or any(type(v) is not int for v in (part,parts,size)) or not 1<=part<=parts<=size<=self.c['output_bytes']):  # GUARD_INVENTORY_BOUNDS
            raise ValueError('monitor_inventory_part')
        if part==1:
            if self.inventory_part is not None:raise ValueError('monitor_inventory_incomplete')
            self.inventory_part=dict(inventory=digest,parts=parts,bytes=size,part=0,raw=bytearray())
        p=self.inventory_part
        if p is None or (digest,parts,size,part)!=(p['inventory'],p['parts'],p['bytes'],p['part']+1):  # GUARD_INVENTORY_ORDER
            raise ValueError('monitor_inventory_order')
        try:chunk=base64.b64decode(data,validate=True)
        except binascii.Error as exc:raise ValueError('monitor_inventory_encoding') from exc
        if len(p['raw'])+len(chunk)>size:raise ValueError('monitor_inventory_size')  # GUARD_INVENTORY_SIZE
        p['raw']+=chunk;p['part']=part
        if part<parts:return
        if len(p['raw'])!=size or hashlib.sha256(p['raw']).hexdigest()!=digest:raise ValueError('monitor_inventory_digest')  # GUARD_INVENTORY_DIGEST
        try:value=json.loads(p['raw'])
        except RecursionError as exc:raise ValueError('monitor_inventory_content') from exc
        if (not isinstance(value,dict) or set(value)!={'framework','roots'}
                or not all(isinstance(v,list) and all(isinstance(x,str) for x in v) for v in value.values())):
            raise ValueError('monitor_inventory_content')
        self.inventory=(digest,tuple(value['roots']),tuple(value['framework']));self.inventory_part=None

    def monitor_fault(self,role,reason):
        # GUARD_MONITOR_STICKY: any monitor channel/framing/JSON/identity/sequence/validation/record fault ends admission for this run.
        if role=='monitor' and self.monitor_poison is None:self.monitor_poison=reason
        self.gate.fault(reason)

    def admit(self,e,received,ack,sequence):
        return self.admit_archive(e,received,ack=ack,sequence=sequence)

    def admit_archive(self,e,received,*,age_only=False,ack=None,sequence=None):
        # Admission and consumers share the original-age predicate and its mutation seam.
        if not age_only:
            # r11 F4: dropped/stale receipts also poison older authority before the next client is served.
            if (type(sequence) is not int or sequence!=self.record_seq or type(ack) is not dict
                    or set(ack)!={'ack'} or type(ack['ack']) is not int or ack['ack']!=sequence):  # GUARD_LEDGER_DURABLE_ACK
                self.monitor_fault('monitor','ledger_durable_ack_missing_or_wrong')
                raise ValueError('ledger_durable_ack_missing_or_wrong')
            if self.monitor_poison is not None:raise ValueError('monitor_admission_poisoned')  # GUARD_ADMIT_POISON
            if time.monotonic()-e['collectionStarted']>self.n['fullDeadline']:raise ValueError('late_durable_receipt')  # GUARD_DURABLE_FRESHNESS
        now=received if age_only else time.monotonic()
        if self.admitted is not None and now-self.admitted['collectionStarted']>self.n['fullDeadline']:
            if age_only:return True
            if self.archive_mode():raise ValueError('late_predecessor_handover')  # GUARD_PREDECESSOR_ORIGINAL_AGE
        if age_only:return self.admitted is None
        if self.archive_mode() and self.gate.unknown:raise ValueError('archive_authority_faulted')
        self.admitted=e;self.last_full=received

    def decode_frame(self,line):
        if len(line)+1>self.c['request_bytes']:raise ValueError('frame_limit')  # GUARD_FRAME_NEWLINE: the delimiter counts
        return json.loads(line)

    def current(self):
        # Readiness authority: an unpoisoned, acknowledged ledger still within fullDeadline of its collectionStarted.
        e=self.admitted
        if self.monitor_poison is not None or e is None or time.monotonic()-e['collectionStarted']>self.n['fullDeadline']:return None  # GUARD_ADMITTED_CURRENT
        return e

    def aged(self,now):
        # GUARD_CONSUMER_COLLECTION_AGE: settlement/health eligibility is the ORIGINAL collectionStarted age (inclusive fullDeadline), never the
        # later last_full reception age, measured at the caller's ONE monotonic evaluation timestamp `now` (health passes its time.monotonic()).
        return self.admit_archive(None,now,age_only=True)

    def pump(self,duration):
        if self.archive_mode():return self.pump_archive(duration)
        native_events=getattr(self,'operational',False) and self.owner_is_native and not self.prebegin
        event_limit=self.c['run_bytes'] if native_events else self.c['output_bytes']
        roles=('owner',) if self.monitor_done else ('owner','monitor')  # FIX3-D4: a finished monitor's EOF must not spin the loop
        handles=[self.children[r].stdout for r in roles]
        readable,_,_=select.select(handles,[],[],max(0,duration))
        for role in roles:
            p=self.children[role]
            if p.stdout not in readable:continue
            size=self.c['request_bytes']+1
            if native_events:
                # Shutdown traffic shares the run budget; never read beyond it or reset consumption.
                if self.output>=event_limit:
                    self.monitor_fault(role,'event_backpressure_limit');continue
                size=min(size,event_limit-self.output)
            chunk=os.read(p.stdout.fileno(),size);self.output+=len(chunk)
            if native_events and self.output>=event_limit-self.c['output_bytes']:
                self.gate.fault('event_shutdown_reserve')
            if not chunk:
                if role=='monitor' and self.monitor_finish_sent:self.monitor_done=True
                elif not self.finish_attempted:self.monitor_fault(role,role+'_channel_lost')
                continue
            self.buffers[role]+=chunk
            if self.output>event_limit or len(self.buffers[role])>self.c['request_bytes']*2:
                self.monitor_fault(role,'event_backpressure_limit');self.buffers[role]=b'';continue
            while b'\n' in self.buffers[role]:
                line,self.buffers[role]=self.buffers[role].split(b'\n',1)
                try:
                    e=self.decode_frame(line)
                    if role=='owner':
                        if getattr(self,'operational',False) and e.get('event') in ('install_begin','boot_begin') and self.owner_state.begin_command is None:raise ValueError('unsolicited_begin')
                        self.owner_state.event(e)
                        self.owner_handoff_event(e)
                        for reason in self.owner_state.unknown:self.gate.fault(reason)  # GUARD_OWNER_FAULT_SYNC: before any readiness check; never monitor poison
                        if self.prebegin and e.get('event') in ('preparation_started','full_hash_begin','full_hash_complete','private_auxiliary_verified','ready','preparation_retained'):print('PREBEGIN '+e['event'],flush=True)
                        if e.get('event')=='preparation_finish_ack':
                            if (self.preparation_finish is None or e.get('command')!='prepare_finish'
                                    or e.get('commandSeq')!=self.preparation_finish or e.get('instance') is not None
                                    or e.get('objectCreated') is not False or e.get('vmStarted') is not False):
                                raise ValueError('preparation_finish_ack_binding')
                            self.preparation_ack=e
                    else:received=time.monotonic();e=self.monitor_event(e,p.pid,received)
                    self.rows[role]=dict(monotonic_ns=time.monotonic_ns(),value=e)
                    # Liveness-only progress has no logical ledger or recorder admission.
                    if role!='monitor' or e['kind']!='progress':
                        sequence=self.record_seq+1;ack=self.record(role+'_event',value=e)
                    if role=='monitor' and e['kind']=='ledger':self.admit(e,received,ack,sequence)
                except (ValueError,KeyError,TypeError,OSError,EOFError,TimeoutError) as exc:
                    self.monitor_fault(role,role+'_invalid_event:'+str(exc))
        handoff=self.owner_handoff_requested and not self.owner_state.instance
        if (self.owner_state.instance or (self.preparation and self.owner_state.owner_start and not self.owner_handoff_sent)) and not self.finish_attempted and self.children['owner'].poll() is None and (handoff or time.monotonic()>=self.next_owner_heartbeat):
            try:
                self.owner_command_send('handoff' if handoff else 'heartbeat')
                self.next_owner_heartbeat=time.monotonic()+min(.25,self.n['controlInterval'])
            except (ValueError,OSError,TimeoutError) as exc:self.gate.fault('owner_control_lost:'+str(exc))

    def owner_handoff_event(self,e):
        if e.get('event')=='preparation_retained' and self.owner_handoff_sent:
            if self.owner_state.instance is not None or e.get('instance') is not None or e.get('objectCreated') is not False:
                raise ValueError('preparation_handoff_abort_binding')
            self.owner_handoff_aborted=True
        if e.get('event')!='preparation_handoff':return
        if (not self.preparation or self.owner_state.instance is not None or e.get('instance') is not None
                or e.get('objectCreated') is not False or self.owner_handoff_requested):
            raise ValueError('preparation_handoff_binding')
        self.owner_handoff_requested=True

    def owner_binding(self):
        s=self.owner_state
        return dict(pid=s.pid,ownerStart=s.owner_start,instance=s.instance) if s else None

    def begin_ready(self):
        self.health()
        s=self.owner_state;e=self.current()
        if (not getattr(self,'operational',False) or self.prebegin or self.finish_attempted
                or self.gate.unknown or s is None or s.started or not s.instance or not s.owner_start
                or not s.stopped or s.finish_command is not None or not self.monitor_identity
                or e is None or e.get('startDecision',{}).get('admitted') is not True
                or e.get('decision',{}).get('admitted') is not True or self.record_busy or self.deferred_events):
            raise ValueError('begin_not_admitted')  # GUARD_OPERATOR_ADMISSION
        if any(p.poll() is not None for p in self.children.values()):raise ValueError('begin_child_lost')
        validate_operational(self.n)
        if digest(self.run_config)!=self.run_config_digest:raise ValueError('run_config_drift')
        return e

    def handle_request(self,request):
        if not getattr(self,'operational',False):return super().handle_request(request)
        if self.events>self.c['request_count']:
            self.gate.fault('request_count_limit');return 'request_count_limit'
        reason=self.gate.request(request,operational=True,owner=self.owner_binding())
        if reason!='ok':return reason
        name=request['command']
        try:
            if name=='begin':
                e=self.begin_ready()
                self.begin_authorization=dict(owner=self.owner_binding(),sequence=self.gate.seq)
                self.record('operator_begin',owner=self.begin_authorization['owner'],sequence=self.gate.seq,
                            collectionStarted=e['collectionStarted'])
                self.owner_command_send('begin')
            elif name=='cancel':
                self.cancel_sent=True;self.owner_command_send('cancel')
            elif name in ('finish','abandon'):
                self.finish_requested=True
                if self.owner_state.started and self.owner_state.callback is None and not self.cancel_sent:
                    self.cancel_sent=True;self.owner_command_send('cancel')
        except (ValueError,OSError,EOFError,TimeoutError) as exc:
            self.gate.fault('operator_dispatch_failure:'+str(exc));return str(exc)
        return 'ok'

    def authorize_owner_command(self,cmd):
        if getattr(self,'operational',False):
            cmd.update(token=self.token,configSHA256=self.run_config_digest)
        return cmd

    def owner_command_send(self,name):
        if name=='begin':
            if (not getattr(self,'operational',False) or self.begin_authorization is None
                    or self.begin_authorization['owner']!=self.owner_binding()
                    or self.begin_authorization['sequence']!=self.gate.seq or self.gate.state!='BEGIN_FENCED'):
                raise ValueError('manual_begin_disabled')
            e=self.begin_ready()
            self.begin_authorization=None

        s=self.owner_state
        if s.instance is None and not (self.preparation and s.owner_start and name in ('heartbeat','prepare_finish','handoff')):raise ValueError('owner_not_bound')
        if s.instance is None and self.owner_handoff_sent and not (self.owner_handoff_aborted and name=='prepare_finish'):raise ValueError('owner_handoff_inflight')
        if name=='handoff':
            if s.instance is not None or not self.owner_handoff_requested or self.finish_attempted:raise ValueError('owner_handoff_state')
            self.owner_handoff_sent=True
        if name=='prepare_finish':
            if s.instance is not None or self.preparation_finish is not None:raise ValueError('preparation_finish_state')
            self.preparation_finish=self.owner_command+1
        self.owner_command+=1
        if name=='finish':s.expect_finish(self.owner_command);self.finish_sequence=self.owner_command
        cmd=dict(command=name,nonce=self.nonce,pid=s.pid,instance=s.instance or '',ownerStart=s.owner_start,commandSeq=self.owner_command)
        if name=='begin':
            s.expect_begin(self.owner_command)
            cmd.update(collectionStarted=e['collectionStarted'],recordSeq=self.record_seq)
        self.authorize_owner_command(cmd)
        if self.archive_mode():
            from transport import write_until
            self.owner_write_pending=True
            try:
                write_until(self.children['owner'].stdin,((json.dumps(cmd)+'\n').encode(),),
                    deadline=min(time.monotonic()+self.c['ack_seconds'],self.finish_end or float('inf')),service=self.io_service)
            finally:self.owner_write_pending=False
        else:write_bounded(self.children['owner'].stdin,(json.dumps(cmd)+'\n').encode(),min(self.c['ack_seconds'],max(0,self.finish_end-time.monotonic())) if self.finish_end is not None else self.c['ack_seconds'])
        self.record('owner_command',command={k:v for k,v in cmd.items() if k!='token'})

    def readiness(self):
        end=time.monotonic()+(self.n['preparationDeadline'] if self.preparation else self.c['ack_seconds'])
        initial=time.monotonic()+self.c['ack_seconds']
        while time.monotonic()<end:
            self.pump(min(self.c['heartbeat_seconds'],max(0,end-time.monotonic())))
            if not self.gate.unknown and self.owner_state.instance and self.monitor_identity and self.current():return True  # GUARD_READY_FAULT_FREE
            if self.preparation:
                self.health();self.serve_one();self.save_status('LIVE')
                if not self.owner_state.owner_start and time.monotonic()>=initial:self.gate.fault('preparation_identity_missing')
                if self.gate.unknown:self.stop_reason=self.stop_reason or 'preparation_health_fault'
                if self.stop_reason:return False
        self.gate.fault('preparation_deadline' if self.preparation else 'ready_ack_missing')
        self.stop_reason=self.stop_reason or 'preparation_deadline' if self.preparation else self.stop_reason
        return False

    def auxiliary_run_bytes(self):
        import stat
        parent=self.run/'auxiliary'
        fd=os.open(parent,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
        try:
            def identity(s):return (s.st_dev,s.st_ino,s.st_uid,s.st_gid,s.st_mode)
            expected=identity(self.auxiliary_parent);before=os.fstat(fd)
            if (identity(before)!=expected or identity(parent.lstat())!=expected or
                    not stat.S_ISDIR(before.st_mode) or stat.S_IMODE(before.st_mode)!=0o700 or
                    before.st_uid!=os.getuid() or before.st_dev!=self.run.lstat().st_dev):
                raise ValueError('auxiliary_parent_changed')
            names=os.listdir(fd)
            if names not in ([],['Auxiliary.private.bin']):raise ValueError('auxiliary_members')
            size=0
            if names:
                leaf=os.stat('Auxiliary.private.bin',dir_fd=fd,follow_symlinks=False)
                if (not stat.S_ISREG(leaf.st_mode) or stat.S_IMODE(leaf.st_mode)!=0o600 or
                        leaf.st_uid!=os.getuid() or leaf.st_dev!=before.st_dev or leaf.st_nlink!=1):
                    raise ValueError('auxiliary_leaf_invalid')
                if self.auxiliary_leaf is not None and identity(leaf)!=self.auxiliary_leaf:
                    raise ValueError('auxiliary_leaf_changed')
                self.auxiliary_leaf=identity(leaf);size=leaf.st_size
            elif self.auxiliary_leaf is not None:raise ValueError('auxiliary_leaf_missing')
            if identity(os.fstat(fd))!=expected or identity(parent.lstat())!=expected:
                raise ValueError('auxiliary_parent_changed')
            return size
        finally:os.close(fd)

    def health(self):
        from config import source_digest
        import shutil,stat
        if integration_digest(self.root)!=self.integration:self.gate.fault('integration_source_drift')
        if source_digest(self.root)!=self.source or digest(self.config_path)!=self.config_digest:self.gate.fault('source_config_drift')
        if getattr(self,'operational',False) and digest(self.run_config)!=self.run_config_digest:self.gate.fault('run_config_drift')
        if shutil.disk_usage(self.run).free<self.c['free_bytes']:self.gate.fault('free_space_limit')
        size=sum(p.lstat().st_size for p in self.run.iterdir() if stat.S_ISREG(p.lstat().st_mode))
        if self.prebegin:
            try:size+=self.auxiliary_run_bytes()
            except (OSError,ValueError):self.gate.fault('auxiliary_runtime_invalid')
        if size>=self.c['run_bytes']:self.gate.fault('run_size_limit');self.record_halted=True  # FIX3-D4
        now=time.monotonic()
        if ((self.last_full is None and (not self.preparation or now-self.started>=self.n['fullDeadline'])) or (self.last_full is not None and (now-self.last_full>self.n['fullDeadline'] or self.aged(now)))):self.gate.fault('stale_full_observation')  # HEALTH_AGE
        for role,p in self.children.items():
            if p.poll() is not None and not self.finish_attempted:self.gate.fault(role+'_early_exit')
            row=self.rows.get(role)
            if role!='recorder' and ((not row and (not self.preparation or now-self.started>=self.c['ack_seconds'])) or (row and time.monotonic_ns()-row['monotonic_ns']>self.c['stale_seconds']*1e9)):self.gate.fault(role+'_stale')
        for reason in self.owner_state.unknown:self.gate.fault(reason)
        s=self.owner_state
        if s.started and s.callback is None and self.gate.unknown and not self.cancel_sent:
            self.cancel_sent=True
            try:self.owner_command_send('cancel')
            except (ValueError,OSError,TimeoutError):self.gate.fault('cancel_control_lost')
        if getattr(self,'operational',False) and s.started and s.callback is None:
            e=self.current()
            if e is not None and e.get('decision',{}).get('admitted') is not True:self.gate.fault('operational_capacity_refused')
            if self.gate.unknown and not self.cancel_sent:
                self.cancel_sent=True
                try:self.owner_command_send('cancel')
                except (ValueError,OSError,TimeoutError):self.gate.fault('cancel_control_lost')
        self.settlement(now)

    def settlement(self,now):
        s=self.owner_state;e=self.admitted
        if self.monitor_poison is not None or not e or not s.started or s.callback is None or not s.stopped or not s.exact_stop or self.children['owner'].poll() is not None or now-self.last_full>self.n['fullDeadline'] or self.aged(now):  # SETTLEMENT_AGE
            self.settlement_at=None;self.completion_settled=False;return False
        for key,limit in (('disk',self.n['disk']),('scratch',self.n['scratch']),('total',self.n['aggregate'])):
            if e[key]>limit and key+'_cap_breach' not in s.breaches:s.breaches.append(key+'_cap_breach')
        if e['free']<self.n['freeFloor'] and 'reserve_breach' not in s.breaches:s.breaches.append('reserve_breach')
        fp=e['settlementFingerprint']
        if fp!=self.settlement_fp or self.settlement_at is None:self.settlement_fp=fp;self.settlement_at=now;return False
        self.completion_settled=now-self.settlement_at>=self.n['settlementSeconds']
        return self.completion_settled

    def close_preparation(self,reason,retry=False):
        if retry:self.finish_end=time.monotonic()+self.c['ack_seconds'];self.retry_spent=True  # FIX3-D3a: one re-evaluation of a retained preparation
        elif (self.archive_mode() or getattr(self,'operational',False)) and self.finish_end is None:self.finish_end=time.monotonic()+self.c['ack_seconds']
        self.finish_attempted=True
        self.gate.state='ABANDON_FENCED'
        owner=self.children['owner']
        if not retry:
            try:self.owner_command_send('prepare_finish')
            except (ValueError,OSError,TimeoutError) as exc:self.gate.fault('preparation_finish_write:'+str(exc))
            end=self.finish_end if self.finish_end is not None else time.monotonic()+self.c['ack_seconds']
            while time.monotonic()<end and owner.poll() is None:self.pump(min(self.c['heartbeat_seconds'],max(0,end-time.monotonic())))
        self.pump(0)
        try:self.waits['owner']=owner.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
        except subprocess.TimeoutExpired:self.gate.fault('owner_wait_timeout')
        # The preparation acknowledgement proves no object was constructed, never exact VZ STOPPED.
        expected=1 if 'owner_transport_fault' in self.owner_state.unknown else 0
        for fault in self.owner_state.unknown:self.gate.fault(fault)
        # Retry: the exited owner's own journal (objectCreated false) is the proof; its exit code is recorded, not required.
        if retry:bad=self.preparation_ack is None or not self.journal_proves_no_object()
        else:bad=self.preparation_ack is None or self.waits.get('owner')!=expected
        if bad:
            self.gate.fault('preparation_finish_ack_missing_or_wrong')
            receipt=self.snapshot('RETAINED_UNKNOWN');receipt.update(reason=reason,control_clean=False,outcome='UNKNOWN',native_state='NO_OBJECT_NOT_PROVED')
            return self.finalize_receipt(receipt,close_eligible=False)
        self.close_peers()
        self.gate.state='HELPERS_CLOSED' if self.control_clean else 'RETAINED_UNKNOWN'
        receipt=self.snapshot('FINAL' if self.control_clean else 'RETAINED_UNKNOWN')
        receipt.update(reason=reason,control_clean=self.control_clean,outcome='UNKNOWN' if self.gate.unknown or not self.control_clean else 'PREPARATION_ABANDONED',native_state='NO_OBJECT_CREATED',owner_exit_code=self.waits.get('owner'),retained_resolution='owner_journal_no_object' if retry else None,recorder_fsync_sequence=self.record_seq,preparation_finish_command=self.preparation_finish,preparation_finish_ack=self.preparation_ack['commandSeq'])
        return self.finalize_receipt(receipt,close_eligible=self.control_clean)

    def send_monitor_finish(self,fresh=False):
        # Existing authenticated monitor finish (commandSeq 1), idempotent so a retained run can stop the scan early and close_peers reuses it.
        if self.monitor_finish_sent:return
        m=self.children['monitor'];identity=self.monitor_identity
        if identity is None:raise ValueError('monitor_finish_identity_missing')
        if (not isinstance(identity,dict) or identity.get('nonce')!=self.nonce or identity.get('pid')!=m.pid
                or not isinstance(identity.get('instance'),str) or not identity['instance']):
            raise ValueError('monitor_finish_identity_binding')
        raw=(json.dumps(dict(command='finish',nonce=self.nonce,pid=m.pid,instance=identity['instance'],commandSeq=1))+'\n').encode()
        self.monitor_finish_sent=True
        if self.archive_mode():
            from transport import write_until
            write_until(m.stdin,(raw,),deadline=self.finish_end if self.finish_end is not None and not fresh else time.monotonic()+self.c['ack_seconds'],service=self.io_service)
        else:write_bounded(m.stdin,raw,min(self.c['ack_seconds'],max(0,self.finish_end-time.monotonic())) if self.finish_end is not None and not fresh else self.c['ack_seconds'])

    def quiesce_monitor(self):
        # FIX3-D4: a retained run or sticky monitor fault must not keep scanning (journal growth). Archive mode binds finish_ack to finish_attempted.
        if self.monitor_finish_sent or self.monitor_identity is None or self.children['monitor'].poll() is not None:return
        if self.archive_mode() and not self.finish_attempted:return
        try:self.send_monitor_finish(fresh=True)  # own window: a retained run's finish_end is long past
        except (ValueError,OSError,EOFError,TimeoutError) as exc:self.gate.fault('monitor_quiesce_failure:'+str(exc))

    def exact_resolvable(self):
        # FIX3-D3b: poisoned monitor, but the live owner itself proved callback + exact stop: finish is still authenticated and sequenced.
        s=self.owner_state
        return bool(self.monitor_poison is not None and s.instance and s.started and s.callback is not None and s.stopped and s.exact_stop
                    and s.finish_command is None and self.children['owner'].poll() is None)

    def journal_proves_no_object(self):
        # r10 F5: the COMPLETE owner journal, from byte 0, of the exact inode created at spawn (and opened by VMOwner), must be a
        # contiguous seq 1..N of complete frames of this run/pid/ownerStart with no object event, ending in the bound prepare_finish ack
        # (optionally followed only by operation_complete). Larger than 262144 B, replaced, linked, partial or ambiguous: not proved.
        s=self.owner_state
        if self.owner_journal_identity is None:return False
        def fixed(v):return (v.st_dev,v.st_ino,v.st_mode,v.st_uid,v.st_nlink,v.st_size,v.st_mtime_ns)
        try:
            fd=os.open(self.run/'owner.jsonl',os.O_RDONLY|os.O_NOFOLLOW)
            try:
                before=os.fstat(fd)
                if (not stat.S_ISREG(before.st_mode) or stat.S_IMODE(before.st_mode)!=0o600 or before.st_uid!=os.getuid()
                        or before.st_nlink!=1 or (before.st_dev,before.st_ino)!=self.owner_journal_identity or not 0<before.st_size<=262144):
                    return False
                raw=b''
                while len(raw)<before.st_size:
                    chunk=os.read(fd,before.st_size-len(raw))
                    if not chunk:return False
                    raw+=chunk
                if os.read(fd,1) or fixed(os.fstat(fd))!=fixed(before):return False
            finally:os.close(fd)
        except OSError:return False
        if not raw.endswith(b'\n'):return False
        events=[]
        for number,line in enumerate(raw[:-1].split(b'\n'),1):
            if len(line)+1>self.c['request_bytes']:return False
            try:e=json.loads(line)
            except ValueError:return False
            if (not isinstance(e,dict) or e.get('seq')!=number or e.get('nonce')!=self.nonce or e.get('pid')!=s.pid
                    or e.get('ownerStart')!=s.owner_start or e.get('instance') is not None
                    or any(key in e and e[key] is not False for key in ('objectCreated','vmStarted'))  # GUARD_JOURNAL_FALSE_ASSERTIONS
                    or e.get('event') in ('owner_created','ready','install_begin','boot_begin')):return False
            events.append(e)
        if events and events[-1].get('event')=='operation_complete':events.pop()
        acks=[e for e in events if e.get('event')=='preparation_finish_ack']
        return bool(events and acks==[events[-1]] and events[-1]==self.preparation_ack
                    and events[-1].get('command')=='prepare_finish' and events[-1].get('commandSeq')==self.preparation_finish
                    and events[-1].get('objectCreated') is False and events[-1].get('vmStarted') is False)

    def retained_kind(self):
        # A retained run is re-evaluated at most once, and only when the owner's own evidence is complete; anything ambiguous stays retained.
        if self.retry_spent:return None
        s=self.owner_state;a=self.preparation_ack
        if s.instance is None:
            if (self.preparation and a is not None and not s.started and self.children['owner'].poll() is not None
                    and a.get('objectCreated') is False and a.get('vmStarted') is False and a.get('instance') is None):return 'preparation'
            return None
        return 'exact' if self.exact_resolvable() else None

    def close_peers(self,owner_finish=None):
        try:
            finish={'preparationFinish':self.preparation_ack} if owner_finish is None else {'ownerFinish':owner_finish}
            self.record('control_finish',**finish,wait=self.waits['owner'])
            m=self.children['monitor']
            self.send_monitor_finish()
            end=self.finish_end if self.finish_end is not None else time.monotonic()+self.c['ack_seconds']
            while time.monotonic()<end and m.poll() is None:self.pump(min(self.c['heartbeat_seconds'],max(0,end-time.monotonic())))
            self.pump(0);self.waits['monitor']=m.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
            if self.monitor_ack is None or self.waits['monitor']!=0:raise ValueError('monitor_finish_ack_missing')
            self.record('recorder_finish',waits=self.waits)
            r=self.children['recorder'];r.stdin.close();self.waits['recorder']=r.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
            if self.waits['recorder']!=0:raise ValueError('recorder_exit')
            if (self.archive_mode() and (self.record_busy or self.deferred_events)) or (self.finish_end is not None and time.monotonic()>self.finish_end):
                raise ValueError('finish_not_quiescent')
            self.control_clean=True
        except (ValueError,OSError,KeyError,EOFError,TimeoutError,subprocess.TimeoutExpired) as exc:self.gate.fault('finish_receipt_failure:'+str(exc))

    def finalization_fault(self,reason):
        self.control_clean=False  # GUARD_FINALIZATION_FAULT
        self.gate.fault(reason)

    def close_channels(self):
        failures=[]
        streams=[self.server]+[stream for child in self.children.values() for stream in (child.stdin,child.stdout)]
        try:
            for index,stream in enumerate(streams):
                if stream is None:continue
                try:stream.close()
                except (OSError,ValueError) as exc:
                    failures.append(exc);self.finalization_fault('owned_channel_close_failure')
                    self.gate.fault('owned_channel_close_failure:'+str(index)+':'+str(exc))
        finally:
            try:os.chdir(self.old_cwd)
            except OSError as exc:
                failures.append(exc);self.finalization_fault('finalization_cwd_failure:'+str(exc))
        if len(failures)==1:raise failures[0]
        if failures:raise ExceptionGroup('owned_channel_close_failures',failures)

    def finalize_receipt(self,receipt,*,close_eligible):
        errors=[]
        def failed(reason,exc):
            errors.append(exc);self.finalization_fault(reason+':'+str(exc))
        def deadline():
            if (self.finish_end is not None and time.monotonic()>self.finish_end
                    and 'finish_deadline' not in self.gate.unknown):
                # A retained owner (not close_eligible) keeps live control: the deadline stays sticky UNKNOWN/non-clean and is published,
                # but the non-clean receipt is RETURNED. An eligible finish that overruns, and any publication/cwd/channel error, still raises.
                if close_eligible:errors.append(TimeoutError('finish_deadline'))
                self.finalization_fault('finish_deadline')  # GUARD_RETAINED_DEADLINE
        def refresh():
            if errors:
                self.gate.state='RETAINED_UNKNOWN'
                receipt.update(phase='RETAINED_UNKNOWN',control_clean=False,outcome='UNKNOWN')
            else:
                self.gate.state='HELPERS_CLOSED' if self.control_clean else 'RETAINED_UNKNOWN'
                receipt.update(control_clean=self.control_clean)
            receipt.update(unknown=list(self.gate.unknown),state=self.gate.state)
            self.final_receipt=receipt
        try:
            if close_eligible:
                try:self.close_channels()  # GUARD_CHANNELS_BEFORE_FINAL
                except (OSError,ValueError,ExceptionGroup) as exc:failed('finalization_channels_failure',exc)
        finally:
            try:os.chdir(self.old_cwd)
            except OSError as exc:failed('finalization_cwd_failure',exc)
        deadline();refresh()
        writers=(('shutdown',lambda:atomic(self.run/'shutdown.json',receipt,limit=self.c['output_bytes'])),
                 ('status',lambda:self.save_status(receipt['phase'])))
        def publish(selected):
            writable=[]
            for name,write in selected:
                deadline();refresh()
                try:write();writable.append((name,write))
                except (OSError,ValueError,EOFError,TimeoutError) as exc:failed('finalization_'+name+'_failure',exc)
                deadline()
            return writable
        publish(writers)
        if errors:
            # One recovery attempt per receipt, then refresh only receipts proved writable.
            # No multi-file atomicity or success is inferred for an inaccessible destination.
            refresh();before=len(errors);writable=publish(writers)
            if len(errors)!=before:publish(writable)
        refresh()
        if len(errors)==1:raise errors[0]
        if errors:raise ExceptionGroup('finalization_failures',errors)
        return receipt

    def close(self,reason):
        if self.preparation and self.owner_handoff_sent and not self.owner_state.instance and not self.finish_attempted:
            # Handoff may already have created an object: observe READY before binding finish.
            if self.finish_end is None:self.finish_end=time.monotonic()+self.c['ack_seconds']
            while time.monotonic()<self.finish_end and not self.owner_state.instance and not self.owner_handoff_aborted and self.children['owner'].poll() is None:
                self.pump(min(self.c['heartbeat_seconds'],max(0,self.finish_end-time.monotonic())))
            if not self.owner_state.instance and not self.owner_handoff_aborted:self.gate.fault('owner_handoff_observation_timeout')
        if self.preparation and not self.owner_state.instance and not self.finish_attempted:return self.close_preparation(reason)
        if self.finish_attempted:
            kind=self.retained_kind()
            if kind=='preparation':return self.close_preparation(reason,retry=True)
            if kind!='exact':return self.snapshot('RETAINED_UNKNOWN')
            self.retry_spent=True;self.finish_end=None  # FIX3-D3b: re-arm the bounded finish window for the sequenced owner finish
        with self.gate.lock:self.gate.state='ABANDON_FENCED'
        if (self.archive_mode() or getattr(self,'operational',False)) and self.finish_end is None:self.finish_end=time.monotonic()+self.c['ack_seconds']
        self.finish_attempted=True
        s=self.owner_state;p=self.children['owner']
        fresh=self.rows.get('owner',{}).get('monotonic_ns',0)
        # GUARD_CLOSE_CURRENT: a cached completion_settled is revalidated against current() (original collection age, no poison) here.
        if not s.instance or not s.stopped or (s.started and not ((self.completion_settled and self.current()) or self.exact_resolvable())) or time.monotonic_ns()-fresh>self.c['stale_seconds']*1e9 or p.poll() is not None:
            self.gate.fault('prebegin_finish_not_exact_live_stopped')
        else:
            try:self.owner_command_send('finish')
            except (ValueError,OSError,EOFError,TimeoutError) as exc:self.gate.fault('owner_finish_write:'+str(exc))
        end=self.finish_end if self.finish_end is not None else time.monotonic()+self.c['ack_seconds']
        while time.monotonic()<end and p.poll() is None:self.pump(min(self.c['heartbeat_seconds'],max(0,end-time.monotonic())))
        self.pump(0)
        try:self.waits['owner']=p.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
        except subprocess.TimeoutExpired:self.gate.fault('owner_wait_timeout')
        # Swift exits 1 after a fault even when its exact prebegin finish was acknowledged.
        faulted_finish=(s.stopped and 'owner_transport_fault' in s.unknown)  # GUARD_FAULTED_FINISH
        expected_exit=1 if faulted_finish else 0
        if faulted_finish:self.gate.fault('owner_transport_fault')
        if s.finish_ack is None or self.waits.get('owner')!=expected_exit:self.gate.fault('owner_finish_ack_missing_or_wrong')
        owner_clean=s.finish_ack is not None and self.waits.get('owner')==expected_exit
        if owner_clean:
            self.close_peers(s.finish_ack)
        if not self.owner_is_native:
            # Only the current harmless Popen objects; never numeric PID adoption.
            for role,child in self.children.items():
                if child.poll() is None:
                    child.terminate()
                    try:child.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
                    except subprocess.TimeoutExpired:child.kill();child.wait(timeout=max(0,self.finish_end-time.monotonic()) if self.finish_end is not None else self.c['ack_seconds'])
        self.gate.state='HELPERS_CLOSED' if self.control_clean else 'RETAINED_UNKNOWN'
        receipt=self.snapshot('FINAL' if self.control_clean else 'RETAINED_UNKNOWN')
        receipt.update(reason=reason,control_clean=self.control_clean,recorder_fsync_sequence=self.record_seq,
                       outcome=self.operational_outcome() if getattr(self,'operational',False) else ('UNKNOWN' if self.gate.unknown or not self.control_clean else 'PREBEGIN_ABANDONED'), # GUARD_STICKY_UNKNOWN
                       native_state='NOT_RUN' if self.fixture else ('EXACT_STOPPED' if self.control_clean and s.exact_stop else 'UNKNOWN'),native_recovery='NOT_RUN')
        return self.finalize_receipt(receipt,close_eligible=self.control_clean or self.fixture)

    def operational_outcome(self):
        s=self.owner_state
        if self.gate.unknown or s.unknown or s.breaches or not self.control_clean:return 'UNKNOWN'
        if not s.started:return 'PREBEGIN_ABANDONED'
        if not s.exact_stop or not s.stopped or s.finish_ack is None:return 'UNKNOWN'
        if s.cancel:return 'CANCELLED'
        if s.callback=='install_callback_success':return 'INSTALLED'
        if s.callback=='boot_callback_success':return 'BOOT_COMPLETED'
        return 'FAIL'

    def run_foreground(self):
        previous=signal.getsignal(signal.SIGINT)
        signal.signal(signal.SIGINT,lambda _s,_f:setattr(self,'stop_reason','Ctrl-C'))
        try:
            self.spawn()
            if self.prebegin:print('PREBEGIN PREPARING: installation disabled; bounded verification; status/finish available',flush=True)
            ready=self.readiness();self.save_status('LIVE')
            if self.archive_mode():self.health()
            # R24-F1: save_status blocks on file+directory fsync. READY needs the readiness result AND the same fault-free, bound, unpoisoned,
            # acknowledged ORIGINAL-age authority resampled after that write, immediately before the announcement (no I/O in between).
            ready=ready and not self.gate.unknown and bool(self.owner_state.instance) and bool(self.monitor_identity) and self.current() is not None  # GUARD_READY_POST_STATUS
            if getattr(self,'operational',False):print('OPERATIONAL PROFILE: explicit authenticated begin required; assumptions unverified',flush=True)
            print(('READY PREBEGIN_NATIVE READY_NO_BEGIN' if self.prebegin else 'READY NATIVE_ADAPTER_PLUMBING_ONLY READY_NO_BEGIN') if ready else 'PREPARING_OR_HELD',flush=True)
            while True:
                self.pump(self.c['heartbeat_seconds']);self.health();self.serve_one();self.save_status('LIVE')
                if self.monitor_poison is not None or self.record_halted or (self.finish_attempted and not self.control_clean):self.quiesce_monitor()  # FIX3-D4
                if time.monotonic()-self.started>=self.c['deadline_seconds']:self.stop_reason=self.stop_reason or 'deadline'
                if getattr(self,'operational',False):
                    if self.stop_reason or self.gate.unknown:self.finish_requested=True
                    if self.finish_requested and self.owner_state.started and self.owner_state.callback is None and not self.cancel_sent:
                        self.cancel_sent=True
                        try:self.owner_command_send('cancel')
                        except (ValueError,OSError,EOFError,TimeoutError) as exc:self.gate.fault('cancel_control_lost:'+str(exc))
                    eligible=(self.owner_state.instance and self.owner_state.stopped and
                              (not self.owner_state.started or (self.owner_state.exact_stop and ((self.completion_settled and self.current()) or self.exact_resolvable()))))
                    if not self.owner_state.instance and self.finish_requested:
                        receipt=self.close(self.stop_reason or 'operator_finish')
                        if self.control_clean:return 0 if receipt['outcome']=='PREPARATION_ABANDONED' else 1
                    if not eligible:continue  # Keep the exact native owner and live control until proof is complete.
                    if self.owner_state.started and self.completion_settled:self.stop_reason=self.stop_reason or 'operation_complete'
                    if self.finish_requested:self.stop_reason=self.stop_reason or 'operator_finish'
                if self.stop_reason:
                    receipt=self.close(self.stop_reason)
                    if self.control_clean or self.fixture:return 0 if receipt['outcome'] in ('PREBEGIN_ABANDONED','PREPARATION_ABANDONED','INSTALLED','BOOT_COMPLETED') else 1
                    self.stop_reason=None  # Exact native owner retained with status/control; no clean timeout.
        finally:signal.signal(signal.SIGINT,previous)


def main():
    p=argparse.ArgumentParser(description='SOURCE_ONLY native control: installation disabled')
    p.add_argument('mode',choices=('host','prebegin-host','client','prebegin-client','operational-host','operational-client'));p.add_argument('config',type=Path)
    p.add_argument('command',choices=('status','finish','abandon','begin','cancel'),nargs='?',default='status');a=p.parse_args()
    prebegin=a.mode.startswith('prebegin-');operational=a.mode.startswith('operational-')
    load_native(a.config.parent/('prebegin-native-config.json' if prebegin else 'operational-native-config.json' if operational else 'native-config.json'),prebegin=prebegin,operational=operational)
    if a.mode in ('host','prebegin-host','operational-host'):return NativeSupervisor(a.config,prebegin=prebegin,operational=operational).run_foreground()
    from config import load
    status=json.loads((a.config.parent/load(a.config)['run_directory']/'status.json').read_text())
    if status.get('integration_sha256')!=integration_digest(a.config.parent):raise ValueError('integration_source_drift')
    print(json.dumps(client(a.config,a.command)));return 0

if __name__=='__main__':
    try:sys.exit(main())
    except (ValueError,OSError) as exc:sys.exit('SOURCE_ONLY UNKNOWN/refused: '+str(exc))
