# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

See MODULES.md for ownership, dependency edges, pin provenance and scaffold checks.
Runtime interfaces are introduced by W03-T003 after its upstream prerequisites integrate.

The CI helper source lives inside the authorized workflow, between WS03_PYTHON and
WS03_WFP markers. Each native job extracts the same reviewed bytes into RUNNER_TEMP.
Local syntax/pure controls may run through the shared wrapper; privileged/runtime
probes require the admitted hosted environment and remain pending until actual CI.
MODULES.md records the native mechanism, configuration, fixture and cleanup contract.

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
