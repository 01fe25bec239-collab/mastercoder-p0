// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use mc_contracts::{ScopeV1, round_trip_projection};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fmt::Debug};

fn vectors() -> Vec<Value> {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/TEST_VECTORS/wire-vectors.json"
    )))
    .unwrap()
}

fn scope() -> Value {
    let projection = vectors()[0]["projection"].clone();
    assert!(serde_json::from_value::<ScopeV1>(projection.clone()).is_ok());
    projection
}

fn error(value: Value) -> String {
    serde_json::from_value::<ScopeV1>(value)
        .expect_err("must reject malformed projection")
        .to_string()
}

fn assert_direct_round_trip<T: DeserializeOwned + Serialize>(value: &Value, context: &str) {
    let text = serde_json::to_string(value).unwrap();
    let from_value: T = serde_json::from_value(value.clone())
        .unwrap_or_else(|failure| panic!("{context} from_value control: {failure}"));
    let from_str: T = serde_json::from_str(&text)
        .unwrap_or_else(|failure| panic!("{context} from_str control: {failure}"));
    assert_eq!(
        serde_json::to_value(&from_value).unwrap(),
        *value,
        "{context} from_value"
    );
    assert_eq!(
        serde_json::to_value(&from_str).unwrap(),
        *value,
        "{context} from_str"
    );
    if value.is_string() {
        assert_eq!(
            serde_json::to_string(&from_value).unwrap(),
            text,
            "{context} from_value text"
        );
        assert_eq!(
            serde_json::to_string(&from_str).unwrap(),
            text,
            "{context} from_str text"
        );
    }
}

fn assert_rejection<T: Debug>(result: Result<T, serde_json::Error>, reason: &str, context: &str) {
    let failure = result.expect_err(context).to_string();
    assert!(failure.contains(reason), "{context}: {failure}");
    if reason.starts_with("invalid type:") {
        assert!(
            failure.contains("expected a string"),
            "{context}: {failure}"
        );
    }
}

fn assert_direct_rejection<T: DeserializeOwned + Debug>(
    value: &Value,
    reason: &str,
    context: &str,
) {
    let text = serde_json::to_string(value).unwrap();
    // Evaluate both public entry paths before assertions; neither is hidden by &&.
    let from_value = serde_json::from_value::<T>(value.clone());
    let from_str = serde_json::from_str::<T>(&text);
    assert_rejection(from_value, reason, &format!("{context} from_value"));
    assert_rejection(from_str, reason, &format!("{context} from_str"));
}

fn check_record_type<R, M>(projection: &Value)
where
    R: DeserializeOwned + Serialize + Debug,
    M: DeserializeOwned + Serialize + Debug,
{
    let name = projection["type"].as_str().unwrap();
    assert_direct_round_trip::<M>(&projection["type"], &format!("{name} marker"));
    assert_direct_round_trip::<R>(projection, name);
    assert_eq!(
        round_trip_projection(projection.clone()).unwrap(),
        *projection
    );

    let unknown = json!("unregistered-contract-type");
    assert_direct_rejection::<M>(&unknown, "unknown variant", &format!("{name} marker"));
    let mut wrong_name = projection.clone();
    wrong_name["type"] = unknown;
    assert_direct_rejection::<R>(&wrong_name, "unknown variant", name);

    // Schema type constants admit strings, including at the direct record field boundary.
    for (invalid, reason) in [
        (json!({name: null}), "invalid type: map"),
        (json!([name]), "invalid type: sequence"),
        (json!(1), "invalid type: integer"),
        (json!(true), "invalid type: boolean"),
        (Value::Null, "invalid type: null"),
    ] {
        assert_direct_rejection::<M>(&invalid, reason, &format!("{name} marker"));
        let mut malformed = projection.clone();
        malformed["type"] = invalid;
        assert_direct_rejection::<R>(&malformed, reason, name);
    }
}

#[test]
fn string_only_all_85_record_type_markers_and_fields() {
    let controls: Vec<_> = vectors()
        .into_iter()
        .filter(|vector| vector["expect"] == "ACCEPT_ENCODING")
        .collect();
    assert_eq!(controls.len(), 85);
    let mut checked = BTreeSet::new();
    macro_rules! check_types {
        ($($record:ident => $marker:ident),+ $(,)?) => {
            $(
                let name = stringify!($record);
                let control = controls.iter().find(|vector| vector["contract"] == name).unwrap();
                check_record_type::<mc_contracts::$record, mc_contracts::$marker>(&control["projection"]);
                assert!(checked.insert(name.to_owned()), "duplicate marker control: {name}");
            )+
        };
    }
    // Exact record names from SCHEMAS/REGISTRY.json; markers are the public generated types.
    check_types!(
        AcquisitionRequestV1 => AcquisitionRequestV1Type,
        AnalysisCompletionReceiptV1 => AnalysisCompletionReceiptV1Type,
        AnalysisPlanV1 => AnalysisPlanV1Type,
        AnalysisUnitV1 => AnalysisUnitV1Type,
        ApprovalChallengeV2 => ApprovalChallengeV2Type,
        ApprovalCredentialBindingV1 => ApprovalCredentialBindingV1Type,
        ApprovalEnrollmentActionV1 => ApprovalEnrollmentActionV1Type,
        ApprovalEnrollmentAuthorizationV1 => ApprovalEnrollmentAuthorizationV1Type,
        ApprovalEnrollmentChallengeV1 => ApprovalEnrollmentChallengeV1Type,
        ApprovalEnrollmentIntentV1 => ApprovalEnrollmentIntentV1Type,
        ApprovalLifecycleChallengeV1 => ApprovalLifecycleChallengeV1Type,
        ApprovalLifecycleConsentV1 => ApprovalLifecycleConsentV1Type,
        ApprovalRecoveryPolicyV1 => ApprovalRecoveryPolicyV1Type,
        ArtifactCustodyEntryV1 => ArtifactCustodyEntryV1Type,
        AttemptBindingV2 => AttemptBindingV2Type,
        AttemptEventV2 => AttemptEventV2Type,
        AuditEnvelopeV1 => AuditEnvelopeV1Type,
        AuthorityScopeV1 => AuthorityScopeV1Type,
        CapabilityProfileV1 => CapabilityProfileV1Type,
        ChangedObjectV1 => ChangedObjectV1Type,
        CheckOutputV1 => CheckOutputV1Type,
        CheckProjectionV2 => CheckProjectionV2Type,
        CheckWriteRequestV1 => CheckWriteRequestV1Type,
        CheckoutGrantV2 => CheckoutGrantV2Type,
        CommitIdentityV1 => CommitIdentityV1Type,
        CreateBlobRequestV1 => CreateBlobRequestV1Type,
        CreateCommitRequestV1 => CreateCommitRequestV1Type,
        CreateFixPrRequestV1 => CreateFixPrRequestV1Type,
        CreateRefRequestV1 => CreateRefRequestV1Type,
        CreateTreeRequestV1 => CreateTreeRequestV1Type,
        CurrentCandidateV1 => CurrentCandidateV1Type,
        DbStartCertificateV1 => DbStartCertificateV1Type,
        DependencyAcquisitionGrantV1 => DependencyAcquisitionGrantV1Type,
        DependencyPackageV1 => DependencyPackageV1Type,
        DescriptorEntryV1 => DescriptorEntryV1Type,
        EffectIntentV2 => EffectIntentV2Type,
        EffectPlanEntryV1 => EffectPlanEntryV1Type,
        EffectStateV1 => EffectStateV1Type,
        EligibilityProofV2 => EligibilityProofV2Type,
        EvaluationIdentityV2 => EvaluationIdentityV2Type,
        EvidenceReceiptV2 => EvidenceReceiptV2Type,
        ExpectedAnalysisManifestV1 => ExpectedAnalysisManifestV1Type,
        ExternalEffectJournalV1 => ExternalEffectJournalV1Type,
        FindingCandidateV2 => FindingCandidateV2Type,
        GateProofV2 => GateProofV2Type,
        GitOidV1 => GitOidV1Type,
        GitPublicationCapabilityV2 => GitPublicationCapabilityV2Type,
        HumanApprovalAssertionV2 => HumanApprovalAssertionV2Type,
        JournalEpochFenceV1 => JournalEpochFenceV1Type,
        JournalWitnessReceiptV1 => JournalWitnessReceiptV1Type,
        ManifestDescriptorV1 => ManifestDescriptorV1Type,
        ManualCaseStateV1 => ManualCaseStateV1Type,
        ManualResolutionActionV1 => ManualResolutionActionV1Type,
        ManualResolutionV1 => ManualResolutionV1Type,
        MergeSubjectV2 => MergeSubjectV2Type,
        ModelContentBlockV1 => ModelContentBlockV1Type,
        ModelContentV1 => ModelContentV1Type,
        ModelIntentV2 => ModelIntentV2Type,
        ModelMessageV1 => ModelMessageV1Type,
        ModelToolV1 => ModelToolV1Type,
        ModelTurnV2 => ModelTurnV2Type,
        OracleTrustClassV1 => OracleTrustClassV1Type,
        PatchCandidateV2 => PatchCandidateV2Type,
        PatchCustodyV1 => PatchCustodyV1Type,
        PolicyVersionV1 => PolicyVersionV1Type,
        ProducerEventV2 => ProducerEventV2Type,
        ProviderEventV2 => ProviderEventV2Type,
        ProviderResultV1 => ProviderResultV1Type,
        PublicationEffectsV1 => PublicationEffectsV1Type,
        PublicationOperationV1 => PublicationOperationV1Type,
        ResolutionChallengeV1 => ResolutionChallengeV1Type,
        ResourceEnvelopeV2 => ResourceEnvelopeV2Type,
        RevocationRecordV1 => RevocationRecordV1Type,
        ScopeV1 => ScopeV1Type,
        SignedEnvelopeV1 => SignedEnvelopeV1Type,
        TreeEntryV1 => TreeEntryV1Type,
        TrustBundleV1 => TrustBundleV1Type,
        TrustMemberV1 => TrustMemberV1Type,
        TrustedDisplayReceiptV1 => TrustedDisplayReceiptV1Type,
        VerificationReceiptV2 => VerificationReceiptV2Type,
        VerificationSetV2 => VerificationSetV2Type,
        VerifiedPublicationPlanV2 => VerifiedPublicationPlanV2Type,
        WebAuthnCeremonyV1 => WebAuthnCeremonyV1Type,
        WebAuthnRegistrationCeremonyV1 => WebAuthnRegistrationCeremonyV1Type,
        WorkflowSpecCoreV2 => WorkflowSpecCoreV2Type,
    );
    let registry: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"
    )))
    .unwrap();
    assert_eq!(checked.len(), 85);
    assert_eq!(
        checked,
        registry["contracts"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    );
}

fn accept_projection(contract: &str) -> Value {
    vectors()
        .into_iter()
        .find(|vector| vector["expect"] == "ACCEPT_ENCODING" && vector["contract"] == contract)
        .unwrap()["projection"]
        .clone()
}

fn assert_nested_map_rejection<T: DeserializeOwned + Debug>(value: Value, context: &str) {
    let text = serde_json::to_string(&value).unwrap();
    let from_value = serde_json::from_value::<T>(value.clone());
    let from_str = serde_json::from_str::<T>(&text);
    let projection = round_trip_projection(value);
    assert_rejection(
        from_value,
        "invalid type: map",
        &format!("{context} from_value"),
    );
    assert_rejection(
        from_str,
        "invalid type: map",
        &format!("{context} from_str"),
    );
    assert_rejection(
        projection,
        "invalid type: map",
        &format!("{context} projection"),
    );
}

#[test]
fn string_only_nested_merge_subject_scope_type_rejects_externally_tagged_object() {
    let mut value = accept_projection(mc_contracts::MergeSubjectV2::TYPE_NAME);
    assert_direct_round_trip::<mc_contracts::MergeSubjectV2>(&value, "MergeSubjectV2");
    assert_eq!(round_trip_projection(value.clone()).unwrap(), value);
    let name = value["scope"]["type"].as_str().unwrap().to_owned();
    value["scope"]["type"] = json!({name: null});
    assert_nested_map_rejection::<mc_contracts::MergeSubjectV2>(value, "MergeSubjectV2 scope.type");
}

#[test]
fn string_only_git_publication_effect_kind_rejects_externally_tagged_object() {
    let mut value = accept_projection(mc_contracts::GitPublicationCapabilityV2::TYPE_NAME);
    assert_direct_round_trip::<mc_contracts::GitPublicationCapabilityV2>(
        &value,
        "GitPublicationCapabilityV2",
    );
    assert_eq!(round_trip_projection(value.clone()).unwrap(), value);
    let literal = value["effect_kind"].as_str().unwrap().to_owned();
    value["effect_kind"] = json!({literal: null});
    assert_nested_map_rejection::<mc_contracts::GitPublicationCapabilityV2>(
        value,
        "GitPublicationCapabilityV2 effect_kind",
    );
}

#[test]
fn projection_level_rejects_all_reject() {
    let mut projections = 0;
    let mut byte_only = 0;
    for vector in vectors()
        .into_iter()
        .filter(|vector| vector["expect"] == "REJECT_ENCODING")
    {
        if let Some(projection) = vector.get("projection") {
            projections += 1;
            assert!(
                round_trip_projection(projection.clone()).is_err(),
                "{}: {}",
                vector["id"],
                vector["reason"]
            );
        } else {
            byte_only += 1;
        }
    }
    assert_eq!(projections, 261);
    assert_eq!(byte_only, 94, "CBOR-only cases are accounted, not executed");
}

#[test]
fn unknown_field_rejected() {
    let mut value = scope();
    value["forged_authority"] = json!(true);
    assert!(error(value).contains("unknown field"));
}

#[test]
fn null_rejected() {
    let mut value = scope();
    value["repository_id"] = Value::Null;
    assert!(error(value).contains("invalid type: null"));
}

#[test]
fn wrong_type_rejected() {
    let mut value = scope();
    value["type"] = json!("ProducerBindingV2");
    assert!(error(value).contains("unknown variant"));
}

#[test]
fn wrong_wire_revision_rejected() {
    let mut value = scope();
    value["wire_revision"] = json!(2);
    assert!(error(value).contains("wire revision"));
}

#[test]
fn every_required_field_and_null_rejected() {
    for vector in vectors()
        .into_iter()
        .filter(|vector| vector["expect"] == "ACCEPT_ENCODING")
    {
        let projection = vector["projection"].clone();
        assert!(round_trip_projection(projection.clone()).is_ok());
        for field in projection.as_object().unwrap().keys() {
            let mut missing = projection.clone();
            missing.as_object_mut().unwrap().remove(field);
            let failure = round_trip_projection(missing).unwrap_err().to_string();
            let expected_reason = if field == "type" {
                "missing or invalid contract type"
            } else {
                "missing field"
            };
            assert!(
                failure.contains(expected_reason),
                "{} {field}: {failure}",
                vector["id"]
            );
            let mut null = projection.clone();
            null[field] = Value::Null;
            assert!(
                round_trip_projection(null).is_err(),
                "{} {field}",
                vector["id"]
            );
        }
    }
}

#[test]
fn duplicate_field_rejected_from_json_text() {
    let value = scope();
    let text = serde_json::to_string(&value).unwrap();
    let duplicate = format!("{{\"repository_id\":\"1\",{}", &text[1..]);
    let failure = serde_json::from_str::<ScopeV1>(&duplicate).unwrap_err();
    assert!(failure.to_string().contains("duplicate field"));
}

#[test]
fn nested_unknown_field_rejected() {
    let mut value = vectors()
        .into_iter()
        .find(|vector| vector["id"] == "MergeSubjectV2-positive")
        .unwrap()["projection"]
        .clone();
    assert!(round_trip_projection(value.clone()).is_ok());
    value["scope"]["forged_authority"] = json!(true);
    assert!(
        round_trip_projection(value)
            .unwrap_err()
            .to_string()
            .contains("unknown field")
    );
}

#[test]
fn positional_json_record_array_rejected() {
    let projection = scope();
    let fields = ScopeV1::FIELD_ORDER;
    let values: Vec<_> = fields
        .iter()
        .map(|name| projection[*name].clone())
        .collect();
    let failure = serde_json::from_value::<ScopeV1>(json!(values)).unwrap_err();
    assert!(failure.to_string().contains("invalid type: sequence"));
}

#[test]
fn wrong_wire_revision_json_kind_rejected() {
    for revision in [json!(1.0), json!("1"), json!(true), json!(null)] {
        let mut projection = scope();
        projection["wire_revision"] = revision;
        assert!(serde_json::from_value::<ScopeV1>(projection).is_err());
    }
}

#[test]
fn aggregate_record_byte_limit_rejected_in_both_directions() {
    // ENCODING.md caps complete canonical record bytes at 16 MiB; each
    // DescriptorEntryV1 value independently fits the registry's 1 MiB cap.
    let mut entries: Vec<_> = (0..16)
        .map(|index| {
            json!({
                "type": "DescriptorEntryV1", "wire_revision": 1,
                "key": format!("unit-{index:02}"), "value": "ab".repeat(1_048_576)
            })
        })
        .collect();
    let last = entries.pop().unwrap();
    let valid = json!({"type":"ManifestDescriptorV1", "wire_revision":1,
        "kind":"SCOPES", "entries":entries});
    let parsed: mc_contracts::ManifestDescriptorV1 = serde_json::from_value(valid.clone()).unwrap();
    assert_eq!(serde_json::to_value(&parsed).unwrap(), valid);
    let mut oversized = valid;
    oversized["entries"]
        .as_array_mut()
        .unwrap()
        .push(last.clone());
    let failure = serde_json::from_value::<mc_contracts::ManifestDescriptorV1>(oversized)
        .unwrap_err()
        .to_string();
    assert!(
        failure.contains(&mc_contracts::primitives::ProjectionError::RecordTooLarge.to_string()),
        "{failure}"
    );
    let mut values = parsed.entries.into_vec();
    values.push(serde_json::from_value(last).unwrap());
    let constructed = mc_contracts::ManifestDescriptorV1 {
        entries: mc_contracts::primitives::SetOf::new(values).unwrap(),
        ..parsed
    };
    let failure = serde_json::to_value(constructed).unwrap_err().to_string();
    assert!(
        failure.contains(&mc_contracts::primitives::ProjectionError::RecordTooLarge.to_string()),
        "{failure}"
    );
}

#[test]
fn record_set_identity_uniqueness_is_independent_of_encoding() {
    // TreeEntryV1.path is a named identity even when object/mode differ.
    let vectors = vectors();
    let entry = vectors
        .iter()
        .find(|v| v["id"] == "TreeEntryV1-positive")
        .unwrap()["projection"]
        .clone();
    let mut other = entry.clone();
    other["mode"] = json!("EXECUTABLE");
    if other == entry {
        other["mode"] = json!("REGULAR");
    }
    let first: mc_contracts::TreeEntryV1 = serde_json::from_value(entry).unwrap();
    let second: mc_contracts::TreeEntryV1 = serde_json::from_value(other).unwrap();
    use mc_contracts::primitives::{CanonicalOrder, SetError, SetOf};
    let values = if first.canonical_cmp(&second).is_lt() {
        vec![first, second]
    } else {
        vec![second, first]
    };
    assert_eq!(SetOf::new(values).unwrap_err(), SetError::DuplicateIdentity);
}

#[test]
fn ordered_record_arrays_reject_duplicate_identities() {
    let fixture = vectors()
        .into_iter()
        .find(|v| v["id"] == "PublicationEffectsV1-positive")
        .unwrap();
    let mut projection = fixture["projection"].clone();
    let parsed: mc_contracts::PublicationEffectsV1 =
        serde_json::from_value(projection.clone()).unwrap();
    let mut other = projection["steps"][0].clone();
    other["request_digest"] = json!("ff".repeat(32));
    projection["steps"]
        .as_array_mut()
        .unwrap()
        .push(other.clone());
    let expected = mc_contracts::primitives::SetError::DuplicateIdentity.to_string();
    let failure =
        serde_json::from_value::<mc_contracts::PublicationEffectsV1>(projection).unwrap_err();
    assert!(failure.to_string().contains(&expected));
    let mut constructed = parsed;
    constructed
        .steps
        .push(serde_json::from_value(other).unwrap());
    let failure = serde_json::to_value(constructed).unwrap_err();
    assert!(failure.to_string().contains(&expected));
}

#[test]
fn ordered_arrays_enforce_item_limit_and_preserve_order() {
    use mc_contracts::primitives::{SetError, U64Str, bounded_array, serialize_bounded_array};
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ArrayControl {
        #[serde(
            deserialize_with = "bounded_array",
            serialize_with = "serialize_bounded_array"
        )]
        values: Vec<U64Str>,
    }
    // Ordered arrays allow duplicate scalar values and preserve unsorted order.
    let value = json!({"values":["2","1","1"]});
    let parsed: ArrayControl = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), value);
    for size in [65_535, 65_536] {
        let value = json!({"values":vec!["0"; size]});
        let parsed: ArrayControl = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), value);
    }
    let expected = SetError::TooMany.to_string();
    let failure =
        serde_json::from_value::<ArrayControl>(json!({"values":vec!["0";65_537]})).unwrap_err();
    assert!(failure.to_string().contains(&expected));
    let failure = serde_json::to_value(ArrayControl {
        values: vec![U64Str(0); 65_537],
    })
    .unwrap_err();
    assert!(failure.to_string().contains(&expected));
}
