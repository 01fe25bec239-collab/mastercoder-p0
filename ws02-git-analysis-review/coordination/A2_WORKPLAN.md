# A2 work plan — WORKSTREAM-02-GIT-ANALYSIS-REVIEW (lane 02, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **36** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **12** |
| **Total planned A2 dispatches** | **48** |

Your work is finished when all 48 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 8. Tasks per wave: 2, 4, 9, 6, 9, 1, 4, 1.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W02-T000 | WORKSPACE SCAFFOLD | W01-T000, W03-T000 |
| 1 | W02-T001 | STAGING CONFIG AS CODE | G0/G1 only |
| 2 | W02-T002 | FAKE GITHUB CORE | W02-T000 |
| 2 | W02-T003 | REDUCER TABLE DOUBLE | W02-T000 |
| 2 | W02-T004 | API FREEZE | W01-T004, W01-T011, W02-T000, W03-T003 |
| 2 | W02-T005 | GIT TEST SUPPORT | W01-T007, W02-T000 |
| 3 | W02-T006 | CHECKOUT GRANT DOUBLE | W02-T004 |
| 3 | W02-T007 | FAKE GITHUB FAULTS RECORDING | W02-T002 |
| 3 | W02-T008 | GITHUB CLIENT | W02-T002, W02-T004 |
| 3 | W02-T009 | GATE REDUCER | W02-T004 |
| 3 | W02-T010 | FINDINGS NORMALIZE | W02-T004 |
| 3 | W02-T011 | ANALYZER REGISTRY | W02-T004 |
| 3 | W02-T012 | ANALYSIS PLANNER PLAN | W02-T004 |
| 3 | W02-T013 | ORACLE VALIDATOR | W02-T004 |
| 3 | W02-T019 | P0 09E HARNESS | W02-T001, W02-T002 |
| 4 | W02-T014 | PROVIDER NORMALIZER | W02-T005, W02-T008 |
| 4 | W02-T015 | ANALYZER ADAPTERS | W02-T010, W02-T011, W03-T003, W05-T009 |
| 4 | W02-T016 | MANIFEST SEALING | W01-T015, W02-T005, W02-T012 |
| 4 | W02-T017 | VERIFICATION AUTHORITY | W02-T005, W02-T013 |
| 4 | W02-T018 | ELIGIBILITY AUTHORITY | W02-T005, W02-T008 |
| 4 | W02-T020 | P0 03B HARNESS | W02-T001, W02-T008 |
| 5 | W02-T021 | CHECK PROJECTION STORE | W02-T005, W02-T014 |
| 5 | W02-T022 | P0 03A HARNESS | W02-T014 |
| 5 | W02-T023 | P0 04A HARNESS | W02-T018 |
| 5 | W02-T024 | P0 04B HARNESS | W02-T007, W02-T018 |
| 5 | W02-T025 | P0 04C HARNESS | W02-T007, W02-T014, W02-T018 |
| 5 | W02-T026 | P0 07A HARNESS | W02-T009, W02-T016 |
| 5 | W02-T027 | P0 07C HARNESS | W02-T017, W03-T006 |
| 5 | W02-T028 | P0 07E HARNESS | W02-T017 |
| 5 | W02-T029 | CHECKOUT GRANT ISSUER | W01-T015, W02-T014 |
| 6 | W02-T030 | CHECK PROJECTION DISPATCH | W01-T018, W01-T020, W02-T020, W02-T021 |
| 7 | W02-T031 | P0 03C HARNESS | W02-T030 |
| 7 | W02-T032 | P0 03E HARNESS | W02-T030 |
| 7 | W02-T033 | EMERGENCY FREEZE | W02-T001, W02-T030 |
| 7 | W02-T034 | FAST WITHOUT MODELS SMOKE | W02-T009, W02-T014, W02-T015, W02-T016, W02-T030, W03-T006 |
| 8 | W02-T035 | P0 03D HARNESS | W02-T033 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W02-RUN-P0-03A | G2 PASS; G3 signed for P0-03; W02-T022 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W02-RUN-P0-03B | G2 PASS; G3 signed for P0-03; W02-T020 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W02-RUN-P0-03C | G2 PASS; G3 signed for P0-03; W02-T031 INTEGRATED; integration milestone(s): IM-3, IM-6; Later explicit prototype authorization; disposable tenants/accounts and… |
| W02-RUN-P0-03D | G2 PASS; G3 signed for P0-03; W02-T035 INTEGRATED; integration milestone(s): IM-3; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W02-RUN-P0-03E | G2 PASS; G3 signed for P0-03; W02-T032 INTEGRATED; integration milestone(s): IM-3; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W02-RUN-P0-04A | G2 PASS; G3 signed for P0-04; W02-T023 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W02-RUN-P0-04B | G2 PASS; G3 signed for P0-04; W02-T024 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W02-RUN-P0-04C | G2 PASS; G3 signed for P0-04; W02-T025 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W02-RUN-P0-07A | G2 PASS; G3 signed for P0-07; W02-T026 INTEGRATED; integration milestone(s): IM-6; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W02-RUN-P0-07C | G2 PASS; G3 signed for P0-07; W02-T027 INTEGRATED; integration milestone(s): IM-6; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W02-RUN-P0-07E | G2 PASS; G3 signed for P0-07; W02-T028 INTEGRATED; integration milestone(s): IM-6; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W02-RUN-P0-09E | G2 PASS; G3 signed for P0-09; W02-T019 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
