# ARCHITECTURE MAP — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

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
| MUST READ | `08_GIT_INTEGRATION.md` | provider identities, inbound truth, MergeSubjectV2, required check, checkout |
| MUST READ | `30_EXACT_GIT_LINEAGE.md` | coordinates, gate subject, freshness, check semantics |
| MUST READ | `54_CHECK_PROJECTION_LIFECYCLE.md` | projection identity, lifecycle, invalidation, freeze |
| MUST READ | `15_IDENTITY_SECRETS.md` | author eligibility |
| MUST READ | `07_POSTGRES_PERSISTENCE_WORKFLOW.md` | writer matrix rows you produce (candidate, manifest, eligibility, verification) |
| MUST READ | `09_ANALYSIS_ENGINE.md` | pipeline, novelty, cache |
| MUST READ | `13_REVIEW_VERIFICATION.md` | C/E classes, precedence, oracle trust, profiles |
| MUST READ | `51_ANALYSIS_COVERAGE_CONTRACT.md` | manifest, receipts, reducer |
| MUST READ | `53_GIT_PUBLICATION_CAPABILITY.md` | provider defense and residual (P0-09E) |
| MUST READ | `25_PR_SEQUENCE_DIAGRAMS.md` | sequence |
| MUST READ | `19_STATE_MACHINES.md` | Evaluation, Verification, Finding, Git Check Projection |
| MUST READ | `SCHEMAS (MergeSubjectV2, CurrentCandidateV1, CheckProjectionV2, CheckWriteRequestV1, CheckOutputV1, EligibilityProofV2, AnalysisPlanV1, AnalysisUnitV1, ExpectedAnalysisManifestV1, AnalysisCompletionReceiptV1, FindingCandidateV2, VerificationReceiptV2, VerificationSetV2, OracleTrustClassV1, CheckoutGrantV2)` | exact wire |
| MUST READ | `TEST_VECTORS/semantic-vectors.json` | eligibility-*, mandatory-analysis-*, oracle-*, check-initial-* |
| MUST READ | `ADRs/ADR-004,010,017,020,021` | decisions |
| MUST READ | `00D_V2_1_INVARIANTS.md` | INV-02,03,06,07,15,21 |
| REFERENCE | `44_OPEN_RISKS.md`, `45_UNRESOLVED_DECISIONS.md`, `46_ASSUMPTIONS.md`, `47_COMPATIBILITY_MATRIX.md` | risks/decisions/assumptions you may resolve with evidence |
| REFERENCE | `TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md`, `REVIEWS/*` | why each repair exists; closed findings |
| HISTORY ONLY | `BASELINE_HISTORY/**` | never normative |
