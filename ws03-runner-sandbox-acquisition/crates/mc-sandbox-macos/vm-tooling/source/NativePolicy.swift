// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
// Pure native boundary policy, also compiled against harmless boundary stubs.
struct NativePolicy {
    let mode: String?
    let prebeginAllowed: Bool?
    let installationAllowed: Bool?
    var prebegin: Bool { mode == "prebegin-existing" && prebeginAllowed == true && installationAllowed == false }
    var operational: Bool { ["reinstall-existing","baseline"].contains(mode ?? "") && prebeginAllowed != true && installationAllowed == true }
    func validate(_ command: String) throws {
        if prebegin { guard command == "prebegin-existing" else { throw NativeFence.permission } }
        else { guard operational, command == mode else { throw NativeFence.permission } } // GUARD_OPERATIONAL_PROFILE
    }
    func requireGuestStart() throws {
        guard operational, mode == "baseline" else { throw NativeFence.guestStart }
    }
    func requireInstaller() throws {
        guard operational, mode == "reinstall-existing" else { throw NativeFence.installer }
    }
}
enum NativeFence: Error { case permission, guestStart, installer }
