# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Independent fsync writer. The optional fragmented protocol uses the SAME logical journal."""
import base64,hashlib,json,os,sys
from pathlib import Path


def append(fd,raw):
    while raw:
        n=os.write(fd,raw)
        if n<=0:raise OSError('short durable write')
        raw=raw[n:]


def fragmented(fd, directory, *, input_stream, output_stream, limit=2**20):
    pending=bytearray();sequence=1;part=0
    while True:
        line=input_stream.readline(4097)
        if not line:
            if pending:raise EOFError('recorder_incomplete_transaction')
            return
        if len(line)>4096 or not line.endswith(b'\n'):raise ValueError('recorder_frame_limit')
        row=json.loads(line)
        if type(row) is not dict or type(row.get('recordSeq')) is not int or row['recordSeq']!=sequence:
            raise ValueError('recorder_sequence')
        if row.get('op')=='part':
            if set(row)!={'op','recordSeq','part','data'} or type(row['part']) is not int or row['part']!=part+1:
                raise ValueError('recorder_part_order')
            raw=base64.b64decode(row['data'],validate=True)
            if not raw or len(pending)+len(raw)>limit:raise ValueError('recorder_pending_limit')
            pending.extend(raw);part+=1
        elif row.get('op')=='commit':
            if set(row)!={'op','recordSeq','parts','bytes','sha256','status'} or type(row['parts']) is not int or type(row['bytes']) is not int or row['parts']!=part or row['bytes']!=len(pending) or not part:
                raise ValueError('recorder_commit_shape')
            if hashlib.sha256(pending).hexdigest()!=row['sha256']:raise ValueError('recorder_commit_digest')
            if not pending.endswith(b'\n') or pending.count(b'\n')!=1:raise ValueError('recorder_logical_framing')
            value=json.loads(pending)
            status=row['status']
            if status is None:
                if type(value) is not dict or value.get('recordSeq')!=sequence:raise ValueError('recorder_logical_sequence')
                if os.fstat(fd).st_size+len(pending)>64*2**20:raise ValueError('recorder_run_limit')
                append(fd,pending);os.fsync(fd)
            else:
                if status!='status.json':raise ValueError('recorder_status_path')
                # Stable inode: current status is a declared live writer. A crash/partial reader
                # observes invalid/UNKNOWN data; matching ack follows the complete fsync only.
                status_fd=os.open(directory/status,os.O_WRONLY|os.O_NOFOLLOW)
                try:
                    os.ftruncate(status_fd,0);append(status_fd,pending);os.fsync(status_fd)
                finally:os.close(status_fd)
            # Neither a partial append nor an fsync failure can send this acknowledgement.
            output_stream.write((json.dumps({'ack':sequence})+'\n').encode());output_stream.flush()
            sequence+=1;part=0;pending.clear()
        else:raise ValueError('recorder_operation')


def ordinary(fd,limit,*,input_stream,output_stream):
    # r10 F4: the journal never exceeds `limit` bytes; a record that would is refused before any byte is written, with no ack.
    for line in input_stream:
        value=json.loads(line);raw=(json.dumps(value,sort_keys=True)+'\n').encode()
        if os.fstat(fd).st_size+len(raw)>limit:raise ValueError('recorder_run_limit')
        append(fd,raw);os.fsync(fd)
        output_stream.write((json.dumps({'ack':value['recordSeq']})+'\n').encode());output_stream.flush()


def main():
    p=Path(sys.argv[1]);fd=os.open(p,os.O_APPEND|os.O_WRONLY|os.O_NOFOLLOW)
    try:
        if sys.argv[2:]==['fragmented']:
            fragmented(fd,p.parent,input_stream=sys.stdin.buffer,output_stream=sys.stdout.buffer)
        elif len(sys.argv)==4 and sys.argv[2]=='ordinary' and sys.argv[3].isdigit() and int(sys.argv[3])>0:
            ordinary(fd,int(sys.argv[3]),input_stream=sys.stdin.buffer,output_stream=sys.stdout.buffer)
        else:raise ValueError('recorder_mode')
    finally:os.close(fd)

if __name__=='__main__':main()
