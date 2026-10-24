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

### Native CI PostgreSQL authority (source only)

The CI helper uses installed initdb/postgres single-user mode before opening any
listener. It creates ws03_ci as LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE
NOINHERIT NOREPLICATION NOBYPASSRLS with no password or role memberships, owns
only its disposable ws03_ci database, and disables bootstrap postgres LOGIN.
Database ownership permits schema/table migrations; cluster administration and
server program/file privileges are withheld. The local development Compose
identity/defaults are separate and unchanged.

An exact six-rule HBA permits only ws03_ci/ws03_ci at 127.0.0.1/32, rejects all
other IPv4/IPv6 users/databases and replication, and rejects local connections.
The server binds IPv4 loopback only and disables Unix sockets. Bootstrap has no
network listener or reusable admin credential; Windows keeps PG/control storage
under the protected runner ACL. Linux bypasses the image's trust bootstrap and
uses the same offline SQL/HBA contract as macOS and Windows. The protocol probe
and restricted Rust DATABASE_URL use the same limited identity.

Each parent probe checks actual role attributes/membership and listener settings,
SELECT, and an actual schema/table migration in a rolled-back transaction. It
requires explicit HBA rejection for alternate/admin/database/replication startup
and SQLSTATE 42501 with privilege-specific reasons for SET ROLE, self-escalation,
dangerous membership, COPY PROGRAM, server file-write and file-read requests. A
missing tool, dead listener, invalid SQL or unrelated file error fails. Runner-side
HBA readback checks the exact file before/after tests. These native controls are
authored; no PostgreSQL server/SQL or native helper body ran on this Mac. F3 and
all native/VM/CI gates remain pending.

Primary interfaces: [single-user postgres](https://www.postgresql.org/docs/current/app-postgres.html),
[HBA first-match and reject](https://www.postgresql.org/docs/current/auth-pg-hba-conf.html),
[role attributes](https://www.postgresql.org/docs/current/sql-createrole.html),
[server COPY privileges](https://www.postgresql.org/docs/current/sql-copy.html),
[predefined roles](https://www.postgresql.org/docs/current/predefined-roles.html).
