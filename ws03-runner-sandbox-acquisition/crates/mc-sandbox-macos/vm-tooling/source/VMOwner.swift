// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
import Foundation
import Virtualization
import CryptoKit
import Darwin

enum Failure: Error { case preparationAbandoned, preparationTimeout, invalidConfig, unsupportedImage, existingBundle, fileMismatch(String), io(Int32), stateUnknown }
struct Config: Decodable {
    let prototype: String, evidence: String, lab: String, imagePath: String, bundlePath: String
    let imageBuild: String, imageVersion: String, imageSHA256: String
    let expectedManifestPath: String, expectedManifestSHA256: String
    let cpu: Int, ram: UInt64, disk: Int64, imageBytes: Int64
    let metadataDeadline: Double, installDeadline: Double, stopDeadline: Double
    let runDeadline: Double?
    let progressInterval: Double?, monitorHeartbeatMaxAge: Double?
    let controllerPID: Int32?, monitorPID: Int32?, monitorHeartbeatPath: String?
    let monitorNonce: String?, installedBaseline: Bool?
    let runNonce: String?, ownerJournalPath: String?, policy: String?, assumptionsVerified: Bool?
    let ownerJournalDevice: Int64?, ownerJournalInode: UInt64?, ownerJournalMaxBytes: Int?, ownerJournalReserveBytes: Int?
    let controlHeartbeatMaxAge: Double?, controlInterval: Double?, controlMaxBytes: Int?
    let prebeginAuxiliaryCopyPath: String?, prebeginAuxiliaryMaxBytes: Int64?
    let prebeginAuxiliaryStage: AuxiliaryStageResult?
    let nativeMode: String?, nativePrebeginAllowed: Bool?, preparationDeadline: Double?
    var nativePolicy: NativePolicy { NativePolicy(mode:nativeMode,prebeginAllowed:nativePrebeginAllowed,installationAllowed:nativeInstallationAllowed) }
    let nativeInstallationAllowed: Bool?
    let ownerTokenSHA256: String?, fullDeadline: Double?
    let installedBaselineEvidencePath: String?, installedBaselineEvidenceSHA256: String?
    func validate() throws {
        guard cpu > 0, ram > 0, disk > 0, disk % 512 == 0, imageBytes > 0,
              metadataDeadline > 0, installDeadline > 0, stopDeadline > 0,
              imageSHA256.count == 64, expectedManifestSHA256.count == 64,
              lab != evidence, bundlePath == URL(fileURLWithPath:lab).appendingPathComponent("Guest.bundle").path,
              imagePath == URL(fileURLWithPath:lab).appendingPathComponent("Restore.ipsw").path else { throw Failure.invalidConfig }
        for path in [lab,evidence,imagePath,expectedManifestPath] { try canonical(path) }
        guard bundlePath.hasPrefix("/"),URL(fileURLWithPath:bundlePath).resolvingSymlinksInPath().path==bundlePath else { throw Failure.invalidConfig }
    }
    func validateNative() throws {
        guard nativeInstallationAllowed == true || nativePolicy.prebegin else { throw Failure.invalidConfig }
        guard let controllerPID, let monitorPID, controllerPID > 1, monitorPID > 1,
              controllerPID != getpid(), monitorPID != getpid(),
              let progressInterval, progressInterval.isFinite, progressInterval > 0,
              let controlInterval, controlInterval.isFinite, controlInterval > 0, controlInterval <= 0.5,
              let controlHeartbeatMaxAge, controlHeartbeatMaxAge.isFinite, controlHeartbeatMaxAge > controlInterval,
              let controlMaxBytes, controlMaxBytes > 0, controlMaxBytes <= 4096,
              let runNonce, runNonce.count >= 32, runNonce.allSatisfy({ $0.isHexDigit }),
              let ownerJournalPath, ownerJournalPath.hasPrefix(evidence + "/"),
              URL(fileURLWithPath:ownerJournalPath).resolvingSymlinksInPath().path == ownerJournalPath,
              (policy == "monitored-pilot-r7" || policy == "monitored-pilot-full5-v1"), assumptionsVerified == false else { throw Failure.invalidConfig }
        if nativePolicy.operational {
            var pipe=stat()
            guard getppid()==controllerPID, fstat(STDIN_FILENO,&pipe)==0, pipe.st_mode & S_IFMT == S_IFIFO,
                  let ownerTokenSHA256,ownerTokenSHA256.count==64,ownerTokenSHA256.allSatisfy({ $0.isHexDigit }),
                  let fullDeadline,fullDeadline.isFinite,fullDeadline>0,fullDeadline<=5,
                  controlHeartbeatMaxAge<=3 else { throw Failure.invalidConfig }
        }
        try canonical(URL(fileURLWithPath:ownerJournalPath).deletingLastPathComponent().path)
    }
}
struct FileRecord: Decodable {
    let path: String, dev: UInt64, ino: UInt64, uid: UInt32, gid: UInt32, mode: UInt32
    let bytes: Int64, mtimeNs: Int64, ctimeNs: Int64, sha256: String
}
struct BundleRecord: Decodable { let path: String, dev: UInt64, ino: UInt64, uid: UInt32, gid: UInt32, mode: UInt32 }
struct Manifest: Decodable {
    let schema: Int, lab: String, bundlePath: String, imagePath: String, diskCapacity: Int64
    let machineIdentifierBase64: String, hardwareModelBase64: String
    let bundle: BundleRecord, files: [String:FileRecord]
}
// NATIVE_TRANSPORT.md: all commands bind the one run, owner PID and VM object.
struct Control: Decodable {
    let command: String, nonce: String, instance: String, ownerStart: String
    let pid: Int32, commandSeq: UInt64
    let reason: String?
    let token: String?, configSHA256: String?
    let collectionStarted: Double?, recordSeq: UInt64?
}

@MainActor final class EventSink {
    let nonce: String
    let ownerStart = UUID().uuidString
    private let journal: Int32?
    private var sequence: UInt64 = 0
    private(set) var healthy = true
    private var outputHealthy = true
    // r10 F4: the journal's byte portion of run_bytes; optional telemetry stops before the closure reserve, nothing exceeds the bound.
    private var journalBytes = 0
    private let journalMax: Int, journalReserve: Int
    init(_ c: Config) throws {
        nonce=c.runNonce ?? ""
        if let path=c.ownerJournalPath {
            guard path.hasPrefix(c.evidence + "/"), URL(fileURLWithPath:path).resolvingSymlinksInPath().path==path,
                  let device=c.ownerJournalDevice, let inode=c.ownerJournalInode,
                  let max=c.ownerJournalMaxBytes, let reserve=c.ownerJournalReserveBytes, reserve > 0, max > reserve else { throw Failure.invalidConfig }
            let fd=open(path,O_WRONLY|O_CREAT|O_APPEND|O_NOFOLLOW,0o600)
            guard fd >= 0 else { throw Failure.io(errno) }
            var value=stat()
            // r10 F5: exactly the inode the controller created and bound in the digest-pinned run config.
            guard fstat(fd,&value)==0, value.st_mode & S_IFMT == S_IFREG,
                  value.st_uid==getuid(), value.st_mode & 0o7777==0o600,
                  value.st_nlink==1, value.st_size==0,
                  Int64(value.st_dev)==device, UInt64(value.st_ino)==inode else { close(fd);throw Failure.fileMismatch("unsafe_or_nonempty_journal") }
            journal=fd;journalMax=max;journalReserve=reserve
            let parent=open(URL(fileURLWithPath:path).deletingLastPathComponent().path,O_RDONLY|O_NOFOLLOW)
            guard parent >= 0 else { close(fd);throw Failure.io(errno) }
            let synced=fsync(parent), saved=errno, closed=close(parent)
            guard synced==0,closed==0 else { close(fd);throw Failure.io(saved) }
        } else { journal=nil;journalMax=0;journalReserve=0 }
        signal(SIGPIPE,SIG_IGN)
        let flags=fcntl(STDOUT_FILENO,F_GETFL)
        guard flags >= 0,fcntl(STDOUT_FILENO,F_SETFL,flags|O_NONBLOCK)==0 else { throw Failure.io(errno) }
    }
    // A native owner never blocks on a disconnected recorder's stdout pipe.
    func emit(_ event: String, _ fields: [String:Any], instance: String?) {
        var value=fields
        value["event"]=event;value["nonce"]=nonce;value["pid"]=getpid();value["seq"]=sequence+1
        value["ownerStart"]=ownerStart
        value["instance"]=instance.map { $0 as Any } ?? NSNull()
        value["timestampEpoch"]=Date().timeIntervalSince1970
        value["monotonicSeconds"]=ProcessInfo.processInfo.systemUptime
        do {
            var data=try JSONSerialization.data(withJSONObject:value,options:[.sortedKeys]);data.append(10)
            if journal != nil {
                // Admission before the sequence number is consumed: optional liveness/progress telemetry stops at the regular bound;
                // lifecycle events may use the closure reserve; nothing is written past the hard bound.
                if (event == "owner_heartbeat" || event == "install_progress") && journalBytes+data.count > journalMax-journalReserve { return }
                guard journalBytes+data.count <= journalMax else { throw Failure.io(EFBIG) }
            }
            sequence += 1
            if let journal {
                try data.withUnsafeBytes { raw in
                    guard let ptr=raw.baseAddress else { throw Failure.invalidConfig }
                    var offset=0
                    while offset<data.count {
                        let count=write(journal,ptr.advanced(by:offset),data.count-offset)
                        if count<0 && errno==EINTR { continue }
                        guard count>0 else { throw Failure.io(errno) };offset += count;journalBytes += count
                    }
                }
                var result=fsync(journal)
                while result<0 && errno==EINTR { result=fsync(journal) }
                guard result==0 else { throw Failure.io(errno) }
            }
            if outputHealthy {
                let written=data.withUnsafeBytes { write(STDOUT_FILENO,$0.baseAddress,data.count) }
                if written != data.count { outputHealthy=false;healthy=false }
            }
        } catch {
            healthy=false;outputHealthy=false
            fputs("Native evidence journal failure; retained UNKNOWN owner\n",stderr)
        }
    }
}
// r6 dispatch B: these are the one preserved image and four saved platform/storage files.
let fileKeys = ["Restore.ipsw","Guest.bundle/Hardware.bin","Guest.bundle/Machine.bin","Guest.bundle/Auxiliary.bin","Guest.bundle/Disk.img"]
func canonical(_ path: String) throws {
    guard path.hasPrefix("/"), URL(fileURLWithPath:path).resolvingSymlinksInPath().path == path,
          FileManager.default.fileExists(atPath:path) else { throw Failure.fileMismatch("noncanonical_or_missing_path") }
}
func statPath(_ path: String) throws -> stat {
    var value=stat()
    guard lstat(path,&value)==0 else { throw Failure.io(errno) }
    return value
}
func matches(_ value: stat, _ record: FileRecord) -> Bool {
    UInt64(value.st_dev)==record.dev && UInt64(value.st_ino)==record.ino && value.st_uid==record.uid &&
    value.st_gid==record.gid && UInt32(value.st_mode & 0o7777)==record.mode && value.st_size==record.bytes &&
    Int64(value.st_mtimespec.tv_sec)*1_000_000_000+Int64(value.st_mtimespec.tv_nsec)==record.mtimeNs &&
    Int64(value.st_ctimespec.tv_sec)*1_000_000_000+Int64(value.st_ctimespec.tv_nsec)==record.ctimeNs
}
@MainActor func verifiedRead(_ path: String, expected: FileRecord? = nil, retain: Bool = false, checkpoint: ((Int) async throws -> Void)? = nil) async throws -> (String,Data) {
    try canonical(path)
    let before=try statPath(path)
    guard before.st_mode & S_IFMT == S_IFREG, before.st_nlink==1,
          before.st_uid==getuid(), before.st_mode & 0o022==0 else { throw Failure.fileMismatch("unsafe_file") }
    if let expected { guard expected.path==path, matches(before,expected) else { throw Failure.fileMismatch("metadata_mismatch") } }
    let fd=open(path,O_RDONLY|O_NOFOLLOW)
    guard fd >= 0 else { throw Failure.io(errno) }
    defer { close(fd) }
    var opened=stat()
    guard fstat(fd,&opened)==0, auxiliarySame(before,opened) else { throw Failure.fileMismatch("open_race") }
    // r5: stream past the unified buffer cache (same bytes, same digest). Cached 26.6 GB image and 64 GiB disk reads
    // evicted the concurrent freshness collector's metadata and slowed every scan during preparation.
    guard fcntl(fd,F_NOCACHE,1) != -1 else { throw Failure.io(errno) }
    // Fully sparse files (st_blocks==0) cause no device I/O: shorter pacing, same per-chunk checks.
    let paceMs=opened.st_blocks==0 ? 2 : 10
    var digest=SHA256(), output=Data(), buffer=[UInt8](repeating:0,count:8*1024*1024)
    while true {
        if let checkpoint { try await checkpoint(paceMs) }
        let count=read(fd,&buffer,buffer.count)
        guard count >= 0 else { throw Failure.io(errno) }
        if count==0 { break }
        buffer.withUnsafeBytes { raw in
            let chunk=UnsafeRawBufferPointer(rebasing:raw[..<count])
            digest.update(bufferPointer:chunk)
            if retain { output.append(contentsOf:chunk) }
        }
    }
    var after=stat()
    guard fstat(fd,&after)==0, auxiliarySame(before,after), auxiliarySame(before,try statPath(path)) else { throw Failure.fileMismatch("read_race") }
    let hash=digest.finalize().map { String(format:"%02x",$0) }.joined()
    if let expected { guard hash==expected.sha256 else { throw Failure.fileMismatch("hash_mismatch") } }
    return (hash,output)
}
func stateName(_ state: VZVirtualMachine.State) -> String {
    switch state {
    case .stopped: return "stopped"
    case .running: return "running"
    case .paused: return "paused"
    case .error: return "error"
    case .starting: return "starting"
    case .pausing: return "pausing"
    case .resuming: return "resuming"
    case .stopping: return "stopping"
    case .saving: return "saving"
    case .restoring: return "restoring"
    @unknown default: return "unknown"
    }
}

@MainActor final class Owner: NSObject, @MainActor VZVirtualMachineDelegate {
    let c: Config
    let sink: EventSink
    var vm: VZVirtualMachine?, installer: VZMacOSInstaller?, timer: Timer?
    var controlSource: DispatchSourceRead?, input=Data(), manifest: Manifest?
    var installActive=false, cancellationRequested=false, began=false, callbackReceived=false
    var stoppedReported=false, evidenceFault=false, controlFault=false, finished=false
    var lastCommand: UInt64=0, lastHeartbeat: Double?, readyAt: Double=0, startedAt: Double=0
    var lastProgress: Double=0, lastState: Int?, cancelAt: Double?, alertReported=false
    var preparing=false, preparationStop=false, preparationHandoff=false
    var preparationStarted: Double=0, preparationFinish: UInt64?
    var installResult=false
    var bootStarting=false, bootStopPending=false, bootResult=false
    let configPath: String, configPin: String
    init(_ c: Config, sink: EventSink, configPath: String, configPin: String) {
        self.c=c;self.sink=sink;self.configPath=configPath;self.configPin=configPin
    }
    func verifyBaseline() throws {
        guard c.installedBaseline==true,let path=c.installedBaselineEvidencePath,let pin=c.installedBaselineEvidenceSHA256,
              let runDeadline=c.runDeadline,runDeadline.isFinite,runDeadline>0 else { throw Failure.invalidConfig }
        let raw=try Data(contentsOf:URL(fileURLWithPath:path))
        guard auxiliaryDigest(raw)==pin,
              let evidence=try JSONSerialization.jsonObject(with:raw) as? [String:Any],
              evidence["installedBaseline"] as? Bool == true,
              evidence["expectedManifestSHA256"] as? String == c.expectedManifestSHA256,
              let receiptPath=evidence["installationReceiptPath"] as? String,
              let receiptPin=evidence["installationReceiptSHA256"] as? String else { throw Failure.invalidConfig }
        let receiptRaw=try Data(contentsOf:URL(fileURLWithPath:receiptPath))
        guard auxiliaryDigest(receiptRaw)==receiptPin,
              let r=try JSONSerialization.jsonObject(with:receiptRaw) as? [String:Any],
              r["outcome"] as? String == "INSTALLED",r["control_clean"] as? Bool == true,
              r["native_state"] as? String == "EXACT_STOPPED",r["operation"] as? String == "reinstall-existing",
              let unknown=r["unknown"] as? [String],unknown.isEmpty,
              let waits=r["child_waits"] as? [String:Int],Set(waits.keys)==Set(["owner","monitor","recorder"]),waits.values.allSatisfy({$0==0}),
              let owner=r["owner"] as? [String:Any],owner["callback"] as? String == "install_callback_success",
              owner["exactStopEvent"] as? Bool == true,owner["finishAckCommandSeq"] as? UInt64 != nil,
              owner["cancelAcknowledged"] as? Bool == false,
              let faults=owner["unknown"] as? [String],faults.isEmpty,
              let breaches=owner["breaches"] as? [String],breaches.isEmpty else { throw Failure.invalidConfig }
    }
    func emit(_ event: String, _ fields: [String:Any] = [:]) {
        sink.emit(event,fields,instance:vm.map { String(describing:ObjectIdentifier($0)) })
        if !sink.healthy && !evidenceFault {
            evidenceFault=true
            sink.emit("transport_fault",["reason":"event_delivery_or_durability_failed","outcome":"UNKNOWN"],instance:vm.map { String(describing:ObjectIdentifier($0)) })
            requestCancellation("event_delivery_or_durability_failed")
        }
    }
    func stateFields(_ machine: VZVirtualMachine) -> [String:Any] {
        ["state":machine.state.rawValue,"stateName":stateName(machine.state),"stopped":machine.state == .stopped,
         "instance":String(describing:ObjectIdentifier(machine)),"pid":getpid()]
    }
    func guestDidStop(_ virtualMachine: VZVirtualMachine) { emit("delegate_guest_stopped",stateFields(virtualMachine)) }
    func virtualMachine(_ virtualMachine: VZVirtualMachine, didStopWithError error: Error) {
        var fields=stateFields(virtualMachine);fields["error"]=String(describing:error);emit("delegate_stop_error",fields);fault("delegate_stop_error")
    }
    func verifyExisting() async throws -> Manifest {
        let (hash,raw)=try await verifiedRead(c.expectedManifestPath,retain:true,checkpoint:preparationCheckpoint)
        guard hash==c.expectedManifestSHA256 else { throw Failure.fileMismatch("manifest_hash_mismatch") }
        let m=try JSONDecoder().decode(Manifest.self,from:raw)
        guard m.schema==1,m.lab==c.lab,m.bundlePath==c.bundlePath,m.imagePath==c.imagePath,
              m.diskCapacity==c.disk,Set(m.files.keys)==Set(fileKeys) else { throw Failure.fileMismatch("manifest_scope_mismatch") }
        let directory=try statPath(c.bundlePath), b=m.bundle
        guard directory.st_mode & S_IFMT == S_IFDIR, directory.st_mode & 0o7777==0o700,
              directory.st_uid==getuid(), b.path==c.bundlePath,UInt64(directory.st_dev)==b.dev,
              UInt64(directory.st_ino)==b.ino,directory.st_uid==b.uid,directory.st_gid==b.gid,
              UInt32(directory.st_mode & 0o7777)==b.mode else { throw Failure.fileMismatch("bundle_identity_mismatch") }
        guard Set(try FileManager.default.contentsOfDirectory(atPath:c.bundlePath))==Set(["Hardware.bin","Machine.bin","Auxiliary.bin","Disk.img"]) else { throw Failure.fileMismatch("unexpected_bundle_members") }
        var identities=Set<String>()
        for key in fileKeys {
            guard let record=m.files[key],record.path==URL(fileURLWithPath:c.lab).appendingPathComponent(key).path,
                  identities.insert("\(record.dev):\(record.ino)").inserted else { throw Failure.fileMismatch("duplicate_or_wrong_path") }
            emit("full_hash_begin",["file":key,"bytes":record.bytes])
            let (_,data)=try await verifiedRead(record.path,expected:record,retain:key.hasSuffix("Hardware.bin") || key.hasSuffix("Machine.bin"),checkpoint:preparationCheckpoint)
            emit("full_hash_complete",["file":key,"sha256":record.sha256])
            if key.hasSuffix("Hardware.bin") { guard data.base64EncodedString()==m.hardwareModelBase64 else { throw Failure.fileMismatch("saved_hardware_mismatch") } }
            if key.hasSuffix("Machine.bin") { guard data.base64EncodedString()==m.machineIdentifierBase64 else { throw Failure.fileMismatch("saved_identifier_mismatch") } }
        }
        guard m.files["Guest.bundle/Disk.img"]?.bytes==c.disk,
              m.files["Restore.ipsw"]?.bytes==c.imageBytes,m.files["Restore.ipsw"]?.sha256==c.imageSHA256 else { throw Failure.fileMismatch("disk_or_image_identity_mismatch") }
        try verifyMetadata(m)
        emit("existing_bundle_full_hash_verified",["manifestSHA256":hash,"machineIdentifierBase64":m.machineIdentifierBase64,"hardwareModelBase64":m.hardwareModelBase64,"diskCapacity":c.disk])
        return m
    }
    func verifyMetadata(_ m: Manifest) throws {
        for record in m.files.values { try canonical(record.path);guard matches(try statPath(record.path),record) else { throw Failure.fileMismatch("post_read_race") } }
        let s=try statPath(c.bundlePath)
        guard UInt64(s.st_dev)==m.bundle.dev,UInt64(s.st_ino)==m.bundle.ino else { throw Failure.fileMismatch("bundle_identity_mismatch") }
    }
    func loadImage() async throws -> VZMacOSRestoreImage {
        let image=try await VZMacOSRestoreImage.image(from:URL(fileURLWithPath:c.imagePath))
        guard let req=image.mostFeaturefulSupportedConfiguration else { throw Failure.unsupportedImage }
        let v=image.operatingSystemVersion, version="\(v.majorVersion).\(v.minorVersion).\(v.patchVersion)"
        let admitted=image.isSupported && req.hardwareModel.isSupported && c.cpu>=req.minimumSupportedCPUCount && c.ram>=req.minimumSupportedMemorySize && image.buildVersion==c.imageBuild && version==c.imageVersion
        emit("local_image",["version":version,"build":image.buildVersion,"supported":image.isSupported,"hardwareSupported":req.hardwareModel.isSupported,"hardwareModelBase64":req.hardwareModel.dataRepresentation.base64EncodedString(),"minimumCPU":req.minimumSupportedCPUCount,"minimumRAM":req.minimumSupportedMemorySize,"admitted":admitted])
        guard admitted else { throw Failure.unsupportedImage }
        return image
    }
    func configuration(image: VZMacOSRestoreImage) async throws -> VZVirtualMachineConfiguration {
        let root=URL(fileURLWithPath:c.bundlePath)
        guard let m=manifest,let hdata=Data(base64Encoded:m.hardwareModelBase64),let mdata=Data(base64Encoded:m.machineIdentifierBase64),
              let hardware=VZMacHardwareModel(dataRepresentation:hdata),hardware.isSupported,
              let machine=VZMacMachineIdentifier(dataRepresentation:mdata) else { throw Failure.fileMismatch("saved_platform_invalid") }
        guard image.mostFeaturefulSupportedConfiguration?.hardwareModel.dataRepresentation==hardware.dataRepresentation else { throw Failure.unsupportedImage }
        // SDK VZMacAuxiliaryStorage.h: use only the newly staged, read-verified private inode.
        var auxiliaryURL=root.appendingPathComponent("Auxiliary.bin")
        if c.nativePolicy.prebegin {
            guard let destination=c.prebeginAuxiliaryCopyPath,let limit=c.prebeginAuxiliaryMaxBytes,let stage=c.prebeginAuxiliaryStage,
                  let journal=c.ownerJournalPath,destination==URL(fileURLWithPath:journal).deletingLastPathComponent().appendingPathComponent("auxiliary/Auxiliary.private.bin").path,
                  stage.version=="w03-aux-prestage-result-v1",auxiliaryHex(stage.nonce,32),stage.manifestPath==c.expectedManifestPath,stage.manifestSHA256==c.expectedManifestSHA256,
                  stage.leafPath==destination,stage.directoryPath==URL(fileURLWithPath:destination).deletingLastPathComponent().path,
                  let record=m.files["Guest.bundle/Auxiliary.bin"],stage.sourcePath==record.path,stage.sourceSHA256==record.sha256,
                  stage.leafIdentity.size==record.bytes,record.bytes>0,record.bytes<=limit,limit<=64*1024*1024,
                  stage.sourceIdentity==AuxiliaryIdentity(try statPath(record.path)) else { throw Failure.invalidConfig }
            try verifyMetadata(m)
            try await verifyPrivateAuxiliary(path:destination,identity:stage.leafIdentity,sha256:record.sha256,directory:stage.directoryIdentity,checkpoint:{ try await self.preparationCheckpoint() })
            try verifyMetadata(m)
            auxiliaryURL=URL(fileURLWithPath:destination)
            emit("private_auxiliary_verified",["bytes":record.bytes,"sha256":record.sha256,"retained":true])
        }
        let aux=VZMacAuxiliaryStorage(url:auxiliaryURL)
        let platform=VZMacPlatformConfiguration();platform.hardwareModel=hardware;platform.machineIdentifier=machine;platform.auxiliaryStorage=aux
        let config=VZVirtualMachineConfiguration();config.platform=platform;config.bootLoader=VZMacOSBootLoader();config.cpuCount=c.cpu;config.memorySize=c.ram
        let disk=try VZDiskImageStorageDeviceAttachment(url:root.appendingPathComponent("Disk.img"),readOnly:c.nativePolicy.prebegin)
        config.storageDevices=[VZVirtioBlockDeviceConfiguration(attachment:disk)]
        let graphics=VZMacGraphicsDeviceConfiguration();graphics.displays=[VZMacGraphicsDisplayConfiguration(widthInPixels:1280,heightInPixels:800,pixelsPerInch:80)]
        config.graphicsDevices=[graphics];config.keyboards=[VZUSBKeyboardConfiguration()];config.pointingDevices=[VZMacTrackpadConfiguration()]
        config.networkDevices=[];config.directorySharingDevices=[];config.socketDevices=[];config.serialPorts=[]
        try config.validate();emit("configuration_validated",["networkDevices":0,"directoryShares":0,"socketDevices":0,"serialPorts":0,"cpu":c.cpu,"memory":c.ram,"existing":true])
        return config
    }
    func own(_ config: VZVirtualMachineConfiguration) {
        let machine=VZVirtualMachine(configuration:config);machine.delegate=self;vm=machine
        emit("owner_created",stateFields(machine));observeState()
    }
    func observeState() {
        guard let machine=vm else { return }
        let state=Int(machine.state.rawValue)
        if lastState != state { lastState=state;emit("state",stateFields(machine)) }
        if callbackReceived && machine.state == .stopped && !stoppedReported {
            stoppedReported=true
            emit("exact_owner_stopped_after_callback",stateFields(machine))
        }
    }
    func requestCancellation(_ reason: String) {
        if c.nativeMode == "baseline" {
            guard began,!callbackReceived,!cancellationRequested else { return }
            cancellationRequested=true;cancelAt=ProcessInfo.processInfo.systemUptime
            emit("cancel_requested",["reason":reason,"cleanup":"UNKNOWN_UNTIL_CALLBACK_AND_EXACT_STOPPED"])
            if !bootStarting { stopGuest() }
            return
        }
        guard installActive,let installer,!cancellationRequested else { return }
        cancellationRequested=true;cancelAt=ProcessInfo.processInfo.systemUptime
        // Apple install/progress documentation: call only after installation was started.
        installer.progress.cancel()
        emit("cancel_requested",["reason":reason,"cleanup":"UNKNOWN_UNTIL_CALLBACK_AND_EXACT_STOPPED"])
    }
    func fault(_ reason: String) {
        guard !controlFault else { return }
        controlFault=true;emit("transport_fault",["reason":reason,"outcome":"UNKNOWN"])
        requestCancellation(reason)
    }
    func processCommand(_ line: Data) {
        do {
            guard c.nativeInstallationAllowed == true || c.nativePolicy.prebegin else { throw Failure.invalidConfig }
            guard let value=try JSONSerialization.jsonObject(with:line) as? [String:Any],
                  Set(value.keys).isSubset(of:Set(["command","nonce","pid","instance","ownerStart","commandSeq","reason","token","configSHA256","collectionStarted","recordSeq"])) else { throw Failure.invalidConfig }
            let command=try JSONDecoder().decode(Control.self,from:line)
            guard command.nonce==c.runNonce,command.pid==getpid(),
                  command.ownerStart==sink.ownerStart, lastCommand<UInt64.max,
                  command.commandSeq==lastCommand+1 else { throw Failure.invalidConfig }
            if c.nativePolicy.operational {
                guard let token=command.token,token.count==64,
                      auxiliaryDigest(Data(token.utf8))==c.ownerTokenSHA256,
                      command.configSHA256==configPin else { throw Failure.invalidConfig } // GUARD_OWNER_TOKEN
            }
            if preparing && vm == nil {
                guard (!preparationHandoff || (preparationStop && command.command == "prepare_finish")),command.instance.isEmpty,["heartbeat","prepare_finish","handoff"].contains(command.command) else { throw Failure.invalidConfig }
                lastCommand=command.commandSeq
                if command.command == "heartbeat" { lastHeartbeat=ProcessInfo.processInfo.systemUptime }
                else if command.command == "handoff" {
                    guard !preparationStop,!controlFault,!evidenceFault,monitorHealthy() else { throw Failure.stateUnknown }
                    preparationHandoff=true
                }
                else { guard preparationFinish == nil else { throw Failure.invalidConfig };preparationFinish=command.commandSeq;preparationStop=true }
                return
            }
            guard let machine=vm,command.instance==String(describing:ObjectIdentifier(machine)),
                  ["heartbeat","begin","cancel","finish"].contains(command.command) else { throw Failure.invalidConfig }
            lastCommand=command.commandSeq
            switch command.command {
            case "heartbeat":
                guard !controlFault,!evidenceFault else { throw Failure.stateUnknown }
                lastHeartbeat=ProcessInfo.processInfo.systemUptime
                // owner_heartbeat is the periodic acknowledgement; no redundant per-heartbeat log.
            case "begin":
                guard c.nativePolicy.operational else { throw Failure.invalidConfig } // GUARD_PREBEGIN_COMMAND
                guard !finished,!began,!controlFault,!evidenceFault,monitorHealthy(),machine.state == .stopped,
                      let manifest else { throw Failure.stateUnknown }
                guard let collected=command.collectionStarted,collected.isFinite,let deadline=c.fullDeadline,
                      collected>0,ProcessInfo.processInfo.systemUptime>=collected,
                      ProcessInfo.processInfo.systemUptime-collected<=deadline,
                      let record=command.recordSeq,record>0 else { throw Failure.stateUnknown } // GUARD_BEGIN_ADMISSION
                let raw=try Data(contentsOf:URL(fileURLWithPath:configPath))
                guard auxiliaryDigest(raw)==configPin else { throw Failure.invalidConfig }
                if c.nativeMode == "baseline" { try verifyBaseline() }
                try verifyMetadata(manifest)
                guard monitorHealthy(),ProcessInfo.processInfo.systemUptime-collected<=deadline else { throw Failure.stateUnknown }
                emit("command_ack",["command":"begin","commandSeq":command.commandSeq])
                guard sink.healthy,monitorHealthy(),ProcessInfo.processInfo.systemUptime-collected<=deadline else { throw Failure.stateUnknown }
                began=true
                if c.nativeMode == "baseline" { beginGuest(machine,collected:collected,deadline:deadline) }
                else { startInstallation() }
            case "cancel":
                emit("command_ack",["command":"cancel","commandSeq":command.commandSeq])
                if !began { fault("cancel_before_begin") }
                requestCancellation(command.reason ?? "supervisor_cancel")
            case "finish":
                guard machine.state == .stopped,(!began || (callbackReceived && stoppedReported)) else { throw Failure.stateUnknown }
                emit("finish_ack",["command":"finish","commandSeq":command.commandSeq,"stateName":"stopped","outcome":(!controlFault && !evidenceFault && !cancellationRequested && (c.nativeMode == "baseline" ? bootResult : installResult)) ? (c.nativeMode == "baseline" ? "BOOT_COMPLETED" : "INSTALLED") : "UNKNOWN_OR_NOT_INSTALLED"])
                finished=true
            default: throw Failure.invalidConfig
            }
        } catch { fault("malformed_stale_or_invalid_command") }
    }
    func readControl() {
        guard let max=c.controlMaxBytes else { fault("missing_control_limit");return }
        var buffer=[UInt8](repeating:0,count:max)
        let count=read(STDIN_FILENO,&buffer,buffer.count)
        if count<0 && (errno==EAGAIN || errno==EINTR) { return }
        guard count>0 else { controlSource?.cancel();fault("control_eof_or_read_error");return }
        input.append(contentsOf:buffer.prefix(count))
        while let newline=input.firstIndex(of:10) {
            let line=Data(input[..<newline]);input.removeSubrange(...newline)
            guard !line.isEmpty,line.count<=max else { fault("oversized_or_empty_command");return }
            processCommand(line)
        }
        if input.count>max { fault("oversized_command");input.removeAll(keepingCapacity:false) }
    }
    func monitorHealthy() -> Bool {
        guard let controller=c.controllerPID,let monitor=c.monitorPID,let age=c.controlHeartbeatMaxAge,
              let lastHeartbeat else { return false }
        let fresh=ProcessInfo.processInfo.systemUptime-lastHeartbeat
        return getppid()==controller && fresh>=0 && fresh<=age && kill(controller,0)==0 && kill(monitor,0)==0
    }
    func tick() {
        let now=ProcessInfo.processInfo.systemUptime
        observeState()
        if preparing,let deadline=c.preparationDeadline,now-preparationStarted >= deadline { fault("preparation_deadline");preparationStop=true }
        emit("owner_heartbeat",["commandSeq":lastCommand,"active":installActive,"callbackReceived":callbackReceived,"fault":controlFault || evidenceFault])
        if let age=c.controlHeartbeatMaxAge, now-readyAt>age && !monitorHealthy() { fault("controller_monitor_or_heartbeat_lost") }
        if c.nativeMode == "baseline",began,!callbackReceived,let deadline=c.runDeadline,now-startedAt>=deadline,!bootStarting { stopGuest() }
        if installActive && now-startedAt>=c.installDeadline { requestCancellation("install_deadline") }
        if let cancelAt,now-cancelAt>=c.stopDeadline,!stoppedReported,!alertReported {
            alertReported=true;emit("owner_retained_unknown",["reason":"cancellation_settlement_deadline","outcome":"UNKNOWN"])
        }
        if let installer,let interval=c.progressInterval,now-lastProgress>=interval {
            lastProgress=now
            emit("install_progress",["fraction":installer.progress.fractionCompleted,"cancelled":installer.progress.isCancelled,"status":String((installer.progress.localizedDescription ?? "").prefix(512)),"active":installActive])
        }
    }
    func beginGuest(_ machine: VZVirtualMachine,collected: Double,deadline: Double) {
        bootStarting=true;startedAt=ProcessInfo.processInfo.systemUptime
        emit("boot_begin",stateFields(machine))
        Task { @MainActor [self] in
            do {
                guard !cancellationRequested,!controlFault,!evidenceFault,monitorHealthy(),ProcessInfo.processInfo.systemUptime-collected<=deadline else { throw Failure.stateUnknown }
                try await startGuest(machine)
                bootStarting=false;bootResult=true;emit("boot_start_success",stateFields(machine))
                if cancellationRequested || controlFault || evidenceFault { stopGuest() }
            } catch {
                bootStarting=false;bootResult=false;fault("guest_start_failed")
                if machine.state == .stopped && !bootStopPending { completeGuest(machine,error:true) }
                else { stopGuest() }
            }
        }
    }
    func stopGuest() {
        guard let machine=vm,began,!bootStarting,!bootStopPending,!callbackReceived else { return }
        bootStopPending=true
        Task { @MainActor [self] in
            do {
                if machine.state != .stopped { try await machine.stop() }
                guard vm === machine,machine.state == .stopped else { throw Failure.stateUnknown }
                completeGuest(machine,error:!bootResult)
            } catch { fault("guest_stop_failed");emit("owner_retained_unknown",["reason":"guest_stop_failed","outcome":"UNKNOWN"]) }
        }
    }
    func completeGuest(_ machine: VZVirtualMachine,error: Bool) {
        guard vm === machine,began,!callbackReceived,machine.state == .stopped else { fault("unexpected_boot_callback");return }
        callbackReceived=true
        emit(error ? "boot_callback_error" : "boot_callback_success",stateFields(machine))
        observeState()
    }
    func startGuest(_ machine: VZVirtualMachine) async throws {
        try c.nativePolicy.requireGuestStart() // GUARD_PREBEGIN_START
        try await machine.start()
    }
    func startInstallation() {
        do { try c.nativePolicy.requireInstaller() } // GUARD_PREBEGIN_INSTALLER
        catch { fault("native_installer_fenced");return }
        guard c.nativeInstallationAllowed == true else { fault("native_installation_disabled");return }
        guard let machine=vm,machine.state == .stopped,!installActive,installer==nil else { fault("invalid_install_state");return }
        let setup=VZMacOSInstaller(virtualMachine:machine,restoringFromImageAt:URL(fileURLWithPath:c.imagePath))
        guard setup.virtualMachine === machine else { fault("installer_vm_mismatch");return }
        installer=setup;startedAt=ProcessInfo.processInfo.systemUptime
        // SDK exposes a synchronous start plus a result callback on this VM's main queue.
        // No control/timer handler can run between the call and setting installActive.
        setup.install { [weak self] result in
            let error: String?
            switch result { case .success: error=nil;case .failure(let failure): error=String(describing:failure) }
            Task { @MainActor [weak self] in self?.installationCompleted(error) }
        }
        installActive=true
        emit("install_begin",stateFields(machine))
        if controlFault || evidenceFault { requestCancellation("fault_during_install_start") }
    }
    func installationCompleted(_ error: String?) {
        guard let machine=vm,began,!callbackReceived else { fault("unexpected_install_callback");return }
        installActive=false;callbackReceived=true;installResult=error==nil
        var fields=stateFields(machine)
        if let error { fields["error"]=String(error.prefix(2048)) }
        emit(error==nil ? "install_callback_success" : "install_callback_error",fields)
        observeState()
    }
    func startControl() throws {
        guard let interval=c.controlInterval else { throw Failure.invalidConfig }
        let flags=fcntl(STDIN_FILENO,F_GETFL)
        guard flags >= 0,fcntl(STDIN_FILENO,F_SETFL,flags|O_NONBLOCK)==0 else { throw Failure.io(errno) }
        let source=DispatchSource.makeReadSource(fileDescriptor:STDIN_FILENO,queue:.main)
        source.setEventHandler { [weak self] in Task { @MainActor [weak self] in self?.readControl() } }
        controlSource=source;source.resume()
        readyAt=ProcessInfo.processInfo.systemUptime
        timer=Timer.scheduledTimer(withTimeInterval:interval,repeats:true) { [weak self] _ in
            Task { @MainActor [weak self] in self?.tick() }
        }
    }
    func preparationCheckpoint(_ paceMs: Int = 10) async throws {
        guard preparing else { return }
        // Bound preparation I/O demand while servicing control and the original deadline.
        try await Task.sleep(for:.milliseconds(paceMs),tolerance:.zero)
        guard let deadline=c.preparationDeadline else { throw Failure.invalidConfig }
        if ProcessInfo.processInfo.systemUptime-preparationStarted >= deadline { fault("preparation_deadline");preparationStop=true }
        if preparationStop || controlFault || evidenceFault { throw Failure.preparationAbandoned }
    }
    func waitForPreparationFinish() async throws {
        guard let interval=c.controlInterval else { throw Failure.invalidConfig }
        while preparationFinish == nil { try await Task.sleep(for:.seconds(interval)) }
        guard let preparationFinish else { throw Failure.stateUnknown }
        emit("preparation_finish_ack",["command":"prepare_finish","commandSeq":preparationFinish,"objectCreated":false,"vmStarted":false])
        finished=true;timer?.invalidate();controlSource?.cancel()
    }
    func serve(_ configuration: VZVirtualMachineConfiguration) async throws {
        try c.validateNative();guard sink.healthy,!preparationStop,!controlFault else { throw Failure.stateUnknown }
        if preparing {
            // FIFO barrier: the controller stops preparation writes before authorizing creation.
            emit("preparation_handoff",["objectCreated":false])
            while !preparationHandoff { try await preparationCheckpoint() }
            try await preparationCheckpoint()
            guard monitorHealthy() else { throw Failure.stateUnknown }
        }
        own(configuration)
        guard let machine=vm,machine.state == .stopped,let interval=c.controlInterval else { throw Failure.stateUnknown }
        if controlSource == nil { try startControl() }
        preparing=false;readyAt=ProcessInfo.processInfo.systemUptime
        emit("ready",stateFields(machine))
        while !finished { try await Task.sleep(for:.seconds(interval)) }
        timer?.invalidate();controlSource?.cancel()
    }

    func execute(_ mode: String) async throws {
        try c.nativePolicy.validate(mode)
        if c.nativePolicy.prebegin || c.nativePolicy.operational {
            try c.validateNative()
            guard let deadline=c.preparationDeadline,deadline.isFinite,deadline>0,deadline<=300 else { throw Failure.invalidConfig }
            preparing=true;preparationStarted=ProcessInfo.processInfo.systemUptime
            try startControl();emit("preparation_started",["deadlineSeconds":deadline,"objectCreated":false])
        }
        // A partial install is never a boot baseline; reject before reading or attaching guest storage.
        if mode == "baseline" { try verifyBaseline() }
        guard VZVirtualMachine.isSupported else { throw Failure.unsupportedImage }
        if mode == "metadata" {
            let (hash,_)=try await verifiedRead(c.imagePath)
            guard hash==c.imageSHA256,(try statPath(c.imagePath)).st_size==c.imageBytes else { throw Failure.fileMismatch("image_identity_mismatch") }
            _=try await loadImage();return
        }
        if mode == "install" || mode == "create-new" {
            // Preserve the old create path's existing-bundle rejection before any writes.
            guard !FileManager.default.fileExists(atPath:c.bundlePath) else { throw Failure.existingBundle }
            throw Failure.invalidConfig // r7 authorizes only the existing bundle; never create a replacement.
        }
        guard mode == "prebegin-existing" || mode == "preflight-existing" || mode == "reinstall-existing" || mode == "baseline" else { throw Failure.invalidConfig }
        do {
        manifest=try await verifyExisting()
        try await preparationCheckpoint()
        let image=try await loadImage()
        try await preparationCheckpoint()
        let config=try await configuration(image:image)
        try await preparationCheckpoint()
        guard let manifest else { throw Failure.invalidConfig }
        // Detect changes during attachment/validation; parent performs the independent after-state full hash.
        try verifyMetadata(manifest)
        if mode == "preflight-existing" { emit("preflight_existing_complete",["vmStarted":false,"labMutationObserved":false]);return }
        try await serve(config)
        } catch {
            if preparing && vm == nil {
                if !preparationStop { fault("preparation_error");preparationStop=true }
                emit("preparation_retained",["objectCreated":false,"outcome":"UNKNOWN"])
                try await waitForPreparationFinish()
            } else { throw error }
        }
    }
    func retainUntilStopped() async {
        guard vm != nil else { return }
        while !finished {
            tick()
            do { try await Task.sleep(for:.seconds(c.controlInterval ?? c.stopDeadline)) }
            catch { emit("retention_wait_error",["error":String(describing:error)]) }
        }
    }
}
@main struct Main {
    @MainActor static func main() async {
        var owner: Owner?
        do {
            guard CommandLine.arguments.count==3 else { throw Failure.invalidConfig }
            let raw=try Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[2]))
            if let object=try JSONSerialization.jsonObject(with:raw) as? [String:Any],let stage=object["prebeginAuxiliaryStage"] {
                try auxiliaryShape(JSONSerialization.data(withJSONObject:stage),result:true)
            }
            let c=try JSONDecoder().decode(Config.self,from:raw)
            try c.nativePolicy.validate(CommandLine.arguments[1])
            try c.validate();let sink=try EventSink(c);let current=Owner(c,sink:sink,configPath:CommandLine.arguments[2],configPin:auxiliaryDigest(raw));owner=current
            try await current.execute(CommandLine.arguments[1]);current.emit("operation_complete");exit(current.evidenceFault || current.controlFault ? 1 : 0)
        } catch {
            let e=error as NSError
            fputs("Owner initialization/operation error: \(String(describing:error))\n",stderr)
            owner?.emit("operation_error",["domain":e.domain,"code":e.code,"description":String(describing:error),"cleanup":"UNKNOWN_UNLESS_EXACT_STOPPED_AFTER_CALLBACK"])
            await owner?.retainUntilStopped();exit(1)
        }
    }
}
