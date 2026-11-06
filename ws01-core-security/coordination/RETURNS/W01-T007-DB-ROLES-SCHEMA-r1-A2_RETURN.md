FROM: W01-A2[W01-T007-DB-ROLES-SCHEMA-r1]
TO: Root A1 (acting W01-A1)
MESSAGE_TYPE: RETURN
TASK_ID: W01-T007-DB-ROLES-SCHEMA
ATTEMPT: r1 (repair of A3 r0-a finding F-1, plus N-1)
DISPOSITION: READY_FOR_A3 (local validation complete on the pinned PostgreSQL 16.14 service; no acceptance claimed; no self-approval)
DATE: 2026-11-06 Asia/Calcutta
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

LOCATION:
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1
  WORKSTREAM_DIR: ws01-core-security/
  BRANCH: ws01/w01-t007-db-roles-schema-r1
  DISPATCH: COORD-ws01/.../DISPATCHES/W01-T007-DB-ROLES-SCHEMA-r1-TENANT-AUTHORITY-REPAIR-DISPATCH.md
  REVIEW REPAIRED: REVIEWS/W01-T007-DB-ROLES-SCHEMA-r0-a-A3_REVIEW.md (sha256 472cda0a…, as cited in the dispatch)

REPAIR_BASE_SHA: 477e58a7f990f68294cac49548f32d1b79b7c778 (tree 535a8ce732e7d079a74e39b3676df93e97f52b6d)
IMPLEMENTATION_SHA: 0f7301593d0b82576999250f8bfa8c457708c18c
TREE: 7bd04c6fa4a0d2ee054b53d87bb076fe0004bc5e
COMMITS: one commit, sole parent = REPAIR_BASE_SHA
SIGNATURE: git verify-commit exit 0 — Good ED25519 signature, Omkar Gavale <01fe25bec239@kletech.ac.in>
WORKTREE STATE: clean (`git status --short` = 0 lines) at IMPLEMENTATION_SHA
ACCEPTED_SHA / MERGED_SHA: NONE. PUSH / PR / MERGE: NONE. REMOTE_CI: NOT_RUN. No reviewer appointed. Old r0 candidate, worktrees, review and failures untouched.

## Verdict on F-1 (what was wrong, what changed)

r0: `set_tenant_context` validated only argument shape, so any of the 22 runtime sessions chose its own tenant. Root's/A3's retained control (0 victim rows before, 1 after an invented assertion as `readonly`) was real.

r1: new migration `db/authority/base/0005_tenant_authorization.sql` (appended, not an edit of applied 0004: applied files are immutable per roles/README rule 6, so r0-migrated databases are neither resealed nor retro-applied; the repair is a forward migration):
- `mc_authority.tenant_authorization (principal name, tenant_id, workload_id, assertion_digest)`, PK = all four columns, tenant ≠ nil; owned by `mc_authority_owner`, ENABLE+FORCE RLS, policy `owner_only` (TO owner), REVOKE ALL FROM PUBLIC, **no grant to any runtime role**.
- `CREATE OR REPLACE FUNCTION set_tenant_context(uuid,uuid,bytea)` (same signature, owner, SECURITY DEFINER, search_path, EXECUTE ACL). Order: shape validation → prune stale rows → existing-row idempotent/`MCT01` mismatch (a row can only exist after authorization succeeded in this same transaction) → **authorization predicate**: a row must exist for `principal = session_user` AND tenant AND workload AND digest, else `RAISE 'tenant authorization denied' ERRCODE 42501` and nothing is inserted → insert context.
- Identity used is `session_user` (the authenticated login). Not caller-supplied, not a GUC, not `current_user` (inside the definer that is the owner; after SET ROLE it is the runtime role). Non-superusers cannot change it (`SET SESSION AUTHORIZATION` refused, proven below).
- Trusted provisioning point: only the privileged bootstrap connection, `SET LOCAL ROLE mc_authority_owner; INSERT INTO mc_authority.tenant_authorization …` (documented in roles/README.md "Provisioning tenant authorization"; test helper `authorize()` in tests/common). Runtime denial point: the table has no runtime grant and the setter predicate. This is a grant/session binding only; the external mTLS/enrollment/signed-receipt pipeline is NOT implemented and not claimed.
- Public Rust API byte-identical in signatures: `VerifiedTenantAssertion` (3 pub fields), `TenantContext::from_verified`, `with_tenant(&mut Transaction, &TenantContext)`, `migrate`, `DbError` (same variants). `src/tenant.rs` changed doc comments only (diff: 7 insertions/3 deletions, all `///`). Unauthorized SQL maps through the existing redacted `DbError::Sqlx`.
- Unchanged: 22 NOLOGIN/NOINHERIT/NOBYPASSRLS roles, memberships, FORCE RLS, insert-only/writer grants, composite keys/FKs, tenant_isolation policies, `current_tenant()`. No dependency/manifest/lockfile/arch change; no `.env` variable added.

## Exact delta (git diff 477e58a..0f73015: 9 files, +694/−21)
| Path | Change |
|---|---|
| db/authority/base/0005_tenant_authorization.sql | NEW (76 lines) |
| db/authority/roles/README.md | N-1 wording; "context ends with transaction" rewritten (eligibility vs storage); provisioning section; grants note (EXECUTE is not the boundary) |
| crates/mc-authority-db/src/tenant.rs | doc comments only |
| crates/mc-authority-db/tests/tenant_authority.rs | NEW, 5 focused tests (510 lines) |
| tests/common/mod.rs | + `authorize()` (explicit trusted provisioning) and `provisioned()` |
| tests/grants.rs, rls.rs, migrate.rs, tenant_context.rs | oracle/setup edits listed below |
Write set: every changed path is under the dispatch's authorized paths (`git diff --name-only` filtered by the four allowed prefixes → empty remainder). No coordination/memory/task/evidence/ARCHITECTURE/manifest file touched. `0001_roles.sql` and `0004_tenant_context.sql` NOT edited (applied-file immutability).

## Old test body/oracle changes (all 17 original method IDs preserved, none weakened)
1. tests/common/mod.rs — added `authorize`, `provisioned`; `begin_as`, `begin_tenant_as`, `assertion`, `seed_tenant` etc. unchanged. `provisioned(pool)` = migrate + authorize **each of the 22 roles for the two canonical tuples assertion(A), assertion(B) only** (not a function of any attacker input).
2. grants.rs — `ordinary_api_cannot_insert_gate_decision`, `insert_only_tables_reject_update_and_delete`: first line `migrated`→`provisioned` (they run authorized tenant transactions; reason: r0 setup relied on the vulnerable self-assertion). `matrix_matches_doc07`: expected-table oracle gains `tenant_authorization` (a new internal table; all 22 roles × 7 privileges still computed by the DB and required to be denied; the real-INSERT-denied loop now covers it automatically).
3. rls.rs — `runtime_role_cannot_disable_rls`, `cross_tenant_select_returns_zero`, `cross_tenant_fk_rejected`: `migrated`→`provisioned`; in `cross_tenant_select_returns_zero` the "skip unreadable internal table" check also skips `tenant_authorization` (same reason as `tenant_context`: runtime roles have no access).
4. migrate.rs — `modules_applied_in_order_on_empty_db` expected ledger list gains `("base","0005_tenant_authorization.sql")` (first targeted run FAILED exactly on this; recorded in targeted-2.log; fixed oracle).
5. tenant_context.rs — `mismatched_assertion_rejected`, `pooled_connection_does_not_leak_tenant`: `migrated`→`provisioned` only (bodies/assertions unchanged; mismatch still `TenantMismatch`, nil tenant still `Sqlx`, pool reuse unchanged). `definer_functions_have_fixed_search_path_and_narrow_execute`: the EXECUTE-list assertion is kept (still exactly 22 roles + owner, never PUBLIC) but the comment now states it is not the trust boundary, and a new assertion requires the setter body to reference `tenant_authorization` and `session_user`. `runtime_role_cannot_write_tenant_context_directly`: unchanged.

## New focused tests (tests/tenant_authority.rs; trusted state is provisioned BEFORE the untrusted step; no helper authorizes an attack tuple)
- `forged_assertions_cannot_establish_tenant_authority`: fixture authorizes manifest_writer for A+B (seeds one manifest each), `readonly`→A only, `gate_writer`→B only. Positive: readonly(A) sees exactly manifest 1; gate_writer(B) sees exactly manifest 2 (public `with_tenant`). Attack matrix: all 22 runtime roles × 8 tuples (invented B/99/ff; invented A; B with wrong workload; B wrong digest; A wrong workload; A wrong digest; canonical A; canonical B) minus the 4 provisioned (role,tuple) pairs → 172 denials, each via `with_tenant` (`Err(DbError::Sqlx)`) AND literal SQL (SQLSTATE 42501, "tenant authorization denied"), then `current_tenant()` is NULL and readonly sees 0 rows. Also: A-authorized readonly cannot read B with a made-up tuple or by copying gate_writer's valid tenant-B tuple; gate_writer cannot copy readonly's A tuple; manifest_writer authorized for A cannot write a B manifest (RLS WITH CHECK 42501); missing context still returns 0 rows.
- `unauthorized_writer_cannot_insert_victim_manifest`: manifest_writer with no authorization: forged/canonical B assertions denied, victim manifest INSERT denied (42501 row-level security), bootstrap superuser count = 0; positive control: after explicit `authorize(manifest_writer, B)` the identical insert succeeds (count 1).
- `real_login_session_binds_tenant_authority` (real non-superuser LOGIN on the pinned service): three fresh logins (random names/passwords, never printed, NOSUPERUSER NOBYPASSRLS NOINHERIT, each member of exactly `readonly`), connect on separate pools, `SET LOCAL ROLE readonly`. Verified per login: `session_user`=login, `current_user`=readonly, neither superuser nor BYPASSRLS. login a→A, b→B provisioned, x→nothing. Denied: x with all 8 tuples (including the valid tuples of a and b); a with b's valid tuple, invented tuple and wrong workload; b with a's tuple, invented tuple, wrong digest — all with literal-SQL 42501, no context, 0 rows. The login cannot INSERT its own authorization (42501), cannot `SET ROLE mc_authority_owner`, cannot `SET SESSION AUTHORIZATION` another login/readonly (all 42501). Positive: a sees only A's manifest, b only B's, through public `with_tenant`. Fixtures dropped by the test (leftover check below).
- `authorization_state_is_owner_only`: every runtime role is denied SELECT/INSERT/UPDATE/DELETE/TRUNCATE (42501); relacl has no non-owner entry; owner is mc_authority_owner; RLS enabled+forced.
- `removing_the_authorization_predicate_reopens_the_forgery` (guard-removal control, fresh scratch database only): intact → forged call denied, 0 rows; the predicate in the live function is mutated (`IF NOT EXISTS (` → `IF FALSE AND NOT EXISTS (` through `pg_get_functiondef`), then the identical forged call returns Ok and the victim tenant-B manifest becomes visible (the A3 reproduction). Intended reason demonstrated; mutation exists only in that disposable test database; committed source contains the real predicate (restored by construction).
Also run directly with psql on a fresh unique database on the pinned service (applied 0001–0005 as superuser; DB dropped): A3's control adapted — before 0 rows; forged `set_tenant_context` as `readonly` → `ERROR: tenant authorization denied`; after 0 rows and `current_tenant()` NULL; session_user/current_user `readonly`, privileged = f (target/a2-r1-logs/a3-control-r1.sql/.log).

## Required commands (all through run_single_build.py with the prescribed cwd; lane env from .env.local; own target; jobs 2 / incremental 0 / sccache from env)
| Command | Exit | Result | Log (ws01-core-security/target/a2-r1-logs/) |
|---|---|---|---|
| cargo test -p mc-authority-db (1st compile) | 101 | compile error: sqlx::test cannot take (PgPool, PgConnectOptions); fixed in test signature | targeted-1.log |
| cargo test -p mc-authority-db --no-fail-fast (targeted) | 101 | 21 pass, 1 FAIL (migrate ledger oracle lacked 0005) — preserved | targeted-2.log |
| cargo fmt --all --check (after final fmt) | 0 | clean | fmt-check.log |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | 0 | clean | clippy.log |
| cargo test --workspace --locked --no-fail-fast (final, at the tree committed as IMPLEMENTATION_SHA; no file changed after) | 0 | all green: 22 mc-authority-db integration tests (17 preserved + 5 new) + all 60 inherited tests; 0 failed, 0 ignored | test-workspace.log |
| python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (repo root) | 0 | prints `"p0_executed": false` | check-spec.log |
| gitleaks detect --redact --log-opts=477e58a..HEAD | 0 | no leaks | gitleaks-range.log |
| psql grant evidence on pinned service (fresh DB, then dropped) | 0 | `\dp mc_authority.*` + function ACLs: tenant_authorization/tenant_context owner-only, setter EXECUTE unchanged (22 roles + owner) | grant-evidence-pinned.txt |
Broad suites were not repeated after the final green run (no source change since). Docker/compose were NOT touched (A2 never starts/stops them); `docker compose ps` was not run. Pinned service identity was verified by connecting as the lane user: `server_version` = 16.14, superuser, listener reached through mapped localhost:55401; the image digest itself was not independently inspected (relied on Root's dispatch). Full-history repo-wide gitleaks (744 pre-existing generic-api-key findings in other lanes, r0 RETURN) was NOT re-run; only the range scan for this commit.

## Service interactions and cleanup
Only uniquely named fixtures I created: sqlx per-test databases (dropped by sqlx on pass), three test logins per run (dropped by the test), one psql evidence database `mc_t007_r1_evidence_*` (dropped), one auth-probe login (dropped). After the final run the cluster has 0 roles `mc_t007_%`, 0 evidence databases and 0 `_sqlx_test%` databases. No existing/retained database, shared credential, Docker, Homebrew PostgreSQL or shared config was touched. If a test panics mid-way its cluster-global `mc_t007_*` logins could leak (a later run will not collide: random names).
Credential check on the pinned service (login-auth-probe.log): password auth is enforced for host connections — right password connects, wrong password gets `password authentication failed` (pg_hba has a trailing scram-sha-256 rule; the trust rules are local/loopback inside the container). The test logins therefore exercise real credential authentication, not only a spoofed username.

## Qualifications and residual risks (honest)
1. Binding is by login **name** (`session_user`). Dropping and recreating a login of the same name inherits its authorization; operators must delete `tenant_authorization` rows when retiring a login. (Could bind role OID later; not done to stay minimal.)
2. A login provisioned for several tenants can assert any of its provisioned tuples (by design); one login per workload/tenant is a provisioning rule, documented. Runtime logins must not be superuser/owner members; a compromised superuser or the offline owner remains the architecture's privileged residual risk.
3. The `authorize()` test helper and the README INSERT are trusted bootstrap fixtures, **not** the verified external identity pipeline (mTLS, enrollment, signed receipts): the digest is accepted as a provisioned opaque value, not recomputed/validated against signed evidence. No signing/MAC protocol was invented.
4. Most attack tests use `SET LOCAL SESSION AUTHORIZATION` on the superuser bootstrap connection (as A3's control did) to act as each of the 22 identities; per dispatch this is supplementary — the real-login test is the end-to-end credential/principal proof. SET ROLE-based login (README model) is what the real-login test uses.
5. Older text in the *applied* files `0001_roles.sql` ("revoked on every run") and the `0004` header ("only writer is set_tenant_context", rows "disappear") cannot be edited without breaking the applied-file hash rule and were left as is; README is authoritative and now states: role SQL heals when actually executed, `migrate()` skips unchanged applied files and is not a role-drift healer; committed context rows are ineligible (not deleted) after their transaction and are pruned on that backend's next setter call or by crash clearing.
6. Not re-run: full-history gitleaks, docker compose ps, remote CI. Production remains NOT AUTHORIZED; no P0 claim; this is not a G2/G3 result.
7. Grant semantic note for A3: EXECUTE on the two functions still goes to all 22 runtime roles on purpose (denial is by authorization, not by EXECUTE); the old EXECUTE-list oracle is preserved and supplemented, not removed.

BLOCKERS: NONE. CCR_DRAFTED: NONE (no contract/write-scope conflict).
SECRETS_CHECK: range gitleaks 0 findings; hardcoding grep (`/Users/|/home/|C:\\|https?://|:5540|AKIA|BEGIN .*PRIVATE KEY|claude-|gpt-`) over added lines → no hits; test credentials are random per run and never logged/committed; no `.env.example` change (no new variable).
ARCHITECTURE_CONTRACTS_USED: doc 06, 07 (Grants and tenant isolation), 18, ADR-013/019, INV-15 (read-only; none modified).
SCHEMAS_USED: none new on the wire; internal SQL table mc_authority.tenant_authorization (no public schema, no REGISTRY name).
SUBAGENTS: none used (small sequential change; every result verified directly).

MEMORY_UPDATES (for Root to apply; memory.md untouched by A2):
- T007 r1 candidate 0f73015… tree 7bd04c6f… closes A3-F1 locally: setter requires owner-only `tenant_authorization` row for `session_user`+tenant+workload+digest (migration 0005, appended not edited). Public Rust API unchanged. A3 must be fresh and exact-SHA.
- Applied migration files are immutable by repo rule: repairs of released SQL must be appended as a new file.
- Trusted provisioning of tenant authorization is operator/bootstrap-only SQL; external verified-identity pipeline remains unimplemented by this task.
- Pinned service facts: PG 16.14 at lane URL, password auth enforced for host connections; test logins named `mc_t007_*`, evidence DBs `mc_t007_r1_evidence_*`.

## Pre-RETURN checklist (SKILLS §15)
[x] Every acceptance item has evidence (above)
[x] Negative tests assert the specific reason (SQLSTATE 42501 + "authorization denied"/"row-level security"; MCT01 preserved) with genuine positive controls
[x] Guard-removal control present and shown to flip the forged result (scratch DB only)
[x] Only write-set paths changed; migration appended, applied files untouched
[x] No invented schema/enum/digest/authority rule beyond the Root-selected internal grant binding
[x] No secrets; gitleaks range clean
[x] PROTOTYPE header on new files
[x] Required commands run at the final tree with exit codes recorded; earlier failures preserved
[x] Limitations stated; unverified items labelled (image digest, full-history gitleaks, compose)
[x] Worktree clean; SHA/tree copied via command; signature verified
[x] Not self-approved; not pushed; no PR; no reviewer appointed
