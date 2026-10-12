# Open Risks

| Risk | Severity | State | Containment / exit |
|---|---|---|---|
| R-001 Safe Codex/Claude mode may be inexpressible or terms-incompatible | Critical | OPEN | Disable failing cells; P0-01 |
| R-002 Cross-OS S2/S3 containment may be unavailable/costly | Critical | OPEN | Disable OS/tier or require S3; P0-02 |
| R-003 Git provider may not expose exact ordinary/queue candidate truth for all merge modes | Critical | OPEN | Support only proven cells; P0-03 |
| R-004 Current author/membership proof may require excess provider permission | High | OPEN | Disable automation mode; P0-04 |
| R-005 Bounded checkout may not support declared monorepo/LFS/submodule cells | High | OPEN | Limit features/envelopes; P0-05 |
| R-006 Independent canonicalizers may diverge | Critical | OPEN | No protocol release until P0-06 |
| R-007 Runner/agent may manufacture apparently strong evidence | Critical | OPEN | Restrict C/E cells; P0-07 |
| R-008 Provider billing can arrive after cancellation and exceed reservation | High | OPEN | Late settlement/debt; lower request caps; P0-08 |
| R-009 WebAuthn/publisher ceremony may contain a retarget/revocation/ambiguity gap | Critical | OPEN | Export-only patches; P0-09 |
| R-010 RLS/transfer/backup/artifact isolation or DR may fail | Critical | OPEN | No production; P0-10 |
| R-011 Database compromise controls internal truth | Critical | RESIDUAL | Separation of external authorities, backups, detection, recovery |
| R-012 Privileged broker or provider App compromise | Critical | RESIDUAL | Least permissions, positive operations, egress, generations, monitoring |
| R-013 Hypervisor/kernel zero-day | Critical | RESIDUAL | S3 for high risk, patch cadence, no secrets, blast-radius limits |
| R-014 Authorized model channel leaks source through prompt | High | RESIDUAL | Policy/disclosure/minimization; cannot be fully prevented by broker |
| R-015 Collusion across claimed independent domains | High | RESIDUAL | Administrative C-class definition and C4 where required |
| R-016 Immutable audit conflicts with later deletion request | Medium | RESIDUAL | No raw data in facts; separate erasable artifacts and legal policy |
| R-017 Single write region outage | High | RESIDUAL | Backups/warm recovery; correctness-first RTO |
| R-018 Provider semantic drift after qualification | High | OPEN | Evidence expiry and requalification triggers |

No OPEN Critical risk is accepted for production. Residual means the architecture narrows and discloses the risk; operations and product policy must decide whether the remaining exposure fits a production profile after gates pass.


## V2.1 residuals and gates

Credential-bearing gateway or actual provider-token compromise retains provider-granted mutation authority; planner confinement is a different claim (53). Internal check revocation has an external unsafe window until provider withdrawal/freeze confirmation (54). Browser/OS/AA-origin compromise remains an approval endpoint threat (52). Two journal copies require independent surviving failure domains; loss of both is outside the guarantee (50). DB rollback requires full fresh independently reviewed security bootstrap and may exceed infrastructure RTO (35). Exact local patch custody availability limits publication; losing bytes never permits alternate reconstruction. These are explicit limits, not undisclosed exceptions to PASS.
