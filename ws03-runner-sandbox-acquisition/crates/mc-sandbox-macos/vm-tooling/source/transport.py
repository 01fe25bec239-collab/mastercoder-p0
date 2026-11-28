# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""r7 bounded pipe transport, one absolute deadline per frame."""
import json,os,select,time

def write_bounded(stream,raw,seconds):
    fd=stream.fileno();os.set_blocking(fd,False);end=time.monotonic()+seconds
    while raw:
        left=end-time.monotonic()
        if left<=0 or not select.select([], [fd], [], max(0,left))[1]:raise TimeoutError('transport_write_deadline')
        try:n=os.write(fd,raw)
        except BlockingIOError:continue
        if n<=0:raise OSError('transport_short_write')
        raw=raw[n:]

def read_ack(stream,seconds,max_bytes):
    fd=stream.fileno();os.set_blocking(fd,False);end=time.monotonic()+seconds;raw=b''
    while not raw.endswith(b'\n'):
        left=end-time.monotonic()
        if left<=0 or not select.select([fd],[],[],max(0,left))[0]:raise TimeoutError('recorder_ack_timeout')
        chunk=os.read(fd,max_bytes+1)
        if not chunk:raise EOFError('recorder_ack_eof')
        raw+=chunk
        if len(raw)>max_bytes:raise ValueError('recorder_ack_oversize')
    return json.loads(raw)


# R37 connected contract: one transaction deadline, cooperative waits, no authority from polling.
def cooperate(service, deadline, clock=time.monotonic):
    if service is not None:service()
    if clock()>deadline:raise TimeoutError('transport_transaction_deadline')


def write_until(stream, chunks, *, deadline, service=None, clock=time.monotonic, quantum=.5):
    fd=stream.fileno();os.set_blocking(fd,False)
    for raw in chunks:
        view=memoryview(raw)
        while view:
            cooperate(service,deadline,clock)
            if not select.select([], [fd], [], max(0,min(quantum,deadline-clock())))[1]:continue
            try:n=os.write(fd,view)
            except BlockingIOError:continue
            if n<=0:raise OSError('transport_short_write')
            view=view[n:]
            cooperate(service,deadline,clock)


def ack_until(stream, *, deadline, max_bytes, service=None, clock=time.monotonic, quantum=.5):
    fd=stream.fileno();os.set_blocking(fd,False);raw=bytearray()
    while b'\n' not in raw:
        cooperate(service,deadline,clock)
        if not select.select([fd],[],[],max(0,min(quantum,deadline-clock())))[0]:continue
        try:part=os.read(fd,max_bytes+1)
        except BlockingIOError:continue
        if not part:raise EOFError('recorder_ack_eof')
        raw.extend(part)
        if len(raw)>max_bytes:raise ValueError('recorder_ack_oversize')
    cooperate(service,deadline,clock)
    if raw.count(b'\n')!=1 or not raw.endswith(b'\n'):raise ValueError('recorder_ack_framing')
    return json.loads(raw)


def json_chunks(value, *, limit, service=None):
    size=0
    for token in json.JSONEncoder(sort_keys=True,allow_nan=False).iterencode(value):
        raw=token.encode()
        size+=len(raw)
        if limit is not None and size+1>limit:raise ValueError('logical_record_limit')
        for pos in range(0,len(raw),2048):
            if service is not None:service()
            yield raw[pos:pos+2048]
    yield b'\n'


def transaction(writer, reader, value, *, sequence, deadline, max_bytes, service=None, status=None):
    import base64,hashlib
    h=hashlib.sha256();parts=0;size=0
    def frames():
        nonlocal parts,size
        for raw in json_chunks(value,limit=max_bytes,service=service):
            h.update(raw);parts+=1;size+=len(raw)
            row=dict(op='part',recordSeq=sequence,part=parts,data=base64.b64encode(raw).decode())
            yield (json.dumps(row,separators=(',',':'))+'\n').encode()
        yield (json.dumps(dict(op='commit',recordSeq=sequence,parts=parts,bytes=size,
                   sha256=h.hexdigest(),status=status),separators=(',',':'))+'\n').encode()
    write_until(writer,frames(),deadline=deadline,service=service)
    ack=ack_until(reader,deadline=deadline,max_bytes=4096,service=service)
    if ack!={'ack':sequence}:raise ValueError('recorder_ack_sequence')
    return ack
