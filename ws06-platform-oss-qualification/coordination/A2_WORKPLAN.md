# A2 work plan — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION (lane 06, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **35** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **10** |
| **Total planned A2 dispatches** | **45** |

Your work is finished when all 45 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 4. Tasks per wave: 9, 13, 12, 1.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W06-T000 | ROOT HARDENING | G0/G1 only |
| 1 | W06-T001 | WS06 WORKSPACE SCAFFOLD | W01-T000, W02-T000, W03-T000, W04-T000 |
| 1 | W06-T002 | TESTDB I14 | G0/G1 only |
| 1 | W06-T003 | EVIDENCE SCHEMAS | G0/G1 only |
| 1 | W06-T004 | OSS GOVERNANCE DRAFTS | G0/G1 only |
| 1 | W06-T005 | STAGING ENVIRONMENT DOC | G0/G1 only |
| 1 | W06-T006 | SUPPLY CHAIN POLICY | G0/G1 only |
| 1 | W06-T033 | HOST LAB INVENTORY | G0/G1 only |
| 1 | W06-T034 | LOCAL SERVICES | G0/G1 only |
| 2 | W06-T007 | SUPPLY CHAIN CI | W06-T000, W06-T006 |
| 2 | W06-T008 | INDEPENDENCE AND REPRO CHECKS | W06-T000 |
| 2 | W06-T009 | EVIDENCE CLI | W06-T001, W06-T003 |
| 2 | W06-T011 | CLAIM LEDGER | W06-T001, W06-T003 |
| 2 | W06-T012 | CANARY SCANNER | W06-T001 |
| 2 | W06-T013 | OBSERVABILITY COLLECTOR | W06-T001 |
| 2 | W06-T014 | ARTIFACT STORE | W06-T001 |
| 2 | W06-T015 | PARITY RUNNER | W06-T001 |
| 2 | W06-T019 | ADMIN DOMAIN IAC | W06-T001, W06-T005 |
| 2 | W06-T020 | LOAD GENERATOR AND CHECKERS | W01-T015, W01-T018, W02-T003, W04-T006, W06-T001, W06-T002 |
| 2 | W06-T021 | UPGRADE ORCHESTRATOR | W01-T023, W03-T022, W06-T001 |
| 2 | W06-T022 | FAILOVER RIG | W01-T015, W01-T018, W06-T001, W06-T002 |
| 2 | W06-T023 | TRANSFER ORCHESTRATOR | W01-T015, W01-T022, W02-T007, W02-T018, W04-T006, W06-T001 |
| 3 | W06-T010 | EVIDENCE STORE | W06-T009 |
| 3 | W06-T016 | P0 06A HARNESS | W06-T015 |
| 3 | W06-T017 | P0 06B HARNESS | W06-T015 |
| 3 | W06-T018 | FREEZE DOSSIER | W06-T011 |
| 3 | W06-T024 | P0 10C HARNESS | W01-T022, W06-T014 |
| 3 | W06-T025 | P0 10D HARNESS | W06-T012, W06-T013 |
| 3 | W06-T026 | P0 07D HARNESS | W06-T019 |
| 3 | W06-T027 | P0 10E HARNESS | W06-T020 |
| 3 | W06-T028 | P0 10F HARNESS | W06-T021 |
| 3 | W06-T029 | P0 10G HARNESS | W06-T022 |
| 3 | W06-T030 | RESTORE RIG | W01-T017, W06-T022 |
| 3 | W06-T031 | P0 10B HARNESS | W06-T023 |
| 4 | W06-T032 | P0 10H HARNESS | W06-T030 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W06-RUN-P0-06A | G2 PASS; G3 signed for P0-06; W06-T016 INTEGRATED; integration milestone(s): IM-1; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-06B | G2 PASS; G3 signed for P0-06; W06-T017 INTEGRATED; integration milestone(s): IM-1; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-07D | G2 PASS; G3 signed for P0-07; W06-T026 INTEGRATED; integration milestone(s): IM-8; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10B | G2 PASS; G3 signed for P0-10; W06-T031 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10C | G2 PASS; G3 signed for P0-10; W06-T024 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W06-RUN-P0-10D | G2 PASS; G3 signed for P0-10; W06-T025 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10E | G2 PASS; G3 signed for P0-10; W06-T027 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10F | G2 PASS; G3 signed for P0-10; W06-T028 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10G | G2 PASS; G3 signed for P0-10; W06-T029 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W06-RUN-P0-10H | G2 PASS; G3 signed for P0-10; W06-T032 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
