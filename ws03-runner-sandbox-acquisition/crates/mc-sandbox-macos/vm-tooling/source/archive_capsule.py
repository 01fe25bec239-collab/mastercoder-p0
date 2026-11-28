# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Pax regular byte members; all original metadata/opaque names are catalog data."""
import hashlib
import os
from pathlib import Path
import stat
import tarfile
from archive_catalog import (SCHEMA,encode,sha,fail,metadata,parse,fixture_path,member_name)


def read_regular(path, *, limit, before=None):
    initial=path.lstat()
    if not stat.S_ISREG(initial.st_mode) or initial.st_size>limit:fail('regular_limit')
    fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW)
    with os.fdopen(fd,'rb') as stream:
        if metadata(os.fstat(stream.fileno()))!=metadata(initial):fail('file_race')
        raw=stream.read(limit+1)
        if len(raw)>limit:fail('regular_limit')
        if metadata(os.fstat(stream.fileno()))!=metadata(initial):fail('file_race')
    if metadata(path.lstat())!=metadata(initial) or before is not None and metadata(initial)!=before:
        fail('file_race')
    return raw


def digest_regular(path, *, limit, before=None):
    initial=metadata(path.lstat())
    if not stat.S_ISREG(initial['mode']) or initial['size']>limit:fail('regular_limit')
    fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW)
    h=hashlib.sha256();length=0
    with os.fdopen(fd,'rb') as stream:
        if metadata(os.fstat(stream.fileno()))!=initial:fail('file_race')
        while True:
            block=stream.read(4096)
            if not block:break
            length+=len(block)
            if length>limit:fail('regular_limit')
            h.update(block)
        if metadata(os.fstat(stream.fileno()))!=initial:fail('file_race')
    if metadata(path.lstat())!=initial or before is not None and before!=initial:fail('file_race')
    return h.hexdigest(),length


def verify_capsules(directory, catalog, limits):
    """Use the actual tar parser, never extraction or tar member name lookup."""
    expected={}
    for row in catalog['records']:
        if row['kind']=='regular':
            expected[row['member']]=(row['metadata']['size'],row['sha256'])
    seen=set()
    for cap in catalog['capsules']:
        path=directory/cap['name'];initial=metadata(path.lstat())
        digest,length=digest_regular(path,limit=limits['capsuleBytes'])
        if length!=cap['bytes'] or digest!=cap['sha256']:fail('capsule_digest')
        # The parser independently rereads the real capsule file after the outer-byte hash.
        fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW)
        try:
            with os.fdopen(fd,'rb') as stream:
                if metadata(os.fstat(stream.fileno()))!=initial:fail('file_race')
                with tarfile.open(fileobj=stream,mode='r|',format=tarfile.PAX_FORMAT) as archive:
                    for item in archive:
                        name=member_name(item.name)
                        if name in seen:fail('duplicate_member')
                        if name not in expected:fail('extra_member')
                        size,digest=expected[name]
                        if not item.isreg() or item.linkname or item.sparse or item.size!=size or item.size>limits['capsuleBytes']:
                            fail('member_type_or_size')
                        payload=archive.extractfile(item)
                        if payload is None:fail('missing_payload')
                        h=hashlib.sha256();length=0
                        with payload:
                            while True:
                                block=payload.read(4096)
                                if not block:break
                                length+=len(block)
                                if length>size:fail('payload_length')
                                h.update(block)
                        if length!=size or h.hexdigest()!=digest:fail('payload_digest')
                        seen.add(name)
                if metadata(os.fstat(stream.fileno()))!=initial:fail('file_race')
        except (tarfile.TarError,EOFError) as exc:fail('capsule_encoding')
        if metadata(path.lstat())!=initial:fail('file_race')
    if seen!=set(expected):fail('missing_member')


def seal_fixture(source, destination, *, fixture_root, limits, provenance):
    """ONLY new synthetic fixture sources. No CLI, selection or deletion operations."""
    source=fixture_path(source,fixture_root);destination=fixture_path(destination,fixture_root)
    if source==destination or destination.is_relative_to(source) or source.is_relative_to(destination):
        fail('fixture_overlap')
    if source.resolve(strict=True)!=source or not source.is_dir():fail('fixture_source')
    destination.mkdir(mode=0o700)
    rows=[];content={};pending=[source];payload_total=0;discovered=1;path_bytes=len(os.fsencode(source))
    while pending:
        path=pending.pop()
        if path.parent.resolve(strict=True)!=path.parent:fail('fixture_redirect')
        s=path.lstat();m=metadata(s)
        if m['uid']!=os.getuid() or m['device']!=source.lstat().st_dev:fail('metadata_identity')
        kinds=((stat.S_ISREG,'regular'),(stat.S_ISDIR,'directory'),(stat.S_ISLNK,'symlink'),
               (stat.S_ISSOCK,'socket'),(stat.S_ISFIFO,'fifo'),(stat.S_ISCHR,'character'),(stat.S_ISBLK,'block'))
        kind=next((name for check,name in kinds if check(s.st_mode)),None)
        if kind is None:fail('metadata_type')
        row=dict(path=str(path),root=str(source),kind=kind,metadata=m,member=None,sha256=None,children=None,target=None)
        if kind=='regular':
            digest,length=digest_regular(path,limit=limits['capsuleBytes'],before=m)
            key=(s.st_dev,s.st_ino)
            if key in content and content[key][2]!=digest:fail('hardlink_bytes')
            if key not in content:
                payload_total+=length+512+(-length % 512)
                encoded_length=-(-(payload_total+1024)//tarfile.RECORDSIZE)*tarfile.RECORDSIZE
                if encoded_length>limits['capsuleBytes']:fail('capsule_limit')
                content[key]=(f'objects/{len(content):08d}',path,digest,m)
            row['member']=content[key][0];row['sha256']=digest
        elif kind=='directory':
            names=[]
            with os.scandir(path) as children:
                for child in children:
                    discovered+=1
                    if discovered>limits['livePaths']:fail('record_count')
                    candidate=path/child.name
                    path_bytes+=len(os.fsencode(candidate))
                    if path_bytes>limits['pathBytes'] or len(candidate.parts)-len(source.parts)>limits['depth']:
                        fail('catalog_work_limit')
                    names.append(child.name)
            row['children']=sorted(names)
            pending.extend(path/name for name in reversed(row['children']))
        elif kind=='symlink':
            row['target']=os.readlink(path)
        if metadata(path.lstat())!=m:fail('source_drift')
        rows.append(row)
        if len(rows)>limits['livePaths']:fail('record_count')
    capsule=destination/'capsule.tar'
    with tarfile.open(capsule,'x:',format=tarfile.PAX_FORMAT) as archive:
        for name,path,digest,m in content.values():
            fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW)
            with os.fdopen(fd,'rb') as stream:
                if metadata(os.fstat(stream.fileno()))!=m:fail('source_drift')
                item=tarfile.TarInfo(name);item.size=m['size'];item.mode=0o600
                archive.addfile(item,stream)
                if metadata(os.fstat(stream.fileno()))!=m:fail('source_drift')
    os.chmod(capsule,0o400)
    capdigest,caplength=digest_regular(capsule,limit=limits['capsuleBytes'])
    catalog=dict(schema=SCHEMA,roots=[str(source)],records=rows,
                 capsules=[dict(name=capsule.name,bytes=caplength,sha256=capdigest)],provenance=provenance)
    raw=encode(catalog)
    parse(raw,uid=os.getuid(),device=source.lstat().st_dev,limits=limits)
    path=destination/'catalog.json'
    with path.open('xb') as stream:stream.write(raw)
    os.chmod(path,0o400)
    verify_capsules(destination,catalog,limits)
    verify_source(catalog,limits)
    return dict(catalogSHA256=sha(raw),capsules={capsule.name:capdigest})


def verify_directory(path, expected, *, limit, reason, control=None):
    """Bound observed names before accumulation; own the scandir handle on all exits."""
    if len(expected)>limit:fail(reason)
    wanted=set(expected)
    if len(wanted)!=len(expected):fail(reason)
    seen=set()
    if control:control.check()
    with os.scandir(path) as children:
        if control:control.check()
        while True:
            if control:control.check()
            try:
                child=next(children)
            except StopIteration:
                if control:control.check()
                break
            if control:control.check(work=1)
            name=child.name
            if len(seen)>=limit or len(seen)>=len(wanted) or name not in wanted or name in seen:
                fail(reason)
            seen.add(name)
    if seen!=wanted:fail(reason)
    return seen


def verify_source(catalog, limits):
    """Fixture sealing/reopen check. Endpoint objects are only lstat/readlink data."""
    for row in catalog['records']:
        path=Path(row['path'])
        if path.parent.resolve(strict=True)!=path.parent:fail('source_drift')
        if metadata(path.lstat())!=row['metadata']:fail('source_drift')
        if row['kind']=='regular':
            if digest_regular(path,limit=limits['capsuleBytes'],before=row['metadata'])[0]!=row['sha256']:fail('source_content')
        elif row['kind']=='directory':
            names=sorted(verify_directory(path,row['children'],limit=limits['livePaths'],reason='source_membership'))
            if names!=row['children']:fail('source_membership')
        elif row['kind']=='symlink' and os.readlink(path)!=row['target']:fail('source_target')
        if metadata(path.lstat())!=row['metadata']:fail('source_drift')
