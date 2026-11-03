// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
import Foundation
import CryptoKit
import CoreFoundation
import Darwin
@main struct AuxiliaryPrestage {
    @MainActor static func main() async {
        do {
            guard CommandLine.arguments.count==1 else { throw AuxiliaryCopyError.source }
            var raw=Data(),byte:UInt8=0
            while raw.count<4096 { guard read(STDIN_FILENO,&byte,1)==1 else { throw AuxiliaryCopyError.source };raw.append(byte);if byte==10 { break } }
            guard raw.last==10,raw.count<=4096 else { throw AuxiliaryCopyError.limit }
            try auxiliaryShape(raw,result:false)
            let q=try JSONDecoder().decode(AuxiliaryStageRequest.self,from:raw)
            guard q.version=="w03-aux-prestage-request-v1",auxiliaryHex(q.nonce,32),auxiliaryHex(q.manifestSHA256,64),auxiliaryHex(q.sourceSHA256,64),q.maxBytes>0,q.maxBytes<=64*1024*1024,q.expectedBytes>0,q.expectedBytes<=q.maxBytes,q.directoryFD>2,
                  q.sourcePath.hasSuffix("/Guest.bundle/Auxiliary.bin") else { throw AuxiliaryCopyError.source }
            for path in [q.manifestPath,q.sourcePath,q.directoryPath] { guard path.hasPrefix("/"),URL(fileURLWithPath:path).standardizedFileURL.path==path,URL(fileURLWithPath:path).resolvingSymlinksInPath().path==path else { throw AuxiliaryCopyError.destination } }
            var ds=stat(),ss=stat()
            guard lstat(q.directoryPath,&ds)==0,AuxiliaryIdentity(ds)==q.directoryIdentity,AuxiliaryIdentity(try auxiliaryStat(q.directoryFD))==q.directoryIdentity,lstat(q.sourcePath,&ss)==0,AuxiliaryIdentity(ss)==q.sourceIdentity else { throw AuxiliaryCopyError.destination }
            let flags=fcntl(STDIN_FILENO,F_GETFL);guard flags>=0,fcntl(STDIN_FILENO,F_SETFL,flags|O_NONBLOCK)==0 else { throw AuxiliaryCopyError.io }
            var sequence=0
            func checkpoint() async throws {
                var b:UInt8=0;let n=read(STDIN_FILENO,&b,1)
                guard n<0 && (errno==EAGAIN || errno==EWOULDBLOCK) else { throw CancellationError() }
                sequence+=1
                let frame=try JSONSerialization.data(withJSONObject:["version":"w03-aux-prestage-progress-v1","nonce":q.nonce,"sequence":sequence])
                guard frame.count+1<=4096 else { throw AuxiliaryCopyError.limit }
                try FileHandle.standardOutput.write(contentsOf:frame+Data([10]));await Task.yield()
            }
            let mf=open(q.manifestPath,O_RDONLY|O_NOFOLLOW);guard mf>=0 else { throw AuxiliaryCopyError.source };defer { close(mf) }
            let mb=try auxiliaryStat(mf);guard mb.st_mode & S_IFMT==S_IFREG,mb.st_nlink==1,mb.st_uid==getuid(),mb.st_mode & 0o022==0,mb.st_size>0,mb.st_size<=1024*1024 else { throw AuxiliaryCopyError.source }
            var manifest=Data(),buffer=[UInt8](repeating:0,count:65536)
            while true { try await checkpoint();let n=read(mf,&buffer,buffer.count);guard n>=0 else { throw AuxiliaryCopyError.io };if n==0 { break };manifest.append(contentsOf:buffer.prefix(n));guard manifest.count<=1024*1024 else { throw AuxiliaryCopyError.limit } }
            var ma=stat();guard auxiliarySame(mb,try auxiliaryStat(mf)),lstat(q.manifestPath,&ma)==0,auxiliarySame(mb,ma),auxiliaryDigest(manifest)==q.manifestSHA256,
                  let m=try JSONSerialization.jsonObject(with:manifest) as? [String:Any],m["schema"] as? Int==1,
                  let files=m["files"] as? [String:Any],Set(files.keys)==Set(["Restore.ipsw","Guest.bundle/Hardware.bin","Guest.bundle/Machine.bin","Guest.bundle/Auxiliary.bin","Guest.bundle/Disk.img"]),
                  let bundle=m["bundlePath"] as? String,q.sourcePath==bundle+"/Auxiliary.bin",
                  let record=files["Guest.bundle/Auxiliary.bin"] as? [String:Any],Set(record.keys).subtracting(["allocatedBytes"])==Set(["path","dev","ino","uid","gid","mode","bytes","mtimeNs","ctimeNs","sha256"]),record["path"] as? String==q.sourcePath,record["sha256"] as? String==q.sourceSHA256,record["bytes"] as? Int64==q.expectedBytes,
                  record["dev"] as? Int64==q.sourceIdentity.device,record["ino"] as? UInt64==q.sourceIdentity.inode,record["uid"] as? UInt32==q.sourceIdentity.uid,record["gid"] as? UInt32==q.sourceIdentity.gid,record["mode"] as? UInt32==q.sourceIdentity.mode & 0o7777,
                  record["mtimeNs"] as? Int64==q.sourceIdentity.mtimeNs,record["ctimeNs"] as? Int64==q.sourceIdentity.ctimeNs else { throw AuxiliaryCopyError.source }
            // Historical allocation is observation only; live copy validation remains authoritative.
            if let observed=record["allocatedBytes"] {
                guard let number=observed as? NSNumber,CFGetTypeID(number) != CFBooleanGetTypeID(),
                      number.doubleValue.isFinite,number.doubleValue>=0,number.doubleValue.rounded(.towardZero)==number.doubleValue else { throw AuxiliaryCopyError.source }
            }
            let leaf=q.directoryPath+"/Auxiliary.private.bin"
            try await copyVerifiedAuxiliary(source:q.sourcePath,destination:leaf,expectedBytes:q.expectedBytes,expectedSHA256:q.sourceSHA256,maxBytes:q.maxBytes,checkpoint:checkpoint)
            guard auxiliaryDirectorySame(ds,try auxiliaryStat(q.directoryFD)),lstat(q.directoryPath,&ds)==0,auxiliaryDirectorySame(ds,try auxiliaryStat(q.directoryFD)),lstat(q.sourcePath,&ss)==0,AuxiliaryIdentity(ss)==q.sourceIdentity else { throw AuxiliaryCopyError.drift }
            var ls=stat();guard lstat(leaf,&ls)==0 else { throw AuxiliaryCopyError.destination }
            let result=AuxiliaryStageResult(version:"w03-aux-prestage-result-v1",nonce:q.nonce,manifestPath:q.manifestPath,manifestSHA256:q.manifestSHA256,sourcePath:q.sourcePath,sourceSHA256:q.sourceSHA256,directoryPath:q.directoryPath,leafPath:leaf,directoryIdentity:AuxiliaryIdentity(ds),leafIdentity:AuxiliaryIdentity(ls),sourceIdentity:q.sourceIdentity)
            try await verifyPrivateAuxiliary(path:leaf,identity:result.leafIdentity,sha256:q.sourceSHA256,directory:result.directoryIdentity,checkpoint:checkpoint)
            let output=try JSONEncoder().encode(result);guard output.count+1<=4096 else { throw AuxiliaryCopyError.limit };try FileHandle.standardOutput.write(contentsOf:output+Data([10]))
        } catch { try? FileHandle.standardError.write(contentsOf:Data("private auxiliary preparation refused\n".utf8));exit(1) }
    }
}
