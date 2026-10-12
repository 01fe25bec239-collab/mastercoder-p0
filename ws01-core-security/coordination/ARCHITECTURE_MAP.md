# ARCHITECTURE MAP — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

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
| MUST READ | `00D_V2_1_INVARIANTS.md` | all INV-01..22 (you enforce most of them) |
| MUST READ | `SCHEMAS/ENCODING.md` | exact bytes, digests, signatures |
| MUST READ | `SCHEMAS/REGISTRY.json` | all 85 contracts |
| MUST READ | `SCHEMAS/CONTRACT_AUTHORITY.md` | producers, writers, digest references, acyclic order |
| MUST READ | `TEST_VECTORS/*` | golden bytes, rejection cases, signature pairs |
| MUST READ | `06_RUST_CONTROL_PLANE.md` | request transaction, gate commitment |
| MUST READ | `07_POSTGRES_PERSISTENCE_WORKFLOW.md` | lease expiry, writer matrix, grants, ledger, eligibility boundary |
| MUST READ | `14_PATCH_APPROVAL_PUBLICATION.md` | plan, custody, per-effect publication |
| MUST READ | `15_IDENTITY_SECRETS.md` | component identity, epochs, break glass |
| MUST READ | `18_DATA_MODEL.md` | constraints |
| MUST READ | `19_STATE_MACHINES.md` | all machines |
| MUST READ | `20_API_CONTRACTS.md` | operations |
| MUST READ | `21_EVENT_CONTRACTS.md` | events |
| MUST READ | `22_TRUST_BOUNDARIES.md` | boundaries |
| MUST READ | `29_CONCURRENCY_FENCING.md` | lock order, fences, dispatch |
| MUST READ | `35_DISASTER_RECOVERY.md` | recovery order, fresh bootstrap |
| MUST READ | `48_COMPONENT_REVOCATION_POLICY.md` | revocation |
| MUST READ | `50_EXTERNAL_EFFECT_RECOVERY.md` | journal protocol |
| MUST READ | `52_APPROVAL_AUTHORITY_CONTRACT.md` | AA display, ceremony, enrollment |
| MUST READ | `53_GIT_PUBLICATION_CAPABILITY.md` | gateway |
| MUST READ | `56_MANUAL_RESOLUTION.md` | resolution |
| MUST READ | `58_FINAL_V2_1_AUDIT_HANDOFF.md` | external audit focus |
| MUST READ | `ADRs/ADR-016,018,019,022,023,024,025,027` | decisions you implement |
| MUST READ | `REVIEWS/*` | closed findings you must not reopen silently |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
