# W01 combined implementation — all remaining waves

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.
FROM: Root/A1
TO: fresh human-hosted A2
TASK_ID: W01-LANE-COMPLETION
ATTEMPT: r0
STATUS: PREPARED_NOT_LAUNCHED; TRUSTED_SCOPE_REGISTRATION_PENDING
WORKTREE: /Users/omkar/.codex/worktrees/w01-combined-waves-r0/repo
BRANCH: ws01/w01-lane-completion-r0
PREPARED_BASE: 603640c870e4470fda9fd3cd0e5011bf5f28771b
PREPARED_TREE: 17b33ab470340a11db143c6ccca530c7a74d3eca
HUMAN SETTINGS: Human chooses model/effort. No role is launched by this file.

## Goal and boundaries
Complete the49remaining numbered W01 BUILD tasks as one ordered campaign. Use the original53-task plan and frozen architecture, not a new design. T000/T004/T007/T009 source already exists on main; preserve its reviews and outstanding qualifications. T007/T009 physical merges are not proof that every remaining acceptance gate is closed. No RUN task or real authority activation belongs to this campaign.

## Read in this order
1. Live canonical /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md sections1–12/13.2/14/15, memory.md, task.md and latest handoff. Read repo SKILL.md/SUBAGENT_POLICY.md. These live files take precedence over the worktree's older coordination snapshot.
2. NOTICES/W01-LANE-COMPLETION-OWNER-DIRECTION-20261127.md and this complete dispatch.
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-LANE-COMPLETION-r0-TASKS.json, SHA256 2a988f4150a8ade5b98397cf3ddd335aed0baa185ee0bfd30d9d025dece9bcf3. It contains every original requirement, file scope, negative test and acceptance criterion for all49tasks. Resolve any authoritative_reference to the complete original field in live task.md and verify its value_sha256. Read a task's complete specification before its phase, including READ-ONLY DEPENDENCIES; numbered PRECONDITIONS alone are not sufficient.
4. Frozen ARCHITECTURE/V2_1 precedence:00D, SCHEMAS/REGISTRY.json, ENCODING.md, CONTRACT_AUTHORITY.md, then the numbered docs cited by that task and its vectors. Read existing callers, exports, fixtures, migrations and test support before editing.

## Startup admission
- Verify worktree/branch/clean HEAD. Fetch main. Confirm main contains W01-LANE-COMPLETION in WRITE_SCOPES.json plus this owner campaign rule; if missing, hold source edits/commits and report TRUSTED_SCOPE_REGISTRATION_PENDING. Root's prepared coordination PR must be reviewed and merged first; never alter hooks/guard to work around it.
- Before source edits, fast-forward this still-clean prepared branch to the registered origin/main. Record the actual new BASE_SHA/TREE; prepared603640c is historical. Never reset/stash/force, or rebase returned/reviewed work.
- Verify G0/G1, configured lane caps and resource admission. Current local cap is1A2,2Cargo jobs,1build,20GiB free; re-read actual nonsecret values rather than copy defaults. Initial preparation reading exceeded20GiB, not a future guarantee. Source private .env.local without tracing/printing; serialize all expensive commands through WORKSPACE/run_single_build.py. Preserve shared services, caches and active other projects.

## Execute dependencies first
Use the exact graph/spec snapshot. Readiness controls execution, not a desire to fill all waves. Internal W01 predecessors may be SOURCE_VALIDATED_IN_BATCH only after clean signed commit, scope proof and their complete required positive/negative checks. Do not call them INTEGRATED or use them to unlock other lanes. Cross-lane prerequisites remain merged/accepted prerequisites. A2 changes no canonical coordination or peer lane files.

Conditional complete topological order if every external gate is met:
1. W01-T001, W01-T002, W01-T003, W01-T005, W01-T006, W01-T008, W01-T010, W01-T011, W01-T052.
2. W01-T012, W01-T013, W01-T015, W01-T018, W01-T022, W01-T026.
3. W01-T014, W01-T016, W01-T017, W01-T020, W01-T021, W01-T023, W01-T024, W01-T025, W01-T027, W01-T028.
4. W01-T019, W01-T029, W01-T030, W01-T032, W01-T033, W01-T034, W01-T035, W01-T036, W01-T037, W01-T038.
5. W01-T031, W01-T039, W01-T041, W01-T042, W01-T043, W01-T044, W01-T045, W01-T046.
6. W01-T040, W01-T047, W01-T048, W01-T049, W01-T050.
7. W01-T051.

Initial conservative available batch: T001,T002,T003,T005,T006,T008,T010,T011; then T015,T018,T022,T026. This is12tasks across parts of waves1–4, not four completed waves. Start high-unlock work such as T011 early. Keep the three API specs, CBOR, wire-vector double, shared test helpers and custody manifest/store within their original phase scopes. Then build the lease/effects/custody/runner doubles after their interfaces validate.

External gates: W06-T003 evidence schemas hold T012 and all claim-harness formats; W02-T000 final integration disposition holds T052; W02-T003 reducer double holds T020; W02-T004 interfaces hold T031; W05-T007 IdP holds T045. W02-T002 fake-GitHub functionality and fault-capability requirements also gate gateway/reconciliation tests. Verify latest peer ledgers/source; source stubs and a substitute local fake do not satisfy an unpublished peer contract. T013 also requires the retained T007 final disposition. Continue every other eligible item instead of pausing the entire campaign. Recheck peer progress at milestone boundaries, not in a busy loop.

Additional ordering: verify T009 RawAssertion before T011; T022 before T025; prefer T015 before T026 and T018 before T021. Verify each actual read-only/test capability before deciding a task is complete. Root/peer contract decisions are not invented by the author.

## Implementation and verification
Keep the smallest correct implementation, reuse existing helpers and dependencies, preserve frozen records, APIs, fail-closed checks, tenant isolation and explicit errors. No speculative frameworks, broad rewrites, fabricated passing evidence or weakening tests. BUILD claim harnesses run only DRY_RUN with synthetic fixtures/fakes and p0_executed:false; real RUN/G2/G3, devices/provider mutations and production remain unauthorized.
For each task make a short requirements→file→test checklist, implement one cohesive step, run the actual required checks and meaningful negative/guard-removal/restored controls, and retain exact argv/exit/HEAD/TREE/log hashes. Include DB/RLS, races/restarts/idempotence, parsers/signatures and one-use authority boundaries wherever required by that task. Use real local pinned PostgreSQL with isolated random test databases where specified. Never skip a required test because a stub compiles. Preserve applied migrations; scope/API or contract conflicts go to Root as bounded issues while independent eligible work continues.
Run focused checks after each meaningful change. At coherent milestones run fmt, workspace locked Clippy -D warnings, affected tests, cross-component checks and secret/hardcoding scans. At the publication boundary run the full required final command set once on exact final source; repeat only for a meaningful change/failure. A deterministic repeated error follows §0.1, not blind retries. Restore missing already-pinned Cargo packages through normal locked Cargo downloads with supported network permissions; the owner's earlier cache cleanup is an environment/cache fact, not a source defect. Missing manifest dependencies use the standing update route except T052's exact serialized exception. Do not expand its four-file authority into general manifest writes.
Keep a compact per-task ledger and evidence under your own target/w01-lane-completion-evidence; retain durable receipts externally if a build clean is needed, with Root-directed custody. Commit coherent validated milestones with task IDs and signatures. On context compaction/resume re-read that ledger/dispatch and verify pins; never lose deferred requirements or claim an entire wave is complete because one task passed.

## Completion, publication and stop conditions
- ALL_SOURCE_COMPLETE means all49tasks have their exact required implementation/positive/negative checks and integrated BUILD tests, no unresolved source defect, clean signed final SHA/TREE and scope/secret checks. It never means product/P0/operational acceptance.
- AVAILABLE_BATCH_COMPLETE means every currently eligible item is implemented and verified, and every remaining item has an exact external gate/owner/evidence pointer. This is a partial campaign, not all49done. Write the completed/blocked task matrix and minimal successor steps; do not invent missing peer code or stop after only the easiest task.
- After either genuine completed coherent batch, the owner explicitly authorizes git push of ws01/w01-lane-completion-r0 and one real PR targeting main, titled [W01-LANE-COMPLETION] Implement W01 security components. Verify no existing PR first, verify main's trusted write-set guard against the exact final SHA, preserve history/no-force and check the public body for private material. Describe only implemented tasks; list deferred external blockers. Update the same PR on continuation rather than opening duplicates. Mark independent A3 review pending; publication is not acceptance. No merge or auto-merge.
- Write /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-LANE-COMPLETION-r0-A2_RETURN.md with exact source/tests/receipts/resources/qualifications, task matrix and PR URL/head/CI status. A2 writes only its own RETURN in canonical coordination. Root arranges the fresh human-hosted independent exact-SHA A3 after author return; the author does not appoint/replace it or self-approve.
- STOP source writes on unresolved contract/security ambiguity, unsupported access/resource admission or repeated deterministic error; preserve evidence and continue only independent already-authorized work. If no eligible item remains, return the genuine completed-batch or exact BLOCKED state. No tests/implementation/roles were run by Root while preparing this dispatch.
