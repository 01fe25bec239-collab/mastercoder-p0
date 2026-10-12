<!-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure) -->
## [WS03][TASK_ID] <title>

| Field | Value |
|---|---|
| Workstream | WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION |
| Task | TASK_ID / ATTEMPT |
| BASE_SHA | 40-hex |
| ACCEPTED_SHA (A3-reviewed) | 40-hex — must be this PR's head or an ancestor reachable after merge |
| TREE | 40-hex |
| A2 return | `wsNN-*/coordination/RETURNS/<TASK_ID>-<ATTEMPT>-A2_RETURN.md` |
| A3 review (verdict) | `wsNN-*/coordination/REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REVIEW.md` (PASS / PASS_WITH_NONBLOCKING_FINDINGS) |
| P0 claims touched | none / P0-… (BUILD only; claim status stays NOT_RUN unless evidence ACCEPTED) |
| Contracts used | registered types / docs |
| CCRs | none / CCR-… |

## Checklist (all must be true)
- [ ] Only paths owned by this workstream changed (CODEOWNERS)
- [ ] No secrets (gitleaks clean); no real customer data; canaries only
- [ ] Every new file carries the PROTOTYPE header
- [ ] Negative tests listed in task.md exist and pass
- [ ] No schema/enum/digest/authority/state invented; ARCHITECTURE/ untouched
- [ ] `python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py` still reports `p0_executed:false`
- [ ] Merge method: **merge commit** (keeps the reviewed SHA reachable). No squash, no rebase.
