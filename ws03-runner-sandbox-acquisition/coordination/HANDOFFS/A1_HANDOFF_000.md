FROM: PROGRAM-ARCHITECT (planning)
TO: W03-A1#1
MESSAGE_TYPE: A1_HANDOFF
HANDOFF_SEQUENCE: 000
DATE_UTC: 2026-10-03
WORKSTREAM: WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION
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
CONTRACT-CHANGE REQUESTS: none
INTERFACE STATE: provides I-17 types (W03-T003), fake-sandbox (W03-T006) and fake-runner (W03-T022) doubles, runner client for IM-2 (W03-T014) (none published); consumes I-01/I-02 (W01), I-03/I-09 lease + Runner Control (W01), I-04 effects (W01) for CHECKOUT_FETCH/DEPENDENCY_FETCH, checkout-grant double (W02), I-11 model endpoint audience (W04) for sandbox allowlist, endpoint policy + dependency grants (W04-T027/T029), escape corpus (W05-T036), host-lab inventory + local Git server (W06-T033/T034), I-13/I-14 (W06), canary-scanner (W06) (use doubles)
CLAIM STATE: P0-02A, P0-02B, P0-02C, P0-02D, P0-05A, P0-05B, P0-05C, P0-05D, P0-05H — NOT_RUN
IMPORTANT DECISIONS: claim and path ownership fixed by COMMON matrices; evidence lenses mapping in DEFINITION_OF_DONE §3.
EXACT NEXT ACTION: Verify G0/G1 in memory.md (else BLOCKED to master). Then dispatch, as r0 and in parallel, every task in wave 1 of task.md "Parallel waves" (see A2_WORKPLAN.md). As each task is INTEGRATED (ask the human to merge every ACCEPTED PR immediately), dispatch every task whose preconditions now hold, up to MC_MAX_PARALLEL_A2 (AGENTS.md §3a); the first one that unblocks other people is W03-T003-API-FREEZE — dispatch it the moment it is ready.
FILES THE NEXT A1 MUST READ FIRST:
  1. coordination/AGENTS.md
  2. coordination/SKILLS.md
  3. coordination/memory.md
  4. this handoff
  5. coordination/task.md
  6. ARCHITECTURE_MAP.md "MUST READ" rows
THINGS I AM NOT SURE ABOUT: provider/hardware availability for this workstream (see PRESTART_CHECKLIST.md Open items).
