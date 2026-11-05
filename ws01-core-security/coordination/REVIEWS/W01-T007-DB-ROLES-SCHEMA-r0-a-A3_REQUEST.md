# A1 → independent A3 — W01-T007-DB-ROLES-SCHEMA r0-a

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.
FROM: Root/A1. TO: Fresh independent human-hosted A3. Human chooses model/settings.
DATE: 2026-11-06 Asia/Calcutta
TASK: W01-T007-DB-ROLES-SCHEMA r0-a
BASE_SHA: f4c4c0ef71eee7387d6373fcd62dc3897aa38eb7
IMPLEMENTATION_SHA / REVIEWED_SHA: 477e58a7f990f68294cac49548f32d1b79b7c778
TREE: 535a8ce732e7d079a74e39b3676df93e97f52b6d
WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0-review-a
FOLDER: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0-review-a/ws01-core-security
CHECKOUT: created detached and verified clean; CREATE not needed.
AUTHOR WORKTREE (read-only): /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0
DISPATCH: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T007-DB-ROLES-SCHEMA-r0-DISPATCH.md
DISPATCH SHA256: e15490b8638a5962914bcf143552dcf0692f04752cf0753cae62054a133f271f
A2 RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T007-DB-ROLES-SCHEMA-r0-A2_RETURN.md
RETURN SHA256: 11ec9a0d44dd51f1306460315ba03a158f53fca2c4c218176b8211e54e1d4c39
ADDITIVE ROOT VALIDATION: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPOSITIONS/W01-T007-DB-ROLES-SCHEMA-r0-PINNED-VALIDATION-ADDENDUM-20261106.md
ROOT EVIDENCE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T007-PINNED-POSTGRES-VALIDATION-20261106
REPORT OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T007-DB-ROLES-SCHEMA-r0-a-A3_REVIEW.md

## Independence and read order

Refuse if you authored/repaired the candidate or participated in its A2 helpers. Root remains A1; this request is independent review, not an A1 replacement or implementation task. Verify exact HEAD/tree and tracked/index clean state at intake and closure; mismatch means INVALID_REVIEW_TARGET. Never reset, stash, repair or edit product source.

Read repository AGENTS.md/SKILL.md/SUBAGENT_POLICY.md, then canonical /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md sections13.3/15, exact task in task.md, the complete dispatch, A2 RETURN and Root additive validation above. Human chooses settings, superseding older model-specific defaults. Review full16-file diff and connected callers/tests, not just the author's explanation. Read architecture doc07, doc18, ADR013/019 and exact schemas/invariants cited by the task.

Authorized implementation paths remain only ws01-core-security/db/authority/roles/**, db/authority/base/** and crates/mc-authority-db/src/**, tests/**. No manifests/lockfile/contracts/other-lane changes were made. Root verified signature, sole base parent, exact16paths and unchanged source; that is intake verification, not an A3 verdict.

## Environment blocker resolved prospectively

The original A2 RETURN honestly remains NOT_READY with scratch-PG-only evidence and placeholder failures; it must not be rewritten. Root started Docker and only the isolated mc-ws01 PostgreSQL service, using the exact pinned image above and a Root-owned loopback override. Root replaced only the two placeholder local test-config values with a generated synthetic test password and valid localhost URL in the existing private0600 lane env. No credential appears in evidence or source. Review .env.local points to that private configuration; never print/source it with tracing.

Pinned server is PostgreSQL16.14, not A2's temporary17.10. ONE Root cargo test --workspace --locked completed77PASS/0FAIL/0ignored in26.962s (60inherited+17authority methods), exact source unchanged. Root separately applied the frozen migrations to one new isolated evidence database and captured the required raw \dp grant matrix; that database is retained, not shared test state. See pinned-workspace-result.json, pinned-workspace.stdout/.stderr, pinned-grant-capture.json and pinned-dp-grant-matrix.txt. All author failures, scratch results, assumptions and prior blocked disposition are retained.

Use shared service only with fresh uniquely named sqlx test databases. A3 must not start/stop Docker/compose, modify shared credentials, alter user's other services or delete any data. If service unavailable, report ENVIRONMENT_BLOCKED without a substitute private server. The old COORD-ws01 product snapshot has no compose.yaml; do not mistake that for a candidate source defect. The exact current compose definition is in the author/review checkout, and Root selected it by absolute path from coordination cwd.

## Independent review focus

1. Compare all22 runtime roles, offline owner, role memberships, table ownership, grants and13base tables against doc07/18 row by row. Check FORCE RLS on every table and composite tenant/repository/epoch constraints; ensure tests cannot pass merely because superuser bypassed the boundary.
2. Examine all seven author assumptionsA1–A7 against exact architecture; do not adopt them because the author calls them inherent. In particular, any runtime role can invoke set_tenant_context with a tenant/workload/digest. Decide whether this supplies the required verified-assertion trust boundary, whether the chosen RLS row mechanism protects against the attacker model, and whether an ordinary/readonly role can establish unauthorized tenant authority. Use a concrete independent control if needed; never accept the assertion by type name alone.
3. Check SECURITY DEFINER ownership, fixed search_path, PUBLIC privileges, RLS owner policy, SQL injection/error paths and privileges on migration/schema objects.
4. Review transaction binding, savepoints, rollback, pooled reuse and the retained-row lifecycle. The original RETURN's claim that rows die with transactions is overbroad: committed rows can remain until same-PID cleanup. Test actual authority semantics and assess boundedness/concurrency; distinguish inaccurate prose from an exploitable source defect.
5. Review migration module ordering/path validation, per-file transaction/checksum ledger, idempotency and role-heal races. Verify exact Rust API/errors and redacted failure behavior. Do not invent a new cryptographic mechanism, API or scope amendment yourself.
6. Check independent test oracles, SQLSTATE assertions and negative/restored controls, plus honest source/scratch/environment deviations. Consolidate concrete findings within the existing scope; no unrelated abstractions or speculative extra test framework.

## Independent validation

Freshly reproduce required local checks in your own detached worktree with its private lane environment. Every Cargo/build/test invocation uses:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T007-DB-ROLES-SCHEMA-r0-review-a/ws01-core-security -- <actual command and arguments>

Required: cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings; cargo test --workspace --locked, containing all17authority methods and60inherited tests; task-specific exact grant/role/RLS/tenant/migration checks; check_spec.py at repository root with p0_executed:false. All DB tests must exercise this pinned service, not mocks or a newly selected PostgreSQL. Check service status read-only; the configured localhost TCP port and exact server identity are recorded above.

Preserve all source/pins. Tiny reviewer-owned guard-removal controls belong in scratch copies, never the author/review candidate. Use existing jobs2/incremental0/sccache and own per-worktree target; one build globally and20GiB free-disk stop remain. Do not increase budgets, share targets or delete output to evade limits. Keep artifacts/logs under your ignored task-owned target area; report exact commands/exits and sampled resource limitations honestly.

The retained744historical scanner findings were independently mapped by Root to738public schema-name contexts and6nonsecret source-path/SHA256 contexts; original nonzero exits remain. Classification: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/THREE-A2-RETURN-PROCESSING-20261106/historical-scan-classification.json. Verify as necessary and run redacted change/review-output scans. No allowlist/baseline was changed. Avoid another all-ref broad scan for this preliminary source request; canonical full-history check limitations remain explicit rather than silently green.

No remote CI has run or been authorized. Even SOURCE_ONLY PASS requires Root processing and the real separately authorized PR/CI stage before task acceptance/integration. No interface notice/consumer unlock follows from this request alone.

## Output

Write only reviewer-owned ignored scratch/log/build artifacts and the absolute report above using the existing A3 template. Record full scope coverage, criterion-by-criterion MET/NOT_MET evidence, exact file:line/repro for each real defect, independent commands/exits, all limitations and MEMORY_UPDATES. Use PASS/PASS_WITH_NOTES/REPAIR_REQUIRED/BLOCKED scoped SOURCE_ONLY; no self-acceptance or next-role appointment.

Stop on identity mismatch, unsafe execution/security exposure, unavailable required materials, resource refusal or repeated-error rules. Ordinary source findings should be consolidated into one scoped report, with no product edits. Final chat is exactly one text code box with verdict, checks, blockers/report path and proposed handoff back to Root/A1. No VM/native-owner operation, push/PR/merge, cleanup, new spending or production claim execution.
