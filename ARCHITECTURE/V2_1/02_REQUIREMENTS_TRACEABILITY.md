# Requirements Traceability

| Requirement | Primary design | Invariant | Gate |
|---|---|---|---|
| Exact, current PR review | 08, 13, 30 | INV-02, INV-03 | P0-03 |
| Private source local by default | 10, 12, 24 | INV-05, INV-19 | P0-01, P0-02 |
| Rust backend and runner | 00F, 06, 10 | INV-20 | P0-02, P0-06 |
| No external queue/cache dependency | 07, 17, 32 | INV-01 | P0-08, P0-10 |
| Durable, idempotent work | 07, 19, 29 | INV-01, INV-13 | P0-08 |
| Safe model access | 11, 12, 22 | INV-05, INV-09 | P0-01 |
| Evidence-quality gating | 09, 13, 40 | INV-06, INV-07 | P0-07 |
| Exact human approval | 14, 15, 27 | INV-10, INV-11 | P0-09 |
| Confined publication | 08, 14, 27 | INV-04, INV-12 | P0-09 |
| Tenant isolation and transfer | 15, 18, 24 | INV-15, INV-16 | P0-10 |
| Budget enforcement | 07, 11, 29 | INV-09, INV-14 | P0-08 |
| Cross-platform execution | 10, 31, 47 | INV-08, INV-18 | P0-02, P0-05 |
| Protocol upgrade safety | 20, 21, 47 | INV-18 | P0-06, P0-10 |
| Revocation and recovery | 28, 35, 48 | INV-13, INV-17 | P0-09, P0-10 |
| Accessible and truthful UX | 16, 25-27 | INV-03, INV-10 | P0-09 |

The complete requirement-to-design expansion is in TRACEABILITY/REQUIREMENT_TO_DESIGN.md. A requirement is not considered satisfied merely because a component is named; the cited gate must demonstrate its security- or provider-dependent property.

