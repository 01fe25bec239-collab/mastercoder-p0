# ARCHITECTURE MAP — WORKSTREAM-04-AGENT-MODEL-GRAPH

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
| MUST READ | `11_AGENT_HARNESS_GRAPH.md` | two modes, ModelTurnV2, tools, graph, cancellation, parity |
| MUST READ | `12_MODEL_ACCESS.md` | broker/dialer split, auth modes, spend, failure, references |
| MUST READ | `07_POSTGRES_PERSISTENCE_WORKFLOW.md` | ledger, budget reservation |
| MUST READ | `29_CONCURRENCY_FENCING.md` | settlement uniqueness |
| MUST READ | `33_SCALING_CAPACITY.md` | root envelope ceilings |
| MUST READ | `50_EXTERNAL_EFFECT_RECOVERY.md` | MODEL_REQUEST effects |
| MUST READ | `47_COMPATIBILITY_MATRIX.md` | model rows |
| MUST READ | `19_STATE_MACHINES.md` | Resource envelope |
| MUST READ | `SCHEMAS (ModelTurnV2, ModelMessageV1, ModelContentV1, ModelContentBlockV1, ModelToolV1, ModelIntentV2, ResourceEnvelopeV2)` | exact wire |
| MUST READ | `ADRs/ADR-008,009,014` | decisions |
| MUST READ | `44_OPEN_RISKS.md` | R-001, R-008, R-014 |
| MUST READ | `00D_V2_1_INVARIANTS.md` | INV-05,09,13,14 |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
