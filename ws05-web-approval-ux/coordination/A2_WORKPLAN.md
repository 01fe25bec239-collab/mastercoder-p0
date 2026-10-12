# A2 work plan — WORKSTREAM-05-WEB-APPROVAL-UX (lane 05, operator: master)

Generated from `task.md` by the program generator. Do not edit by hand.

## The exact count

| | A2 tasks |
|---|---|
| BUILD tasks (write code, tests and claim harnesses; allowed after G1) | **37** |
| RUN tasks (execute one P0 claim and submit evidence; allowed only after G2 + G3) | **1** |
| **Total planned A2 dispatches** | **38** |

Your work is finished when all 38 are ACCEPTED (BUILD tasks also merged = INTEGRATED; RUN tasks = claim ACCEPTED by two evidence reviewers with an honest outcome).

Each task is one A2 session: every task in `task.md` already states the objective, exact files it may touch, the interfaces it uses and publishes, the steps, the tests, the negative tests and the acceptance criteria. A1 copies it into a DISPATCH and does not need to plan anything.

**The only extra A2 sessions** are repairs: if an A3 review returns REPAIR_REQUIRED, the same task gets one more A2 (attempt r1, r2 …) with the reviewer's findings. That is not new work; it is the same task being fixed, and it cannot be known in advance. Dependency requests (AGENTS.md §3a) are the only other extra dispatch.

Every A2 RETURN is reviewed by one fresh A3. Every RUN task is additionally reviewed by two evidence reviewers from other workstreams.

## Parallel waves (BUILD)

A task starts as soon as **its own** preconditions are INTEGRATED (merged to main); it does not wait for the whole previous wave. Ask the human to merge each ACCEPTED PR straight away: every merge unblocks the next tasks. Up to `MC_MAX_PARALLEL_A2` (`.env`) run at once. Waves: 7. Tasks per wave: 1, 15, 5, 11, 1, 3, 1.

| WAVE | TASK | NAME | NEEDS |
|---|---|---|---|
| 1 | W05-T000 | WORKSPACE SCAFFOLD | G0/G1 only |
| 2 | W05-T001 | CONTRACT TYPES TS | W05-T000 |
| 2 | W05-T002 | STATE LANGUAGE | W05-T000 |
| 2 | W05-T003 | WIRE TS CBOR CORE | W05-T000 |
| 2 | W05-T004 | TRUSTED ORIGIN KIT | W05-T000 |
| 2 | W05-T005 | EXTERNAL AUDIT PACKET | W05-T000 |
| 2 | W05-T006 | G047 RESOLUTION ORIGIN TRIAGE | W05-T000 |
| 2 | W05-T007 | TEST IDP | W05-T000 |
| 2 | W05-T008 | WEBAUTHN CROSSCHECK FIXTURES | W05-T000 |
| 2 | W05-T009 | SYNTHETIC VULNERABLE REPOS | W05-T000 |
| 2 | W05-T010 | PATH FIXTURES | W05-T000 |
| 2 | W05-T011 | PACK FIXTURES | W05-T000 |
| 2 | W05-T012 | REGISTRY AND ARCHIVE FIXTURES | W05-T000 |
| 2 | W05-T013 | AA API MOCK SERVER | W01-T001, W05-T000 |
| 2 | W05-T014 | RESOLUTION API MOCK SERVER | W01-T002, W05-T000 |
| 2 | W05-T036 | ESCAPE CORPUS | W05-T000 |
| 3 | W05-T015 | WIRE TS CANONICAL | W05-T003, W06-T003 |
| 3 | W05-T016 | ORDINARY API MOCKS | W05-T001 |
| 3 | W05-T017 | AA ORIGIN SHELL | W05-T002, W05-T004, W05-T013 |
| 3 | W05-T018 | RESOLUTION ORIGIN SHELL | W05-T002, W05-T004, W05-T006, W05-T014 |
| 3 | W05-T019 | WEBAUTHN HARNESS CI | W05-T004 |
| 4 | W05-T020 | REVIEW COVERAGE VIEW | W05-T002, W05-T016 |
| 4 | W05-T021 | REVIEW FINDINGS VIEW | W05-T002, W05-T016 |
| 4 | W05-T022 | REVIEW PROJECTION PANEL | W05-T002, W05-T016 |
| 4 | W05-T023 | ATTEMPT VIEW | W05-T002, W05-T016 |
| 4 | W05-T024 | GRAPH VIEW | W05-T002, W05-T016 |
| 4 | W05-T025 | SETUP VIEW | W05-T002, W05-T016 |
| 4 | W05-T026 | PATCH VIEW | W05-T002, W05-T016 |
| 4 | W05-T027 | ADMIN VIEW | W05-T002, W05-T016 |
| 4 | W05-T028 | DISCLOSURE BUDGET VIEW | W05-T002, W05-T016 |
| 4 | W05-T029 | AA DIFF RENDERER | W05-T017 |
| 4 | W05-T030 | RESOLUTION FLOWS UI | W05-T018, W05-T019 |
| 5 | W05-T031 | AA CEREMONY | W05-T019, W05-T029 |
| 6 | W05-T032 | AUTHENTICATOR MATRIX | W05-T019, W05-T031 |
| 6 | W05-T033 | P0 09A HARNESS | W05-T031 |
| 6 | W05-T034 | A11Y AUTOMATED | W05-T020, W05-T021, W05-T022, W05-T023, W05-T024, W05-T025, W05-T026, W05-T027, W05-T028, W05-T030, W05-T031 |
| 7 | W05-T035 | A11Y MANUAL | W05-T034 |

## RUN tasks (after G2 external audit PASS and G3 for the claim family)

| TASK | PRECONDITIONS |
|---|---|
| W05-RUN-P0-09A | G2 PASS; G3 signed for P0-09; W05-T033 INTEGRATED; integration milestone(s): IM-4; Later explicit prototype authorization; disposable tenants/accounts and synth… |
