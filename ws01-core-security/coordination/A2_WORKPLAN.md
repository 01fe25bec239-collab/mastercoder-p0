# A2 work plan — WORKSTREAM-01-CORE-SECURITY-AUTHORITY (lane 01, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **53** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **13** |
| **Total planned A2 dispatches** | **66** |

Your work is finished when all 66 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 8. Tasks per wave: 4, 8, 3, 14, 10, 8, 5, 1.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W01-T000 | WORKSPACE SCAFFOLD | G0/G1 only |
| 1 | W01-T001 | AA API OPENAPI | G0/G1 only |
| 1 | W01-T002 | RESOLUTION API OPENAPI | G0/G1 only |
| 1 | W01-T003 | RUNNER CONTROL PROTOCOL SPEC | G0/G1 only |
| 2 | W01-T004 | CONTRACT TYPES CODEGEN | W01-T000 |
| 2 | W01-T005 | CBOR CORE | W01-T000 |
| 2 | W01-T006 | WIRE VECTOR DOUBLE | W01-T000 |
| 2 | W01-T007 | DB ROLES SCHEMA | W01-T000 |
| 2 | W01-T008 | TEST SUPPORT | W01-T000 |
| 2 | W01-T009 | WEBAUTHN VERIFY | W01-T000 |
| 2 | W01-T010 | CUSTODY MANIFEST STORE | W01-T000 |
| 2 | W01-T052 | CROSS WORKSPACE DEPS | W01-T000, W02-T000 |
| 3 | W01-T011 | API FREEZE | W01-T004 |
| 3 | W01-T012 | WIRE CANONICAL | W01-T004, W01-T005, W06-T003 |
| 3 | W01-T013 | P0 10A HARNESS | W01-T007, W01-T008 |
| 4 | W01-T014 | LEASE CORE | W01-T007, W01-T011, W01-T012 |
| 4 | W01-T015 | LEASE DOUBLE | W01-T011 |
| 4 | W01-T016 | EFFECTS STORE | W01-T007, W01-T011, W01-T012 |
| 4 | W01-T017 | JOURNAL WITNESS | W01-T011, W01-T012 |
| 4 | W01-T018 | EFFECTS JOURNAL DOUBLE | W01-T011 |
| 4 | W01-T020 | GATE AUTHORITY DOUBLE | W01-T011, W01-T012, W01-T052, W02-T003 |
| 4 | W01-T021 | CUSTODY PINS | W01-T007, W01-T010, W01-T011, W01-T012 |
| 4 | W01-T022 | CUSTODY DOUBLE | W01-T011 |
| 4 | W01-T023 | VERSION NEGOTIATION | W01-T003, W01-T011, W01-T012 |
| 4 | W01-T024 | P0 06C HARNESS | W01-T012 |
| 4 | W01-T025 | AA PREPARE | W01-T001, W01-T007, W01-T011, W01-T012 |
| 4 | W01-T026 | RUNNER CONTROL DOUBLE | W01-T003, W01-T011 |
| 4 | W01-T027 | RESOLUTION CORE | W01-T002, W01-T007, W01-T009, W01-T011, W01-T012 |
| 4 | W01-T028 | CREDENTIAL ENROLLMENT | W01-T007, W01-T009, W01-T011, W01-T012 |
| 5 | W01-T019 | EVIDENCE INGESTION | W01-T007, W01-T011, W01-T012, W01-T014, W01-T015 |
| 5 | W01-T029 | RUNNER CONTROL SERVER | W01-T014, W01-T023 |
| 5 | W01-T030 | EFFECT DISPATCH | W01-T016, W01-T017 |
| 5 | W01-T032 | AA FINISH | W01-T009, W01-T025, W01-T028 |
| 5 | W01-T033 | PLAN VERIFIER | W01-T021 |
| 5 | W01-T034 | P0 09F HARNESS | W01-T021 |
| 5 | W01-T035 | RESOLUTION FLOWS | W01-T016, W01-T027 |
| 5 | W01-T036 | CREDENTIAL OPERATIONS | W01-T028 |
| 5 | W01-T037 | EPOCH FENCE DENY | W01-T017 |
| 5 | W01-T038 | P0 06D HARNESS | W01-T023 |
| 6 | W01-T031 | GATE COMMIT | W01-T015, W01-T019, W01-T052, W02-T004 |
| 6 | W01-T039 | P0 08A HARNESS | W01-T029 |
| 6 | W01-T041 | P0 09C HARNESS | W01-T028, W01-T032 |
| 6 | W01-T042 | PUBLICATION PLANNER | W01-T033 |
| 6 | W01-T043 | GIT GATEWAY | W01-T030, W01-T032, W01-T033 |
| 6 | W01-T044 | P0 10J HARNESS | W01-T035 |
| 6 | W01-T045 | P0 09B HARNESS | W01-T036, W05-T007 |
| 6 | W01-T046 | FRESH BOOTSTRAP FLOW | W01-T036, W01-T037 |
| 7 | W01-T040 | P0 07B HARNESS | W01-T031 |
| 7 | W01-T047 | GATEWAY RECONCILE | W01-T043 |
| 7 | W01-T048 | P0 09D HARNESS | W01-T042, W01-T043 |
| 7 | W01-T049 | P0 09G HARNESS | W01-T043 |
| 7 | W01-T050 | P0 10I HARNESS | W01-T046 |
| 8 | W01-T051 | P0 09H HARNESS | W01-T047 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W01-RUN-P0-06C | G2 PASS; G3 signed for P0-06; W01-T024 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W01-RUN-P0-06D | G2 PASS; G3 signed for P0-06; W01-T038 INTEGRATED; integration milestone(s): IM-2; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-07B | G2 PASS; G3 signed for P0-07; W01-T040 INTEGRATED; integration milestone(s): IM-6; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-08A | G2 PASS; G3 signed for P0-08; W01-T039 INTEGRATED; integration milestone(s): IM-2; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-09B | G2 PASS; G3 signed for P0-09; W01-T045 INTEGRATED; integration milestone(s): IM-4; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-09C | G2 PASS; G3 signed for P0-09; W01-T041 INTEGRATED; integration milestone(s): IM-4; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-09D | G2 PASS; G3 signed for P0-09; W01-T048 INTEGRATED; integration milestone(s): IM-5; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-09F | G2 PASS; G3 signed for P0-09; W01-T034 INTEGRATED; integration milestone(s): IM-5; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-09G | G2 PASS; G3 signed for P0-09; W01-T049 INTEGRATED; integration milestone(s): IM-5; CCR-W01-001 (audit F-23) RELEASED or REJECTED by the Program Contract Board; … |
| W01-RUN-P0-09H | G2 PASS; G3 signed for P0-09; W01-T051 INTEGRATED; integration milestone(s): IM-5; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-10A | G2 PASS; G3 signed for P0-10; W01-T013 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
| W01-RUN-P0-10I | G2 PASS; G3 signed for P0-10; W01-T050 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synth… |
| W01-RUN-P0-10J | G2 PASS; G3 signed for P0-10; W01-T044 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and syntheti… |
