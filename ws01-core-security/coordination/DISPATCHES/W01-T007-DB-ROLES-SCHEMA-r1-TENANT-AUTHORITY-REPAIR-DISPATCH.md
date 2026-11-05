# A1 → fresh A2 repair — W01-T007-DB-ROLES-SCHEMA r1

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.
FROM: Root/A1. TO: Fresh human-hosted A2. Human chooses model/settings.
DATE:2026-11-06 Asia/Calcutta
TASK_ID:W01-T007-DB-ROLES-SCHEMA
ATTEMPT:r1
FOLDER:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1/ws01-core-security
WORKTREE:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1
BRANCH:ws01/w01-t007-db-roles-schema-r1
REPAIR_BASE_SHA / INITIAL_HEAD:477e58a7f990f68294cac49548f32d1b79b7c778
INITIAL_TREE:535a8ce732e7d079a74e39b3676df93e97f52b6d
ORIGINAL_BASE_SHA:f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
FROZEN_ARCHITECTURE_SHA:58b6ab38b0a514d94d8851679ad66ef9bf696c9e
STATUS:Created and verified clean on frozen r0; old author/review checkouts remain immutable.
REVIEW:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T007-DB-ROLES-SCHEMA-r0-a-A3_REVIEW.md
REVIEW SHA256:472cda0a4378483ddfe25e1b5ed98ae6bf74eb836674ea7d1796c3f8f8fce3d1
ORIGINAL DISPATCH:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T007-DB-ROLES-SCHEMA-r0-DISPATCH.md
RETURN OUTPUT:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T007-DB-ROLES-SCHEMA-r1-A2_RETURN.md

## Objective and read order

Close HIGH F1: ordinary/read-only database sessions must not establish tenant authority from caller-invented tenant/workload/digest fields. Keep the existing public Rust API and all writer/RLS protections. Correct N1's role-healing wording. No unrelated redesign or speculative framework.

Read repository instructions and live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md13.2/15, exact task/dispatch, r0 RETURN, complete A3 review, and Root's prospective pinned validation addendum. Read architecture06,07,18, ADR013/019 and INV15. Source credential separation is mandatory; a Rust type, digest shape, custom GUC or role label is not authentication. Root checked the retained independent control:0rows before;1victim tenant row after invented assertion. Do not dismiss it because77existing tests pass.

## Findings verbatim

F-1 | BLOCKING | HIGH | db/authority/base/0004_tenant_context.sql:36–61,92; crates/mc-authority-db/tests/tenant_context.rs:128–154

Any runtime role can authenticate itself as any tenant. The definer setter validates only non-null values, a nonzero tenant UUID and digest length. It never authenticates the workload, checks the digest against verified state, or binds tenant to the connecting identity. EXECUTE is granted to all22 roles, including readonly and ordinary_api. Moving the caller-selected tenant from a GUC into a protected row changes storage, not the authentication boundary. The Rust type is explicitly constructible by callers under the required API and cannot protect direct SQL calls.

Independent pinned16.14 reproduction in a fresh unique database: seed one tenantB manifest as manifest_writer, then BEGIN; SET LOCAL SESSION AUTHORIZATION readonly; SELECT count(*) FROM mc_authority.expected_manifest returns0. Call set_tenant_context with tenantB, invented workload99 and arbitrary32 bytes of ff, then identical SELECT returns1. No valid scoped assertion is provided. The query runs as readonly, not as a bypassing superuser; session authorization is used only to reproduce that actual runtime identity. Full synthetic script/output: target/a3-evidence/control.py and control.log. Same setter enables dedicated writer roles to insert under an arbitrary victim tenant; ordinary_api still lacks table grants, so this finding does not claim it can directly insert gate decisions.

Requirement violated: task objective 'tenant context set only from a verified assertion', doc07 Grants and tenant isolation and INV-15 authenticated tenant grants. Author's residual-risk statement is candid, but ordinary runtime-role compromise is not the architecture's privileged-superuser residual risk. The setter's ACL test currently requires the vulnerable grant list and thus cannot establish this property.

Required repair: establish an enforceable verified scoped identity boundary before accepting tenant context; untrusted ordinary/readonly runtime sessions must not be able to manufacture the input that establishes authority. Retain exact public API unless Root routes a contract amendment. Add direct-SQL forged-assertion denials with genuine authorized controls (including tenantB read denial), and update the narrow-EXECUTE oracle to the actual trust boundary. Root must choose the architecture-consistent route; this review does not invent a new signing/MAC protocol or authorize a scope change.

N-1 | NONBLOCKING | LOW | db/authority/roles/README.md:21–24; src/migrate.rs:94–95
Role SQL heals attributes/memberships when executed, but migrate skips already-recorded unchanged files. 'On every run' is misleading if read as every migrate invocation. Clarify that rerunning the migration runner is not a role-drift repair operation. Privileged drift itself is not claimed to be a new runtime exploit.

## Root-selected bounded implementation route

Use the existing PostgreSQL authenticated-login and offline-owner trust boundary already described by the role README and architecture; enforce it in the database instead of trusting caller fields. Preserve22non-owner NOLOGIN/NOINHERIT/NOBYPASSRLS runtime roles and the existing offline owner. Workload LOGIN identities are separate from those privilege roles.

A small protected owner-provisioned authorization relation may bind the actual authenticated session identity to the explicitly verified tenant, workload and assertion digest. The setter must require that protected scoped authorization before establishing context. Runtime sessions must have no path to insert/update/delete/authorize themselves in this state or acquire the offline owner's privileges. Unknown principal, absent authorization or disagreement fails closed. Use the database-controlled authenticated/session identity, not a caller-supplied identity/GUC or current_user of a SECURITY DEFINER function (that is the owner inside the function). PostgreSQL16 references: https://www.postgresql.org/docs/16/functions-info.html and https://www.postgresql.org/docs/16/sql-set-session-authorization.html . These document why SET ROLE/SECURITY DEFINER effective identity is distinct from session identity and why non-superusers cannot select another authenticated session user.

This is an internal SQL grant/session binding within T007's database-role foundation, not a new wire schema, public API, signing/MAC protocol or authentication service. Its trusted provisioning must be explicit and unavailable to runtime roles. Do not introduce hardcoded approved digests, name-derived tenant identities, shared superuser pools, universal grants or a fallback accepting arbitrary caller assertions. No new dependency is necessary: sqlx/uuid and PostgreSQL primitives already exist. Show where a trusted verifier/operator supplies authorization and where runtime callers are denied; do not claim the full external mTLS/enrollment/signed-receipt pipeline is implemented by fixture provisioning.

Preserve VerifiedTenantAssertion's exact public fields, TenantContext::from_verified, with_tenant transaction signature, migrate and existing DbError variants. Unauthorized SQL can map through existing redacted error behavior; do not widen the public API or turn rejection into success. Keep transaction mismatch, savepoint rollback and pool reuse semantics. Keep insert-only producer grants, composite keys/FKs and FORCE RLS unchanged except an internal authorization/context policy needed for this repair.

## Authorized write set

Stay within the existing protected T007 source scope, narrowed by this repair objective:
- ws01-core-security/db/authority/base/**: tenant authority/context SQL and the smallest internal authorization state necessary; no unrelated gate tables/transitions.
- ws01-core-security/db/authority/roles/**: only necessary trust-boundary grant corrections and README clarification; do not add a runtime authority role or broaden memberships.
- ws01-core-security/crates/mc-authority-db/src/**: only corresponding tenant/error documentation or binding changes, exact public API preserved.
- ws01-core-security/crates/mc-authority-db/tests/**: necessary affected fixtures/oracles plus focused attack/authorized controls.

Do not edit manifests, lockfiles, architecture, public contracts, other crates/lanes, coordination memory/task or retained evidence. If this route needs such a change, return the exact conflict to Root before editing; no improvised replacement protocol. Work only on fresh test databases; changed migration files must not be retroactively applied to or reseal retained r0 databases/proofs.

## Required checks and genuine controls

1. Preserve all17existing method IDs and their original security properties; adapt only test setup/oracles that embodied the vulnerable authorization. Keep source-side APIs and the60inherited methods unchanged. Record exact old-test body changes and reasons, not an assertion that all old bytes stayed unchanged.
2. Add focused direct-SQL attack tests: raw readonly and ordinary identities with invented tenant/workload/digest cannot establish context; a principal authorized for tenantA cannot read tenantB with a made-up assertion; wrong workload/digest, absent authorized scope and copying another principal's valid tuple all deny. An unauthorized writer cannot insert a victim-tenant manifest. Failed authorization must leave no usable context.
3. Genuine positive controls must exercise authorized tenantA and separately authorized tenantB, through the public with_tenant API and actual dedicated writer/read roles; a blanket always-deny implementation is not a fix. Missing context remains deny-by-default; mismatch, savepoint and pool reuse still hold.
4. Provision trusted fixture state explicitly under a privileged bootstrap connection BEFORE entering untrusted execution. Do not write a helper that automatically grants whatever assertion the attack under test supplies. At least one end-to-end control uses a real non-superuser LOGIN connection on the pinned service (fresh synthetic credentials kept private), verifies actual session/current identities and absence of superuser/BYPASSRLS, then proves cross-tenant denial and authorized access. SET SESSION AUTHORIZATION on a bootstrap connection may supplement role tests, but must not be the sole proof of credential/principal binding.
5. Runtime roles must be unable to forge or modify the protected authorization state, impersonate another login, or gain offline-owner permissions. Focused guard-removal/restored control should remove the actual authorization predicate in SCRATCH/fresh fixture only and make the new forged-assertion negative fail for the intended reason. Restore before final source/checks; preserve all failures.
6. Run targeted authority tests after the repair, then the normal required final validation: cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings; cargo test --workspace --locked (all17preserved authority methods+new focused cases+60inherited); check_spec.py with p0_executed:false; required grant evidence on the pinned service; redacted exact-change/output scans. Do not repeat broad suites after successful unchanged checks without a new failure/change justifying it.
7. Correct README N1: the role SQL heals when actually executed; migrate skips unchanged applied files and is not an automatic role-drift healer. Correct any nearby claim that committed context rows vanish at transaction end; describe authority eligibility separately from storage lifetime. Do not add a cleanup daemon or unrelated retention mechanism.

## Resources, service and stop rules

Every Cargo/build/test command acquires /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py with cwd/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1/ws01-core-security. Existing jobs2/incremental0/sccache, own target, one build globally and20GiB free-disk stop apply. Current free space at preparation38.147GiB. No new RAM/storage cap, paid resources or deletion. New test fixtures remain small and synthetic; do not create another compiler/observer system.

Root's pinned service mc-ws01-postgres-1 is PostgreSQL16.14 at existing private lane URL, localhost55401, image postgres@sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777. .env.local links to the valid private test configuration; never print it or change shared credentials. A2 never starts/stops Docker/compose, changes existing databases or substitutes Homebrew PostgreSQL. Uniquely named owned test databases/logins may be created for the required tests; teardown is only for fixtures created by those tests, never retained r0/evidence/user databases. Service failure is a Root callback, not a silent skip.

Stop on a real contract/write-scope conflict, missing trusted input, resource refusal or repeated failure under live rules. Keep all old candidates/reviews/failures and return concrete remaining work. This repair does not authorize network providers, VM/native-owner operation, privilege escalation outside isolated test bootstrap, cache cleanup or remote actions.

## Completion and return

F1complete means the independent attack is denied AND genuine scoped positive controls succeed, unauthorized runtime sessions cannot manufacture grants, prior security checks remain and required local results are retained. Freeze a normally signed candidate/clean worktree, record SHA/tree, exact source/test delta, migration/deployment boundary, command exits and qualifications. Record publicAPI unchanged,17method IDs preserved, all changed test oracles and trusted fixture provisioning. Write the separate RETURN above with MEMORY_UPDATES, then stop. Root processes it and prepares fresh independent exact-candidate A3. No self-approval, role appointment, push/PR/merge, interface publication or consumer unlock. Final chat is exactly one text code box with a proposed handoff to Root/A1.
