# ARCHITECTURE MAP — WORKSTREAM-05-WEB-APPROVAL-UX

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
| MUST READ | `16_NEXTJS_REACT_WEB_UX.md` | views, approval ceremony, state language, accessibility, disclosure |
| MUST READ | `52_APPROVAL_AUTHORITY_CONTRACT.md` | AA origin, CSP, rendering obligations, ceremony |
| MUST READ | `56_MANUAL_RESOLUTION.md` | resolution flows and trusted UI |
| MUST READ | `14_PATCH_APPROVAL_PUBLICATION.md` | what DISPLAYED=APPROVED means |
| MUST READ | `24_PRIVACY_DATA_CLASSIFICATION.md` | disclosure |
| MUST READ | `54_CHECK_PROJECTION_LIFECYCLE.md` | EXTERNAL_GATE_UNSAFE, freeze display |
| MUST READ | `13_REVIEW_VERIFICATION.md` | verdict vocabulary, classes |
| MUST READ | `SCHEMAS/ENCODING.md + REGISTRY.json + TEST_VECTORS` | TS codec |
| MUST READ | `SCHEMAS (TrustedDisplayReceiptV1, ApprovalChallengeV2, HumanApprovalAssertionV2, WebAuthnCeremonyV1, WebAuthnRegistrationCeremonyV1, ManualResolutionActionV1, ResolutionChallengeV1)` | fields you display/drive |
| MUST READ | `ADRs/ADR-018,024,027` | decisions |
| MUST READ | `00D_V2_1_INVARIANTS.md` | INV-03,10,11,22 |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
