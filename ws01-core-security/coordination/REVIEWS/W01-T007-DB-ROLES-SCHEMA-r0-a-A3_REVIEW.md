# Independent A3 source review — W01-T007-DB-ROLES-SCHEMA r0-a
PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

FROM: W01-A3[W01-T007-DB-ROLES-SCHEMA-r0-a]
TO: Root/A1
MESSAGE_TYPE: REVIEW_RESULT
DISPOSITION / VERDICT: REPAIR_REQUIRED — SOURCE_ONLY
DATE_UTC: 2026-11-05T19:10:52.709179+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
REVIEW_WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0-review-a
BRANCH: detached HEAD
BASE_SHA: f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
REVIEWED_SHA: 477e58a7f990f68294cac49548f32d1b79b7c778
TREE: 535a8ce732e7d079a74e39b3676df93e97f52b6d
HEAD_VERIFIED_AT_START / END: 477e58a7f990f68294cac49548f32d1b79b7c778
TREE_VERIFIED: YES; clean tracked/index state at intake and closure.
INDEPENDENCE: YES. I did not author, repair or dispatch the candidate. Helpers were fresh read-only reviewers, not additional A2/A3 appointments.
FROZEN_ARCHITECTURE: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1; import58b6ab38b0a514d94d8851679ad66ef9bf696c9e; REGISTRY_SHA256 b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787.

## Findings

F-1 | BLOCKING | HIGH | db/authority/base/0004_tenant_context.sql:36–61,92; crates/mc-authority-db/tests/tenant_context.rs:128–154

Any runtime role can authenticate itself as any tenant. The definer setter validates only non-null values, a nonzero tenant UUID and digest length. It never authenticates the workload, checks the digest against verified state, or binds tenant to the connecting identity. EXECUTE is granted to all22 roles, including readonly and ordinary_api. Moving the caller-selected tenant from a GUC into a protected row changes storage, not the authentication boundary. The Rust type is explicitly constructible by callers under the required API and cannot protect direct SQL calls.

Independent pinned16.14 reproduction in a fresh unique database: seed one tenantB manifest as manifest_writer, then BEGIN; SET LOCAL SESSION AUTHORIZATION readonly; SELECT count(*) FROM mc_authority.expected_manifest returns0. Call set_tenant_context with tenantB, invented workload99 and arbitrary32 bytes of ff, then identical SELECT returns1. No valid scoped assertion is provided. The query runs as readonly, not as a bypassing superuser; session authorization is used only to reproduce that actual runtime identity. Full synthetic script/output: target/a3-evidence/control.py and control.log. Same setter enables dedicated writer roles to insert under an arbitrary victim tenant; ordinary_api still lacks table grants, so this finding does not claim it can directly insert gate decisions.

Requirement violated: task objective 'tenant context set only from a verified assertion', doc07 Grants and tenant isolation and INV-15 authenticated tenant grants. Author's residual-risk statement is candid, but ordinary runtime-role compromise is not the architecture's privileged-superuser residual risk. The setter's ACL test currently requires the vulnerable grant list and thus cannot establish this property.

Required repair: establish an enforceable verified scoped identity boundary before accepting tenant context; untrusted ordinary/readonly runtime sessions must not be able to manufacture the input that establishes authority. Retain exact public API unless Root routes a contract amendment. Add direct-SQL forged-assertion denials with genuine authorized controls (including tenantB read denial), and update the narrow-EXECUTE oracle to the actual trust boundary. Root must choose the architecture-consistent route; this review does not invent a new signing/MAC protocol or authorize a scope change.

N-1 | NONBLOCKING | LOW | db/authority/roles/README.md:21–24; src/migrate.rs:94–95
Role SQL heals attributes/memberships when executed, but migrate skips already-recorded unchanged files. 'On every run' is misleading if read as every migrate invocation. Clarify that rerunning the migration runner is not a role-drift repair operation. Privileged drift itself is not claimed to be a new runtime exploit.

## Acceptance criteria

| Criterion | Status | Independent evidence |
|---|---|---|
| Role list exact | MET for fresh deployment | roles::all_22_roles_exist_nologin_noinherit_nobypassrls; all22 roles plus offline owner |
| FORCE RLS on every base table | MET | force_rls_on_every_base_table, runtime_role_cannot_disable_rls;13 tenant tables plus internal context table |
| Grants match doc07 row by row | MET for table producer matrix; NOT_MET for authenticated context authority | grants::matrix_matches_doc07 and raw grants.txt agree with SQL; F-1 defeats the scope required by doc07 |
| Migrations apply cleanly on an empty DB | MET | all3 migrate tests; fresh pinned-service SQL application control |
| Tenant context only from verified assertion (objective/security invariant) | NOT_MET | F-1, before=0 / forged_read=1 |
| Exact Rust public APIs | MET | lib/tenant/migrate/error signatures match dispatch |

## Coverage and independent assessment

All16 changed files reviewed, not just hunks. Source files: src/error.rs (redacted mapping), lib.rs (exports), migrate.rs (planning/ledger/transactions), tenant.rs (assertion forwarding); tests/common/mod.rs (actual-role session authorization and fixtures), grants.rs (table/op/column oracle and real denials), migrate.rs (order, replay, checksum/errors), rls.rs (FORCE, FK and GUC negatives), roles.rs (attributes/ownership/SET ROLE), tenant_context.rs (mismatch, reuse and function ACLs); SQL base/0001_schema.sql,0002_tables.sql,0003_rls_and_grants.sql,0004_tenant_context.sql; roles/0001_roles.sql and README.md. Connected MODULES and callers were read. Comparison used doc07, doc18, ADR013/019, INV06/15 and task/dispatch. No product edits.

Roles are nonowners, NOLOGIN/NOINHERIT/NOBYPASSRLS and lack authority-role memberships on the fresh cluster. The real role tests exercise permission denials rather than silently relying on superuser RLS behavior. All defined tenant PK/FKs contain tenant_id; repository FKs include repository identity/epoch. Table grants preserve insert-only records and limited pointer-update columns. PUBLIC has no protected relation/function privileges. Definers belong to offline owner, qualify relation names and fix search_path with pg_temp last; no caller-controlled dynamic SQL. These properties do not repair F-1.

Migration planning validates module names/duplicates and reads files before DB changes; lexical SQL order follows MODULES. Operator-controlled SQL/path trust is explicit. Files commit separately, with checksum ledger and per-database transaction advisory lock; partial prior-file success is retained and replayable. RESET ROLE restores ledger-writer identity. Concurrent cluster-global role creation uses duplicate handling and bounded heal retries; no independent live cluster-role corruption/stress was performed. Revoke only addresses direct memberships among named roles: deployment must separately ensure no external/transitive authority membership; no shared-cluster mutations were made to test hostile preexisting grants.

Context binds backend PID + full transaction ID, rejects disagreements, and rolls back with savepoints. Existing tests pass for same-backend pool reuse after commit/rollback. Independent savepoint control reports savepoint_cleared=true. A committed context row remains (retained_rows=1); it is not eligible in another xid. This corrects overbroad historical prose without claiming stale rows leak authority. Rows from exited backends remain until PID reuse/cleanup; bounded lifetime/storage was not proven, and long connection churn can accumulate them. No invented cleanup job is recommended as part of this review.

Author assumptions independently assessed:
- A1: per-subject evaluation-sequence uniqueness is narrower than doc18's unspecified wording; not independently established as a defect without a defined sequence scope.
- A2: subject-keyed current_candidate cannot itself select a replacement subject for one stable PR; currentness/CAS consumers are absent. Do not treat this as proven complete currentness authority or unlock consumers. A later authority design must establish supersession. No fabricated schema amendment in this review.
- A3: representative relational subset is disclosed; checks use registry enum literals/domains, not complete signed-record validation. Consumer signature checks remain mandatory in owning tasks.
- A4: SELECT for gate consumers and pointer CAS readers is reasonable; no undefined approved views invented for ordinary_api.
- A5: row-based transaction context defeats GUC forgery and reuse, but does not supply authentication (F-1).
- A6: separate migration ledger is operator-owned infrastructure, not a tenant table. No runtime grants were added to it.
- A7: tests need privileged bootstrap/session authorization; application checks then act as runtime identities. PostgreSQL16.14 was independently observed, not inferred from scratch17 results.

## Validation and retained evidence

All artifacts below are ignored under this review checkout's ws01-core-security/target/a3-evidence/. Existing lane environment was loaded privately; no values printed. All Cargo commands used:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0-review-a/ws01-core-security -- COMMAND

| Command | Exit/result | Artifact |
|---|---|---|
| docker compose --env-file .env.local -f compose.yaml ps --status running |0; pinned postgres image running, loopback service | observed read-only; control.log independently records16.14 |
| cargo fmt --all --check |0 | fmt.log |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings |0 | clippy.log |
| cargo test --workspace --locked | 0;77 passed /0 failed /0 ignored (17 authority +60 inherited) | tests.log |
| python3 ../ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py |0; p0_executed:false | spec.log |
| python3 target/a3-evidence/control.py |0; unauthorized read reproduced; fresh unique database | control.log; grants.txt contains raw\dp |
| python3 target/a3-evidence/mutation.py |0; guard removed sees1, restored sees0; savepoint clearing true | mutation.log |
| gitleaks detect --no-banner --redact --log-opts=f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7..477e58a7f990f68294cac49548f32d1b79b7c778 |0; no leaks | gitleaks-change.log |
| added-line hardcoding rg |1 = no matches | hardcoding.log |

Required12 named tests plus5 extra authority tests are present and independently passed. Denials assert SQLSTATE42501/23503 and Rust TenantMismatch; ordinary_api INSERT, cross-role SET ROLE and runtime ALTER TABLE have actual DB controls. Workspace includes inherited tests. Passing77 tests does not cover forged assertions. Independent guard-removal SQL spot-check altered only the isolated scratch DB inside a rollback transaction, never candidate source; positive unbound read count changed1→0 on restoration, demonstrating the read oracle reacts to RLS removal. Author's12 historical mutations were reviewed as author evidence, not represented as independently repeated.

Historical A2 Docker/placeholder failures, scratch17.10 results, scan failures and NOT_READY return remain untouched. Root's later pinned77PASS addendum is separate evidence. No environment or reviewer-harness error was relabeled as a product defect. One attempted optional skill-file read used a nonexistent path (exit1); relevant skill files were subsequently read at their actual paths. No required materials were missing. Full-history gitleaks was deliberately not repeated per review request; historical744 findings remain historical nonzero results with Root classification, not a green full-history claim.

Resource samples:41GiB available before build,40GiB after; wrapper enforces20GiB stop, one build slot, existing jobs2/incremental0/sccache/own target. Peak RAM not measured. No daemon changes, credential changes, external services, push, PR, merge or production/P0 claim execution. Fresh control DB a3_t007_5eaa13e33cea41d5a8d9547da1609bfc is retained; no existing data deleted.

## Required repairs / Root handoff
F-1 is the blocking source repair. Route to an authorized implementer and fresh exact-SHA independent review; do not accept this candidate on test success alone. N-1 is a narrow documentation correction. No task acceptance, consumer unlock or interface notice is authorized by this report; remote CI is still NOT_RUN.

MEMORY_UPDATES:
- Pinned16.14 independently reproduced caller-forged tenant authority through the definer setter despite all17 DB tests passing.
- Context rows survive commit; PID/xid prevents reuse but exited-backend retention is not bounded here.
- Migration runner replay skips role healing; drift needs an explicit offline operation.
- Historical A2 failures and Root's prospective environment resolution must remain distinct.

Checklist: exact SHA/tree and16-path write set verified; all required named negative tests present/passing; independent SQL mutation control retained; prototype headers/no new config/no hardcoding checked; change scanner clean, full-history not repeated; assumptions/limitations recorded; report only coordination write; product unchanged. SOURCE_ONLY REPAIR_REQUIRED, not acceptance.
