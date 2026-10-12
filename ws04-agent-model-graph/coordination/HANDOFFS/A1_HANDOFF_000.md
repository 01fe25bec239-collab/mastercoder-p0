FROM: PROGRAM-ARCHITECT (planning)
TO: W04-A1#1
MESSAGE_TYPE: A1_HANDOFF
HANDOFF_SEQUENCE: 000
DATE_UTC: 2026-10-03
WORKSTREAM: WORKSTREAM-04-AGENT-MODEL-GRAPH
CURRENT ARCHITECTURE VERSION: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1 (REGISTRY_SHA256 b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787; ARCH_IMPORT_COMMIT: record from memory.md; amendments: none)
CURRENT REPOSITORY SHA: none yet — repository created at G0 by master (COMMON/DAY0_RUNBOOK.md)
CURRENT BRANCH / WORKTREE: none
GIT STATUS: n/a
COMPLETED TASKS: none
ACCEPTED SHAS: none
OPEN TASKS: all tasks in task.md (NOT_STARTED)
CURRENT TASK: none
IN FLIGHT: none (board: top of task.md)
FAILED ATTEMPTS: none
REVIEW FINDINGS OPEN: none
KNOWN BLOCKERS: G0 and G1 must be recorded in memory.md before any dispatch; G2+G3 before any RUN.
CONTRACT-CHANGE REQUESTS: CCR-W04-001 (DRAFT — triage first), CCR-W04-002 (DRAFT — triage first)
INTERFACE STATE: provides I-11 (double W04-T006, real W04-T011 + W04-T015), I-15 fake model provider (W04-T004), graph fixtures for W05 (W04-T012), lease-bound dependency endpoint for IM-2 (W04-T029), endpoint policy crate for W03 LFS/submodule checks (W04-T027), dependency grants and staged packages for W03 offline install (W04-T029) (none published); consumes I-01/I-02 (W01), I-03 lease (W01), I-04 effects/journal (W01), I-13/I-14 (W06), fake-sandbox (W03), checkout broker for Git commit dependencies (W03-T016), local registries (W06-T034), registry/archive fixtures (W05-T012), canary-scanner (W06) (use doubles)
CLAIM STATE: P0-01A, P0-01B, P0-01C, P0-01D, P0-05E, P0-05F, P0-05G, P0-08B, P0-08C, P0-08D — NOT_RUN
IMPORTANT DECISIONS: claim and path ownership fixed by COMMON matrices; evidence lenses mapping in DEFINITION_OF_DONE §3.
EXACT NEXT ACTION: Verify G0/G1 in memory.md (else BLOCKED to master). Then dispatch, as r0 and in parallel, every task in wave 1 of task.md "Parallel waves" (see A2_WORKPLAN.md). As each task is INTEGRATED (ask the human to merge every ACCEPTED PR immediately), dispatch every task whose preconditions now hold, up to MC_MAX_PARALLEL_A2 (AGENTS.md §3a); the first one that unblocks other people is W04-T006-BROKER-DOUBLE — dispatch it the moment it is ready.
FILES THE NEXT A1 MUST READ FIRST:
  1. coordination/AGENTS.md
  2. coordination/SKILLS.md
  3. coordination/memory.md
  4. this handoff
  5. coordination/task.md
  6. ARCHITECTURE_MAP.md "MUST READ" rows
THINGS I AM NOT SURE ABOUT: provider/hardware availability for this workstream (see PRESTART_CHECKLIST.md Open items).
