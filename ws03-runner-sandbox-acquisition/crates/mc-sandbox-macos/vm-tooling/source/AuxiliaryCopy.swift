// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
import Foundation
import CryptoKit
import Darwin

enum AuxiliaryCopyError: Error { case limit, source, destination, drift, hash, io }
func auxiliaryStat(_ fd: Int32) throws -> stat {
    var s=stat();guard fstat(fd,&s)==0 else { throw AuxiliaryCopyError.io };return s
}
func auxiliarySame(_ a: stat,_ b: stat) -> Bool {
    a.st_dev==b.st_dev && a.st_ino==b.st_ino && a.st_size==b.st_size && a.st_uid==b.st_uid && a.st_gid==b.st_gid && a.st_mode==b.st_mode && a.st_nlink==b.st_nlink && a.st_mtimespec.tv_sec==b.st_mtimespec.tv_sec && a.st_mtimespec.tv_nsec==b.st_mtimespec.tv_nsec && a.st_ctimespec.tv_sec==b.st_ctimespec.tv_sec && a.st_ctimespec.tv_nsec==b.st_ctimespec.tv_nsec
}
func auxiliaryDirectorySame(_ a: stat,_ b: stat) -> Bool {
    a.st_dev==b.st_dev && a.st_ino==b.st_ino && a.st_uid==b.st_uid && a.st_gid==b.st_gid && a.st_mode==b.st_mode
}
@MainActor func copyVerifiedAuxiliary(source: String,destination: String,expectedBytes: Int64,expectedSHA256: String,maxBytes: Int64,checkpoint: () async throws -> Void) async throws {
    guard maxBytes>0,maxBytes<=64*1024*1024,expectedBytes>0,expectedBytes<=maxBytes else { throw AuxiliaryCopyError.limit }
    let parent=URL(fileURLWithPath:destination).deletingLastPathComponent().path
    guard source != destination,URL(fileURLWithPath:source).resolvingSymlinksInPath().path==source,
          URL(fileURLWithPath:parent).resolvingSymlinksInPath().path==parent else { throw AuxiliaryCopyError.destination }
    var directory=stat();guard lstat(parent,&directory)==0,directory.st_mode & S_IFMT == S_IFDIR,directory.st_uid==getuid(),directory.st_mode & 0o7777 == 0o700 else { throw AuxiliaryCopyError.destination }
    let parentFD=open(parent,O_RDONLY|O_DIRECTORY|O_NOFOLLOW);guard parentFD>=0 else { throw AuxiliaryCopyError.destination };defer { close(parentFD) }
    guard auxiliarySame(directory,try auxiliaryStat(parentFD)) else { throw AuxiliaryCopyError.destination }
    let leaf=URL(fileURLWithPath:destination).lastPathComponent
    func verifyParent() throws {
        var current=stat()
        guard lstat(parent,&current)==0,auxiliaryDirectorySame(directory,current),
              auxiliaryDirectorySame(directory,try auxiliaryStat(parentFD)),
              URL(fileURLWithPath:parent).resolvingSymlinksInPath().path==parent else { throw AuxiliaryCopyError.destination }
    }
    let input=open(source,O_RDONLY|O_NOFOLLOW);guard input>=0 else { throw AuxiliaryCopyError.source };defer { close(input) }
    let before=try auxiliaryStat(input)
    guard before.st_mode & S_IFMT == S_IFREG,before.st_nlink==1,before.st_uid==getuid(),before.st_mode & 0o022==0,before.st_size==expectedBytes else { throw AuxiliaryCopyError.source }
    let output=openat(parentFD,leaf,O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW,0o600)
    guard output>=0 else { throw AuxiliaryCopyError.destination };defer { close(output) }
    var hash=SHA256(),total:Int64=0,buffer=[UInt8](repeating:0,count:1024*1024)
    while true {
        try await checkpoint()
        let n=read(input,&buffer,buffer.count);guard n>=0 else { throw AuxiliaryCopyError.io };if n==0 { break }
        total += Int64(n);guard total<=expectedBytes,total<=maxBytes else { throw AuxiliaryCopyError.limit }
        hash.update(data:Data(buffer.prefix(n)))
        try buffer.withUnsafeBytes { raw in
            guard let base=raw.baseAddress else { throw AuxiliaryCopyError.io };var offset=0
            while offset<n { let count=write(output,base.advanced(by:offset),n-offset);guard count>0 else { throw AuxiliaryCopyError.io };offset += count }
        }
    }
    var pathStat=stat()
    guard total==expectedBytes,auxiliarySame(before,try auxiliaryStat(input)),lstat(source,&pathStat)==0,auxiliarySame(before,pathStat) else { throw AuxiliaryCopyError.drift }
    guard hash.finalize().map({String(format:"%02x",$0)}).joined()==expectedSHA256 else { throw AuxiliaryCopyError.hash }
    guard fsync(output)==0 else { throw AuxiliaryCopyError.io }
    try verifyParent()
    let copy=openat(parentFD,leaf,O_RDONLY|O_NOFOLLOW);guard copy>=0 else { throw AuxiliaryCopyError.destination };defer { close(copy) }
    let copyBefore=try auxiliaryStat(copy)
    guard auxiliarySame(try auxiliaryStat(output),copyBefore),copyBefore.st_size==expectedBytes,copyBefore.st_nlink==1,copyBefore.st_uid==getuid(),copyBefore.st_mode & 0o7777==0o600 else { throw AuxiliaryCopyError.destination }
    var copiedHash=SHA256()
    while true { try await checkpoint();let n=read(copy,&buffer,buffer.count);guard n>=0 else { throw AuxiliaryCopyError.io };if n==0 { break };copiedHash.update(data:Data(buffer.prefix(n))) }
    guard auxiliarySame(copyBefore,try auxiliaryStat(copy)),lstat(destination,&pathStat)==0,auxiliarySame(copyBefore,pathStat),copiedHash.finalize().map({String(format:"%02x",$0)}).joined()==expectedSHA256 else { throw AuxiliaryCopyError.hash }
    try verifyParent()
    guard fsync(parentFD)==0 else { throw AuxiliaryCopyError.io }
}

struct AuxiliaryIdentity: Codable, Equatable {
    let device: Int64, inode: UInt64, uid: UInt32, gid: UInt32, mode: UInt32, nlink: UInt64, size: Int64, mtimeNs: Int64, ctimeNs: Int64
    init(_ s: stat) {
        device=Int64(s.st_dev);inode=UInt64(s.st_ino);uid=s.st_uid;gid=s.st_gid;mode=UInt32(s.st_mode);nlink=UInt64(s.st_nlink);size=s.st_size
        mtimeNs=Int64(s.st_mtimespec.tv_sec)*1_000_000_000+Int64(s.st_mtimespec.tv_nsec)
        ctimeNs=Int64(s.st_ctimespec.tv_sec)*1_000_000_000+Int64(s.st_ctimespec.tv_nsec)
    }
}
struct AuxiliaryStageRequest: Codable {
    let version: String, nonce: String, manifestPath: String, manifestSHA256: String, sourcePath: String, sourceSHA256: String, directoryPath: String
    let expectedBytes: Int64, maxBytes: Int64, directoryFD: Int32
    let directoryIdentity: AuxiliaryIdentity, sourceIdentity: AuxiliaryIdentity
}
struct AuxiliaryStageResult: Codable {
    let version: String, nonce: String, manifestPath: String, manifestSHA256: String, sourcePath: String, sourceSHA256: String, directoryPath: String, leafPath: String
    let directoryIdentity: AuxiliaryIdentity, leafIdentity: AuxiliaryIdentity, sourceIdentity: AuxiliaryIdentity
}
let auxiliaryIdentityKeys=Set(["device","inode","uid","gid","mode","nlink","size","mtimeNs","ctimeNs"])
func auxiliaryShape(_ raw: Data,result: Bool) throws {
    guard let object=try JSONSerialization.jsonObject(with:raw) as? [String:Any] else { throw AuxiliaryCopyError.source }
    let common=Set(["version","nonce","manifestPath","manifestSHA256","sourcePath","sourceSHA256","directoryPath","directoryIdentity","sourceIdentity"])
    guard Set(object.keys)==common.union(result ? ["leafPath","leafIdentity"]:["expectedBytes","maxBytes","directoryFD"]) else { throw AuxiliaryCopyError.source }
    for name in result ? ["directoryIdentity","sourceIdentity","leafIdentity"]:["directoryIdentity","sourceIdentity"] {
        guard let identity=object[name] as? [String:Any],Set(identity.keys)==auxiliaryIdentityKeys else { throw AuxiliaryCopyError.source }
    }
}
func auxiliaryDigest(_ data: Data) -> String { SHA256.hash(data:data).map{String(format:"%02x",$0)}.joined() }
func auxiliaryHex(_ value: String,_ count: Int) -> Bool { value.count==count && value.utf8.allSatisfy { (48...57).contains($0) || (97...102).contains($0) } }
@MainActor func verifyPrivateAuxiliary(path: String,identity: AuxiliaryIdentity,sha256: String,directory: AuxiliaryIdentity,checkpoint: () async throws -> Void) async throws {
    let parent=URL(fileURLWithPath:path).deletingLastPathComponent().path
    guard URL(fileURLWithPath:path).resolvingSymlinksInPath().path==path else { throw AuxiliaryCopyError.destination }
    var ds=stat();guard lstat(parent,&ds)==0,AuxiliaryIdentity(ds)==directory,ds.st_mode & S_IFMT==S_IFDIR,ds.st_uid==getuid(),ds.st_mode & 0o7777==0o700 else { throw AuxiliaryCopyError.destination }
    let fd=open(path,O_RDONLY|O_NOFOLLOW);guard fd>=0 else { throw AuxiliaryCopyError.destination };defer { close(fd) }
    let before=try auxiliaryStat(fd)
    guard AuxiliaryIdentity(before)==identity,before.st_mode & S_IFMT==S_IFREG,before.st_nlink==1,before.st_uid==getuid(),before.st_mode & 0o7777==0o600,before.st_dev==ds.st_dev,before.st_size>0,before.st_size<=64*1024*1024 else { throw AuxiliaryCopyError.destination }
    var hash=SHA256(),buffer=[UInt8](repeating:0,count:1024*1024)
    while true { try await checkpoint();let n=read(fd,&buffer,buffer.count);guard n>=0 else { throw AuxiliaryCopyError.io };if n==0 { break };hash.update(data:Data(buffer.prefix(n))) }
    var leaf=stat(),after=stat()
    guard auxiliarySame(before,try auxiliaryStat(fd)),lstat(path,&leaf)==0,auxiliarySame(before,leaf),lstat(parent,&after)==0,auxiliarySame(ds,after),hash.finalize().map({String(format:"%02x",$0)}).joined()==sha256 else { throw AuxiliaryCopyError.hash }
}
