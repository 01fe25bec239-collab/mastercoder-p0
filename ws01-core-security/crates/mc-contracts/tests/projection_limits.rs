// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use mc_contracts::primitives::{CanonicalOrder, I64Str, Opt, ProjectionError, U64Str};
use mc_contracts::*;
use serde_json::Value;
#[test]
fn private_size_accounting_matches_all_85_golden_records() {
    let vectors: Vec<Value> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/TEST_VECTORS/wire-vectors.json"
    )))
    .unwrap();
    let mut count = 0;
    for v in vectors
        .into_iter()
        .filter(|v| v["expect"] == "ACCEPT_ENCODING")
    {
        let p = v["projection"].clone();
        let size = match v["contract"].as_str().unwrap() {
            "ScopeV1" => serde_json::from_value::<ScopeV1>(p)
                .unwrap()
                .canonical_size(),
            "GitOidV1" => serde_json::from_value::<GitOidV1>(p)
                .unwrap()
                .canonical_size(),
            "MergeSubjectV2" => serde_json::from_value::<MergeSubjectV2>(p)
                .unwrap()
                .canonical_size(),
            "WorkflowSpecCoreV2" => serde_json::from_value::<WorkflowSpecCoreV2>(p)
                .unwrap()
                .canonical_size(),
            "AttemptBindingV2" => serde_json::from_value::<AttemptBindingV2>(p)
                .unwrap()
                .canonical_size(),
            "AnalysisUnitV1" => serde_json::from_value::<AnalysisUnitV1>(p)
                .unwrap()
                .canonical_size(),
            "ManifestDescriptorV1" => serde_json::from_value::<ManifestDescriptorV1>(p)
                .unwrap()
                .canonical_size(),
            "DescriptorEntryV1" => serde_json::from_value::<DescriptorEntryV1>(p)
                .unwrap()
                .canonical_size(),
            "ExpectedAnalysisManifestV1" => serde_json::from_value::<ExpectedAnalysisManifestV1>(p)
                .unwrap()
                .canonical_size(),
            "AnalysisCompletionReceiptV1" => {
                serde_json::from_value::<AnalysisCompletionReceiptV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "OracleTrustClassV1" => serde_json::from_value::<OracleTrustClassV1>(p)
                .unwrap()
                .canonical_size(),
            "EvidenceReceiptV2" => serde_json::from_value::<EvidenceReceiptV2>(p)
                .unwrap()
                .canonical_size(),
            "VerificationReceiptV2" => serde_json::from_value::<VerificationReceiptV2>(p)
                .unwrap()
                .canonical_size(),
            "VerificationSetV2" => serde_json::from_value::<VerificationSetV2>(p)
                .unwrap()
                .canonical_size(),
            "EvaluationIdentityV2" => serde_json::from_value::<EvaluationIdentityV2>(p)
                .unwrap()
                .canonical_size(),
            "GateProofV2" => serde_json::from_value::<GateProofV2>(p)
                .unwrap()
                .canonical_size(),
            "PolicyVersionV1" => serde_json::from_value::<PolicyVersionV1>(p)
                .unwrap()
                .canonical_size(),
            "TrustBundleV1" => serde_json::from_value::<TrustBundleV1>(p)
                .unwrap()
                .canonical_size(),
            "TrustMemberV1" => serde_json::from_value::<TrustMemberV1>(p)
                .unwrap()
                .canonical_size(),
            "RevocationRecordV1" => serde_json::from_value::<RevocationRecordV1>(p)
                .unwrap()
                .canonical_size(),
            "CurrentCandidateV1" => serde_json::from_value::<CurrentCandidateV1>(p)
                .unwrap()
                .canonical_size(),
            "ChangedObjectV1" => serde_json::from_value::<ChangedObjectV1>(p)
                .unwrap()
                .canonical_size(),
            "VerifiedPublicationPlanV2" => serde_json::from_value::<VerifiedPublicationPlanV2>(p)
                .unwrap()
                .canonical_size(),
            "ArtifactCustodyEntryV1" => serde_json::from_value::<ArtifactCustodyEntryV1>(p)
                .unwrap()
                .canonical_size(),
            "PatchCustodyV1" => serde_json::from_value::<PatchCustodyV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalCredentialBindingV1" => {
                serde_json::from_value::<ApprovalCredentialBindingV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "TrustedDisplayReceiptV1" => serde_json::from_value::<TrustedDisplayReceiptV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalChallengeV2" => serde_json::from_value::<ApprovalChallengeV2>(p)
                .unwrap()
                .canonical_size(),
            "HumanApprovalAssertionV2" => serde_json::from_value::<HumanApprovalAssertionV2>(p)
                .unwrap()
                .canonical_size(),
            "WebAuthnCeremonyV1" => serde_json::from_value::<WebAuthnCeremonyV1>(p)
                .unwrap()
                .canonical_size(),
            "PublicationOperationV1" => serde_json::from_value::<PublicationOperationV1>(p)
                .unwrap()
                .canonical_size(),
            "GitPublicationCapabilityV2" => serde_json::from_value::<GitPublicationCapabilityV2>(p)
                .unwrap()
                .canonical_size(),
            "EffectIntentV2" => serde_json::from_value::<EffectIntentV2>(p)
                .unwrap()
                .canonical_size(),
            "EffectStateV1" => serde_json::from_value::<EffectStateV1>(p)
                .unwrap()
                .canonical_size(),
            "ExternalEffectJournalV1" => serde_json::from_value::<ExternalEffectJournalV1>(p)
                .unwrap()
                .canonical_size(),
            "JournalWitnessReceiptV1" => serde_json::from_value::<JournalWitnessReceiptV1>(p)
                .unwrap()
                .canonical_size(),
            "CheckProjectionV2" => serde_json::from_value::<CheckProjectionV2>(p)
                .unwrap()
                .canonical_size(),
            "ModelIntentV2" => serde_json::from_value::<ModelIntentV2>(p)
                .unwrap()
                .canonical_size(),
            "ModelMessageV1" => serde_json::from_value::<ModelMessageV1>(p)
                .unwrap()
                .canonical_size(),
            "ModelToolV1" => serde_json::from_value::<ModelToolV1>(p)
                .unwrap()
                .canonical_size(),
            "ModelTurnV2" => serde_json::from_value::<ModelTurnV2>(p)
                .unwrap()
                .canonical_size(),
            "ResourceEnvelopeV2" => serde_json::from_value::<ResourceEnvelopeV2>(p)
                .unwrap()
                .canonical_size(),
            "EligibilityProofV2" => serde_json::from_value::<EligibilityProofV2>(p)
                .unwrap()
                .canonical_size(),
            "CheckoutGrantV2" => serde_json::from_value::<CheckoutGrantV2>(p)
                .unwrap()
                .canonical_size(),
            "DependencyPackageV1" => serde_json::from_value::<DependencyPackageV1>(p)
                .unwrap()
                .canonical_size(),
            "DependencyAcquisitionGrantV1" => {
                serde_json::from_value::<DependencyAcquisitionGrantV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "ManualResolutionV1" => serde_json::from_value::<ManualResolutionV1>(p)
                .unwrap()
                .canonical_size(),
            "FindingCandidateV2" => serde_json::from_value::<FindingCandidateV2>(p)
                .unwrap()
                .canonical_size(),
            "PatchCandidateV2" => serde_json::from_value::<PatchCandidateV2>(p)
                .unwrap()
                .canonical_size(),
            "AttemptEventV2" => serde_json::from_value::<AttemptEventV2>(p)
                .unwrap()
                .canonical_size(),
            "ProducerEventV2" => serde_json::from_value::<ProducerEventV2>(p)
                .unwrap()
                .canonical_size(),
            "ProviderEventV2" => serde_json::from_value::<ProviderEventV2>(p)
                .unwrap()
                .canonical_size(),
            "AuditEnvelopeV1" => serde_json::from_value::<AuditEnvelopeV1>(p)
                .unwrap()
                .canonical_size(),
            "SignedEnvelopeV1" => serde_json::from_value::<SignedEnvelopeV1>(p)
                .unwrap()
                .canonical_size(),
            "CreateBlobRequestV1" => serde_json::from_value::<CreateBlobRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "TreeEntryV1" => serde_json::from_value::<TreeEntryV1>(p)
                .unwrap()
                .canonical_size(),
            "CreateTreeRequestV1" => serde_json::from_value::<CreateTreeRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "CommitIdentityV1" => serde_json::from_value::<CommitIdentityV1>(p)
                .unwrap()
                .canonical_size(),
            "CreateCommitRequestV1" => serde_json::from_value::<CreateCommitRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "CreateRefRequestV1" => serde_json::from_value::<CreateRefRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "CreateFixPrRequestV1" => serde_json::from_value::<CreateFixPrRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "CheckWriteRequestV1" => serde_json::from_value::<CheckWriteRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "EffectPlanEntryV1" => serde_json::from_value::<EffectPlanEntryV1>(p)
                .unwrap()
                .canonical_size(),
            "PublicationEffectsV1" => serde_json::from_value::<PublicationEffectsV1>(p)
                .unwrap()
                .canonical_size(),
            "AuthorityScopeV1" => serde_json::from_value::<AuthorityScopeV1>(p)
                .unwrap()
                .canonical_size(),
            "ProviderResultV1" => serde_json::from_value::<ProviderResultV1>(p)
                .unwrap()
                .canonical_size(),
            "DbStartCertificateV1" => serde_json::from_value::<DbStartCertificateV1>(p)
                .unwrap()
                .canonical_size(),
            "JournalEpochFenceV1" => serde_json::from_value::<JournalEpochFenceV1>(p)
                .unwrap()
                .canonical_size(),
            "ManualResolutionActionV1" => serde_json::from_value::<ManualResolutionActionV1>(p)
                .unwrap()
                .canonical_size(),
            "ResolutionChallengeV1" => serde_json::from_value::<ResolutionChallengeV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalEnrollmentActionV1" => serde_json::from_value::<ApprovalEnrollmentActionV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalEnrollmentChallengeV1" => {
                serde_json::from_value::<ApprovalEnrollmentChallengeV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "WebAuthnRegistrationCeremonyV1" => {
                serde_json::from_value::<WebAuthnRegistrationCeremonyV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "ApprovalRecoveryPolicyV1" => serde_json::from_value::<ApprovalRecoveryPolicyV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalEnrollmentAuthorizationV1" => {
                serde_json::from_value::<ApprovalEnrollmentAuthorizationV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "ModelContentBlockV1" => serde_json::from_value::<ModelContentBlockV1>(p)
                .unwrap()
                .canonical_size(),
            "ModelContentV1" => serde_json::from_value::<ModelContentV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalEnrollmentIntentV1" => serde_json::from_value::<ApprovalEnrollmentIntentV1>(p)
                .unwrap()
                .canonical_size(),
            "AnalysisPlanV1" => serde_json::from_value::<AnalysisPlanV1>(p)
                .unwrap()
                .canonical_size(),
            "CapabilityProfileV1" => serde_json::from_value::<CapabilityProfileV1>(p)
                .unwrap()
                .canonical_size(),
            "CheckOutputV1" => serde_json::from_value::<CheckOutputV1>(p)
                .unwrap()
                .canonical_size(),
            "ApprovalLifecycleChallengeV1" => {
                serde_json::from_value::<ApprovalLifecycleChallengeV1>(p)
                    .unwrap()
                    .canonical_size()
            }
            "ApprovalLifecycleConsentV1" => serde_json::from_value::<ApprovalLifecycleConsentV1>(p)
                .unwrap()
                .canonical_size(),
            "AcquisitionRequestV1" => serde_json::from_value::<AcquisitionRequestV1>(p)
                .unwrap()
                .canonical_size(),
            "ManualCaseStateV1" => serde_json::from_value::<ManualCaseStateV1>(p)
                .unwrap()
                .canonical_size(),
            _ => panic!("unregistered fixture"),
        };
        assert_eq!(
            size,
            v["cbor_hex"].as_str().unwrap().len() / 2,
            "{}",
            v["id"]
        );
        count += 1;
    }
    assert_eq!(count, 85);
}
#[test]
fn integer_size_boundaries_and_nested_array_depth() {
    for (n, size) in [
        (0, 1),
        (23, 1),
        (24, 2),
        (255, 2),
        (256, 3),
        (65535, 3),
        (65536, 5),
        (4294967295, 5),
        (4294967296, 9),
        (u64::MAX, 9),
    ] {
        assert_eq!(U64Str(n).canonical_size(), size);
    }
    for (n, size) in [
        (-1, 1),
        (-24, 1),
        (-25, 2),
        (-256, 2),
        (-257, 3),
        (i64::MIN, 9),
    ] {
        assert_eq!(I64Str(n).canonical_size(), size);
    }
    let nested = Opt(Some(vec![Opt(Some(U64Str(24)))]));
    assert_eq!(nested.canonical_size(), 5);
    assert_eq!(nested.canonical_nesting(), 3);
}
#[test]
fn record_limit_boundaries_are_inclusive() {
    use mc_contracts::primitives::validate_projection_limits;
    assert_eq!(validate_projection_limits(16777215, 31), Ok(()));
    assert_eq!(validate_projection_limits(16777216, 32), Ok(()));
    assert_eq!(
        validate_projection_limits(16777217, 32),
        Err(ProjectionError::RecordTooLarge)
    );
    assert_eq!(
        validate_projection_limits(16777216, 33),
        Err(ProjectionError::TooDeep)
    );
}
