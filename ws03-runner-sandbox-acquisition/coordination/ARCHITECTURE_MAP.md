# ARCHITECTURE MAP — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION

The complete V2.1 package is in `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/` (and `repo/ARCHITECTURE/V2_1/`). Everyone reads the "ALL" rows; this workstream must additionally master the "MUST READ" rows.

| Priority | File | Why |
|---|---|---|
| ALL | `00_READ_ME_FIRST.md`, `00A_V2_1_FINAL_STATUS.md`, `00E_ARCHITECTURE_FREEZE_STATUS.md`, `00F_ATOMIC_PROTOTYPE_STATUS.md` | status you must never misstate |
| ALL | `00D_V2_1_INVARIANTS.md` | INV-01..22 override everything |
| ALL | `00F_TECHNOLOGY_STACK.md` | frozen stack and exclusions |
| ALL | `05_DEPENDENCY_DAG.md`, `39_WORK_ITEM_DAG.md`, `42_PROTOTYPE_GATES.md` | FAST independence, gates |
| ALL | `49_CANONICAL_PROTOCOL_GLOSSARY.md` | canonical names, no aliases |
| ALL | `SCHEMAS/ENCODING.md`, `SCHEMAS/CONTRACT_AUTHORITY.md` | wire + authority |
| ALL | `57_ATOMIC_PROTOTYPE_CLAIMS.md` (your claims) | evidence contracts |
| MUST READ | `10_RUNNER_SANDBOX.md` | runner protocol, tiers, host protections, receipts, cancellation |
| MUST READ | `31_CROSS_PLATFORM.md` | capability dimensions, matrix, scheduling, admin independence |
| MUST READ | `55_DEPENDENCY_ACQUISITION.md` | grants, resolver/dialer split, classes, storage |
| MUST READ | `08_GIT_INTEGRATION.md` | checkout grants, per-call AcquisitionRequestV1 |
| MUST READ | `22_TRUST_BOUNDARIES.md` | Checkout→Parser, Repository→Sandbox |
| MUST READ | `23_THREAT_MODEL.md` | malicious PR author, shared kernel |
| MUST READ | `36_SUPPLY_CHAIN.md` | tool registry, dependency trust |
| MUST READ | `50_EXTERNAL_EFFECT_RECOVERY.md` | per-call effects for fetches |
| MUST READ | `SCHEMAS (AttemptBindingV2, AttemptEventV2, CapabilityProfileV1, CheckoutGrantV2, AcquisitionRequestV1, DependencyAcquisitionGrantV1, DependencyPackageV1, GitOidV1)` | exact wire |
| MUST READ | `SCHEMAS/ENCODING.md` | Git path byte rules |
| MUST READ | `ADRs/ADR-007,015,026` | decisions |
| MUST READ | `45_UNRESOLVED_DECISIONS.md` | U-001, U-004 |
| MUST READ | `00D_V2_1_INVARIANTS.md` | INV-05,08,18,19 |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
