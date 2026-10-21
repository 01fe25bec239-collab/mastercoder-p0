# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# Workspace modules and dependencies

T000 creates exactly 20 members: nine crates, two TEST_ONLY doubles and nine empty
harnesses. The following owns future runtime behavior; T000 owns this scaffold.

| Crate | Owning W03 tasks | Declared modules |
| --- | --- | --- |
| mc-checkout-broker | T003, T016, T025, T026, T044 | api, counters, dialer, effects, lfs, submodule |
| mc-egress-proxy | T008 | dns, proxy, routes |
| mc-pack-parser | T003, T009 | api, materialize, parse, process, verify |
| mc-runner | T007, T014 | cancel, client, enroll, lease_loop, negotiate, probe |
| mc-runner-test-support | T004 | testdb, vm |
| mc-sandbox-core | T003, T010, T033 | api, capability_match, limits, offline_install, unit |
| mc-sandbox-linux | T015 | backend, limits, overlay, reset, tree_kill, vm |
| mc-sandbox-macos | T019 | backend |
| mc-sandbox-windows | T020 | backend |

## Crate → imports

Derived from all W03 BUILD tasks’ Exact public API and read-only dependency fields.
Rows describe declared package dependencies, including libraries needed by the
listed future modules; an empty source file imports no runtime items yet.

| Package | Normal imports | Target-specific imports | Development imports |
| --- | --- | --- | --- |
| mc-checkout-broker | mc-contracts, mc-wire, mc-effects, tokio, reqwest, async-trait, thiserror, serde, serde_json, uuid, sha2, hex, tracing | NONE | proptest, tempfile, effects-journal-double |
| mc-egress-proxy | mc-sandbox-core, tokio, axum, hyper, rustls, tokio-rustls, thiserror, tracing | NONE | proptest, tempfile |
| mc-pack-parser | mc-contracts, sha1, sha2, hex, flate2, serde, serde_json, thiserror, clap | cfg(target_os = "linux"): nix | proptest, tempfile |
| mc-runner | mc-sandbox-core, mc-contracts, mc-wire, mc-negotiation, tokio, reqwest, rustls, tokio-rustls, rcgen, ed25519-dalek, serde, serde_json, uuid, sha2, hex, clap, thiserror, tracing, async-trait | NONE | proptest, tempfile, runner-control-double, fake-sandbox |
| mc-runner-test-support | sqlx, tokio, uuid, thiserror, tempfile | NONE | proptest, tempfile |
| mc-sandbox-core | mc-contracts, async-trait, tokio, serde, thiserror, sha2, hex, tar, flate2, tracing | NONE | proptest, tempfile |
| mc-sandbox-linux | mc-sandbox-core, mc-egress-proxy | cfg(target_os = "linux"): tokio, async-trait, thiserror, tracing, nix | proptest, tempfile, mc-runner-test-support |
| mc-sandbox-macos | mc-sandbox-core, mc-egress-proxy | cfg(target_os = "macos"): tokio, async-trait, thiserror, tracing | proptest, tempfile |
| mc-sandbox-windows | mc-sandbox-core, mc-egress-proxy | cfg(target_os = "windows"): tokio, async-trait, thiserror, tracing | proptest, tempfile |
| fake-runner | mc-runner, fake-sandbox | NONE | NONE |
| fake-sandbox | mc-sandbox-core, tokio | NONE | NONE |

The broker → mc-effects / effects-journal-double edges support T016’s
EffectAuthority and per-call effect protocol. Runner → mc-negotiation and
runner-control-double support T007/T014. Test support → sqlx/tokio supports T004.
Platform crates use target-specific native dependency tables. T015’s test-support
edge is development-only. The core never imports a concrete backend, avoiding a
core/backend cycle for T033. Backend → egress proxy edges support T015/T019/T020.
The runner's fake-sandbox development edge supports T014.

mc-lease and lease-double are available workspace aliases as requested by T000.
T003 names W01 lease/effects API inputs but its listed signatures do not require
a lease type; no guessed lease implementation is imported by the parser or guest.
mc-effects::api already exists as an empty upstream module at this base.
T033’s future StagedArtifact/dependency-authority source is unspecified here and
must be resolved by that owning task; no peer crate is fabricated.

T044 adds W04 mc-acquisition-dialer and W02 checkout-grant-double to the broker
after their workspaces integrate. Fixtures, W01 role SQL, corpus, lab inventory,
backend decisions and W06 local services are read-only data/service inputs,
not fabricated crate imports. Native Virtualization.framework/Hyper-V integration
choices belong to T019/T020; no speculative binding library is added.

The fake-sandbox and fake-runner v1 packages reserve T006 and T022.
fake-runner exposes library identity fake_runner and an empty binary target using
the same authorized src/lib.rs. Cargo may report the shared source path; both
targets must compile with Clippy warnings denied.

## Harness ownership and imports

| Claim | Owning BUILD tasks | Runtime imports reserved by scaffold |
| --- | --- | --- |
| P0-02A | T029 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-sandbox-core, mc-sandbox-linux, mc-sandbox-macos, mc-sandbox-windows, mc-egress-proxy |
| P0-02B | T030 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-sandbox-core, mc-sandbox-linux, mc-sandbox-macos, mc-sandbox-windows, mc-egress-proxy |
| P0-02C | T031 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-sandbox-core, mc-sandbox-linux, mc-sandbox-macos, mc-sandbox-windows, mc-egress-proxy |
| P0-02D | T021 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-sandbox-core, mc-sandbox-linux, mc-sandbox-macos, mc-sandbox-windows, mc-egress-proxy, mc-pack-parser |
| P0-05A | T023 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-checkout-broker, mc-pack-parser |
| P0-05B | T024 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-checkout-broker, mc-pack-parser |
| P0-05C | T034 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-checkout-broker, mc-pack-parser |
| P0-05D | T035 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-checkout-broker, mc-pack-parser |
| P0-05H | T043 | tokio, clap, serde_json, mc-contracts, mc-runner-test-support, mc-sandbox-core, mc-sandbox-linux, mc-sandbox-macos, mc-sandbox-windows, mc-egress-proxy |

P0-02A/B/C also declare cells::{linux,macos,windows}; these are empty.
All claims and platform cells remain NOT_RUN; empty harness exits are not evidence.

## Exact pins and provenance

W01-overlapping versions and features are copied unchanged from this task’s base
ws01-core-security/Cargo.toml. reqwest =0.13.5 uses default-features=false and
features=[rustls,json], per the owner-approved dispatch clarification.
The root Rust 1.97.1 pin remains sole authority. W03 does not upgrade upstream pins.

| Registry crate | Exact version | Provenance |
| --- | --- | --- |
| sha1 | =0.11.0 | W01 base manifest (copied unchanged) |
| tokio | =1.53.2 | W01 base manifest (copied unchanged) |
| axum | =0.8.9 | W01 base manifest (copied unchanged) |
| sqlx | =0.9.0 | W01 base manifest (copied unchanged) |
| serde | =1.0.229 | W01 base manifest (copied unchanged) |
| serde_json | =1.0.154 | W01 base manifest (copied unchanged) |
| uuid | =1.28.0 | W01 base manifest (copied unchanged) |
| sha2 | =0.11.0 | W01 base manifest (copied unchanged) |
| hex | =0.4.3 | W01 base manifest (copied unchanged) |
| ed25519-dalek | =3.0.0 | W01 base manifest (copied unchanged) |
| thiserror | =2.0.21 | W01 base manifest (copied unchanged) |
| tracing | =0.1.44 | W01 base manifest (copied unchanged) |
| rustls | =0.23.45 | W01 base manifest (copied unchanged) |
| tokio-rustls | =0.26.6 | W01 base manifest (copied unchanged) |
| rcgen | =0.14.10 | W01 base manifest (copied unchanged) |
| reqwest | =0.13.5 | W01 base manifest (copied unchanged) |
| clap | =4.6.7 | W01 base manifest (copied unchanged) |
| tempfile | =3.27.0 | W01 base manifest (copied unchanged) |
| proptest | =1.12.0 | W01 base manifest (copied unchanged) |
| async-trait | =0.1.92 | W01 base manifest (copied unchanged) |
| hyper | =1.12.0 | [crates.io hyper API](https://crates.io/api/v1/crates/hyper) |
| flate2 | =1.1.10 | [crates.io flate2 API](https://crates.io/api/v1/crates/flate2) |
| nix | =0.31.3 | [crates.io nix API](https://crates.io/api/v1/crates/nix) |
| tar | =0.4.46 | [crates.io tar API](https://crates.io/api/v1/crates/tar) |

W03-exclusive lookup timestamp: 2026-10-13T17:23:43.517104+00:00. All selected releases
were the API's max_stable_version and not yanked. Cargo.lock records checksums
for every registry package. Exclusive source archive checksums:

| Crate | SHA-256 | API response SHA-256 |
| --- | --- | --- |
| hyper | 2c3e324da4c95177d6291d4c8730197c0d1822f8a9766814a4a44fa5ab797c9c | 8b2a7ff9eb24c1646529340880b8b31311d5408f9c3ad6684df82ef5b25b0831 |
| flate2 | 6e634e2e0ebac1ee034020da1ca582e17ffe4e0f5e985823721e168928136dcb | 3cc0f18d0f2ee4311b28d076924a63b1bb8797ef2cc7a984d8bede8b535445d6 |
| nix | cf20d2fde8ff38632c426f1165ed7436270b44f199fc55284c38276f9db47c3d | 3a8912babc11a247d72d08a51b2df21ccf4023659cd61a2066e2d6cda1f6491e |
| tar | 3f6221d9a6003c78398e3b239969f352578258df48c8eb051caadae0015bc840 | bea69b21cf4ce29571f96f4d8eefef36c12c7acd11323ba84bcfa22f63858dbe |

PostgreSQL immutable index verified at 2026-10-13T17:24:17.990289+00:00 from the
[upstream registry manifest](https://registry-1.docker.io/v2/library/postgres/manifests/sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777). The response body SHA-256 equals
57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777; Docker-Content-Digest matched it. Linux amd64 and arm64
platform manifests exist. The configured image is copied to .env.example;
compose.yaml requires an explicit configuration value and only defines PostgreSQL.

## Dependency ownership and checks

Only T000 edits shared scaffold manifests/lockfile; later changes use the standing
W03-DEPENDENCY-UPDATE or W03-LOCKFILE-SYNC tasks, or T044’s explicit scope.
No runtime logic, SQL, migrations, services beyond PostgreSQL or schema types are
implemented by T000. The parser’s full normal closure must contain none of
reqwest, hyper, rustls or tokio-rustls (INV-05); it has no broker/core edge.

Required checks: exact20member_and_path_identity, declared_empty_modules,
shared_exact_versions, parser_normal_closure_no_network (current host and all
targets), required_ci_triggers_and_change_gate, action_full_sha_pins,
complete_T003_scratch_signature_compile, write_scope, no_secrets,
signed_candidate, and specification p0_executed:false; plus locked metadata,
fmt, build, Clippy with -D warnings and test.

The scratch T003 check copies the workspace and read-only W01 packages outside
the tracked tree, inserts every listed signature, and compiles them. Since W01
contract generation is still a separate task, opaque temporary GitOidV1,
CheckoutGrantV2 and registry-derived SandboxTier stubs are clearly marked.
This tests dependency/type plumbing, not upstream implementation fidelity.
Original source hashes must remain unchanged; no API stub is committed.

## CI and local evidence limits

The required ws03-ci job always reports on PRs/pushes to main. The change gate
covers this lane/workflow, every W01 crates/doubles path and W01 Cargo.toml,
plus root toolchain, deny policy and architecture. Its no-change path skips heavy
steps while reporting success. Standard hosted ubuntu-24.04 → macos-15 →
windows-2025 jobs serialize actual builds; repository-wide workflow concurrency
queues runs without cancelling evidence. Windows still runs after a failed Mac
job when Linux recorded a relevant change. Platform jobs have no enable flag.
The original self-hosted task text is retained in the dispatch; the owner-approved
hosted arrangement supersedes it for T000 CI only. No VM-grade claim is inferred.

### CI-only native interfaces

| OS | Boundary | Allowed setup and cleanup |
| --- | --- | --- |
| Linux | Docker none network namespace; synthetic dummy fixture interface; inet nftables output default-drop except IPv4/IPv6 numeric loopback; nsenter + setpriv clears groups/capabilities and sets no-new-privs | Owned container/anonymous volume destroyed; private mount namespace binds a fixture-only resolv.conf without changing the host resolver |
| Native macOS | sandbox-exec Seatbelt network deny, numeric localhost exception and DNS broker mach-lookup deny | Process/session scoped profile; one random /etc/resolver suffix file created exclusively and removed only if its hash still matches |
| Native Windows | STARTUPINFOEX zero-capability AppContainer + atomic Job Object membership; exact package SID/low-integrity/token checks; SID-specific loopback exemption; dynamic WFP nonloopback-deny filters bound to that SID | Fresh profile collisions fail; only copied disposable paths get SID ACL/low integrity; owned NRPT suffix rule, exemption, profile and dynamic WFP session are removed and checked |

The owner explicitly approved temporary per-job WFP filters in the r1 A2 session
on 2026-10-14. They compensate for the same-host reachability enabled by the
AppContainer loopback exemption. No global firewall setting, capability grant,
account, larger runner, paid resource or remote action is authorized here.
All native mechanisms remain runtime-unverified until their actual hosted jobs.
Seatbelt's CLI is deprecated; missing/unsupported behavior fails without fallback.

Helpers are embedded in the workflow's WS03_PYTHON/WS03_WFP source sections and
materialized unchanged into job-local temporary files. WFP C++ compiles against
the installed native Windows SDK/MSVC, without a new product dependency. API
constants/structs are native SDK definitions, not invented architecture types.
Each job records source SHA, image OS/version, native identity, tool versions and
binary/helper/policy hashes. Native tool/IPv6/resolver/ACL failures are failures.

Compilation, dependency fetch and native PostgreSQL tooling setup occur before
restriction. Every actual cargo test/rustdoc child executes under the boundary;
Windows Job Objects, Linux PID namespaces and POSIX sessions bound cleanup. Linux
namespace exit terminates descendants even after a new session is created. Mac
cleanup currently covers the owned process group; detached-session behavior is
not proven by T000's ordinary child fixture and must be an explicit native review
focus. This CI network boundary is not a hostile-process/S2 containment claim.
Environment is
rebuilt from an allowlist, with copied cache/toolchain/source paths and no checkout
metadata, local env files, tokens, provider credentials or proxy variables.

### Controlled fixtures and meaningful tests

Listeners are outside the restricted child. Linux uses synthetic addresses inside
its owned namespace; native Mac/Windows discover existing nonloopback addresses
without changing network adapters. TCP/UDP ports bind zero; DNS uses a dynamically
recorded Mac resolver port or the protocol's DNS port on Linux/Windows. Every
required IPv4/IPv6 listener must actually bind and reply with the generated canary.
Loopback TCP/UDP controls and a real PostgreSQL protocol SELECT 1 run in boundary.
PostgreSQL trust authentication is limited to the disposable loopback-only job
cluster. No reusable database credential enters test children.

The actual system getaddrinfo resolver/broker queries a random .test name; the DNS
service records each query. Raw UDP DNS is additional coverage. Unique phase/child
names and zero TTL prevent a cached positive result from masquerading as traffic.
Baseline and post-denial controls must reach the same services. Restricted parent
and descendant must fail both direct nonloopback families/protocols and system DNS,
with no matching broker query at the fixture. Connection-refused, unsupported
interfaces or failed baseline never count as blocked traffic.

The removed-guard mutation uses the same checker and endpoints: Linux removes its
owned nft table; Mac omits Seatbelt; Windows omits its AppContainer attribute and
closes the owned WFP session. The unrestricted result must fail the denial checker.
The guard is then restored, all denials rerun, tests executed, and denials rerun
after tests. Mutations have no public/customer endpoint or user-configurable bypass.
No actual native test result can be inferred from pure/static mutation controls.

Cleanup kills/drains owned descendants, stops fixtures/PostgreSQL, removes only
verified owned policy/resolver/profile state and removes the fresh copied root.
Unexpected ownership/collision/cleanup failures fail the job. Evidence includes
parent/child probe JSON, DNS observations, policies, PostgreSQL query result,
compilation/test logs, provenance, cleanup results and a SHA-256 manifest. Upload
runs even after failure, with bounded retention; logs contain no reusable secrets.

### Reproduction and pending gates

Config reads documented MC_WS03_CI_* defaults from .env.example once and validates
environment overrides. Fixture address/port/run identity are runtime outputs.
For static review, extract the workflow marker sections, AST-parse Python, parse
PowerShell with its native parser, run Bash syntax checks and the Python self-test.
The external r1 evidence retains executable local controls and exact commands;
agents must use the shared build wrapper. The workflow's run operation refuses
execution outside an admitted standard GitHub-hosted environment. Do not run
guarded/native/hostile probes on the operator's personal Mac.

After all applicable local checks, the signed exact-SHA candidate may receive
preliminary independent A3 review. Final disposition remains NOT_READY:
NATIVE_RUNTIME_AND_PR_CI_PENDING. A1 rechecks included quota and USD 0 budget,
obtains remaining outward authorization, and retains actual CI URLs/results.
All three native controls, Linux builds/PostgreSQL and PR CI remain mandatory;
empty scaffold tests establish compilation only. No acceptance/downstream unlock,
S2 qualification, production grant or claim execution follows from local review.

Primary interface references checked 2026-10-14 (mechanisms, not runtime proof):

- [GitHub hosted runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
- [AppContainer launch](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer)
- [Process attributes and job list](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)
- [Windows loopback isolation](https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/troubleshooting-uwp-firewall)
- [WFP dynamic sessions](https://learn.microsoft.com/en-us/windows/win32/api/fwpmtypes/ns-fwpmtypes-fwpm_session0)
- [WFP filter API](https://learn.microsoft.com/en-us/windows/win32/api/fwpmu/nf-fwpmu-fwpmfilteradd0)
- [Windows native runner inventory](https://github.com/actions/runner-images/blob/main/images/windows/Windows2025-Readme.md)
- Apple-owned installed sandbox-exec(1), /usr/share/sandbox profiles and libsandbox interface; deprecated and native execution pending.

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
