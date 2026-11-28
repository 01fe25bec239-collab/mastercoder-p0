# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).
"""One shared six-entry fixture; real monitor/walker cancellation, isolated storage expression."""
import ast, contextlib, gc, inspect, io, json, os, sys, textwrap
from pathlib import Path
from unittest.mock import patch
Q=Path(__file__).resolve().parents[1];sys.path.insert(0,str(Q/'source'))
import live_ledger as ll, monitor
from pilot_policy import FULL5_POLICY
OUT=Q/'new-fixtures/control';OUT.mkdir(mode=0o700)
TREE=OUT/'tree';TREE.mkdir();(TREE/'leaf').write_bytes(b'x');(TREE/'sub').mkdir()
CONFIG=OUT/'config.json';CONFIG.write_text(json.dumps(dict(policy=FULL5_POLICY,runNonce='1'*32,fastDeadline=.5)))
s=OUT.stat();IDENTITY=(s.st_dev,s.st_ino);RESULTS=[]
def fds():return len(os.listdir('/dev/fd'))
def walk(**kw):return ll._installer_tree(str(TREE),os.getuid(),s.st_dev,IDENTITY,**kw)
class Trigger:
 def __init__(self,stream,send):self.stream,self.send,self.done=stream,send,False
 def __enter__(self):return self
 def __iter__(self):return self
 def __next__(self):
  item=next(self.stream)
  if not self.done:self.done=True;self.send()
  return item
 def __exit__(self,*a):self.stream.close()
def monitor_case(stage):
 read,write=os.pipe();incoming=os.fdopen(read,'r');capture=io.StringIO();sent=False;calls=0;checks=0
 real_scan,real_check,real_walk=os.scandir,ll._installer_fd_check,ll._installer_walk
 def send():
  nonlocal sent
  if sent:return
  ready=next(json.loads(x) for x in capture.getvalue().splitlines() if json.loads(x)['kind']=='ready')
  os.write(write,json.dumps(dict(command='finish',nonce=ready['nonce'],pid=ready['pid'],instance=ready['instance'],commandSeq=1)).encode()+b'\n');sent=True
 def scan(path):
  stream=real_scan(path)
  target=(stage=='descriptor_inventory' and path=='/dev/fd') or (stage=='child_enumeration' and type(path) is int)
  return Trigger(stream,send) if target else stream
 def fd_check(state):
  nonlocal checks
  checks+=1
  if stage=='preopen' and checks==2:send()
  return real_check(state)
 def recurse(*args,**kwargs):
  result=real_walk(*args,**kwargs)
  if stage=='postwalk' and args[1]=='sub':send()
  return result
 def collect(config,control,progress):
  nonlocal calls
  calls+=1
  if calls>1:raise SystemExit('finish not serviced')
  if stage=='before_walk':send()
  def progress_cancel(*a):send();raise InterruptedError('progress_cancel')
  return walk(control=control,progress=progress_cancel if stage=='progress' else progress)
 baseline=fds();enabled=gc.isenabled()
 try:
  os.write(write,b'start\n')
  with patch.object(sys,'argv',['monitor.py',str(CONFIG),'native']),patch.object(sys,'stdin',incoming),patch.object(monitor,'validate_monitor_access',lambda c:None),patch.object(monitor,'collect',side_effect=collect),patch.object(ll.os,'scandir',side_effect=scan),patch.object(ll,'_installer_fd_check',side_effect=fd_check),patch.object(ll,'_installer_walk',side_effect=recurse),contextlib.redirect_stdout(capture):
   code=monitor.main()
  rows=[json.loads(x) for x in capture.getvalue().splitlines()]
  assert sent and code==0 and fds()==baseline
  assert rows[-1]['kind']=='finish_ack' and rows[-1]['commandSeq']==1
  return dict(stage=stage,exit=code,fdBefore=baseline,fdAfter=fds(),rows=rows)
 finally:
  if enabled:gc.enable()
  incoming.close();os.close(write)
def no_error(row):assert [e['kind'] for e in row['rows']]==['ready','finish_ack'],'cancelled collection emitted ledger_error'
def mutant(function):
 tree=ast.parse(textwrap.dedent(inspect.getsource(function)));removed=0
 for node in ast.walk(tree):
  if isinstance(node,ast.Try):
   prior=len(node.handlers);node.handlers=[h for h in node.handlers if not (isinstance(h.type,ast.Name) and h.type.id=='InterruptedError')];removed+=prior-len(node.handlers)
 assert removed==(1 if function.__name__=='_installer_tree' else 3)
 namespace=dict(function.__globals__);exec(compile(ast.fix_missing_locations(tree),'<broken-r12-cancellation>','exec'),namespace)
 result=namespace[function.__name__]
 namespace['_installer_walk']=lambda *a,**kw:ll._installer_walk(*a,**kw)
 namespace['_installer_fd_check']=lambda *a,**kw:ll._installer_fd_check(*a,**kw)
 return result
# Uncancelled and strict positives share the tree; no new output per case.
baseline=fds();assert len(walk())==3 and len(walk(strict=True))==3 and fds()==baseline
for stage in ('before_walk','descriptor_inventory','child_enumeration','preopen','postwalk','progress'):
 row=monitor_case(stage);RESULTS.append(dict(control='restored',**row));no_error(row)
# Remove only new handlers in memory. Same monitor assertions must fail, then restored must pass.
for stage,name in (('descriptor_inventory','_installer_tree'),('child_enumeration','_installer_walk'),('preopen','_installer_walk'),('postwalk','_installer_walk'),('progress','_installer_tree')):
 function=getattr(ll,name)
 with patch.object(ll,name,mutant(function)):
  row=monitor_case(stage)
 RESULTS.append(dict(control='broken',**row))
 try:no_error(row)
 except AssertionError:pass
 else:raise AssertionError('broken guard did not fail intended no-ledger_error assertion')
 row=monitor_case(stage);RESULTS.append(dict(control='restored-after-mutation',**row));no_error(row)
# Ordinary errors keep their existing LedgerError categories and finally cleanup.
real_open,real_stat,real_scan=os.open,os.stat,os.scandir
for stage,expected in (('inventory','installer_tree_unreadable:'),('preopen','installer_tree_substituted:'),('enumeration','installer_tree_unreadable:'),('postwalk','installer_tree_unreadable:')):
 count=0
 def opening(path,*a,**kw):
  if stage=='preopen' and kw.get('dir_fd') is not None:raise PermissionError('synthetic ordinary open')
  return real_open(path,*a,**kw)
 def scanning(path):
  if stage=='enumeration' and type(path) is int:raise PermissionError('synthetic ordinary scan')
  return real_scan(path)
 def stating(path,*a,**kw):
  global count
  if kw.get('dir_fd') is not None and path==TREE.name:
   count+=1
   if stage=='postwalk' and count==2:raise PermissionError('synthetic ordinary postwalk')
  return real_stat(path,*a,**kw)
 baseline=fds()
 with contextlib.ExitStack() as stack:
  stack.enter_context(patch.object(ll.os,'open',side_effect=opening));stack.enter_context(patch.object(ll.os,'stat',side_effect=stating));stack.enter_context(patch.object(ll.os,'scandir',side_effect=scanning))
  if stage=='inventory':stack.enter_context(patch.object(ll.resource,'getrlimit',side_effect=PermissionError('synthetic ordinary inventory')))
  try:walk()
  except ll.LedgerError as exc:assert str(exc).startswith(expected),str(exc)
  else:raise AssertionError('ordinary error not converted')
 assert fds()==baseline
 RESULTS.append(dict(control='ordinary-filesystem-error',stage=stage,fdBefore=baseline,fdAfter=fds(),category=expected))
# Strict removal still refuses, active disappearance is tolerated, and no-follow open flags stay enforced.
def gone(path,flags,*a,**kw):
 if kw.get('dir_fd') is not None:
  assert flags & os.O_NOFOLLOW and flags & os.O_DIRECTORY
  raise FileNotFoundError('synthetic disappearance')
 return real_open(path,flags,*a,**kw)
with patch.object(ll.os,'open',side_effect=gone):
 assert len(walk())==1
 try:walk(strict=True)
 except ll.LedgerError as exc:assert str(exc).startswith('retained_installer_tree_stale:')
 else:raise AssertionError('strict disappearance accepted')
assert len(walk())==3 and fds()==baseline
# Evaluate ONLY the first charged expression with synthetic W/alloc; no prelaunch import or execution.
tree=ast.parse((Q/'tests/fixtures/retained_roots.py.txt').read_text());assignment=next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='charged' for t in n.targets))
required=('w03-vm-operational-r10-repair','w03-vm-operational-r11-python-repair','w03-vm-operational-r12-minimum-corrections')
class Synthetic:
 def __init__(self,name=''):self.name=name
 def __truediv__(self,name):return Synthetic(name)
 def exists(self):return True
seen=[]
def alloc(p):seen.append(p.name);return 7 if p.name in required else 0
expr=ast.Expression(assignment.value);value=eval(compile(expr,'<isolated-root-calculation>','eval'),dict(W=Synthetic(),alloc=alloc))
def membership(names):assert all(names.count(n)==1 for n in required),'retained/current root missing or duplicated'
membership(seen);assert value==21
try:membership([n for n in seen if n!=required[0]])
except AssertionError:pass
else:raise AssertionError('missing r10 mutation was not caught')
membership(seen);RESULTS.append(dict(control='isolated-storage-membership',roots=seen,syntheticCharged=value))
with (OUT/'results.json').open('x') as f:json.dump(RESULTS,f,indent=2)
stamps=[p.lstat() for p in [OUT,*OUT.rglob('*')]]
assert len(stamps)==6 and max(sum(s.st_size for s in stamps),sum(s.st_blocks*512 for s in stamps))<=65536
print(json.dumps(dict(status='PASS',results=len(RESULTS),fixtureEntries=len(stamps),logicalBytes=sum(s.st_size for s in stamps),allocatedBytes=sum(s.st_blocks*512 for s in stamps),events=[dict(control=r['control'],stage=r.get('stage'),events=[e['kind'] for e in r.get('rows',[])]) for r in RESULTS])))
