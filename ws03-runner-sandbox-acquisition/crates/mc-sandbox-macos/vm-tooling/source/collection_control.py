# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""One serial cursor and pinned archive admission. No authority from progress."""
from dataclasses import dataclass, replace
import gc
import os
from pathlib import Path
import stat
import time
from archive_catalog import canonical,encode,fail,metadata,parse,pin,sha,migration_rows
from archive_capsule import read_regular,verify_capsules,verify_source,verify_directory
from native_config import validate_archive_config
from resource_ledger import LedgerError


class Cancelled(LedgerError):
    pass


@dataclass(frozen=True)
class ArchiveBinding:
    directory: str
    catalog_digest: str
    config_digest: str
    capsules: tuple
    historical_roots: tuple
    absent: tuple
    stamps: tuple
    live_source_catalog: dict

    def check(self,c,*,content=False,control=None):
        step=(lambda fn,*args:control.call(fn,*args)) if control else (lambda fn,*args:fn(*args))
        a=validate_archive_config(c)
        if sha(encode(a))!=self.config_digest:fail('binding_changed')
        directory=Path(self.directory)
        wanted=dict(self.stamps)
        if directory.resolve(strict=True)!=directory:fail('anchor_redirect')
        expected={Path(x).name for x in wanted if x!=self.directory}
        verify_directory(directory,expected,limit=a['caps']['capsules']+1,reason='directory_membership',control=control)
        for path,expected in self.stamps:
            if metadata(step(Path(path).lstat))!=expected:fail('container_drift')
        for root in self.absent:
            if os.path.lexists(root):fail('retired_reappeared')
        for row in self.live_source_catalog['records']:
            if metadata(step(Path(row['path']).lstat))!=row['metadata']:fail('source_drift')
        if content:
            raw=read_regular(directory/a['catalogName'],limit=a['caps']['catalogBytes'])
            if sha(raw)!=self.catalog_digest:fail('catalog_digest')
            catalog=parse(raw,uid=c['expectedUID'],device=Path(c['imagePath']).lstat().st_dev,limits=a['caps'])
            verify_capsules(directory,catalog,a['caps'])


def bind_archive(c):
    """Pre-run verification only, independently pinned by supplied run config."""
    a=validate_archive_config(c);directory=canonical(a['directory'])
    uid=c['expectedUID'];device=Path(c['imagePath']).lstat().st_dev
    if directory.resolve(strict=True)!=directory:fail('anchor_redirect')
    ds=directory.lstat()
    if not stat.S_ISDIR(ds.st_mode) or ds.st_uid!=uid or ds.st_dev!=device or stat.S_IMODE(ds.st_mode)!=0o700:
        fail('directory_identity')
    raw=read_regular(directory/a['catalogName'],limit=a['caps']['catalogBytes'])
    if sha(raw)!=a['catalogSHA256']:fail('catalog_digest')
    catalog=parse(raw,uid=uid,device=device,limits=a['caps'])
    caps={row['name']:row['sha256'] for row in catalog['capsules']}
    if caps!=a['capsuleSHA256']:fail('capsule_binding')
    expected={a['catalogName'],*caps}
    verify_directory(directory,expected,limit=a['caps']['capsules']+1,reason='directory_membership')
    roots=tuple(catalog['roots'])
    protected=[Path(c[k]) for k in ('lab','imagePath','bundlePath','frameworkTempRoot','evidence') if k in c]
    protected.append(canonical(a['currentSourceRoot']))
    for p in protected:
        if p.resolve(strict=True)!=p:fail('anchor_redirect')

    if 'runDir' in c:protected.append(Path(c['runDir']))
    owned=set(c['ownedRoots'].values())
    parent=Path(c['relatedRootParent'])
    if any(directory==Path(p) or directory.is_relative_to(p) or Path(p).is_relative_to(directory) for p in owned):
        fail('root_overlap')
    if any(directory.is_relative_to(p) or p.is_relative_to(directory) for p in protected):
        fail('root_overlap')
    if directory.parent==parent and (directory.name.startswith(c['relatedRootPrefix']) or directory.name in c['relatedRootAdditionalNames']):
        fail('root_overlap')
    for root in map(Path,roots):
        if root==directory or directory.is_relative_to(root) or root.is_relative_to(directory):
            fail('root_overlap')
        if any(root.is_relative_to(p) or p.is_relative_to(root) for p in protected):
            fail('ineligible_root')
        if root.parent!=parent or not (root.name.startswith(c['relatedRootPrefix']) or root.name in c['relatedRootAdditionalNames']) or str(root) not in owned:
            fail('migration_root_scope')
        if any(root!=Path(other) and (root.is_relative_to(other) or Path(other).is_relative_to(root)) for other in owned):
            fail('root_overlap')
    states=migration_rows(a['migrationRows'],a['catalogSHA256'],roots)
    if len(a['readerDischarges'])!=len(set(a['readerDischarges'])) or any(x not in roots for x in a['readerDischarges']):
        fail('reader_discharge')
    absent=[]
    for root in roots:
        state=states.get(root)
        exists=os.path.lexists(root)
        if state and state['stage']=='CUTOVER':
            if exists:fail('retired_reappeared')
            if root not in a['readerDischarges']:fail('reader_unresolved')
            absent.append(root)
        elif not exists:fail('incomplete_cutover')
    live_catalog=dict(catalog,records=[row for row in catalog['records'] if row['root'] not in absent])
    if live_catalog['records']:verify_source(live_catalog,a['caps'])
    paths=[directory,*(directory/name for name in sorted(expected))]
    stamps=[];keys=set();allocated=0
    for path in paths:
        s=path.lstat();m=metadata(s);key=(s.st_dev,s.st_ino)
        if s.st_uid!=uid or s.st_dev!=device or (path!=directory and (not stat.S_ISREG(s.st_mode) or s.st_nlink!=1)):
            fail('container_identity')
        if key in keys:fail('container_alias')
        keys.add(key);allocated+=s.st_blocks*512;stamps.append((str(path),m))
    if allocated>a['caps']['archiveAllocatedBytes']:fail('allocation_limit')
    binding=ArchiveBinding(str(directory),a['catalogSHA256'],sha(encode(a)),tuple(str(directory/x) for x in sorted(caps)),
                           roots,tuple(absent),tuple(stamps),live_catalog)
    verify_capsules(directory,catalog,a['caps'])
    binding.check(c)
    return binding


def discover(parent,prefix,additional,uid,device,binding,control):
    from live_ledger import _stat
    for root in binding.absent:
        if os.path.lexists(root):fail('retired_reappeared')
    absent_names={Path(x).name for x in binding.absent if Path(x).parent==parent}
    wanted={str(parent/name) for name in additional if name not in absent_names}
    found=[];size=0
    with os.scandir(parent) as children:
        for child in children:
            control.check(work=1)
            if child.name.startswith(prefix) or child.name in additional:
                path=str(parent/child.name);size+=len(os.fsencode(path))
                if len(found)>=control.caps['livePaths'] or size>control.caps['pathBytes']:
                    raise LedgerError('controlled_live_limit')
                found.append(path)
    if not wanted.issubset(found):raise LedgerError('missing_additional_related_root')
    for path in found:
        control.check()
        if not stat.S_ISDIR(_stat(path,uid,device).st_mode):raise LedgerError('ambiguous_discovered_root:'+path)
    return tuple(sorted(found))


def validate_coverage(snapshot,c):
    a=validate_archive_config(c)
    fields=('archive_catalog_digest','archive_absent_roots','archive_historical_roots','archive_capsule_paths','archive_binding')
    if any(not hasattr(snapshot,k) for k in fields):fail('snapshot_required')
    if snapshot.archive_catalog_digest!=a['catalogSHA256']:fail('snapshot_catalog')
    binding=snapshot.archive_binding
    if not isinstance(binding,ArchiveBinding) or binding.config_digest!=sha(encode(a)) or binding.catalog_digest!=a['catalogSHA256'] or tuple(snapshot.archive_historical_roots)!=binding.historical_roots or tuple(snapshot.archive_capsule_paths)!=binding.capsules:
        fail('snapshot_binding')
    states=migration_rows(a['migrationRows'],a['catalogSHA256'],snapshot.archive_historical_roots)
    absent={root for root,row in states.items() if row['stage']=='CUTOVER'}
    if absent!=set(snapshot.archive_absent_roots) or absent & set(snapshot.discovered_related_roots):
        fail('coverage_partition')
    expected=(set(c['ownedRoots'].values())|set(snapshot.discovered_related_roots)|{a['directory']})-absent
    if expected!=set(snapshot.required_roots) or set(snapshot.measured_roots)!=expected or len(snapshot.required_roots)!=len(set(snapshot.required_roots)):
        fail('coverage_partition')
    measured={path for e in snapshot.entries for path in e.paths}
    if not set(snapshot.archive_capsule_paths).issubset(measured) or str(Path(a['directory'])/a['catalogName']) not in measured:
        fail('unmeasured_container')


class Control:
    def __init__(self, *, service, clock=time.monotonic, fast=.5, deadline, caps):
        self.service=service;self.clock=clock;self.fast=fast;self.deadline=deadline
        self.caps=caps;self.completed=0;self.cancelled=False;self.paths=0;self.path_bytes=0
        self.last=clock();self.started=self.last;self.phase='collect';self.pending_names=0;self.pending_bytes=0

    def call(self,fn,*args,**kwargs):
        self.check()
        result=fn(*args,**kwargs)
        self.check(work=1)
        return result

    def check(self,phase=None,*,work=0):
        phase=phase or self.phase
        phases=('collect','validate','encode','write')
        if phase not in phases or phases.index(phase)<phases.index(self.phase):raise LedgerError('controlled_phase_regression')
        self.phase=phase
        now=self.clock()
        if now>self.deadline:raise LedgerError('controlled_collection_deadline')
        if now-self.last>self.fast:raise LedgerError('controlled_fast_service_gap')
        self.completed+=work
        self.service(phase,self.completed)
        self.last=self.clock()
        if self.last-now>self.fast:raise LedgerError('controlled_fast_service_gap')
        if self.cancelled:raise Cancelled('controlled_collection_cancelled')
        if self.last>self.deadline:raise LedgerError('controlled_collection_deadline')

    def insert(self,path,root_depth,unique):
        size=len(os.fsencode(path))
        if self.paths+1>self.caps['livePaths'] or unique>self.caps['liveInodes'] or len(Path(path).parts)-root_depth>self.caps['depth'] or self.path_bytes+size>self.caps['pathBytes']:
            raise LedgerError('controlled_live_limit')
        self.paths+=1;self.path_bytes+=size


def collect_controlled(c, *, binding, control, clock=time.monotonic):
    import live_ledger
    binding.check(c,control=control)
    cursor=live_ledger._collect_archive(c,clock,binding,control)
    try:
        while True:
            control.check()
            enabled=gc.isenabled();gc.disable()
            try:
                phase=next(cursor)
            except StopIteration as done:
                snapshot=done.value
                validate_coverage(snapshot,c)
                binding.check(c,control=control)
                return snapshot
            finally:
                if enabled:gc.enable()
            control.check(phase)
    finally:cursor.close()
