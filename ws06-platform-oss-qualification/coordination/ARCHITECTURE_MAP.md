# ARCHITECTURE MAP — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION

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
| MUST READ | `17_PLATFORM_RELIABILITY.md` | objectives, failure behavior |
| MUST READ | `32_DEPLOYMENT_TOPOLOGY.md` | roles, network policy, regions |
| MUST READ | `33_SCALING_CAPACITY.md` | load envelope |
| MUST READ | `34_OBSERVABILITY.md` | signals, audit vs telemetry, alerts |
| MUST READ | `35_DISASTER_RECOVERY.md` | restore procedure, acceptance record |
| MUST READ | `36_SUPPLY_CHAIN.md` | build/release, dependency policy |
| MUST READ | `40_TEST_ARCHITECTURE.md` | layers, determinism, release evidence |
| MUST READ | `41_SECURITY_VALIDATION.md` | domains, independence, stop conditions |
| MUST READ | `42_PROTOTYPE_GATES.md` | family computation, inapplicability |
| MUST READ | `47_COMPATIBILITY_MATRIX.md` | N/N-1, upgrade order, expiry |
| MUST READ | `57_ATOMIC_PROTOTYPE_CLAIMS.md + TRACEABILITY/atomic-prototype-claims.json` | all claims |
| MUST READ | `24_PRIVACY_DATA_CLASSIFICATION.md` | artifacts, logging, lifecycle |
| MUST READ | `13_REVIEW_VERIFICATION.md` | deployment profiles (P0-07D) |
| MUST READ | `SCHEMAS/ENCODING.md + TEST_VECTORS` | parity |
| MUST READ | `SCHEMAS/AtomicPrototypeClaimV1.schema.json` | ledger |
| MUST READ | `00E_ARCHITECTURE_FREEZE_STATUS.md` | freeze rules |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
