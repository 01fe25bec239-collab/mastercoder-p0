FROM: W01-A2[W01-T007-DB-ROLES-SCHEMA-r0]
TO: Root A1 (acting W01-A1)
MESSAGE_TYPE: RETURN
TASK_ID: W01-T007-DB-ROLES-SCHEMA
ATTEMPT: r0
DISPOSITION: NOT_READY: LOCAL_SOURCE_COMPLETE but the required checks against the pinned PostgreSQL service are PENDING (Docker daemon not running, DATABASE_URL in the lane env is the placeholder, nothing listens on the lane port). All 17 mc-authority-db tests pass only against a private scratch PostgreSQL 17.10, which is NOT the required evidence. Root must provide the DB service (HUMAN COMMANDS below) and the required commands re-run; then A3.
DATE_UTC: 2026-11-05T09:33:00Z
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0
  BRANCH: ws01/w01-t007-db-roles-schema-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

TASK_ID: W01-T007-DB-ROLES-SCHEMA
BASE_SHA: f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
INITIAL_TREE: 7bf7946954917e67931bce10fa686d978a782890
IMPLEMENTATION_SHA: 477e58a7f990f68294cac49548f32d1b79b7c778
TREE: 535a8ce732e7d079a74e39b3676df93e97f52b6d
COMMITS: 477e58a7f990f68294cac49548f32d1b79b7c778 (single commit, sole parent = BASE_SHA)
SIGNATURE: git verify-commit exit 0 (Good ED25519 signature, Omkar Gavale <01fe25bec239@kletech.ac.in>); commit.gpgsign=true; core.hooksPath=.githooks
AMEND NOTE: the commit was amended once BEFORE this RETURN existed (earlier local candidate 9caf4cad… was never reported); 477e58a… is the only candidate. Everything below was re-run at 477e58a….
ACCEPTED_SHA / MERGED_SHA: NONE
CHANGED_FILES (git diff --stat BASE..IMPL, 16 files, 1925 insertions, 1 deletion):
  ws01-core-security/crates/mc-authority-db/src/{error.rs,lib.rs(M),migrate.rs,tenant.rs}
  ws01-core-security/crates/mc-authority-db/tests/{common/mod.rs,grants.rs,migrate.rs,rls.rs,roles.rs,tenant_context.rs}
  ws01-core-security/db/authority/base/{0001_schema.sql,0002_tables.sql,0003_rls_and_grants.sql,0004_tenant_context.sql}
  ws01-core-security/db/authority/roles/{0001_roles.sql,README.md}
WRITE_SET_CHECK: PASS — `git diff --name-only BASE..IMPL` lists exactly the 16 paths above; every path matches ws01-core-security/crates/mc-authority-db/{src,tests}/**, db/authority/base/** or db/authority/roles/**. No Cargo manifest/lockfile, toolchain, contract, other-lane, coordination or architecture file touched. Only this RETURN is written in the COORD worktree (not committed by A2). Worktree clean (`git status --short` empty) at IMPL.
PUSH / PR / MERGE: NONE
REMOTE_CI: NOT_RUN / PR_AND_CI_PENDING (no push/PR authorized)
READY_FOR_A3_REVIEW: NO (reason: pinned-PostgreSQL checks pending; source is complete and frozen at IMPL_SHA)

## What was built

Roles (db/authority/roles/0001_roles.sql): `mc_authority_owner` (offline migration owner) + the 22 runtime roles exactly as listed in the task, all NOLOGIN NOINHERIT NOBYPASSRLS, no superuser/createdb/createrole/replication, no role memberships between them (stray ones are revoked on every run). Idempotent and race-tolerant across databases of one cluster (roles are cluster-global): create-if-missing, heal-only-if-different with retry on "tuple concurrently updated".

Schema (db/authority/base): 0001 schema `mc_authority` owned by the owner, no PUBLIC. 0002 domains `digest`/`u64` + 13 tables (expected_manifest, evaluation, gate_decision, analysis_completion_receipt, evidence_receipt, verification_receipt, verification_set, policy_version, policy_pointer, trust_bundle, trust_pointer, revocation, current_candidate): `tenant_id` in every PK and FK, repository-scoped FKs also bind repository_id + repository_tenant_epoch, doc 18 uniques. 0003 ENABLE+FORCE RLS on every base table, nothing to PUBLIC, doc 07 grants (insert-only; only the three pointer tables are updatable and only on non-identity columns; nobody has DELETE/TRUNCATE). 0004 tenant context (below) and the `tenant_isolation` policies.

Tenant context (GUC is not authentication): the policies do NOT read any GUC. They call `mc_authority.current_tenant()`, backed by `mc_authority.tenant_context`, a per-(backend pid, transaction id) row that no runtime role can read or write (no privilege; FORCE RLS; policy only for the owner). The only writer is SECURITY DEFINER `mc_authority.set_tenant_context(tenant, workload, digest)` (search_path `pg_catalog, mc_authority, pg_temp`, EXECUTE revoked from PUBLIC and granted to the 22 roles only). A second call in the same transaction with a different tenant/workload/digest raises SQLSTATE `MCT01` → `DbError::TenantMismatch`. The row dies with the transaction (and with rollback/savepoint rollback), so a pooled connection can never carry a tenant over. A forged `SET LOCAL`/`set_config` grants nothing.

Rust (crates/mc-authority-db): exact binding API — `migrate(&PgPool, &Path) -> Result<(), DbError>`, `VerifiedTenantAssertion {tenant_id, workload_id, assertion_digest:[u8;32]}`, `TenantContext::from_verified(&VerifiedTenantAssertion) -> Self` (private field; only constructor), `with_tenant(&mut Transaction<'_, Postgres>, &TenantContext) -> Result<(), DbError>`, `DbError {Migration(String), TenantMismatch, Sqlx}` (Sqlx is a unit variant as specified; no SQL text can leak). migrate(): reads MODULES and every module file BEFORE touching the DB, applies modules in listed order and `*.sql` in lexical order, one transaction + advisory lock per file, records SHA-256 in `mc_migration.applied` (re-run skips, edited applied file → `Migration(file)`), errors name file/module only, module names are validated (no path escapes).

## Requirement / acceptance table

| Requirement | Evidence | Status |
|---|---|---|
| Role list exact; NOLOGIN NOINHERIT NOBYPASSRLS; no owner/SET ROLE paths | roles::* (3 tests); 23 roles checked, 462+ SET ROLE pairs + owner | MET (scratch PG only) |
| FORCE RLS on every base table | rls::force_rls_on_every_base_table (every table: enabled+forced+≥1 policy) | MET (scratch PG only) |
| Grants match doc 07 row by row | grants::matrix_matches_doc07: role×table×op via has_table_privilege/has_any_column_privilege against an independent oracle, discovered table set must equal the oracle, PUBLIC has nothing, identity columns never updatable, real INSERT denials 42501 | MET (scratch PG only) |
| Cross-tenant FK rejected / select returns zero | rls::cross_tenant_fk_rejected, rls::cross_tenant_select_returns_zero (+ forged GUC grants nothing) | MET (scratch PG only) |
| Tenant context only from verified assertion; mismatch rejects; no pooled leak | tenant_context::* (4 tests); pool max_connections=1, same backend pid asserted | MET (scratch PG only) |
| Migrations apply cleanly on an empty DB, in module order | migrate::modules_applied_in_order_on_empty_db + 2 extra | MET (scratch PG only) |
| Required checks on the PINNED PostgreSQL service | not runnable: compose ps → "Cannot connect to the Docker daemon"; lane DATABASE_URL is the placeholder; port 55401 had no listener | PENDING |
| Remote CI / A3 / acceptance | not part of this step | NOT_RUN |

## Tests

TESTS_ADDED (all in crates/mc-authority-db/tests, `#[sqlx::test(migrations = false)]`, fresh DB per test, acting as the real roles via `SET LOCAL SESSION AUTHORIZATION`):
 Required (12, all exist and pass on scratch PG):
 - roles::all_22_roles_exist_nologin_noinherit_nobypassrls — NEGATIVE/POSITIVE
 - roles::no_runtime_role_owns_tables — NEGATIVE (tables, functions, schemas, types, db-wide) + owner positive control
 - roles::set_role_between_runtime_roles_fails — NEGATIVE (all 22×21 pairs + to owner, each asserts SQLSTATE 42501; self SET ROLE positive control; zero pg_auth_members)
 - grants::matrix_matches_doc07 — table-driven role × table × op
 - grants::ordinary_api_cannot_insert_gate_decision — NEGATIVE ("permission denied for table gate_decision"), gate_writer positive control with the identical statement
 - rls::force_rls_on_every_base_table
 - rls::runtime_role_cannot_disable_rls — NEGATIVE (22 roles × 14 tables × {DISABLE RLS, NO FORCE RLS, DROP POLICY} = 42501; `SET row_security=off` cannot bypass)
 - rls::cross_tenant_select_returns_zero — incl. no-context ⇒ 0 rows on every table, forged GUC ⇒ 0 rows
 - rls::cross_tenant_fk_rejected — NEGATIVE (23503, and RLS WITH CHECK 42501), same-tenant positive controls
 - tenant_context::mismatched_assertion_rejected
 - tenant_context::pooled_connection_does_not_leak_tenant
 - migrate::modules_applied_in_order_on_empty_db
 Extra (5, cheap and security-relevant): grants::insert_only_tables_reject_update_and_delete; tenant_context::definer_functions_have_fixed_search_path_and_narrow_execute (also catches EXECUTE for PUBLIC); tenant_context::runtime_role_cannot_write_tenant_context_directly; migrate::modules_order_follows_modules_file_not_alphabet; migrate::failures_name_the_file_only_and_edited_files_are_rejected.

TESTS_RUN / TEST_RESULTS (all at IMPL 477e58a…, every cargo/git-commit through run_single_build.py, CARGO_BUILD_JOBS=2/CARGO_INCREMENTAL=0/sccache from the lane env, own target dir):

| Command (cwd = ws01-core-security unless noted) | Exit | Result | Log (ignored, under target/a2-evidence/) |
|---|---:|---|---|
| docker compose --env-file .env.local -f compose.yaml ps --status running (cwd repo root form from dispatch) | 1 | Docker daemon not running — PostgreSQL NOT listed | final-compose-ps.log |
| cargo fmt --all --check | 0 | clean | final-fmt.log |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | 0 | clean | final-clippy.log |
| cargo clippy -p mc-authority-db --all-targets --locked -- -D warnings | 0 | clean | final-clippy-crate.log |
| cargo test --workspace --locked --no-fail-fast, LANE ENV exactly as prescribed | 101 | 60 passed / 17 failed / 0 ignored: ALL 17 mc-authority-db tests fail at `failed to parse DATABASE_URL: RelativeUrlWithoutBase` (placeholder). Every other workspace test passes | final-workspace-test-lane-env.log |
| SAME command with DATABASE_URL pointing to a PRIVATE SCRATCH PostgreSQL 17.10 (see below) | 0 | 77 passed / 0 failed / 0 ignored (60 pre-existing + 17 new) | final-workspace-test-scratch-db.log |
| python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (cwd repo root) | 0 | prints `"p0_executed": false`; 40 semantic specification cases (not runtime evidence) | final-check-spec.log |
| gitleaks detect --no-banner --log-opts=BASE..IMPL | 0 | 1 commit scanned, no leaks | final-gitleaks-range.log |
| gitleaks detect --no-banner (repo root, full history of all refs, as prescribed) | 1 | 744 findings (742 on the first run; the shared repo is live), ALL rule `generic-api-key`, NONE in IMPL or in any path of this task (0 in 477e58a…, 0 under db/authority or mc-authority-db): 738 in ws01-core-security/coordination/task.md and 6 in ws03-runner-sandbox-acquisition/coordination/RETURNS, all in commits on other refs/lanes. Re-run with --redact/JSON; matched values NOT inspected (digest-like false positives likely, UNVERIFIED). Pre-existing, outside this task; A1 should triage. | final-gitleaks-full.log, final-gitleaks-full-redacted.log, gitleaks-full-redacted.json |

SCRATCH POSTGRES DISCLOSURE: Docker was down and the lane DATABASE_URL is unset, so — without touching any shared service, the user's own running Homebrew postgres, a compose file, a volume or any credential — I started a private throwaway cluster from the installed Homebrew PostgreSQL 17.10 binaries (`initdb`/`pg_ctl`, trust auth, `listen_addresses=''`, Unix socket in a mktemp dir under /tmp, fsync off) to run the SQL and tests during development, and stopped it afterwards (`pg_ctl stop`, no process left). Its data directory /tmp/mcpg.DY9srs (trust auth, no secrets, no listener) could NOT be deleted because the delete command was denied by the harness; master may remove it. It is NOT the pinned image `postgres@sha256:57c72f…` (its major version is UNVERIFIED because Docker was down), it is not the lane service, and none of its results counts as the required DB check. It proves the SQL/tests are internally consistent on PG 17.10 only.

Stress runs on the scratch cluster (concurrent first-time role creation across 17 parallel test databases ×2; concurrent healing of deliberately corrupted roles/memberships ×3): all green after fixing a real race I found (see "Problem handling").

## Mutation spot-check (SKILLS §6.4) — each run `cargo test -p mc-authority-db --no-fail-fast` at IMPL with ONE guard mutated in place, file restored byte-for-byte (sha256 asserted; worktree clean afterwards)

| Mutation | Killed by (failed tests) |
|---|---|
| GRANT INSERT on gate_decision to ordinary_api | matrix_matches_doc07, ordinary_api_cannot_insert_gate_decision |
| remove FORCE ROW LEVEL SECURITY | force_rls_on_every_base_table, runtime_role_cannot_disable_rls |
| evaluation→manifest FK without tenant_id (+ UNIQUE(manifest_digest)) | cross_tenant_fk_rejected |
| policy USING (true) | cross_tenant_select_returns_zero, pooled_connection_does_not_leak_tenant |
| set_tenant_context: mismatch returns instead of raising | mismatched_assertion_rejected |
| tenant context not keyed to the transaction (pooled leak) | cross_tenant_select_returns_zero, pooled_connection_does_not_leak_tenant |
| policies trust a caller-set GUC | cross_tenant_fk_rejected, cross_tenant_select_returns_zero, ordinary_api_cannot_insert_gate_decision, pooled_connection_does_not_leak_tenant |
| GRANT gate_writer TO ordinary_api (SET ROLE path) | set_role_between_runtime_roles_fails |
| roles created/healed with BYPASSRLS (cluster pre-dropped) | all_22_roles_exist_nologin_noinherit_nobypassrls (+4 more) |
| leave set_tenant_context EXECUTE with PUBLIC | definer_functions_have_fixed_search_path_and_narrow_execute |
| Rust: MCT01 not mapped to TenantMismatch | mismatched_assertion_rejected |
| GRANT DELETE on revocation | insert_only_tables_reject_update_and_delete, matrix_matches_doc07 |
12/12 mutations killed (exit 101, no compile errors, logs mutation-*.log + mutation-results-*.json). Two mutation-harness/test defects found and fixed along the way (see Problem handling).

## Grant matrix evidence
`\dp mc_authority.*` and `\df+ mc_authority.*` captured from a scratch DB migrated with the committed SQL: ws01-core-security/target/a2-evidence/dp-grant-matrix.txt and df-functions.txt (scratch PG 17.10, NOT pinned service — re-capture on the pinned service). Access-privilege column (owner row `mc_authority_owner=arwdDxtm` omitted; `a`=INSERT `r`=SELECT, column-level `w` on the three pointers):
 - evaluation, gate_decision: gate_writer=ar, readonly=r
 - analysis_completion_receipt, evidence_receipt: evidence_ingest_writer=a, gate_writer=r, readonly=r
 - verification_receipt: verification_ingest_writer=a, gate_writer=r, readonly=r
 - verification_set: verification_set_writer=a, gate_writer=r, readonly=r
 - expected_manifest: manifest_writer=a, gate_writer=r, readonly=r
 - revocation: revocation_writer=a, gate_writer=r, readonly=r
 - policy_version: policy_writer=a, gate_writer=r, readonly=r; trust_bundle: trust_writer=a, gate_writer=r, readonly=r
 - policy_pointer: policy_writer=ar + UPDATE(policy_id, generation, updated_at); trust_pointer: trust_writer=ar + UPDATE(trust_bundle_id, generation, updated_at); current_candidate: candidate_writer=ar + UPDATE(provider_generation, observation_digest, writer_id, observed_at, expires_at); gate_writer=r, readonly=r on each
 - tenant_context: owner only
 No grants to PUBLIC; none to ordinary_api or the 11 other roles on these tables; nobody has DELETE/TRUNCATE.

## Problem handling (AGENTS.md §0.2 — classes and fixes, all within scope; no check weakened)
1. Class ENVIRONMENT (open): Docker daemon down, DATABASE_URL placeholder, no listener on the lane PG port → pinned-service checks PENDING. Not worked around beyond the disclosed private scratch cluster.
2. Class CODE: migrate() wrote its ledger row while a file's `SET LOCAL ROLE mc_authority_owner` was still in effect (permission denied; surfaced only as opaque `Sqlx`) → added `RESET ROLE` before the ledger insert.
3. Class TEST (mine): `DROP POLICY IF EXISTS tenant_isolation` was a no-op on tenant_context (different policy name) → now drops each table's real policy by catalog name. Mutation run exposed a second test defect: the PUBLIC-EXECUTE check joined pg_roles and dropped grantee 0 → fixed with LEFT JOIN; the `execute-to-public` mutation is now killed.
4. Class CODE (real hardening gap found by mutation): 0001_roles.sql re-asserted attributes but not inter-role memberships, and a concurrent heal of several roles by parallel databases failed with "tuple concurrently updated" (reproduced, server log retained) → membership revoke added; ALTER ROLE/REVOKE retry on internal_error; re-stress green.
5. Mutation-harness defects (not product): the first FK mutation silently did not apply (two edits to one file overwrote each other) and the BYPASSRLS mutation was a no-op on a cluster where roles already existed; both fixed (chained edits, roles dropped first) and re-run at IMPL.

## Assumptions (none weakens an invariant; each is flagged for A3/A1)
- A-1 evaluation_sequence is unique per exact subject: UNIQUE(tenant, repository, epoch, subject_digest, evaluation_sequence). Doc 18 only says "evaluation sequence".
- A-2 current_candidate has one pointer row per (tenant, repository, epoch, subject_digest); only the observation columns are updatable. CurrentCandidateV1 has no PR identity; a base/head change yields a new subject_digest (doc 07 "supersede identity").
- A-3 Only the columns the constraints need are kept from the registry records; `required_units`, `members`, `completion_receipt_digests`, `reason_codes` etc. are left to the owning modules. u64 → `numeric(20,0)` domain, i64 → bigint, digests → bytea(32) domain; enums copied from REGISTRY.json (GateProofV2.conclusion, outcome/grade/independence_class sets).
- A-4 gate_writer gets SELECT on all gate inputs (it is the consumer that verifies them); pointer writers get SELECT on their own pointer table (a CAS UPDATE needs it); all other writers are write-only. ordinary_api has no table privilege: doc 07's "approved views" are not defined in this task.
- A-5 Design choice vs the task wording "sets a transaction-local tenant": a transaction-local ROW in a definer-only table instead of a transaction-local GUC, because policies reading a GUC would let any runtime role `SET LOCAL` its way into any tenant (the very failure doc 07/ADR-013 amendment names). Public API and signatures are unchanged.
- A-6 The migration ledger lives in `mc_migration.applied` (created by migrate(), outside mc_authority, no grants), because roles/base files run before mc_authority exists and the base tables must all be FORCE-RLS.
- A-7 Migrating and test connections must be a superuser (or able to `SET ROLE mc_authority_owner`); the tests also use SET SESSION AUTHORIZATION. Documented in roles/README.md. Requires PostgreSQL with xid8/pg_current_xact_id_if_assigned (≥13); only 17.10 verified.

## Known limitations / residual risks (PROTOTYPE_ONLY unless noted)
- RESIDUAL, by specification: the database cannot verify a VerifiedTenantAssertion cryptographically. Any role holding EXECUTE on set_tenant_context (all 22) can bind any tenant it names if it connects as that role and calls the function directly. The closed holes: no direct GUC/SET forging, no direct table write, no leak across transactions, mismatch rejection, no cross-role SET ROLE. The remaining boundary is "only an authenticating caller holds a login for the role" plus per-workload logins (README). A future CCR could bind assertions with a MAC key held by the authority; not done (no invented mechanism).
- Not in this base (owning modules): gate currentness events/CAS current pointer for evaluations, immutable observation table behind current_candidate, intake/quarantine tables, approved views, ordinary_api request tables. doc 07's "only gate_writer appends a terminal currentness event" is therefore not testable here.
- tenant_context is UNLOGGED and needs a writable (non-standby) session; a cleanly exited session leaves one small stale row that is never matched (lookups require the current xid) and is cleaned by the next set on that pid.
- migrate() applies one transaction per file (a failed run leaves earlier files applied and recorded; a re-run resumes).
- tenant_context's PK has no tenant_id (internal per-transaction authentication state, not tenant-owned data).
- No monotonic-generation trigger on the pointer tables (CAS monotonicity belongs to the authority transitions of later tasks; roles that can UPDATE are single trusted writers).
- Role names are cluster-global; 0001_roles.sql will "heal" a pre-existing role of the same name.
- Sub-agent swarm (AGENTS.md §3c): NOT used; this small coherent change set was done directly by the single A2 session (Claude Sonnet 5.5). Disclosed deviation; no helper output exists to verify.

BLOCKERS: PINNED POSTGRESQL SERVICE UNAVAILABLE (owner: Root/A1 + master) — see HUMAN COMMANDS. No contract/CCR issue.
CCR_DRAFTED: NONE
SECRETS_CHECK: `gitleaks detect --no-banner --log-opts=BASE..IMPL` → exit 0, no leaks (1 commit). Full-history prescribed scan exits 1 with 744 pre-existing `generic-api-key` findings outside this task (see TEST_RESULTS row; none in IMPL or this task's paths), so the prescribed command is NOT green repo-wide and needs A1 triage. Hardcoding grep (`/Users/|/home/|C:\\|https?://|@x.(com|io)|:5540x|AKIA|BEGIN .*PRIVATE KEY|claude-|gpt-`) over the added lines → no hits. No secrets, ports, URLs or credentials in source; tests use synthetic UUIDs/digests and `env!("CARGO_MANIFEST_DIR")`-relative paths; DATABASE_URL (existing variable) is read by sqlx::test; NO new variable → `.env.example` unchanged. PROTOTYPE header on all 16 files (checked).

ARCHITECTURE_CONTRACTS_USED:
  - doc 07 "Authority and DB-write matrix", "Grants and tenant isolation"; doc 18 "Required relational constraints"; ADR-013 (+V2.1 amendment), ADR-019; INV-06, INV-15; doc 29 not needed.
SCHEMAS_USED:
  - ScopeV1, EvaluationIdentityV2, GateProofV2 (conclusion enum), AnalysisCompletionReceiptV1, EvidenceReceiptV2, VerificationReceiptV2, VerificationSetV2, PolicyVersionV1, TrustBundleV1, RevocationRecordV1, CurrentCandidateV1, ExpectedAnalysisManifestV1 (field names/enums from SCHEMAS/REGISTRY.json; no vectors needed)
DOUBLES_USED: NONE

MEMORY_UPDATES:
  - Interface pins — role framework v1: 22 runtime roles + `mc_authority_owner` (all NOLOGIN NOINHERIT NOBYPASSRLS, no inter-role membership); schema `mc_authority`; other lanes add tables as owner with FORCE RLS + `tenant_isolation` policy on `mc_authority.current_tenant()`; Rust API `mc_authority_db::{migrate, VerifiedTenantAssertion, TenantContext::from_verified, with_tenant, DbError}`; rules in db/authority/roles/README.md (INTERFACE_NOTICE "role framework v1" to W02–W06 is A1's, after acceptance).
  - Decisions — tenant context is a definer-only per-transaction row (not a GUC) so a runtime role cannot SET its way into a tenant; mismatch = SQLSTATE MCT01 → TenantMismatch.
  - Known pitfalls — roles are cluster-global and `#[sqlx::test]` databases share a cluster: role migration must be idempotent and retry "tuple concurrently updated"; advisory locks are per database so they do not serialize role creation; `SET LOCAL ROLE` in a migration file leaks into the runner's later statements unless reset; the DB-test connection must be a superuser; has_table_privilege-based checks must LEFT JOIN pg_roles or PUBLIC (grantee 0) is missed.
  - Environment — pinned-PostgreSQL checks for W01-T007 were not runnable (Docker daemon down, lane DATABASE_URL = placeholder, nothing on the lane PG port); only a private scratch PG 17.10 was used. The lane `.env` DATABASE_URL must be set before any `cargo test --workspace` (mc-authority-db tests read it via sqlx::test).
  - Failed approaches — policies reading a transaction-local GUC (rejected: forgeable by any runtime role); advisory-lock serialization of role creation (per-database, ineffective across test databases).

## HUMAN COMMANDS (for Root/master — A2 never runs compose up/down)
1. Start Docker Desktop, then from the COORDINATION worktree (as the dispatch states): `cd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security && docker compose --env-file .env.local -f compose.yaml up -d` (compose.yaml needs POSTGRES_PASSWORD and MC_WS01_PG_PORT from the lane env).
2. Set DATABASE_URL in the lane env (`/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/LANE_01_CORE_SECURITY_AUTHORITY/.env`, currently the placeholder `__SET_ME__`) to the compose PostgreSQL on the lane port using the image's superuser (needed: CREATEDB, CREATEROLE, SET SESSION AUTHORIZATION). Do not paste the value into chat or files.
3. Then (A2/A3 or Root): `cd <task worktree>/ws01-core-security && (set -a && . ./.env.local && set +a && python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd "$PWD" -- cargo test -p mc-authority-db --locked)` expecting 17 passed, then the full prescribed block; re-capture `\dp mc_authority.*` on that service.

## Pre-RETURN checklist (SKILLS §15)
[x] Every acceptance criterion has evidence (see table; DB-service evidence PENDING, scratch evidence only)
[x] Every NEGATIVE TEST exists, asserts the specific reason (SQLSTATE 42501/23503/MCT01 + message), passes on scratch PG
[x] Mutation spot-check done on the key guards (12/12 killed)
[x] Only write-set paths changed
[x] No invented schema/enum/digest/authority/state beyond flagged assumptions A-1..A-7; names from REGISTRY.json and the task
[x] No secrets or real data; gitleaks BASE..IMPL clean; full-history result recorded above
[x] PROTOTYPE header on every new file
[x] No hardcoding; no new .env variables
[ ] All REQUIRED COMMANDS re-run at the final SHA with exit codes recorded — recorded, but the compose check failed and the DB-dependent tests could not pass on the pinned service
[x] Assumptions and limitations written honestly; UNVERIFIED items labeled (pinned image PG major version)
[x] Durable facts listed under MEMORY_UPDATES; memory.md untouched
[x] Worktree clean; IMPLEMENTATION_SHA and TREE copied via command
[x] Final response is exactly one text code block
[x] Next agent's folder exists (COORD worktree, verified)
[x] No unresolved guesses
