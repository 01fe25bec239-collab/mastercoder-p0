# Requirement to Design

| Requirement | Design records | Owner | Validation |
|---|---|---|---|
| Frozen Rust backend | 00F, 01, 06, ADR-001 | A2 Rust Control Plane | P0-06/P0-10 |
| PostgreSQL sole authority/workflow | 00F, 07, 18, 19, 29, ADR-002 | A2 Persistence | P0-08/P0-10 |
| Next.js/React/TypeScript web | 00F, 16 | A2 Web UX | UX/accessibility + P0-09 |
| Rust customer runner | 10, 31, ADR-015 | A2 Runner Sandbox | P0-02/P0-05 |
| No required authoritative cache | 00F, 07, 09 | Control/Persistence | P0-10 |
| Exact merge gate | 08, 13, 25, 30, ADR-004 | A2 Git/Review | P0-03 |
| Safe checkout | 08, 10, 30 | A2 Git/Runner | P0-05 |
| Safe model tools | 11, 12, ADR-008/009 | Harness/Model | P0-01/P0-08 |
| Hostile execution containment | 10, 22, 23, ADR-007 | Runner/Threat | P0-02 |
| Qualified evidence | 09, 13, ADR-006 | Analysis/Review | P0-07 |
| Stable eligibility | 15, ADR-010 | Identity/Git | P0-04 |
| Human-only exact approval | 14-16, 27, ADR-011 | Patch/Identity/Web | P0-09 |
| Confined exact publication | 08, 14, 27, ADR-003/012 | Patch/Git | P0-09 |
| Budget/fencing/idempotency | 07, 19, 29, ADR-009/014 | Persistence/Model | P0-08 |
| Tenant isolation/transfer/deletion | 15, 18, 24, ADR-013 | Identity/Persistence | P0-10 |
| Privacy/audit truth | 24, 34, 36 | Threat/Reliability | P0-10 |
| Cross-platform honesty | 10, 31, 47, ADR-015 | Runner | P0-02/P0-05 |
| Recovery/revocation/skew | 28, 35, 47, 48, ADR-014 | Reliability/Identity | P0-06/P0-09/P0-10 |
| Complete audit adjudication | 00C, FINDING_TO_DECISION | Threat/Requirements | A4 Requirements |
| No implementation authorization | 00A, 00G, 38, 39, 49 | Root architecture | Status check |

