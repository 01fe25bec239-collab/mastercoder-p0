# Independent A3 source review — W01-T007-DB-ROLES-SCHEMA r1-a
PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

FROM: fresh W01-A3[r1-a]
TO: Root/A1
MESSAGE_TYPE: REVIEW_RESULT
VERDICT: PASS_WITH_NOTES — SOURCE_ONLY
DATE: 2026-11-06 Asia/Kolkata
REPAIR_BASE_SHA: 477e58a7f990f68294cac49548f32d1b79b7c778
ORIGINAL_BASE_SHA: f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
REVIEWED_SHA: 0f7301593d0b82576999250f8bfa8c457708c18c
TREE: 7bd04c6fa4a0d2ee054b53d87bb076fe0004bc5e
WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1-review-a
INDEPENDENCE: fresh session; not r0 reviewer, A2, or an implementation helper. Two fresh read-only helpers reviewed standards and specification; main reviewer independently checked their conclusions and ran validation. Explicit helper spawn settings: gpt-6.1-sol/high, as permitted by SUBAGENT_POLICY; host offers this model/effort. Actual execution/context telemetry is not exposed to helpers; no measured context claim. No helper edits or builds.
INTAKE/CLOSURE: detached exact HEAD/tree above, clean tracked/index state both times. Source immutable throughout. Only new report in COORDINATION and ignored reviewer scratch/logs written.

## Findings and prior disposition

No new blocking source defect established. F-1 CLOSED for this scoped database boundary; N-1 CLOSED. This is a source review verdict, not acceptance, interface publication, consumer unlock, G2/G3 evidence or production authorization.

0005_tenant_authorization.sql:16–28 creates protected owner-only authorization state, PK containing principal/tenant/workload/digest and tenant_id, ENABLE/FORCE RLS, no runtime grants. Lines63–70 require the exact tuple for session_user, independently of caller-controlled fields or the effective SECURITY DEFINER identity. The ordinary authenticated login cannot change session identity, provision authorization, or become the offline owner. Copying another principal's tuple fails. Fixed search_path, owner and narrow existing EXECUTE ACL survive CREATE OR REPLACE; EXECUTE alone is explicitly no longer represented as authentication.

Independent raw SQL on the pinned service: original invented tenant B/workload99/ff assertion denied with 'tenant authorization denied', with zero visible manifests before the call. Real authenticated login authorized for A sees only A; B assertion denied. In reviewer scratch DB only, disabling the actual IF NOT EXISTS predicate makes the same forged B call reveal one B row. Restoring the original function denies it again. Explicit B provisioning then permits B access. Full script and output retained. Candidate matrix independently passed all 172 unprovisioned role/tuple combinations through both with_tenant and direct SQL, checking 42501/authorization reason, no surviving context and readonly victim denial; authorized A/B controls and unauthorized-writer/authorized-writer controls pass. No attack argument is automatically authorized by the focused fixtures.

N-1 README:23–28 now correctly limits healing to execution of role SQL; unchanged migration replay does not heal drift. README:62–67 distinguishes eligible context from physical storage. Committed context rows can persist; no bounded storage/churn cleanup proof is claimed.

## Scope and connected coverage

All nine changed files reviewed: src/tenant.rs (comments only; executable public API unchanged); tests/common/mod.rs (explicit authorize helper and canonical A/B provisioning for old regressions); grants.rs (internal-table oracle plus setup); migrate.rs (expected ledger addition); rls.rs (setup plus unreadable internal table exclusion); tenant_context.rs (setup plus body-reference assertion, original ACL oracle retained); tenant_authority.rs (five new methods); base/0005_tenant_authorization.sql; roles/README.md. Nine paths +694/-21, all inside authorized task scope. No manifest/lock/dependency/contracts/architecture/other-lane changes. Previous five SQL files are unchanged.

Connected review covered original base schema/tables/RLS/context, roles SQL, MODULES, lib/error/tenant/migrate, all original tests, original and repair dispatches, full r1 return and r0 review. Architecture comparison: docs06/07/18, ADR013/019, INV15. All 17 original authority method IDs/security properties survive; all 60 inherited tests pass. Setup provisioning grants only the two fixed canonical tuples, never arbitrary attacks. Internal-table omissions from readable-table queries are appropriate because grants matrix and focused DML tests separately prove denial. The tenant_authorization PK includes tenant_id; its owner-only policy is infrastructure authorization state, not a runtime tenant-table grant.

Unchanged role separation remains 22 NOLOGIN/NOINHERIT/NOBYPASSRLS runtime roles, no cross-role/owner authority; writer table grants and permitted column updates, FORCE RLS and composite tenant/repository keys remain. Actual DB controls cover forbidden SET ROLE, RLS disabling, gate insertion, cross-tenant FK, insert-only writes and error redaction. Function bodies use qualified relations and fixed search_path; no caller SQL is interpolated in product code. VerifiedTenantAssertion fields, TenantContext construction, with_tenant transaction signature, migrate and DbError variants remain unchanged.

Migration: per-file transaction makes new table plus setter replacement atomic. Old files/checksums are not rewritten. Fresh deployment passes the workspace suite. An independent scratch executable calling the actual candidate migrate function applies five original files extracted from r0, snapshots ledger entries/checksums, upgrades using current MODULES, asserts all five entries unchanged and exactly 0005 appended, then replays successfully. The raw SQL control also applies r0 followed by 0005. Deployment must keep runtime admission closed until migration succeeds; running r0 while an upgrade is pending retains r0's known vulnerability. No claim of an admission manager implemented here.

## Notes and explicit limits

1. Principal is a PostgreSQL login NAME (0005:17,65). Privileged drop/recreation with the same name can inherit stale grants. This limitation is documented in the full A2 RETURN, not explicitly in the README; preserve it in operator handoff and remove authorization rows when retiring a login. A live NOSUPERUSER/NOCREATEROLE runtime login cannot perform this lifecycle operation. Within the selected offline provisioning boundary, this is a privileged-operation residual rather than an established ordinary-runtime exploit.
2. Idempotent context return precedes a second authorization lookup (0005:51–59). A context already established in a transaction remains usable until transaction end even if an operator removes the grant. README 'remove a row to revoke' means future context establishment, not immediate fencing of an open transaction. No immediate context-revocation contract was established in this foundation scope. Same-principal consistency holds for ordinary logins; privileged SET SESSION AUTHORIZATION is outside this runtime boundary. Mismatch, savepoint rollback and pool reuse tests pass.
3. Trusted SQL provisioning is a session/tuple grant. External mTLS/enrollment/signed-receipt verification and source signatures remain external/owning-task obligations; these tests do not implement or prove them.
4. Context storage may accumulate for departed backends; PID/full-xid eligibility prevents transaction reuse but is not a retention bound. No cleanup daemon added or claimed.
5. Remote CI NOT_RUN. Historical full-ref scanner failures remain qualified historical nonzero results; no broad-history rescan or green-history claim. No production/P0 execution, provider mutation, push/PR/merge or acceptance.

## Independent validation and retained artifacts

Artifacts: review checkout ws01-core-security/target/a3-evidence/ (ignored). Private .env.local link used without printing credentials/URLs. Every Cargo/build command used:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1-review-a/ws01-core-security -- COMMAND

| Command | Exit / actual result | Artifact |
|---|---|---|
| cargo fmt --all --check | 0 | fmt.log |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | 0 | clippy.log |
| cargo test --workspace --locked | 0;82 passed,0 failed,0 ignored =22 authority+60 inherited | tests.log |
| python3 ../ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py | 0;p0_executed:false;40 semantic cases | spec.log |
| python3 target/a3-evidence/control.py | 0;actual password positive/negative, raw exploit denial, A/B controls, guard removal/restoration, grants/policies | control.py/control.log/grants.txt |
| python3 target/a3-evidence/build_upgrade.py (rustc, wrapper) then target/a3-evidence/upgrade | 0 /0;actual migrate r0→r1→replay, ledger preservation | upgrade.rs/upgrade-build.log/upgrade.log; r0-sql/ |
| gitleaks detect --no-banner --redact --log-opts=477e58a7f990f68294cac49548f32d1b79b7c778..0f7301593d0b82576999250f8bfa8c457708c18c | 0;no leaks | gitleaks-change.log |
| gitleaks detect --no-git --no-banner --redact --source target/a3-evidence | 0;no leaks | gitleaks-output.log |
| added-line hardcoding rg; git diff --check BASE HEAD | 1=no matches;0 | hardcoding.log |

Pinned service independently inspected read-only: mc-ws01-postgres-1 running, Config.Image postgres@sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777; SQL reports PostgreSQL16.14. Correct synthetic password connects, false password specifically fails password authentication. SET ROLE preserves session_user, effective readonly has no superuser/BYPASSRLS. Committed real-login test independently checks failed impersonation/self-provisioning and exact A/B access via with_tenant. Raw catalog evidence contains relation ACL/owners/FORCE flags, policy expressions and function ACL/search_path.

Reviewer harness failures preserved: control-first-failure.log/control-first.py contain initial exit1 caused by a missing semicolon after pg_get_functiondef output; corrected scratch harness passed. upgrade-build-first-failure.log contains initial exit1 from assuming exactly one tokio rlib (three feature artifacts existed); selection of an existing compiled artifact corrected it, build and runtime passed. Premature invocation of missing executable returned127; this was a harness sequencing failure, not product behavior. No repeated same failure and no product edits. Neither failure was relabeled a product result.

Only uniquely named scratch DBs/logins owned by these probes were created/dropped; retained r0/Root evidence untouched. sqlx manages its fresh suite databases. No Docker start/stop, shared credential change, cache copy/deletion or alternate PostgreSQL. Disk roughly32GiB before and30GiB after; wrapper enforced20GiB stop and one executing build globally. Existing jobs2/incremental0/sccache and own target used; peak RAM not measured. Broad green suites run once only.

Preservation verified: r0 review SHA256 remains472cda0a4378483ddfe25e1b5ed98ae6bf74eb836674ea7d1796c3f8f8fce3d1; full r1 RETURN remainsa6a159388284b13590774d02a7fe263950b3746022f84f184f03b56a8dc5ca70. Author failures and previous evidence files were not edited.

MEMORY_UPDATES:
- Exact r1 candidate receives independent SOURCE_ONLY PASS_WITH_NOTES: F1 database forgery fixed and N1 wording corrected;82 tests pass independently.
- Real host password rejection independently established; guard removal exposes one victim row and restoration denies again.
- Actual migrate r0→r1 preserves five previous ledger entries/checksums and appends only0005; replay succeeds.
- Preserve privileged login-name reuse/retirement and transaction-scoped grant-deletion qualifications; external identity pipeline and context retention bound are unimplemented/unproved.

Checklist: acceptance criteria and exact negative methods evidenced; mutation sensitivity proved; write set/public contracts/prototype headers/hardcoding checked; range and output scans clean; all required checks re-run; limits explicit; exact HEAD/tree clean at closure; next Root coordination location verified existing, branch ws01/coordination, clean before new report. No self-approval or role appointment.

Root handoff: interpret this exact-SHA scoped report under existing gates. Do not treat local PASS_WITH_NOTES as acceptance or permission for remote actions.
