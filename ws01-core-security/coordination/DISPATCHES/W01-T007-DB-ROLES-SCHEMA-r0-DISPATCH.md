# A1 → A2 DISPATCH — W01-T007-DB-ROLES-SCHEMA r0

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).

FROM: Root/A1 (owner requested actual A2 implementation handoffs, not replacement A1s).
TO: Fresh human-hosted A2. Human chooses model/settings; this dispatch does not select them or appoint A3.
DATE: 2026-11-03 Asia/Calcutta
TASK_ID: W01-T007-DB-ROLES-SCHEMA
ATTEMPT: r0
FOLDER: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0/ws01-core-security
WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0
BRANCH: ws01/w01-t007-db-roles-schema-r0
BASE_SHA / INITIAL_HEAD: f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
INITIAL_TREE: 7bf7946954917e67931bce10fa686d978a782890
FROZEN_ARCHITECTURE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
COORDINATION: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T007-DB-ROLES-SCHEMA-r0-A2_RETURN.md
STATUS: Worktree created, initial HEAD/tree/branch and clean index verified. No A2 launched and no build/test executed by Root.

## Objective

Implement the PostgreSQL roles, grants, FORCE RLS and authenticated tenant context; preserve all exact APIs and isolation checks.
Keep the smallest complete implementation of this existing task. Reuse existing code and pinned dependencies. No speculative frameworks, unrelated optimization or extra task expansion. Do every required positive/negative check; do not delete or weaken tests/security to make a result pass.

## Authority and verified prerequisites

Read repository AGENTS.md, SKILL.md, SUBAGENT_POLICY.md and live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md sections13.2/15, memory.md and the exact task. Read architecture references needed by the specification; do not edit architecture. The owner hosts roles and chooses their settings, superseding generic model instructions in older snapshots.

G0 phased/scoped PRESTART is OWNER_APPROVED in /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/AUTHORIZATION/G0_PHASED_BUILD_AUTHORIZATION.json; G1 record is /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/AUTHORIZATION/G1_PROTOTYPE_BUILD_AUTHORIZATION.md. Existing budgets remain binding, with no claim that unused allowance was freshly measured. No new cloud/provider spending, real model API use or production/claim execution.

Root fetched main and verified W01-T000 merge9a7922bf29c0a6689903b96896dab8851d1eeeeb and W03-T000 merge d6e5c37666e08c31795796359f27bd69ad954568 as ancestors of BASE_SHA. W01-T004 merge3b3507fe52025c23f1b335dd93f68dd41d09e99b is also integrated. Old coordination handoffs' missing-W03 holds are historical, not current prerequisites. Read sibling product crates from this current task checkout, not the old COORD checkout.

## Authorized writes and preservation

The verbatim task below is the production write set. Main's protected scope entry was verified:
{
  "allow": [
    "ws01-core-security/crates/mc-authority-db/src/**",
    "ws01-core-security/crates/mc-authority-db/tests/**",
    "ws01-core-security/db/authority/base/**",
    "ws01-core-security/db/authority/roles/**"
  ],
  "deny": []
}

Do not edit other lanes, root toolchain, frozen contracts, coordination task/memory, or the protected scope to bypass a check. Your absolute RETURN is the sole coordination output exception. Keep local logs/scratch under an ignored, task-owned location; do not commit fixtures, generated build output, credentials or private logs. Existing .env.local is a symlink to the lane configuration: do not copy its values into source, evidence or chat or change its target.

For W01 non-scaffold work, Cargo manifests/lockfiles are read-only. A real LOCKFILE_SYNC or missing dependency outside scope is an exact Root callback, not permission to patch another task. For scaffold tasks, use their explicit manifest/lockfile scope and existing W01 exact dependency pins; never silently upgrade shared crates.

## Resources, execution and service ownership

Three active A2s globally, one per lane; these are the three prepared lanes. One executing build/test command through:
/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py

Wrap every Cargo/build/test invocation using the existing command form:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0/ws01-core-security -- <actual command and arguments>

Source the existing lane environment privately for commands that need it; never print or trace it. Keep CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0 and existing sccache configuration. Every worktree uses its own target; no shared CARGO_TARGET_DIR. Existing shared cache allowance5GiB and free-disk stop20GiB remain. Task-local limits and normal error-stop rules remain; no RAM/storage increase is granted. W03-specific VM/compiler budgets are not blanket limits for these Rust tasks.

Check PostgreSQL before DB-dependent validation. Shared dev services belong to Root/A1; A2 must not run compose up/down, remove volumes, install daemons or invent credentials. W02/W04 currently create their compose definitions as part of the scaffold. If a required service/configuration is unavailable, complete independent in-scope work and return the exact setup needed to Root, marking DB checks pending; never report mocked/skipped checks as passed. No DB service was freshly established by this handoff.

## Verbatim task specification

### W01-T007-DB-ROLES-SCHEMA
- **TASK_ID:** W01-T007-DB-ROLES-SCHEMA
- **OBJECTIVE:** **In plain words:** Create the PostgreSQL security foundation: one database role per kind of writer, row-level security so one tenant can never see another's data, and the core gate tables. Every other database task builds on these roles. **Precisely:** Implement the PostgreSQL authority framework: the offline migration owner, all non-owner NOINHERIT NOBYPASSRLS runtime roles, schema `mc_authority` with the representative gate-critical tables (FORCE RLS, composite tenant keys and FKs, per-role grants from doc 07), the module-ordered migration runner, and `mc-authority-db` (tenant context set only from a verified assertion).
- **WHY IT EXISTS:** 07/ADR-013/ADR-019 make tenant isolation and writer separation structural; every other DB unit adds its own module tables on top.
- **ARCHITECTURE REQUIREMENTS:** 07 "Authority and DB-write matrix", "Grants and tenant isolation"; 18 relational constraints; ADR-013/019; INV-06, INV-15; a caller-set tenant GUC is not authentication.
- **P0 CLAIMS:** foundation for P0-10A, P0-07B.
- **INPUT CONTRACTS:** ScopeV1; doc 07 matrix.
- **OUTPUT CONTRACTS:** `db/authority/roles/*.sql`, `db/authority/base/*.sql`; `mc_authority_db::{migrate(pool) /* applies MODULES in order */, TenantContext::from_verified(assertion) , with_tenant(tx, ctx)}`; role framework documented in `db/authority/roles/README.md` for other workstreams. **Exact public API (names and signatures are binding; do not rename):** `pub async fn migrate(pool: &PgPool, modules_file: &Path) -> Result<(), DbError>`; `pub struct VerifiedTenantAssertion { pub tenant_id: Uuid, pub workload_id: Uuid, pub assertion_digest: [u8; 32] }` (constructed only by an authenticating caller); `pub struct TenantContext(..)` with `TenantContext::from_verified(&VerifiedTenantAssertion) -> Self`; `pub async fn with_tenant(tx: &mut Transaction<'_, Postgres>, ctx: &TenantContext) -> Result<(), DbError>`; `pub enum DbError { Migration(String /* file name only */), TenantMismatch, Sqlx }`.
- **EXACT WRITE SCOPE:** `ws01-core-security/db/authority/roles/**`, `ws01-core-security/db/authority/base/**`, `ws01-core-security/crates/mc-authority-db/src/**`, `ws01-core-security/crates/mc-authority-db/tests/**`
- **READ-ONLY DEPENDENCIES:** I-14 test DB harness.
- **PRECONDITIONS:** W01-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws01-core-security/`):** `db/authority/roles/0001_roles.sql`; `db/authority/roles/README.md`; `db/authority/base/0001_schema.sql`; `db/authority/base/0002_tables.sql`; `db/authority/base/0003_rls_and_grants.sql`; `db/authority/base/0004_tenant_context.sql`; `crates/mc-authority-db/src/lib.rs (owned)`; `crates/mc-authority-db/src/{migrate.rs,tenant.rs,error.rs}`; `crates/mc-authority-db/tests/{roles.rs,grants.rs,rls.rs,tenant_context.rs,migrate.rs}`. **Steps:** 1. Roles exactly: ordinary_api, runner_control, evidence_ingest_writer, verification_ingest_writer, verification_set_writer, gate_writer, policy_writer, candidate_writer, projection_writer, trust_writer, revocation_writer, manifest_writer, eligibility_writer, approval_writer, custody_writer, plan_writer, publication_writer, effect_writer, budget_writer, dependency_writer, resolution_writer, readonly — non-owner, NOINHERIT, NOBYPASSRLS, no SET ROLE paths. 2. Base tables: evaluation, gate_decision, analysis_completion_receipt, evidence_receipt, verification_receipt, verification_set, policy_version + pointer, trust_bundle + pointer, revocation, current_candidate pointer, expected_manifest — `tenant_id` in every PK/FK, FORCE RLS, grants per doc 07 (insert-only where immutable). 3. Tenant context: a SECURITY DEFINER function with fixed `search_path` receives the verified assertion and sets a transaction-local tenant; mismatch rejects. 4. `migrate()` reads `db/authority/MODULES` and applies each module directory in order.
- **SECURITY INVARIANTS:** INV-15; no runtime role owns tables or has BYPASSRLS; SECURITY DEFINER only with fixed search_path and narrow EXECUTE.
- **FAILURE CASES:** Pooled connection leaks the previous tenant; FK allows cross-tenant reference.
- **TESTS REQUIRED:** `cargo test -p mc-authority-db` using a fresh database per test (`mc-test-support` not yet available: use `#[sqlx::test]`); grants test per role × table. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `roles::all_22_roles_exist_nologin_noinherit_nobypassrls`; `roles::no_runtime_role_owns_tables`; `roles::set_role_between_runtime_roles_fails`; `grants::matrix_matches_doc07 (table-driven: role × table × op)`; `grants::ordinary_api_cannot_insert_gate_decision`; `rls::force_rls_on_every_base_table`; `rls::runtime_role_cannot_disable_rls`; `rls::cross_tenant_select_returns_zero`; `rls::cross_tenant_fk_rejected`; `tenant_context::mismatched_assertion_rejected`; `tenant_context::pooled_connection_does_not_leak_tenant`; `migrate::modules_applied_in_order_on_empty_db`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** `SET ROLE` between runtime roles fails; `ALTER TABLE … DISABLE ROW LEVEL SECURITY` by a runtime role fails; ordinary_api INSERT into gate_decision fails.
- **ACCEPTANCE CRITERIA:** Role list exact; FORCE RLS on every base table; grants match doc 07 row by row; migrations apply cleanly on an empty DB.
- **EVIDENCE TO RETURN:** Grant matrix (`\dp` output); test report. A1 sends INTERFACE_NOTICE "role framework v1" to W02–W06.
- **OUT-OF-SCOPE:** P0-10A harness (T013); module tables of other units.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Grants vs doc 07; GUC-not-authentication; SECURITY DEFINER hygiene.

## Mandatory lane checks

## 11. Required testing

Every A2 runs, and every A3 re-runs:
```
    REPO_ROOT="$(git rev-parse --show-toplevel)"
    docker compose --env-file "$REPO_ROOT/ws01-core-security/.env.local" -f "$REPO_ROOT/ws01-core-security/compose.yaml" ps --status running   # PostgreSQL (port 55401) must be listed; if not, ask the human to start it from the COORDINATION worktree — A2/A3 never run up/down
    (set -a && . "$REPO_ROOT/ws01-core-security/.env.local" && set +a && cd "$REPO_ROOT/ws01-core-security" && cargo fmt --all --check && cargo clippy --workspace --all-targets --all-features --locked -- -D warnings && cargo test --workspace --locked)
    (cd "$REPO_ROOT" && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)   # must print p0_executed:false
    (cd "$REPO_ROOT" && gitleaks detect --no-banner)
```
Plus the task's TESTS REQUIRED and every NEGATIVE TEST. A test that would still pass with the guarded logic removed does not count (A3 spot-checks this).

The canonical command block specifies what must run; all build/test commands still acquire the shared wrapper above. Also run the task's complete build, stub-signature, metadata/tree and negative checks. For scratch stub compilation, use a separate task-owned copy so the submitted scaffold stays empty. Preserve failed output and identify commands/results accurately; no unrelated full-suite repetitions.

Remote CI is required for final acceptance but no push/PR is authorized here. Finish local implementation and checks, freeze a signed candidate, and return LOCAL_SOURCE_COMPLETE / REMOTE_CI_PENDING if that is the only remaining gate. Root routes independent review and the separately authorized PR/CI step. Do not invent a green CI URL or mark the task ACCEPTED/INTEGRATED.

## Stop and completion

Stop on scope/contract ambiguity, a missing prerequisite, resource refusal, repeated identical failures under live AGENTS.md section0.1, unsafe credentials/network demand, or inability to preserve the exact candidate. Do not blindly retry, reset/stash/force anything, silently add defaults or rewrite failed evidence. Report a concrete minimal blocker and remaining work to Root.

Local completion: all in-scope required files/interfaces and security boundaries implemented; all available required local positive/negative checks have actual retained results; failures and unmet checks explicitly listed; no out-of-scope diff; gitleaks/spec checks retained; signed commit on the task branch; clean index/worktree apart from permitted ignored runtime artifacts. Record BASE/IMPLEMENTATION_SHA/TREE, exact write-set diff, test names/results, evidence paths and MEMORY_UPDATES in the RETURN using the existing A2 template. Full task completion still requires independent A3 plus real CI and authorized integration.

Write the complete RETURN to /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T007-DB-ROLES-SCHEMA-r0-A2_RETURN.md. Then stop. A2 does not appoint A3, self-review/approve, push, open PRs, merge, clean other worktrees, start any VM or execute P0 claims. Final chat is exactly one text code box with result, checks, limitations, RETURN path and a proposed handoff to Root/A1; it is not an appointment.
