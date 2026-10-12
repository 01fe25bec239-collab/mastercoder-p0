# SCOPE — WORKSTREAM-02-GIT-ANALYSIS-REVIEW (Git, Analysis & Review)

**Mission.** Establish GitHub truth and the deterministic review path: staging org and Apps, provider normalizer and exact MergeSubjectV2, required-check semantics, CheckProjectionV2 lifecycle with withdrawal and emergency freeze, eligibility proofs, analysis planner with the sealed ExpectedAnalysisManifestV1, digest-pinned deterministic analyzers and findings, the pure gate reducer, oracle trust and verification authority, Publication App token residual characterization, checkout grant issuance, and a model-free FAST smoke.

## Owns (primary, exclusive)
- GitHub staging org config-as-code (Review App, Publication App, rulesets, freeze operator)
- Fake GitHub REST double (I-12) (the synthetic vulnerable repos are built by W05-T009)
- mc-github, mc-provider-normalizer (MergeSubjectV2, CurrentCandidateV1, ProviderEventV2)
- mc-review-broker (CheckProjectionV2, freeze_state)
- mc-eligibility (EligibilityProofV2)
- mc-analysis-planner, mc-analyzers, mc-findings
- mc-gate-reducer (I-06, pure)
- mc-verification (VerificationReceiptV2/SetV2, OracleTrustClassV1)
- CheckoutGrantV2 issuer
- FAST-without-models smoke

## Does NOT own (read-only or other workstream)
- Gate signing, DB writer roles, evidence ingestion (W01)
- Git publication gateway (W01) — you only characterize the token (P0-09E)
- Pack fetching/parsing (W03)
- Model components (W04)
- UI (W05)

## Primary P0 claims (12)
P0-03A, P0-03B, P0-03C, P0-03D, P0-03E, P0-04A, P0-04B, P0-04C, P0-07A, P0-07C, P0-07E, P0-09E — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
