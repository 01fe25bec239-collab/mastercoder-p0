# A2 work plan — WORKSTREAM-04-AGENT-MODEL-GRAPH (lane 04, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **34** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **10** |
| **Total planned A2 dispatches** | **44** |

Your work is finished when all 44 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 7. Tasks per wave: 2, 3, 8, 7, 6, 6, 2.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W04-T000 | WORKSPACE SCAFFOLD | W01-T000, W03-T000 |
| 1 | W04-T001 | MODEL CELLS DOC | G0/G1 only |
| 2 | W04-T002 | API FREEZE | W01-T004, W01-T011, W04-T000 |
| 2 | W04-T003 | MODEL TEST SUPPORT | W01-T007, W04-T000 |
| 2 | W04-T004 | FAKE MODEL PROVIDER | W04-T000 |
| 3 | W04-T005 | MODEL TURN VALIDATOR | W04-T002 |
| 3 | W04-T006 | BROKER DOUBLE | W04-T002 |
| 3 | W04-T007 | GRAPH ENGINE CORE | W04-T002, W04-T003 |
| 3 | W04-T008 | LEDGER CORE | W04-T002, W04-T003 |
| 3 | W04-T009 | SECRET DIALER | W04-T002, W04-T004 |
| 3 | W04-T026 | NPM RESOLVER | W04-T002 |
| 3 | W04-T027 | ENDPOINT POLICY | W04-T002 |
| 3 | W04-T029 | DEPENDENCY AUTHORITY | W01-T015, W04-T002, W04-T003 |
| 4 | W04-T010 | OPENAI COMPAT ADAPTER | W04-T001, W04-T004, W04-T005 |
| 4 | W04-T011 | MODEL BROKER INTENTS | W01-T015, W01-T018, W04-T003, W04-T005, W04-T009 |
| 4 | W04-T012 | GRAPH EVENTS REPLAY FIXTURES | W04-T007 |
| 4 | W04-T013 | LEDGER CANCELLATION SETTLEMENT | W04-T008 |
| 4 | W04-T014 | GRAPH AGENT RUNTIME | W04-T005, W04-T006, W04-T007 |
| 4 | W04-T028 | ACQUISITION DIALER | W01-T018, W04-T027, W06-T034 |
| 4 | W04-T030 | GIT TOOLCHAIN PINNING | W03-T016, W04-T026 |
| 5 | W04-T015 | BROKER HTTP WIRING | W04-T010, W04-T011 |
| 5 | W04-T018 | P0 08C HARNESS | W04-T011, W04-T013 |
| 5 | W04-T024 | ENTITLEMENT CHECKLIST | W04-T010 |
| 5 | W04-T031 | P0 05E HARNESS | W04-T026, W04-T028, W04-T029, W05-T012 |
| 5 | W04-T032 | P0 05F HARNESS | W04-T028, W05-T012 |
| 5 | W04-T033 | P0 05G HARNESS | W03-T016, W04-T030 |
| 6 | W04-T016 | GRAPH ROOT ENVELOPE | W04-T007, W04-T011, W04-T015 |
| 6 | W04-T017 | BROKER RESTART RECOVERY | W04-T011, W04-T013, W04-T015 |
| 6 | W04-T019 | P0 01A HARNESS | W04-T015 |
| 6 | W04-T020 | P0 01B HARNESS | W03-T006, W04-T015 |
| 6 | W04-T021 | P0 01C HARNESS | W04-T013, W04-T015 |
| 6 | W04-T025 | P0 01D HARNESS | W04-T024 |
| 7 | W04-T022 | P0 08B HARNESS | W04-T016 |
| 7 | W04-T023 | P0 08D HARNESS | W04-T017 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W04-RUN-P0-01A | G2 PASS; G3 signed for P0-01; W04-T019 INTEGRATED; integration milestone(s): IM-2, IM-3; Later explicit prototype authorization; disposable tenants/accounts and… |
| W04-RUN-P0-01B | G2 PASS; G3 signed for P0-01; W04-T020 INTEGRATED; integration milestone(s): IM-2; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W04-RUN-P0-01C | G2 PASS; G3 signed for P0-01; W04-T021 INTEGRATED; integration milestone(s): IM-2, IM-3; Later explicit prototype authorization; disposable tenants/accounts and… |
| W04-RUN-P0-01D | G2 PASS; G3 signed for P0-01; W04-T025 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W04-RUN-P0-05E | G2 PASS; G3 signed for P0-05; W04-T031 INTEGRATED; integration milestone(s): IM-2; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W04-RUN-P0-05F | G2 PASS; G3 signed for P0-05; W04-T032 INTEGRATED; integration milestone(s): IM-2; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W04-RUN-P0-05G | G2 PASS; G3 signed for P0-05; W04-T033 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W04-RUN-P0-08B | G2 PASS; G3 signed for P0-08; W04-T022 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W04-RUN-P0-08C | G2 PASS; G3 signed for P0-08; W04-T018 INTEGRATED; integration milestone(s): IM-3; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W04-RUN-P0-08D | G2 PASS; G3 signed for P0-08; W04-T023 INTEGRATED; integration milestone(s): IM-3; Later explicit prototype authorization; disposable tenants/accounts and synth… |
