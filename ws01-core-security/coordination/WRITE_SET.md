# WRITE SET — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS01 — `ws01-core-security/` (HARD AUTHORITY)

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `ws01-core-security/Cargo.toml`, `Cargo.lock`, `compose.yaml`, `.env.example`, every crate's `Cargo.toml` and `src/lib.rs` | W01 (scaffold task W01-T000 only) | — | IMPL(config) | — |
| `ws01-core-security/crates/mc-cbor/`, `crates/mc-webauthn-verify/`, `crates/mc-negotiation/` | W01 | W06 (mc-cbor via mc-wire) | IMPL, TEST | ENCODING.md; WebAuthn rules (52); CapabilityProfileV1 |
| `ws01-core-security/crates/mc-test-support/` (dev-only) | W01 | — | TEST | — |
| `ws01-core-security/crates/mc-contracts/` | W01 | ALL (path dep) | GEN from `ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json` only | all 85 registry contracts |
| `ws01-core-security/crates/mc-wire/` | W01 | ALL except W05 source readers (independence rule) | IMPL, TEST | ENCODING.md, REGISTRY.json, wire/signature vectors |
| `ws01-core-security/db/authority/**`, `crates/mc-authority-db/` | W01 | ALL (role names) | IMPL(SQL), TEST | doc 07 writer matrix, 18, 29, ScopeV1 |
| `ws01-core-security/crates/mc-lease/`, `crates/mc-runner-control/` | W01 | W03, W04, W06 | IMPL, TEST | AttemptBindingV2, AttemptEventV2 |
| `ws01-core-security/crates/mc-gate-authority/`, `crates/mc-ingestion/` | W01 | W02, W06 | IMPL, TEST | AnalysisCompletionReceiptV1, EvidenceReceiptV2, VerificationReceiptV2/SetV2, EvaluationIdentityV2, GateProofV2, PolicyVersionV1, TrustBundleV1, RevocationRecordV1, ProducerEventV2 |
| `ws01-core-security/crates/mc-effects/`, `crates/mc-journal/` | W01 | W02, W03, W04, W06 | IMPL, TEST | EffectIntentV2, EffectStateV1, DbStartCertificateV1, ExternalEffectJournalV1, JournalWitnessReceiptV1, JournalEpochFenceV1, ProviderResultV1 |
| `ws01-core-security/crates/mc-approval-authority/`, `crates/mc-credential-lifecycle/` | W01 | W05 | IMPL, TEST | TrustedDisplayReceiptV1, ApprovalChallengeV2, WebAuthnCeremonyV1, HumanApprovalAssertionV2, ApprovalCredentialBindingV1, AuthorityScopeV1, Approval* enrollment/lifecycle/recovery contracts, WebAuthnRegistrationCeremonyV1 |
| `ws01-core-security/crates/mc-custody/` | W01 | W05, W06 | IMPL, TEST | PatchCustodyV1, ArtifactCustodyEntryV1, PatchCandidateV2, ChangedObjectV1 |
| `ws01-core-security/crates/mc-git-gateway/`, `crates/mc-publication-planner/`, `crates/mc-plan-verifier/` | W01 | W02 | IMPL, TEST | GitPublicationCapabilityV2, VerifiedPublicationPlanV2, PublicationOperationV1, PublicationEffectsV1, EffectPlanEntryV1, Create{Blob,Tree,Commit,Ref,FixPr}RequestV1, TreeEntryV1, CommitIdentityV1, EligibilityProofV2 |
| `ws01-core-security/crates/mc-resolution/` | W01 | W05 | IMPL, TEST | ManualResolutionActionV1, ResolutionChallengeV1, ManualResolutionV1, ManualCaseStateV1 |
| `ws01-core-security/crates/mc-bootstrap/` | W01 | W06 | IMPL, TEST | JournalEpochFenceV1, TrustBundleV1, ApprovalRecoveryPolicyV1 |
| `ws01-core-security/doubles/**` (code doubles + OpenAPI specs for I-07, I-08, I-09) | W01 | ALL | IMPL, TEST (versioned; breaking change = new major dir) | per interface I-01..I-09, I-16 |
| `ws01-core-security/tools/codegen/` | W01 | — | IMPL | REGISTRY.json |
| `ws01-core-security/claims/{P0-06C,P0-06D,P0-07B,P0-08A,P0-09B,P0-09C,P0-09D,P0-09F,P0-09G,P0-09H,P0-10A,P0-10I,P0-10J}/` | W01 | W06 (ledger), evidence A3s | IMPL, TEST, DOC | claim JSON |
| `ws01-core-security/coordination/**` | W01 | master | COORD | — |



## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws01-core-security/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws01-core-security/`
- `.github/workflows/ws01-*.yml` only
