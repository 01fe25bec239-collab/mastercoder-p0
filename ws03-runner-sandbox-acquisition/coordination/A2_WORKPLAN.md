# A2 work plan — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION (lane 03, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **34** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **17** |
| **Total planned A2 dispatches** | **51** |

Your work is finished when all 51 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 6. Tasks per wave: 1, 3, 7, 9, 7, 7.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W03-T000 | WORKSPACE SCAFFOLD | W01-T000 |
| 2 | W03-T003 | API FREEZE | W01-T004, W01-T011, W03-T000 |
| 2 | W03-T004 | RUNNER TEST SUPPORT | W01-T007, W03-T000 |
| 2 | W03-T044 | CROSS WORKSPACE DEPS | W02-T000, W03-T000, W04-T000 |
| 3 | W03-T006 | FAKE SANDBOX DOUBLE | W03-T003 |
| 3 | W03-T007 | RUNNER ENROLLMENT CAPABILITY | W03-T003 |
| 3 | W03-T008 | EGRESS PROXY | W03-T003 |
| 3 | W03-T009 | PACK PARSER | W03-T003, W05-T011 |
| 3 | W03-T010 | CAPABILITY MATCH | W03-T003 |
| 3 | W03-T013 | LINUX BACKEND DECISION | W03-T003, W06-T033 |
| 3 | W03-T016 | CHECKOUT DIALER | W01-T018, W02-T006, W03-T003, W03-T044, W06-T034 |
| 4 | W03-T014 | RUNNER LEASE LOOP | W01-T026, W03-T006, W03-T007 |
| 4 | W03-T015 | LINUX S2 BACKEND | W03-T004, W03-T008, W03-T013 |
| 4 | W03-T019 | MACOS BACKEND | W03-T008, W05-T036, W06-T033 |
| 4 | W03-T020 | WINDOWS BACKEND | W03-T008, W05-T036, W06-T033 |
| 4 | W03-T021 | P0 02D HARNESS | W03-T009, W03-T010, W05-T010 |
| 4 | W03-T023 | P0 05A HARNESS | W03-T009, W03-T016, W05-T011 |
| 4 | W03-T024 | P0 05B HARNESS | W03-T009, W03-T016, W05-T011 |
| 4 | W03-T025 | LFS | W03-T016, W03-T044, W04-T027 |
| 4 | W03-T026 | SUBMODULES | W03-T016 |
| 5 | W03-T022 | FAKE RUNNER DOUBLE | W03-T014 |
| 5 | W03-T029 | P0 02A HARNESS LINUX | W03-T015, W05-T036 |
| 5 | W03-T030 | P0 02B HARNESS LINUX | W03-T015 |
| 5 | W03-T031 | P0 02C HARNESS LINUX | W03-T015 |
| 5 | W03-T033 | OFFLINE INSTALL | W03-T015, W04-T029, W05-T012 |
| 5 | W03-T034 | P0 05C HARNESS | W03-T025 |
| 5 | W03-T035 | P0 05D HARNESS | W03-T026 |
| 6 | W03-T036 | P0 02A HARNESS MACOS | W03-T019, W03-T029 |
| 6 | W03-T037 | P0 02B HARNESS MACOS | W03-T019, W03-T030 |
| 6 | W03-T038 | P0 02C HARNESS MACOS | W03-T019, W03-T031 |
| 6 | W03-T039 | P0 02A HARNESS WINDOWS | W03-T020, W03-T029 |
| 6 | W03-T040 | P0 02B HARNESS WINDOWS | W03-T020, W03-T030 |
| 6 | W03-T041 | P0 02C HARNESS WINDOWS | W03-T020, W03-T031 |
| 6 | W03-T043 | P0 05H HARNESS | W03-T033 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W03-RUN-P0-02A-LINUX | G2 PASS; G3 signed for P0-02; W03-T029 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02A-MACOS | G2 PASS; G3 signed for P0-02; W03-T036 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02A-WINDOWS | G2 PASS; G3 signed for P0-02; W03-T039 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02B-LINUX | G2 PASS; G3 signed for P0-02; W03-T030 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02B-MACOS | G2 PASS; G3 signed for P0-02; W03-T037 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02B-WINDOWS | G2 PASS; G3 signed for P0-02; W03-T040 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02C-LINUX | G2 PASS; G3 signed for P0-02; W03-T031 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02C-MACOS | G2 PASS; G3 signed for P0-02; W03-T038 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02C-WINDOWS | G2 PASS; G3 signed for P0-02; W03-T041 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02D-LINUX | G2 PASS; G3 signed for P0-02; W03-T021 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02D-MACOS | G2 PASS; G3 signed for P0-02; W03-T021 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-02D-WINDOWS | G2 PASS; G3 signed for P0-02; W03-T021 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-05A | G2 PASS; G3 signed for P0-05; W03-T023 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-05B | G2 PASS; G3 signed for P0-05; W03-T024 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-05C | G2 PASS; G3 signed for P0-05; W03-T034 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-05D | G2 PASS; G3 signed for P0-05; W03-T035 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W03-RUN-P0-05H | G2 PASS; G3 signed for P0-05; W03-T043 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
