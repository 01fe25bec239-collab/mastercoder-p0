# ws03-runner-sandbox-acquisition
PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).
Owner and scope: coordination/SCOPE.md. Write set: coordination/WRITE_SET.md. Agent rules: coordination/AGENTS.md + coordination/SKILLS.md.

This virtual workspace contains nine crates, two TEST_ONLY doubles and nine empty
claim harnesses. Modules contain documentation only. See docs/MODULES.md for
ownership, imports and exact dependency pins. The repository root Rust toolchain
is the sole pin; this lane has no rust-toolchain.toml.

Cargo commands run from this directory: cargo metadata --locked --format-version 1,
cargo fmt --all --check, cargo build --workspace --locked,
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings,
and cargo test --workspace --locked. Local agents must queue every executable
validation through the program's shared build wrapper and use this worktree's target/.
The parser boundary is checked with cargo tree -p mc-pack-parser -e normal --locked.

compose.yaml defines only PostgreSQL. Copy the documented nonsecret defaults from
.env.example into the lane's local configuration and provide a disposable password
privately. The existing .env.local symlink supplies configuration; never log it.
Missing Compose values fail interpolation. A1 starts the shared service only after
review and integration; A2 does not start or stop Docker. No SQL or migrations are
implemented here: W03 currently has no owned database tables (W03-T004).

T000 CI uses serialized standard hosted Ubuntu 24.04, native macOS 15 and native
Windows Server 2025 jobs, per the owner-approved T000 host arrangement. The required
ws03-ci job always reports; native macOS/Windows checks retain their separate names
and both remain mandatory for task completion. Dependency acquisition and compilation
precede restricted test execution. The workflow materializes reviewed CI helpers from
its own source; these helpers implement no product backend or claim runtime.

Linux tests enter an owned PostgreSQL network namespace with only loopback and a
synthetic fixture interface, nftables default-drop for both IP families and no process
capabilities or escalation. macOS uses native Seatbelt with network denied except
numeric loopback, including denial of DNS broker lookup. Windows uses a fresh native
AppContainer with zero network capabilities and a profile-specific loopback exemption;
owner-approved dynamic WFP filters also deny nonloopback traffic for that exact SID.
No global firewall setting is changed. Tests and descendants use the same boundary.

Controlled TCP/UDP listeners bind dynamic ports. The actual system resolver queries
a random synthetic DNS suffix, directed to an owned DNS service. Same endpoints must
work before/after restriction and fail inside, while PostgreSQL SELECT 1 and loopback
controls succeed. A removed guard must be detected by the denial checker. Unavailable
IPv6, DNS, native tools, test execution or owned cleanup fails the job. No probe targets
the public internet. See docs/MODULES.md for provenance, cleanup and reproduction.

Local ordinary-account macOS compilation is not native containment validation.
Local/static review may yield a signed preliminary candidate; actual native controls,
Linux/PostgreSQL execution and PR CI remain NOT_RUN until separately authorized runs.
This is NOT_READY pending those gates, with no acceptance, downstream unlock, S2
qualification or P0 execution. A1 rechecks included Actions quota and the effective
USD 0 budget before any remote execution; this scaffold grants no push/PR/merge.

### Native CI repair candidate (runtime pending)

Linux fixture Python starts after setpriv drops identity, supplementary groups and
all capability sets with no-new-privileges. Port 53 remains usable through the
owned network namespace's unprivileged-port setting. Unix privileged cleanup uses
validated root-owned system executables, a minimal environment and trusted cwd;
no privileged Python interpreter/import chain is used.

Windows keeps control scripts, Python/stdlib, toolchain, WFP DLL, DNS observations
and PostgreSQL data at medium integrity. The package receives read/execute only
for required runtime/config paths; Modify and low integrity apply only to work,
home, temp, cargo, rustup and output. Probe logs in output are untrusted artifacts;
DNS observations and runtime control decisions remain in protected storage.
Native probes attempt runtime/control/PID-file writes and PID deletion and require
access denial. Foreground PostgreSQL belongs to an atomic, drained Job Object;
its shutdown does not depend on a PID file. WFP is loaded once, final scope close
and readback precede checked FreeLibrary, and deletion follows successful cleanup.

Set MC_WS03_CI_WINDOWS_PGBIN (or image PGBIN) to the installed PostgreSQL bin
directory, or expose installed postgres.exe on PATH. All three initdb.exe,
pg_ctl.exe and postgres.exe must exist there; actual versions/hashes are recorded.
No installer or guessed image path is supplied; absence fails with an explicit
prerequisite error. Native SDK/kernel/ACL/inheritance/DNS/PostgreSQL and CI results
remain NOT_RUN locally. F-3 macOS detached-session descendant ownership remains
OPEN for the separate Apple VM route; original process-group cleanup does not
close it. This scoped candidate is not T000 acceptance or VM readiness.
