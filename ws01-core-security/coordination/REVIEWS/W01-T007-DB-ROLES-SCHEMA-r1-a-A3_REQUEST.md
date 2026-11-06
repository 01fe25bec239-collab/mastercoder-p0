# A1 → fresh independent A3 — W01-T007 r1 tenant authority repair

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.
FROM:Root/A1. TO:FRESH independent human-hosted A3. Human chooses model/settings.
DATE:2026-11-06 Asia/Calcutta
TASK_ID:W01-T007-DB-ROLES-SCHEMA
ATTEMPT:r1-a
REVIEWED_SHA:0f7301593d0b82576999250f8bfa8c457708c18c
TREE:7bd04c6fa4a0d2ee054b53d87bb076fe0004bc5e
REPAIR_BASE_SHA:477e58a7f990f68294cac49548f32d1b79b7c778
ORIGINAL_BASE_SHA:f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
WORKTREE:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1-review-a
FOLDER:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1-review-a/ws01-core-security
CHECKOUT:Created, clean detached at exact SHA/tree; CREATE not needed.
AUTHOR WORKTREE (read-only):/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1
DISPATCH:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T007-DB-ROLES-SCHEMA-r1-TENANT-AUTHORITY-REPAIR-DISPATCH.md
DISPATCH SHA256:e854e9cf242cc90e4cdefa942f70be4cb64ec0692c8eed3bf4825b268dd8ab36
A2 RETURN:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T007-DB-ROLES-SCHEMA-r1-A2_RETURN.md
RETURN SHA256:a6a159388284b13590774d02a7fe263950b3746022f84f184f03b56a8dc5ca70
PRIOR REVIEW:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T007-DB-ROLES-SCHEMA-r0-a-A3_REVIEW.md
PRIOR REVIEW SHA256:472cda0a4378483ddfe25e1b5ed98ae6bf74eb836674ea7d1796c3f8f8fce3d1
ROOT INTAKE EVIDENCE:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T007-r1-RETURN-PROCESSING-20261106
REPORT OUTPUT:/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T007-DB-ROLES-SCHEMA-r1-a-A3_REVIEW.md

## Independence and read order

You must not be the r0reviewer, A2 or their implementation helpers. Refuse participation if not fresh/independent. Root remains A1. Human chooses role settings. Verify actual HEAD/tree and clean tracked/index state at intake and closure; never reset/stash/change a mismatched target.

Read repository instructions, canonical /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md13.3/15, taskT007, original/repair dispatches, full r1RETURN and original A3report. Consult architecture06/07/18/ADR013/019/INV15. Prior77passes and r0review are retained historical evidence, not your own completed review or current source acceptance.

Scope: all9r1changed files and connected r0 tenant/role/grant/migration/test boundaries needed to assess F1/N1. Root independently verified9files+694/-21, clean signed sole-parent candidate, public tenant Rust executable source unchanged (docs only), all5previous SQL files unchanged and new0005migration appended. Author full82passes=17original+5new authority methods+60inherited. Root ran only the new forged-assertion matrix method once, exit0/1pass. These facts admit independent review; they are not its verdict.

Authorized implementation remains T007 base/rolesSQL/docs and mc-authority-db src/tests. No manifests/lock/contracts/other-lane changes are permitted or present. You edit no product code. Use ignored reviewer-owned logs/scratch plus the absolute report only.

## Required independent focus

1. F1must be closed at the database boundary, not just by argument shape, a Rust type or new table location. Review0005setter predicate, protected tenant_authorization table/PK/policies/grants and offline provisioning. Prove raw untrusted readonly/ordinary sessions cannot invent tenant/workload/digest authority, copy another principal's authorized tuple, edit authorization state or become owner. Check SECURITY DEFINER/session_user versus effective identity and actual runtime permissions.
2. Genuine authorized tenantA/B read/write controls must work through with_tenant and real runtime roles. Validate at least one actual non-superuser authenticated LOGIN connection on the pinned service: known correct credentials connect, false credentials fail, session/current identities and privileges are asserted, SETROLE does not alter authenticated identity, unauthorized impersonation fails. Bootstrap SETSESSIONAUTH on a superuser connection may supplement but not replace this proof.
3. Review the tests themselves: provisioned() grants canonical A/Btuples for general regressions; the focused security tests must not auto-authorize arbitrary attack arguments. Examine all172denials and positive controls, error reasons, lack of surviving context after failed authorization, writer isolation, protected-state DML denials and guard-removal sensitivity. Preserve original17method IDs/security properties; inspect every changed setup/oracle, not just new tests.
4. Assess stated residuals rather than accepting the author's labels: principal is bound by login NAME, so retirement/recreation can reuse existing authorization rows. Decide against architecture and documented offline provisioning/retirement boundary whether this is a defect or a clearly bounded privileged-operation risk. Check that a live ordinary login cannot exploit it. No silent claim that external mTLS/enrollment/signed-receipt verification is implemented by trusted SQL fixtures.
5. Assess idempotent-context-before-auth ordering, savepoint/rollback/pool reuse, principal consistency and relevant authorization changes within existing transaction semantics. Do not invent unrelated expiry/revocation APIs or treat future absent components as implemented. Report a concrete source-contract failure if one is established; otherwise retain limits explicitly.
6. Forward migration matters: previous applied SQL is immutable. Check fresh0001–0005deployment and r0→r1migration path without rewriting checksums/history or admitting a partially migrated runtime. Fixedsearch_path/owner/narrowEXECUTE,22roles/no cross-role authority, writer grants/FORCERLS/compositekeys and error redaction must remain. Check new internal-table exception to tenant-PK/table-oracle assumptions and no unrelated grants broadened.
7. N1README must accurately state role healing occurs only when role SQL actually executes, not unchanged migrate replays. Context row storage persists after commit while eligibility ends; don't claim physical deletion. Retain the storage-bound qualification without adding a new cleanup daemon.

## Independent validation

Use own clean detached checkout and private .env.local link. Every Cargo/build/test command acquires:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r1-review-a/ws01-core-security -- <actual command and arguments>

Run required final checks once serially: cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings; cargo test --workspace --locked (all22authority+60inherited, report actual counts); speccheckp0_executed:false; redacted repair/report scans. Independently reproduce the old direct-SQL exploit against repaired source with authorized controls and a small scratch guard-removal/restored control for the actual authorization predicate. Capture grant/policy evidence. Prior author/root runs are not substitutes. Do not repeat successful unchanged broad suites or expand into other claim/VM/performance tests.

Pinned Root service mc-ws01-postgres-1/localhost55401/PG16.14 is available through existing private config; image postgres@sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777. Check current version/status read-only; if access fails, report exact environment blocker. Do not start/stop Docker/compose, modify shared credentials or substitute Homebrew PostgreSQL. Use only uniquely named synthetic test DB/logins and teardown only fixtures you create; preserve all retained r0/Root/A3evidence DBs. Never print passwords or full URLs.

Existing jobs2/incremental0/sccache, own target, one executing build globally and20GiB diskstop remain. Free space observed31.776GiB; check before heavy work. No new budget/cache copy/deletion. Copy tracked source only, never target/.envfiles. Supported tool permissions may be needed for existing sccache andlocalhostDB access; do not replace actual permission refusal with a code workaround. No provider/network/VM/native-owner operations.

Historical full-ref scanner failures remain classified/qualified, not green; no repeat requested. Scan exact current changes and new outputs with redaction. No baseline/exclusion adjustment. RemoteCIhasnotrun; no push/PRauthorization or acceptance follows even from scopedPASS.

## Report and stop

Write the exact new report above using existing A3template: scoped SOURCE_ONLY PASS/PASS_WITH_NOTES/REPAIR_REQUIRED/BLOCKED, F1/N1disposition, all9file+connectedcoverage, exacttestcriteria/commands/exits, concretefile:line+reproforfindings, actualresourcequalifications andMEMORY_UPDATES. Preserve r0report/repro and r1authorfailures (compile signature and missing migration-ledger oracle) byte-for-byte. Source stays immutable.

Consolidate ordinary concrete findings; do not fix product. Stop on identity mismatch, unsafe operation, resource refusal, missing required materials or repeated-error rules. Entire final chat one text block with report/verdict and proposed handoff to Root/A1; appoint no next role. No source acceptance, interface publication, consumer unlock, remote action or production/P0claim.
