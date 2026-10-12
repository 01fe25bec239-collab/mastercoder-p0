# WRITE SET — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS02 — `ws02-git-analysis-review/`

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `ws02-git-analysis-review/Cargo.toml`, `Cargo.lock`, `compose.yaml`, `.env.example`, every crate's `Cargo.toml` and `src/lib.rs` | W02 (scaffold task W02-T000 only) | — | IMPL(config) | — |
| `crates/mc-git-test-support/` (dev-only) | W02 | — | TEST | — |
| `crates/mc-checkout-grant/` (CheckoutGrantV2 issuer, G-040) | W02 | W03 | IMPL, TEST | CheckoutGrantV2, MergeSubjectV2, AttemptBindingV2 |
| `crates/mc-fast-smoke/` (test-only) | W02 | W06 | TEST | — |
| `doubles/reducer-table/**`, `doubles/checkout-grant/**` | W02 | W01, W03 | IMPL, TEST (versioned) | I-06; CheckoutGrantV2 |
| `crates/mc-github/` (Review App client, webhook verify, read-back) | W02 | W01 (read), W06 | IMPL, TEST | ProviderEventV2 |
| `crates/mc-provider-normalizer/` | W02 | W01, W06 | IMPL, TEST | MergeSubjectV2, CurrentCandidateV1, GitOidV1 |
| `crates/mc-review-broker/` | W02 | W01, W06 | IMPL, TEST | CheckProjectionV2, CheckWriteRequestV1, CheckOutputV1, GateProofV2 (read), EffectIntentV2 via I-04 |
| `crates/mc-eligibility/` | W02 | W01, W04 | IMPL, TEST | EligibilityProofV2 |
| `crates/mc-analysis-planner/` | W02 | W01, W03 | IMPL, TEST | AnalysisPlanV1, AnalysisUnitV1, ExpectedAnalysisManifestV1, ManifestDescriptorV1, DescriptorEntryV1, WorkflowSpecCoreV2 |
| `crates/mc-analyzers/` (registry + deterministic adapters) | W02 | W03 | IMPL, TEST | AnalysisCompletionReceiptV1, CapabilityProfileV1 |
| `crates/mc-findings/` | W02 | W05 (fixtures) | IMPL, TEST | FindingCandidateV2 |
| `crates/mc-gate-reducer/` (pure, credential-free) | W02 | W01 (path dep) | IMPL, TEST | EvaluationIdentityV2, GateProofV2 conclusion vocabulary |
| `crates/mc-verification/` | W02 | W01 | IMPL, TEST | VerificationReceiptV2, VerificationSetV2, OracleTrustClassV1, EvidenceReceiptV2 |
| `db/git/**` (schema `mc_git`) | W02 | W06 | IMPL(SQL) | W01 role framework (read-only) |
| `doubles/fake-github/**` | W02 | W01, W03, W06 | IMPL, TEST (versioned) | — |
| `fixtures/github-recordings/**`, `fixtures/findings/**`, `fixtures/analysis/**` | W02 | ALL | TEST | — |
| `staging/**` (rulesets, App manifests, permissions — no secrets) | W02 | W01, W06 | IMPL(config) | 53/54 provider posture |
| `claims/{P0-03A..E,P0-04A..C,P0-07A,P0-07C,P0-07E,P0-09E}/` | W02 | W06, evidence A3s | IMPL, TEST, DOC | claim JSON |
| `coordination/**` | W02 | master | COORD | — |



## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws02-git-analysis-review/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws02-git-analysis-review/`
- `.github/workflows/ws02-*.yml` only
