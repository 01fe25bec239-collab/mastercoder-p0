// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
// Generated from SCHEMAS/REGISTRY.json; do not edit by hand.
use crate::primitives::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
// ENCODING.md §Registry identity: SHA-256 of the frozen raw registry bytes.
pub const REGISTRY_SHA256: &str =
    "b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectKind {
    #[serde(rename = "CREATE_BLOB")]
    CreateBlob,
    #[serde(rename = "CREATE_TREE")]
    CreateTree,
    #[serde(rename = "CREATE_COMMIT")]
    CreateCommit,
    #[serde(rename = "CREATE_REF")]
    CreateRef,
    #[serde(rename = "CREATE_FIX_PR")]
    CreateFixPr,
    #[serde(rename = "CREATE_CHECK")]
    CreateCheck,
    #[serde(rename = "UPDATE_CHECK")]
    UpdateCheck,
    #[serde(rename = "MODEL_REQUEST")]
    ModelRequest,
    #[serde(rename = "CHECKOUT_FETCH")]
    CheckoutFetch,
    #[serde(rename = "DEPENDENCY_FETCH")]
    DependencyFetch,
}
impl EffectKind {
    pub fn code(&self) -> u64 {
        match self {
            Self::CreateBlob => 0,
            Self::CreateTree => 1,
            Self::CreateCommit => 2,
            Self::CreateRef => 3,
            Self::CreateFixPr => 4,
            Self::CreateCheck => 5,
            Self::UpdateCheck => 6,
            Self::ModelRequest => 7,
            Self::CheckoutFetch => 8,
            Self::DependencyFetch => 9,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::CreateBlob),
            1 => Some(Self::CreateTree),
            2 => Some(Self::CreateCommit),
            3 => Some(Self::CreateRef),
            4 => Some(Self::CreateFixPr),
            5 => Some(Self::CreateCheck),
            6 => Some(Self::UpdateCheck),
            7 => Some(Self::ModelRequest),
            8 => Some(Self::CheckoutFetch),
            9 => Some(Self::DependencyFetch),
            _ => None,
        }
    }
}
impl CanonicalOrder for EffectKind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "GITHUB")]
    Github,
    #[serde(rename = "OTHER")]
    Other,
}
impl Provider {
    pub fn code(&self) -> u64 {
        match self {
            Self::Github => 0,
            Self::Other => 1,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Github),
            1 => Some(Self::Other),
            _ => None,
        }
    }
}
impl CanonicalOrder for Provider {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionRequestV1EffectKind {
    #[serde(rename = "CHECKOUT_FETCH")]
    CheckoutFetch,
    #[serde(rename = "DEPENDENCY_FETCH")]
    DependencyFetch,
}
impl AcquisitionRequestV1EffectKind {
    pub fn code(&self) -> u64 {
        match self {
            Self::CheckoutFetch => 0,
            Self::DependencyFetch => 1,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::CheckoutFetch),
            1 => Some(Self::DependencyFetch),
            _ => None,
        }
    }
}
impl CanonicalOrder for AcquisitionRequestV1EffectKind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionRequestV1Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}
impl AcquisitionRequestV1Method {
    pub fn code(&self) -> u64 {
        match self {
            Self::Get => 0,
            Self::Post => 1,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Get),
            1 => Some(Self::Post),
            _ => None,
        }
    }
}
impl CanonicalOrder for AcquisitionRequestV1Method {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionRequestV1Type {
    #[serde(rename = "AcquisitionRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AcquisitionRequestV1Fields>"
)]
pub struct AcquisitionRequestV1 {
    pub r#type: AcquisitionRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub grant_digest: Hex<32>,
    pub effect_kind: AcquisitionRequestV1EffectKind,
    pub provider_origin: Origin,
    pub method: AcquisitionRequestV1Method,
    pub target: Bytes,
    pub request_body: Bytes,
    pub selection_digest: Hex<32>,
    pub maximum_response_bytes: U64Str,
    pub expected_content_digest: Opt<Hex<32>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AcquisitionRequestV1Fields {
    r#type: AcquisitionRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    grant_digest: Hex<32>,
    effect_kind: AcquisitionRequestV1EffectKind,
    provider_origin: Origin,
    method: AcquisitionRequestV1Method,
    target: Bytes,
    request_body: Bytes,
    selection_digest: Hex<32>,
    maximum_response_bytes: U64Str,
    expected_content_digest: Opt<Hex<32>>,
}
impl TryFrom<crate::primitives::ObjectOnly<AcquisitionRequestV1Fields>> for AcquisitionRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AcquisitionRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            grant_digest: fields.grant_digest,
            effect_kind: fields.effect_kind,
            provider_origin: fields.provider_origin,
            method: fields.method,
            target: fields.target,
            request_body: fields.request_body,
            selection_digest: fields.selection_digest,
            maximum_response_bytes: fields.maximum_response_bytes,
            expected_content_digest: fields.expected_content_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AcquisitionRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("grant_digest", &self.grant_digest)?;
        state.serialize_field("effect_kind", &self.effect_kind)?;
        state.serialize_field("provider_origin", &self.provider_origin)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("target", &self.target)?;
        state.serialize_field("request_body", &self.request_body)?;
        state.serialize_field("selection_digest", &self.selection_digest)?;
        state.serialize_field("maximum_response_bytes", &self.maximum_response_bytes)?;
        state.serialize_field("expected_content_digest", &self.expected_content_digest)?;
        state.end()
    }
}
impl AcquisitionRequestV1 {
    pub const TYPE_NAME: &'static str = "AcquisitionRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "grant_digest",
        "effect_kind",
        "provider_origin",
        "method",
        "target",
        "request_body",
        "selection_digest",
        "maximum_response_bytes",
        "expected_content_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AcquisitionRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.grant_digest.canonical_cmp(&other.grant_digest))
            .then_with(|| self.effect_kind.canonical_cmp(&other.effect_kind))
            .then_with(|| self.provider_origin.canonical_cmp(&other.provider_origin))
            .then_with(|| self.method.canonical_cmp(&other.method))
            .then_with(|| self.target.canonical_cmp(&other.target))
            .then_with(|| self.request_body.canonical_cmp(&other.request_body))
            .then_with(|| self.selection_digest.canonical_cmp(&other.selection_digest))
            .then_with(|| {
                self.maximum_response_bytes
                    .canonical_cmp(&other.maximum_response_bytes)
            })
            .then_with(|| {
                self.expected_content_digest
                    .canonical_cmp(&other.expected_content_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.grant_digest.canonical_size())
            .saturating_add(self.effect_kind.canonical_size())
            .saturating_add(self.provider_origin.canonical_size())
            .saturating_add(self.method.canonical_size())
            .saturating_add(self.target.canonical_size())
            .saturating_add(self.request_body.canonical_size())
            .saturating_add(self.selection_digest.canonical_size())
            .saturating_add(self.maximum_response_bytes.canonical_size())
            .saturating_add(self.expected_content_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.grant_digest.canonical_nesting())
                .max(self.effect_kind.canonical_nesting())
                .max(self.provider_origin.canonical_nesting())
                .max(self.method.canonical_nesting())
                .max(self.target.canonical_nesting())
                .max(self.request_body.canonical_nesting())
                .max(self.selection_digest.canonical_nesting())
                .max(self.maximum_response_bytes.canonical_nesting())
                .max(self.expected_content_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisCompletionReceiptV1Outcome {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "UNSUPPORTED")]
    Unsupported,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}
impl AnalysisCompletionReceiptV1Outcome {
    pub fn code(&self) -> u64 {
        match self {
            Self::Completed => 0,
            Self::Failed => 1,
            Self::Unsupported => 2,
            Self::Cancelled => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Completed),
            1 => Some(Self::Failed),
            2 => Some(Self::Unsupported),
            3 => Some(Self::Cancelled),
            _ => None,
        }
    }
}
impl CanonicalOrder for AnalysisCompletionReceiptV1Outcome {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisCompletionReceiptV1Type {
    #[serde(rename = "AnalysisCompletionReceiptV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AnalysisCompletionReceiptV1Fields>"
)]
pub struct AnalysisCompletionReceiptV1 {
    pub r#type: AnalysisCompletionReceiptV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub receipt_id: Uuid,
    pub manifest_digest: Hex<32>,
    pub unit_id: Identifier,
    pub attempt_binding_digest: Hex<32>,
    pub actual_analyzer_bundle_digest: Hex<32>,
    pub actual_configuration_digest: Hex<32>,
    pub actual_scope_digest: Hex<32>,
    pub coverage_digest: Hex<32>,
    pub artifact_digests: SetOf<Hex<32>>,
    pub finding_count: u32,
    pub outcome: AnalysisCompletionReceiptV1Outcome,
    pub producer_id: Uuid,
    pub started_at: I64Str,
    pub completed_at: I64Str,
    pub lease_generation: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisCompletionReceiptV1Fields {
    r#type: AnalysisCompletionReceiptV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    receipt_id: Uuid,
    manifest_digest: Hex<32>,
    unit_id: Identifier,
    attempt_binding_digest: Hex<32>,
    actual_analyzer_bundle_digest: Hex<32>,
    actual_configuration_digest: Hex<32>,
    actual_scope_digest: Hex<32>,
    coverage_digest: Hex<32>,
    artifact_digests: SetOf<Hex<32>>,
    finding_count: u32,
    outcome: AnalysisCompletionReceiptV1Outcome,
    producer_id: Uuid,
    started_at: I64Str,
    completed_at: I64Str,
    lease_generation: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<AnalysisCompletionReceiptV1Fields>>
    for AnalysisCompletionReceiptV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AnalysisCompletionReceiptV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            receipt_id: fields.receipt_id,
            manifest_digest: fields.manifest_digest,
            unit_id: fields.unit_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            actual_analyzer_bundle_digest: fields.actual_analyzer_bundle_digest,
            actual_configuration_digest: fields.actual_configuration_digest,
            actual_scope_digest: fields.actual_scope_digest,
            coverage_digest: fields.coverage_digest,
            artifact_digests: fields.artifact_digests,
            finding_count: fields.finding_count,
            outcome: fields.outcome,
            producer_id: fields.producer_id,
            started_at: fields.started_at,
            completed_at: fields.completed_at,
            lease_generation: fields.lease_generation,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AnalysisCompletionReceiptV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 18)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("receipt_id", &self.receipt_id)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("unit_id", &self.unit_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field(
            "actual_analyzer_bundle_digest",
            &self.actual_analyzer_bundle_digest,
        )?;
        state.serialize_field(
            "actual_configuration_digest",
            &self.actual_configuration_digest,
        )?;
        state.serialize_field("actual_scope_digest", &self.actual_scope_digest)?;
        state.serialize_field("coverage_digest", &self.coverage_digest)?;
        state.serialize_field("artifact_digests", &self.artifact_digests)?;
        state.serialize_field("finding_count", &self.finding_count)?;
        state.serialize_field("outcome", &self.outcome)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("started_at", &self.started_at)?;
        state.serialize_field("completed_at", &self.completed_at)?;
        state.serialize_field("lease_generation", &self.lease_generation)?;
        state.end()
    }
}
impl AnalysisCompletionReceiptV1 {
    pub const TYPE_NAME: &'static str = "AnalysisCompletionReceiptV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "receipt_id",
        "manifest_digest",
        "unit_id",
        "attempt_binding_digest",
        "actual_analyzer_bundle_digest",
        "actual_configuration_digest",
        "actual_scope_digest",
        "coverage_digest",
        "artifact_digests",
        "finding_count",
        "outcome",
        "producer_id",
        "started_at",
        "completed_at",
        "lease_generation",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AnalysisCompletionReceiptV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.receipt_id.canonical_cmp(&other.receipt_id))
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| self.unit_id.canonical_cmp(&other.unit_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| {
                self.actual_analyzer_bundle_digest
                    .canonical_cmp(&other.actual_analyzer_bundle_digest)
            })
            .then_with(|| {
                self.actual_configuration_digest
                    .canonical_cmp(&other.actual_configuration_digest)
            })
            .then_with(|| {
                self.actual_scope_digest
                    .canonical_cmp(&other.actual_scope_digest)
            })
            .then_with(|| self.coverage_digest.canonical_cmp(&other.coverage_digest))
            .then_with(|| self.artifact_digests.canonical_cmp(&other.artifact_digests))
            .then_with(|| self.finding_count.canonical_cmp(&other.finding_count))
            .then_with(|| self.outcome.canonical_cmp(&other.outcome))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| self.started_at.canonical_cmp(&other.started_at))
            .then_with(|| self.completed_at.canonical_cmp(&other.completed_at))
            .then_with(|| self.lease_generation.canonical_cmp(&other.lease_generation))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(18u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.receipt_id.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.unit_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.actual_analyzer_bundle_digest.canonical_size())
            .saturating_add(self.actual_configuration_digest.canonical_size())
            .saturating_add(self.actual_scope_digest.canonical_size())
            .saturating_add(self.coverage_digest.canonical_size())
            .saturating_add(self.artifact_digests.canonical_size())
            .saturating_add(self.finding_count.canonical_size())
            .saturating_add(self.outcome.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.started_at.canonical_size())
            .saturating_add(self.completed_at.canonical_size())
            .saturating_add(self.lease_generation.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.receipt_id.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.unit_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.actual_analyzer_bundle_digest.canonical_nesting())
                .max(self.actual_configuration_digest.canonical_nesting())
                .max(self.actual_scope_digest.canonical_nesting())
                .max(self.coverage_digest.canonical_nesting())
                .max(self.artifact_digests.canonical_nesting())
                .max(self.finding_count.canonical_nesting())
                .max(self.outcome.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.started_at.canonical_nesting())
                .max(self.completed_at.canonical_nesting())
                .max(self.lease_generation.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.unit_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisPlanV1Type {
    #[serde(rename = "AnalysisPlanV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AnalysisPlanV1Fields>"
)]
pub struct AnalysisPlanV1 {
    pub r#type: AnalysisPlanV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub plan_id: Uuid,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub capability_digest: Hex<32>,
    pub required_units: SetOf<AnalysisUnitV1>,
    pub planner_id: Uuid,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisPlanV1Fields {
    r#type: AnalysisPlanV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    plan_id: Uuid,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    capability_digest: Hex<32>,
    required_units: SetOf<AnalysisUnitV1>,
    planner_id: Uuid,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<AnalysisPlanV1Fields>> for AnalysisPlanV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AnalysisPlanV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            plan_id: fields.plan_id,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            capability_digest: fields.capability_digest,
            required_units: fields.required_units,
            planner_id: fields.planner_id,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AnalysisPlanV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("plan_id", &self.plan_id)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.serialize_field("required_units", &self.required_units)?;
        state.serialize_field("planner_id", &self.planner_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl AnalysisPlanV1 {
    pub const TYPE_NAME: &'static str = "AnalysisPlanV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "plan_id",
        "subject_digest",
        "policy_digest",
        "capability_digest",
        "required_units",
        "planner_id",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AnalysisPlanV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.plan_id.canonical_cmp(&other.plan_id))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
            .then_with(|| self.required_units.canonical_cmp(&other.required_units))
            .then_with(|| self.planner_id.canonical_cmp(&other.planner_id))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.plan_id.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
            .saturating_add(self.required_units.canonical_size())
            .saturating_add(self.planner_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.plan_id.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting())
                .max(self.required_units.canonical_nesting())
                .max(self.planner_id.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisUnitV1Type {
    #[serde(rename = "AnalysisUnitV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AnalysisUnitV1Fields>"
)]
pub struct AnalysisUnitV1 {
    pub r#type: AnalysisUnitV1Type,
    pub wire_revision: WireRevision,
    pub unit_id: Identifier,
    pub analyzer_id: Identifier,
    pub analyzer_bundle_digest: Hex<32>,
    pub effective_configuration_digest: Hex<32>,
    pub scopes_digest: Hex<32>,
    pub required_files_symbols_digest: Hex<32>,
    pub allowed_exclusions_digest: Hex<32>,
    pub sandbox_requirements_digest: Hex<32>,
    pub mandatory_build_test_units: SetOf<Identifier>,
    pub coverage_expectations_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisUnitV1Fields {
    r#type: AnalysisUnitV1Type,
    wire_revision: WireRevision,
    unit_id: Identifier,
    analyzer_id: Identifier,
    analyzer_bundle_digest: Hex<32>,
    effective_configuration_digest: Hex<32>,
    scopes_digest: Hex<32>,
    required_files_symbols_digest: Hex<32>,
    allowed_exclusions_digest: Hex<32>,
    sandbox_requirements_digest: Hex<32>,
    mandatory_build_test_units: SetOf<Identifier>,
    coverage_expectations_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<AnalysisUnitV1Fields>> for AnalysisUnitV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AnalysisUnitV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            unit_id: fields.unit_id,
            analyzer_id: fields.analyzer_id,
            analyzer_bundle_digest: fields.analyzer_bundle_digest,
            effective_configuration_digest: fields.effective_configuration_digest,
            scopes_digest: fields.scopes_digest,
            required_files_symbols_digest: fields.required_files_symbols_digest,
            allowed_exclusions_digest: fields.allowed_exclusions_digest,
            sandbox_requirements_digest: fields.sandbox_requirements_digest,
            mandatory_build_test_units: fields.mandatory_build_test_units,
            coverage_expectations_digest: fields.coverage_expectations_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AnalysisUnitV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("unit_id", &self.unit_id)?;
        state.serialize_field("analyzer_id", &self.analyzer_id)?;
        state.serialize_field("analyzer_bundle_digest", &self.analyzer_bundle_digest)?;
        state.serialize_field(
            "effective_configuration_digest",
            &self.effective_configuration_digest,
        )?;
        state.serialize_field("scopes_digest", &self.scopes_digest)?;
        state.serialize_field(
            "required_files_symbols_digest",
            &self.required_files_symbols_digest,
        )?;
        state.serialize_field("allowed_exclusions_digest", &self.allowed_exclusions_digest)?;
        state.serialize_field(
            "sandbox_requirements_digest",
            &self.sandbox_requirements_digest,
        )?;
        state.serialize_field(
            "mandatory_build_test_units",
            &self.mandatory_build_test_units,
        )?;
        state.serialize_field(
            "coverage_expectations_digest",
            &self.coverage_expectations_digest,
        )?;
        state.end()
    }
}
impl AnalysisUnitV1 {
    pub const TYPE_NAME: &'static str = "AnalysisUnitV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "unit_id",
        "analyzer_id",
        "analyzer_bundle_digest",
        "effective_configuration_digest",
        "scopes_digest",
        "required_files_symbols_digest",
        "allowed_exclusions_digest",
        "sandbox_requirements_digest",
        "mandatory_build_test_units",
        "coverage_expectations_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AnalysisUnitV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.unit_id.canonical_cmp(&other.unit_id))
            .then_with(|| self.analyzer_id.canonical_cmp(&other.analyzer_id))
            .then_with(|| {
                self.analyzer_bundle_digest
                    .canonical_cmp(&other.analyzer_bundle_digest)
            })
            .then_with(|| {
                self.effective_configuration_digest
                    .canonical_cmp(&other.effective_configuration_digest)
            })
            .then_with(|| self.scopes_digest.canonical_cmp(&other.scopes_digest))
            .then_with(|| {
                self.required_files_symbols_digest
                    .canonical_cmp(&other.required_files_symbols_digest)
            })
            .then_with(|| {
                self.allowed_exclusions_digest
                    .canonical_cmp(&other.allowed_exclusions_digest)
            })
            .then_with(|| {
                self.sandbox_requirements_digest
                    .canonical_cmp(&other.sandbox_requirements_digest)
            })
            .then_with(|| {
                self.mandatory_build_test_units
                    .canonical_cmp(&other.mandatory_build_test_units)
            })
            .then_with(|| {
                self.coverage_expectations_digest
                    .canonical_cmp(&other.coverage_expectations_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.unit_id.canonical_size())
            .saturating_add(self.analyzer_id.canonical_size())
            .saturating_add(self.analyzer_bundle_digest.canonical_size())
            .saturating_add(self.effective_configuration_digest.canonical_size())
            .saturating_add(self.scopes_digest.canonical_size())
            .saturating_add(self.required_files_symbols_digest.canonical_size())
            .saturating_add(self.allowed_exclusions_digest.canonical_size())
            .saturating_add(self.sandbox_requirements_digest.canonical_size())
            .saturating_add(self.mandatory_build_test_units.canonical_size())
            .saturating_add(self.coverage_expectations_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.unit_id
                .canonical_nesting()
                .max(self.analyzer_id.canonical_nesting())
                .max(self.analyzer_bundle_digest.canonical_nesting())
                .max(self.effective_configuration_digest.canonical_nesting())
                .max(self.scopes_digest.canonical_nesting())
                .max(self.required_files_symbols_digest.canonical_nesting())
                .max(self.allowed_exclusions_digest.canonical_nesting())
                .max(self.sandbox_requirements_digest.canonical_nesting())
                .max(self.mandatory_build_test_units.canonical_nesting())
                .max(self.coverage_expectations_digest.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.unit_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalChallengeV2Type {
    #[serde(rename = "ApprovalChallengeV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalChallengeV2Fields>"
)]
pub struct ApprovalChallengeV2 {
    pub r#type: ApprovalChallengeV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub challenge_id: Uuid,
    pub plan_digest: Hex<32>,
    pub display_receipt_digest: Hex<32>,
    pub credential_binding_id: Uuid,
    pub credential_generation: U64Str,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub nonce: Hex<32>,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalChallengeV2Fields {
    r#type: ApprovalChallengeV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    challenge_id: Uuid,
    plan_digest: Hex<32>,
    display_receipt_digest: Hex<32>,
    credential_binding_id: Uuid,
    credential_generation: U64Str,
    user_id: Uuid,
    session_id: Uuid,
    nonce: Hex<32>,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalChallengeV2Fields>> for ApprovalChallengeV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalChallengeV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            challenge_id: fields.challenge_id,
            plan_digest: fields.plan_digest,
            display_receipt_digest: fields.display_receipt_digest,
            credential_binding_id: fields.credential_binding_id,
            credential_generation: fields.credential_generation,
            user_id: fields.user_id,
            session_id: fields.session_id,
            nonce: fields.nonce,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalChallengeV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("challenge_id", &self.challenge_id)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("display_receipt_digest", &self.display_receipt_digest)?;
        state.serialize_field("credential_binding_id", &self.credential_binding_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalChallengeV2 {
    pub const TYPE_NAME: &'static str = "ApprovalChallengeV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "challenge_id",
        "plan_digest",
        "display_receipt_digest",
        "credential_binding_id",
        "credential_generation",
        "user_id",
        "session_id",
        "nonce",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalChallengeV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.challenge_id.canonical_cmp(&other.challenge_id))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| {
                self.display_receipt_digest
                    .canonical_cmp(&other.display_receipt_digest)
            })
            .then_with(|| {
                self.credential_binding_id
                    .canonical_cmp(&other.credential_binding_id)
            })
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.challenge_id.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.display_receipt_digest.canonical_size())
            .saturating_add(self.credential_binding_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.challenge_id.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.display_receipt_digest.canonical_nesting())
                .max(self.credential_binding_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalCredentialBindingV1State {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "REVOKED")]
    Revoked,
}
impl ApprovalCredentialBindingV1State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Active => 0,
            Self::Suspended => 1,
            Self::Revoked => 2,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Active),
            1 => Some(Self::Suspended),
            2 => Some(Self::Revoked),
            _ => None,
        }
    }
}
impl CanonicalOrder for ApprovalCredentialBindingV1State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalCredentialBindingV1Type {
    #[serde(rename = "ApprovalCredentialBindingV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalCredentialBindingV1Fields>"
)]
pub struct ApprovalCredentialBindingV1 {
    pub r#type: ApprovalCredentialBindingV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub binding_id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Bytes,
    pub public_key_cose: Bytes,
    pub rp_id: Hostname,
    pub allowed_origin: Origin,
    pub authorization_scope_digest: Hex<32>,
    pub registration_ceremony_digest: Hex<32>,
    pub registration_authority_id: Uuid,
    pub created_at: I64Str,
    pub state: ApprovalCredentialBindingV1State,
    pub credential_generation: U64Str,
    pub recovery_lineage: Vec<Hex<32>>,
    pub audit_record_id: Uuid,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalCredentialBindingV1Fields {
    r#type: ApprovalCredentialBindingV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    binding_id: Uuid,
    user_id: Uuid,
    credential_id: Bytes,
    public_key_cose: Bytes,
    rp_id: Hostname,
    allowed_origin: Origin,
    authorization_scope_digest: Hex<32>,
    registration_ceremony_digest: Hex<32>,
    registration_authority_id: Uuid,
    created_at: I64Str,
    state: ApprovalCredentialBindingV1State,
    credential_generation: U64Str,
    #[serde(deserialize_with = "crate::primitives::bounded_array")]
    recovery_lineage: Vec<Hex<32>>,
    audit_record_id: Uuid,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalCredentialBindingV1Fields>>
    for ApprovalCredentialBindingV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalCredentialBindingV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            binding_id: fields.binding_id,
            user_id: fields.user_id,
            credential_id: fields.credential_id,
            public_key_cose: fields.public_key_cose,
            rp_id: fields.rp_id,
            allowed_origin: fields.allowed_origin,
            authorization_scope_digest: fields.authorization_scope_digest,
            registration_ceremony_digest: fields.registration_ceremony_digest,
            registration_authority_id: fields.registration_authority_id,
            created_at: fields.created_at,
            state: fields.state,
            credential_generation: fields.credential_generation,
            recovery_lineage: fields.recovery_lineage,
            audit_record_id: fields.audit_record_id,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalCredentialBindingV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 17)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("binding_id", &self.binding_id)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("credential_id", &self.credential_id)?;
        state.serialize_field("public_key_cose", &self.public_key_cose)?;
        state.serialize_field("rp_id", &self.rp_id)?;
        state.serialize_field("allowed_origin", &self.allowed_origin)?;
        state.serialize_field(
            "authorization_scope_digest",
            &self.authorization_scope_digest,
        )?;
        state.serialize_field(
            "registration_ceremony_digest",
            &self.registration_ceremony_digest,
        )?;
        state.serialize_field("registration_authority_id", &self.registration_authority_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field(
            "recovery_lineage",
            &crate::primitives::BoundedArrayRef(&self.recovery_lineage),
        )?;
        state.serialize_field("audit_record_id", &self.audit_record_id)?;
        state.end()
    }
}
impl ApprovalCredentialBindingV1 {
    pub const TYPE_NAME: &'static str = "ApprovalCredentialBindingV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "binding_id",
        "user_id",
        "credential_id",
        "public_key_cose",
        "rp_id",
        "allowed_origin",
        "authorization_scope_digest",
        "registration_ceremony_digest",
        "registration_authority_id",
        "created_at",
        "state",
        "credential_generation",
        "recovery_lineage",
        "audit_record_id",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalCredentialBindingV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.binding_id.canonical_cmp(&other.binding_id))
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| self.credential_id.canonical_cmp(&other.credential_id))
            .then_with(|| self.public_key_cose.canonical_cmp(&other.public_key_cose))
            .then_with(|| self.rp_id.canonical_cmp(&other.rp_id))
            .then_with(|| self.allowed_origin.canonical_cmp(&other.allowed_origin))
            .then_with(|| {
                self.authorization_scope_digest
                    .canonical_cmp(&other.authorization_scope_digest)
            })
            .then_with(|| {
                self.registration_ceremony_digest
                    .canonical_cmp(&other.registration_ceremony_digest)
            })
            .then_with(|| {
                self.registration_authority_id
                    .canonical_cmp(&other.registration_authority_id)
            })
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.recovery_lineage.canonical_cmp(&other.recovery_lineage))
            .then_with(|| self.audit_record_id.canonical_cmp(&other.audit_record_id))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(17u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.binding_id.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.credential_id.canonical_size())
            .saturating_add(self.public_key_cose.canonical_size())
            .saturating_add(self.rp_id.canonical_size())
            .saturating_add(self.allowed_origin.canonical_size())
            .saturating_add(self.authorization_scope_digest.canonical_size())
            .saturating_add(self.registration_ceremony_digest.canonical_size())
            .saturating_add(self.registration_authority_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.recovery_lineage.canonical_size())
            .saturating_add(self.audit_record_id.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.binding_id.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.credential_id.canonical_nesting())
                .max(self.public_key_cose.canonical_nesting())
                .max(self.rp_id.canonical_nesting())
                .max(self.allowed_origin.canonical_nesting())
                .max(self.authorization_scope_digest.canonical_nesting())
                .max(self.registration_ceremony_digest.canonical_nesting())
                .max(self.registration_authority_id.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.recovery_lineage.canonical_nesting())
                .max(self.audit_record_id.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalEnrollmentActionV1Type {
    #[serde(rename = "ApprovalEnrollmentActionV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalEnrollmentActionV1Fields>"
)]
pub struct ApprovalEnrollmentActionV1 {
    pub r#type: ApprovalEnrollmentActionV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub authority_epoch: Uuid,
    pub action_id: Uuid,
    pub intent_digest: Hex<32>,
    pub registration_ceremony_digest: Opt<Hex<32>>,
    pub new_credential_id: Bytes,
    pub new_public_key_cose: Bytes,
    pub registration_authority_id: Uuid,
    pub created_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalEnrollmentActionV1Fields {
    r#type: ApprovalEnrollmentActionV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    authority_epoch: Uuid,
    action_id: Uuid,
    intent_digest: Hex<32>,
    registration_ceremony_digest: Opt<Hex<32>>,
    new_credential_id: Bytes,
    new_public_key_cose: Bytes,
    registration_authority_id: Uuid,
    created_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalEnrollmentActionV1Fields>>
    for ApprovalEnrollmentActionV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalEnrollmentActionV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            authority_epoch: fields.authority_epoch,
            action_id: fields.action_id,
            intent_digest: fields.intent_digest,
            registration_ceremony_digest: fields.registration_ceremony_digest,
            new_credential_id: fields.new_credential_id,
            new_public_key_cose: fields.new_public_key_cose,
            registration_authority_id: fields.registration_authority_id,
            created_at: fields.created_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalEnrollmentActionV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("action_id", &self.action_id)?;
        state.serialize_field("intent_digest", &self.intent_digest)?;
        state.serialize_field(
            "registration_ceremony_digest",
            &self.registration_ceremony_digest,
        )?;
        state.serialize_field("new_credential_id", &self.new_credential_id)?;
        state.serialize_field("new_public_key_cose", &self.new_public_key_cose)?;
        state.serialize_field("registration_authority_id", &self.registration_authority_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalEnrollmentActionV1 {
    pub const TYPE_NAME: &'static str = "ApprovalEnrollmentActionV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "authority_epoch",
        "action_id",
        "intent_digest",
        "registration_ceremony_digest",
        "new_credential_id",
        "new_public_key_cose",
        "registration_authority_id",
        "created_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalEnrollmentActionV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.action_id.canonical_cmp(&other.action_id))
            .then_with(|| self.intent_digest.canonical_cmp(&other.intent_digest))
            .then_with(|| {
                self.registration_ceremony_digest
                    .canonical_cmp(&other.registration_ceremony_digest)
            })
            .then_with(|| {
                self.new_credential_id
                    .canonical_cmp(&other.new_credential_id)
            })
            .then_with(|| {
                self.new_public_key_cose
                    .canonical_cmp(&other.new_public_key_cose)
            })
            .then_with(|| {
                self.registration_authority_id
                    .canonical_cmp(&other.registration_authority_id)
            })
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.action_id.canonical_size())
            .saturating_add(self.intent_digest.canonical_size())
            .saturating_add(self.registration_ceremony_digest.canonical_size())
            .saturating_add(self.new_credential_id.canonical_size())
            .saturating_add(self.new_public_key_cose.canonical_size())
            .saturating_add(self.registration_authority_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.authority_epoch.canonical_nesting())
                .max(self.action_id.canonical_nesting())
                .max(self.intent_digest.canonical_nesting())
                .max(self.registration_ceremony_digest.canonical_nesting())
                .max(self.new_credential_id.canonical_nesting())
                .max(self.new_public_key_cose.canonical_nesting())
                .max(self.registration_authority_id.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalEnrollmentAuthorizationV1Type {
    #[serde(rename = "ApprovalEnrollmentAuthorizationV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalEnrollmentAuthorizationV1Fields>"
)]
pub struct ApprovalEnrollmentAuthorizationV1 {
    pub r#type: ApprovalEnrollmentAuthorizationV1Type,
    pub wire_revision: WireRevision,
    pub action_digest: Hex<32>,
    pub recovery_policy_digest: Hex<32>,
    pub consent_receipt_digests: SetOf<Hex<32>>,
    pub authority_signature_envelope_digests: SetOf<Hex<32>>,
    pub enroller_id: Uuid,
    pub committed_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalEnrollmentAuthorizationV1Fields {
    r#type: ApprovalEnrollmentAuthorizationV1Type,
    wire_revision: WireRevision,
    action_digest: Hex<32>,
    recovery_policy_digest: Hex<32>,
    consent_receipt_digests: SetOf<Hex<32>>,
    authority_signature_envelope_digests: SetOf<Hex<32>>,
    enroller_id: Uuid,
    committed_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalEnrollmentAuthorizationV1Fields>>
    for ApprovalEnrollmentAuthorizationV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalEnrollmentAuthorizationV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            action_digest: fields.action_digest,
            recovery_policy_digest: fields.recovery_policy_digest,
            consent_receipt_digests: fields.consent_receipt_digests,
            authority_signature_envelope_digests: fields.authority_signature_envelope_digests,
            enroller_id: fields.enroller_id,
            committed_at: fields.committed_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalEnrollmentAuthorizationV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("action_digest", &self.action_digest)?;
        state.serialize_field("recovery_policy_digest", &self.recovery_policy_digest)?;
        state.serialize_field("consent_receipt_digests", &self.consent_receipt_digests)?;
        state.serialize_field(
            "authority_signature_envelope_digests",
            &self.authority_signature_envelope_digests,
        )?;
        state.serialize_field("enroller_id", &self.enroller_id)?;
        state.serialize_field("committed_at", &self.committed_at)?;
        state.end()
    }
}
impl ApprovalEnrollmentAuthorizationV1 {
    pub const TYPE_NAME: &'static str = "ApprovalEnrollmentAuthorizationV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "action_digest",
        "recovery_policy_digest",
        "consent_receipt_digests",
        "authority_signature_envelope_digests",
        "enroller_id",
        "committed_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalEnrollmentAuthorizationV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.action_digest.canonical_cmp(&other.action_digest))
            .then_with(|| {
                self.recovery_policy_digest
                    .canonical_cmp(&other.recovery_policy_digest)
            })
            .then_with(|| {
                self.consent_receipt_digests
                    .canonical_cmp(&other.consent_receipt_digests)
            })
            .then_with(|| {
                self.authority_signature_envelope_digests
                    .canonical_cmp(&other.authority_signature_envelope_digests)
            })
            .then_with(|| self.enroller_id.canonical_cmp(&other.enroller_id))
            .then_with(|| self.committed_at.canonical_cmp(&other.committed_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.action_digest.canonical_size())
            .saturating_add(self.recovery_policy_digest.canonical_size())
            .saturating_add(self.consent_receipt_digests.canonical_size())
            .saturating_add(self.authority_signature_envelope_digests.canonical_size())
            .saturating_add(self.enroller_id.canonical_size())
            .saturating_add(self.committed_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.action_digest
                .canonical_nesting()
                .max(self.recovery_policy_digest.canonical_nesting())
                .max(self.consent_receipt_digests.canonical_nesting())
                .max(
                    self.authority_signature_envelope_digests
                        .canonical_nesting(),
                )
                .max(self.enroller_id.canonical_nesting())
                .max(self.committed_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalEnrollmentChallengeV1Type {
    #[serde(rename = "ApprovalEnrollmentChallengeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalEnrollmentChallengeV1Fields>"
)]
pub struct ApprovalEnrollmentChallengeV1 {
    pub r#type: ApprovalEnrollmentChallengeV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub authority_epoch: Uuid,
    pub challenge_id: Uuid,
    pub intent_digest: Hex<32>,
    pub user_id: Uuid,
    pub rp_id: Hostname,
    pub allowed_origin: Origin,
    pub session_id: Uuid,
    pub nonce: Hex<32>,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalEnrollmentChallengeV1Fields {
    r#type: ApprovalEnrollmentChallengeV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    authority_epoch: Uuid,
    challenge_id: Uuid,
    intent_digest: Hex<32>,
    user_id: Uuid,
    rp_id: Hostname,
    allowed_origin: Origin,
    session_id: Uuid,
    nonce: Hex<32>,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalEnrollmentChallengeV1Fields>>
    for ApprovalEnrollmentChallengeV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalEnrollmentChallengeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            authority_epoch: fields.authority_epoch,
            challenge_id: fields.challenge_id,
            intent_digest: fields.intent_digest,
            user_id: fields.user_id,
            rp_id: fields.rp_id,
            allowed_origin: fields.allowed_origin,
            session_id: fields.session_id,
            nonce: fields.nonce,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalEnrollmentChallengeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("challenge_id", &self.challenge_id)?;
        state.serialize_field("intent_digest", &self.intent_digest)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("rp_id", &self.rp_id)?;
        state.serialize_field("allowed_origin", &self.allowed_origin)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalEnrollmentChallengeV1 {
    pub const TYPE_NAME: &'static str = "ApprovalEnrollmentChallengeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "authority_epoch",
        "challenge_id",
        "intent_digest",
        "user_id",
        "rp_id",
        "allowed_origin",
        "session_id",
        "nonce",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalEnrollmentChallengeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.challenge_id.canonical_cmp(&other.challenge_id))
            .then_with(|| self.intent_digest.canonical_cmp(&other.intent_digest))
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| self.rp_id.canonical_cmp(&other.rp_id))
            .then_with(|| self.allowed_origin.canonical_cmp(&other.allowed_origin))
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.challenge_id.canonical_size())
            .saturating_add(self.intent_digest.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.rp_id.canonical_size())
            .saturating_add(self.allowed_origin.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.authority_epoch.canonical_nesting())
                .max(self.challenge_id.canonical_nesting())
                .max(self.intent_digest.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.rp_id.canonical_nesting())
                .max(self.allowed_origin.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalEnrollmentIntentV1Operation {
    #[serde(rename = "INITIAL_BOOTSTRAP")]
    InitialBootstrap,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "REVOKE")]
    Revoke,
    #[serde(rename = "RECOVER")]
    Recover,
    #[serde(rename = "CHANGE_BINDING")]
    ChangeBinding,
    #[serde(rename = "INITIAL_REGISTRATION")]
    InitialRegistration,
    #[serde(rename = "SUSPEND")]
    Suspend,
}
impl ApprovalEnrollmentIntentV1Operation {
    pub fn code(&self) -> u64 {
        match self {
            Self::InitialBootstrap => 0,
            Self::Add => 1,
            Self::Replace => 2,
            Self::Revoke => 3,
            Self::Recover => 4,
            Self::ChangeBinding => 5,
            Self::InitialRegistration => 6,
            Self::Suspend => 7,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::InitialBootstrap),
            1 => Some(Self::Add),
            2 => Some(Self::Replace),
            3 => Some(Self::Revoke),
            4 => Some(Self::Recover),
            5 => Some(Self::ChangeBinding),
            6 => Some(Self::InitialRegistration),
            7 => Some(Self::Suspend),
            _ => None,
        }
    }
}
impl CanonicalOrder for ApprovalEnrollmentIntentV1Operation {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalEnrollmentIntentV1Type {
    #[serde(rename = "ApprovalEnrollmentIntentV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalEnrollmentIntentV1Fields>"
)]
pub struct ApprovalEnrollmentIntentV1 {
    pub r#type: ApprovalEnrollmentIntentV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub authority_epoch: Uuid,
    pub intent_id: Uuid,
    pub user_id: Uuid,
    pub authorization_scope_digest: Hex<32>,
    pub operation: ApprovalEnrollmentIntentV1Operation,
    pub previous_binding_digest: Opt<Hex<32>>,
    pub recovery_policy_digest: Hex<32>,
    pub session_id: Uuid,
    pub nonce: Hex<32>,
    pub created_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalEnrollmentIntentV1Fields {
    r#type: ApprovalEnrollmentIntentV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    authority_epoch: Uuid,
    intent_id: Uuid,
    user_id: Uuid,
    authorization_scope_digest: Hex<32>,
    operation: ApprovalEnrollmentIntentV1Operation,
    previous_binding_digest: Opt<Hex<32>>,
    recovery_policy_digest: Hex<32>,
    session_id: Uuid,
    nonce: Hex<32>,
    created_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalEnrollmentIntentV1Fields>>
    for ApprovalEnrollmentIntentV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalEnrollmentIntentV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            authority_epoch: fields.authority_epoch,
            intent_id: fields.intent_id,
            user_id: fields.user_id,
            authorization_scope_digest: fields.authorization_scope_digest,
            operation: fields.operation,
            previous_binding_digest: fields.previous_binding_digest,
            recovery_policy_digest: fields.recovery_policy_digest,
            session_id: fields.session_id,
            nonce: fields.nonce,
            created_at: fields.created_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalEnrollmentIntentV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 14)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("intent_id", &self.intent_id)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field(
            "authorization_scope_digest",
            &self.authorization_scope_digest,
        )?;
        state.serialize_field("operation", &self.operation)?;
        state.serialize_field("previous_binding_digest", &self.previous_binding_digest)?;
        state.serialize_field("recovery_policy_digest", &self.recovery_policy_digest)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalEnrollmentIntentV1 {
    pub const TYPE_NAME: &'static str = "ApprovalEnrollmentIntentV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "authority_epoch",
        "intent_id",
        "user_id",
        "authorization_scope_digest",
        "operation",
        "previous_binding_digest",
        "recovery_policy_digest",
        "session_id",
        "nonce",
        "created_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalEnrollmentIntentV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.intent_id.canonical_cmp(&other.intent_id))
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| {
                self.authorization_scope_digest
                    .canonical_cmp(&other.authorization_scope_digest)
            })
            .then_with(|| self.operation.canonical_cmp(&other.operation))
            .then_with(|| {
                self.previous_binding_digest
                    .canonical_cmp(&other.previous_binding_digest)
            })
            .then_with(|| {
                self.recovery_policy_digest
                    .canonical_cmp(&other.recovery_policy_digest)
            })
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(14u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.intent_id.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.authorization_scope_digest.canonical_size())
            .saturating_add(self.operation.canonical_size())
            .saturating_add(self.previous_binding_digest.canonical_size())
            .saturating_add(self.recovery_policy_digest.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.authority_epoch.canonical_nesting())
                .max(self.intent_id.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.authorization_scope_digest.canonical_nesting())
                .max(self.operation.canonical_nesting())
                .max(self.previous_binding_digest.canonical_nesting())
                .max(self.recovery_policy_digest.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalLifecycleChallengeV1Purpose {
    #[serde(rename = "OWNER_CONSENT")]
    OwnerConsent,
    #[serde(rename = "ADMIN_CONSENT")]
    AdminConsent,
    #[serde(rename = "SECURITY_SUSPEND")]
    SecuritySuspend,
}
impl ApprovalLifecycleChallengeV1Purpose {
    pub fn code(&self) -> u64 {
        match self {
            Self::OwnerConsent => 0,
            Self::AdminConsent => 1,
            Self::SecuritySuspend => 2,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::OwnerConsent),
            1 => Some(Self::AdminConsent),
            2 => Some(Self::SecuritySuspend),
            _ => None,
        }
    }
}
impl CanonicalOrder for ApprovalLifecycleChallengeV1Purpose {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalLifecycleChallengeV1Type {
    #[serde(rename = "ApprovalLifecycleChallengeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalLifecycleChallengeV1Fields>"
)]
pub struct ApprovalLifecycleChallengeV1 {
    pub r#type: ApprovalLifecycleChallengeV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub authority_epoch: Uuid,
    pub challenge_id: Uuid,
    pub action_digest: Hex<32>,
    pub actor_id: Uuid,
    pub credential_binding_id: Uuid,
    pub credential_generation: U64Str,
    pub session_id: Uuid,
    pub nonce: Hex<32>,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
    pub purpose: ApprovalLifecycleChallengeV1Purpose,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalLifecycleChallengeV1Fields {
    r#type: ApprovalLifecycleChallengeV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    authority_epoch: Uuid,
    challenge_id: Uuid,
    action_digest: Hex<32>,
    actor_id: Uuid,
    credential_binding_id: Uuid,
    credential_generation: U64Str,
    session_id: Uuid,
    nonce: Hex<32>,
    issued_at: I64Str,
    expires_at: I64Str,
    purpose: ApprovalLifecycleChallengeV1Purpose,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalLifecycleChallengeV1Fields>>
    for ApprovalLifecycleChallengeV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalLifecycleChallengeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            authority_epoch: fields.authority_epoch,
            challenge_id: fields.challenge_id,
            action_digest: fields.action_digest,
            actor_id: fields.actor_id,
            credential_binding_id: fields.credential_binding_id,
            credential_generation: fields.credential_generation,
            session_id: fields.session_id,
            nonce: fields.nonce,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
            purpose: fields.purpose,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalLifecycleChallengeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 14)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("challenge_id", &self.challenge_id)?;
        state.serialize_field("action_digest", &self.action_digest)?;
        state.serialize_field("actor_id", &self.actor_id)?;
        state.serialize_field("credential_binding_id", &self.credential_binding_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("purpose", &self.purpose)?;
        state.end()
    }
}
impl ApprovalLifecycleChallengeV1 {
    pub const TYPE_NAME: &'static str = "ApprovalLifecycleChallengeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "authority_epoch",
        "challenge_id",
        "action_digest",
        "actor_id",
        "credential_binding_id",
        "credential_generation",
        "session_id",
        "nonce",
        "issued_at",
        "expires_at",
        "purpose",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalLifecycleChallengeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.challenge_id.canonical_cmp(&other.challenge_id))
            .then_with(|| self.action_digest.canonical_cmp(&other.action_digest))
            .then_with(|| self.actor_id.canonical_cmp(&other.actor_id))
            .then_with(|| {
                self.credential_binding_id
                    .canonical_cmp(&other.credential_binding_id)
            })
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| self.purpose.canonical_cmp(&other.purpose))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(14u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.challenge_id.canonical_size())
            .saturating_add(self.action_digest.canonical_size())
            .saturating_add(self.actor_id.canonical_size())
            .saturating_add(self.credential_binding_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.purpose.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.authority_epoch.canonical_nesting())
                .max(self.challenge_id.canonical_nesting())
                .max(self.action_digest.canonical_nesting())
                .max(self.actor_id.canonical_nesting())
                .max(self.credential_binding_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.purpose.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalLifecycleConsentV1Purpose {
    #[serde(rename = "OWNER_CONSENT")]
    OwnerConsent,
    #[serde(rename = "ADMIN_CONSENT")]
    AdminConsent,
    #[serde(rename = "SECURITY_SUSPEND")]
    SecuritySuspend,
}
impl ApprovalLifecycleConsentV1Purpose {
    pub fn code(&self) -> u64 {
        match self {
            Self::OwnerConsent => 0,
            Self::AdminConsent => 1,
            Self::SecuritySuspend => 2,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::OwnerConsent),
            1 => Some(Self::AdminConsent),
            2 => Some(Self::SecuritySuspend),
            _ => None,
        }
    }
}
impl CanonicalOrder for ApprovalLifecycleConsentV1Purpose {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalLifecycleConsentV1Type {
    #[serde(rename = "ApprovalLifecycleConsentV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalLifecycleConsentV1Fields>"
)]
pub struct ApprovalLifecycleConsentV1 {
    pub r#type: ApprovalLifecycleConsentV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub authority_epoch: Uuid,
    pub action_digest: Hex<32>,
    pub challenge_digest: Hex<32>,
    pub ceremony_digest: Hex<32>,
    pub actor_id: Uuid,
    pub credential_binding_id: Uuid,
    pub credential_generation: U64Str,
    pub purpose: ApprovalLifecycleConsentV1Purpose,
    pub approval_authority_id: Uuid,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalLifecycleConsentV1Fields {
    r#type: ApprovalLifecycleConsentV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    authority_epoch: Uuid,
    action_digest: Hex<32>,
    challenge_digest: Hex<32>,
    ceremony_digest: Hex<32>,
    actor_id: Uuid,
    credential_binding_id: Uuid,
    credential_generation: U64Str,
    purpose: ApprovalLifecycleConsentV1Purpose,
    approval_authority_id: Uuid,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalLifecycleConsentV1Fields>>
    for ApprovalLifecycleConsentV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalLifecycleConsentV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            authority_epoch: fields.authority_epoch,
            action_digest: fields.action_digest,
            challenge_digest: fields.challenge_digest,
            ceremony_digest: fields.ceremony_digest,
            actor_id: fields.actor_id,
            credential_binding_id: fields.credential_binding_id,
            credential_generation: fields.credential_generation,
            purpose: fields.purpose,
            approval_authority_id: fields.approval_authority_id,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalLifecycleConsentV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 14)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("action_digest", &self.action_digest)?;
        state.serialize_field("challenge_digest", &self.challenge_digest)?;
        state.serialize_field("ceremony_digest", &self.ceremony_digest)?;
        state.serialize_field("actor_id", &self.actor_id)?;
        state.serialize_field("credential_binding_id", &self.credential_binding_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("purpose", &self.purpose)?;
        state.serialize_field("approval_authority_id", &self.approval_authority_id)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalLifecycleConsentV1 {
    pub const TYPE_NAME: &'static str = "ApprovalLifecycleConsentV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "authority_epoch",
        "action_digest",
        "challenge_digest",
        "ceremony_digest",
        "actor_id",
        "credential_binding_id",
        "credential_generation",
        "purpose",
        "approval_authority_id",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalLifecycleConsentV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.action_digest.canonical_cmp(&other.action_digest))
            .then_with(|| self.challenge_digest.canonical_cmp(&other.challenge_digest))
            .then_with(|| self.ceremony_digest.canonical_cmp(&other.ceremony_digest))
            .then_with(|| self.actor_id.canonical_cmp(&other.actor_id))
            .then_with(|| {
                self.credential_binding_id
                    .canonical_cmp(&other.credential_binding_id)
            })
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.purpose.canonical_cmp(&other.purpose))
            .then_with(|| {
                self.approval_authority_id
                    .canonical_cmp(&other.approval_authority_id)
            })
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(14u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.action_digest.canonical_size())
            .saturating_add(self.challenge_digest.canonical_size())
            .saturating_add(self.ceremony_digest.canonical_size())
            .saturating_add(self.actor_id.canonical_size())
            .saturating_add(self.credential_binding_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.purpose.canonical_size())
            .saturating_add(self.approval_authority_id.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.authority_epoch.canonical_nesting())
                .max(self.action_digest.canonical_nesting())
                .max(self.challenge_digest.canonical_nesting())
                .max(self.ceremony_digest.canonical_nesting())
                .max(self.actor_id.canonical_nesting())
                .max(self.credential_binding_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.purpose.canonical_nesting())
                .max(self.approval_authority_id.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalRecoveryPolicyV1Profile {
    #[serde(rename = "DUAL_ADMIN")]
    DualAdmin,
    #[serde(rename = "INDIVIDUAL_OFFLINE")]
    IndividualOffline,
}
impl ApprovalRecoveryPolicyV1Profile {
    pub fn code(&self) -> u64 {
        match self {
            Self::DualAdmin => 0,
            Self::IndividualOffline => 1,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::DualAdmin),
            1 => Some(Self::IndividualOffline),
            _ => None,
        }
    }
}
impl CanonicalOrder for ApprovalRecoveryPolicyV1Profile {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalRecoveryPolicyV1Type {
    #[serde(rename = "ApprovalRecoveryPolicyV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ApprovalRecoveryPolicyV1Fields>"
)]
pub struct ApprovalRecoveryPolicyV1 {
    pub r#type: ApprovalRecoveryPolicyV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub policy_id: Uuid,
    pub generation: U64Str,
    pub profile: ApprovalRecoveryPolicyV1Profile,
    pub registration_authority_ids: SetOf<Uuid>,
    pub minimum_authorities: u16,
    pub offline_recovery_key_id: Opt<Uuid>,
    pub provider_ownership_evidence_digest: Hex<32>,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ApprovalRecoveryPolicyV1Fields {
    r#type: ApprovalRecoveryPolicyV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    policy_id: Uuid,
    generation: U64Str,
    profile: ApprovalRecoveryPolicyV1Profile,
    registration_authority_ids: SetOf<Uuid>,
    minimum_authorities: u16,
    offline_recovery_key_id: Opt<Uuid>,
    provider_ownership_evidence_digest: Hex<32>,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ApprovalRecoveryPolicyV1Fields>>
    for ApprovalRecoveryPolicyV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ApprovalRecoveryPolicyV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            policy_id: fields.policy_id,
            generation: fields.generation,
            profile: fields.profile,
            registration_authority_ids: fields.registration_authority_ids,
            minimum_authorities: fields.minimum_authorities,
            offline_recovery_key_id: fields.offline_recovery_key_id,
            provider_ownership_evidence_digest: fields.provider_ownership_evidence_digest,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ApprovalRecoveryPolicyV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("policy_id", &self.policy_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("profile", &self.profile)?;
        state.serialize_field(
            "registration_authority_ids",
            &self.registration_authority_ids,
        )?;
        state.serialize_field("minimum_authorities", &self.minimum_authorities)?;
        state.serialize_field("offline_recovery_key_id", &self.offline_recovery_key_id)?;
        state.serialize_field(
            "provider_ownership_evidence_digest",
            &self.provider_ownership_evidence_digest,
        )?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ApprovalRecoveryPolicyV1 {
    pub const TYPE_NAME: &'static str = "ApprovalRecoveryPolicyV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "policy_id",
        "generation",
        "profile",
        "registration_authority_ids",
        "minimum_authorities",
        "offline_recovery_key_id",
        "provider_ownership_evidence_digest",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ApprovalRecoveryPolicyV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.policy_id.canonical_cmp(&other.policy_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.profile.canonical_cmp(&other.profile))
            .then_with(|| {
                self.registration_authority_ids
                    .canonical_cmp(&other.registration_authority_ids)
            })
            .then_with(|| {
                self.minimum_authorities
                    .canonical_cmp(&other.minimum_authorities)
            })
            .then_with(|| {
                self.offline_recovery_key_id
                    .canonical_cmp(&other.offline_recovery_key_id)
            })
            .then_with(|| {
                self.provider_ownership_evidence_digest
                    .canonical_cmp(&other.provider_ownership_evidence_digest)
            })
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.policy_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.profile.canonical_size())
            .saturating_add(self.registration_authority_ids.canonical_size())
            .saturating_add(self.minimum_authorities.canonical_size())
            .saturating_add(self.offline_recovery_key_id.canonical_size())
            .saturating_add(self.provider_ownership_evidence_digest.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.policy_id.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.profile.canonical_nesting())
                .max(self.registration_authority_ids.canonical_nesting())
                .max(self.minimum_authorities.canonical_nesting())
                .max(self.offline_recovery_key_id.canonical_nesting())
                .max(self.provider_ownership_evidence_digest.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactCustodyEntryV1Type {
    #[serde(rename = "ArtifactCustodyEntryV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ArtifactCustodyEntryV1Fields>"
)]
pub struct ArtifactCustodyEntryV1 {
    pub r#type: ArtifactCustodyEntryV1Type,
    pub wire_revision: WireRevision,
    pub artifact_digest: Hex<32>,
    pub byte_length: U64Str,
    pub location: Identifier,
    pub region: Identifier,
    pub encryption_key_ref: Identifier,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactCustodyEntryV1Fields {
    r#type: ArtifactCustodyEntryV1Type,
    wire_revision: WireRevision,
    artifact_digest: Hex<32>,
    byte_length: U64Str,
    location: Identifier,
    region: Identifier,
    encryption_key_ref: Identifier,
}
impl TryFrom<crate::primitives::ObjectOnly<ArtifactCustodyEntryV1Fields>>
    for ArtifactCustodyEntryV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ArtifactCustodyEntryV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            artifact_digest: fields.artifact_digest,
            byte_length: fields.byte_length,
            location: fields.location,
            region: fields.region,
            encryption_key_ref: fields.encryption_key_ref,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ArtifactCustodyEntryV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("artifact_digest", &self.artifact_digest)?;
        state.serialize_field("byte_length", &self.byte_length)?;
        state.serialize_field("location", &self.location)?;
        state.serialize_field("region", &self.region)?;
        state.serialize_field("encryption_key_ref", &self.encryption_key_ref)?;
        state.end()
    }
}
impl ArtifactCustodyEntryV1 {
    pub const TYPE_NAME: &'static str = "ArtifactCustodyEntryV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "artifact_digest",
        "byte_length",
        "location",
        "region",
        "encryption_key_ref",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ArtifactCustodyEntryV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.artifact_digest.canonical_cmp(&other.artifact_digest))
            .then_with(|| self.byte_length.canonical_cmp(&other.byte_length))
            .then_with(|| self.location.canonical_cmp(&other.location))
            .then_with(|| self.region.canonical_cmp(&other.region))
            .then_with(|| {
                self.encryption_key_ref
                    .canonical_cmp(&other.encryption_key_ref)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.artifact_digest.canonical_size())
            .saturating_add(self.byte_length.canonical_size())
            .saturating_add(self.location.canonical_size())
            .saturating_add(self.region.canonical_size())
            .saturating_add(self.encryption_key_ref.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.artifact_digest
                .canonical_nesting()
                .max(self.byte_length.canonical_nesting())
                .max(self.location.canonical_nesting())
                .max(self.region.canonical_nesting())
                .max(self.encryption_key_ref.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptBindingV2SandboxTier {
    #[serde(rename = "S0")]
    S0,
    #[serde(rename = "S1")]
    S1,
    #[serde(rename = "S2")]
    S2,
    #[serde(rename = "S3")]
    S3,
}
impl AttemptBindingV2SandboxTier {
    pub fn code(&self) -> u64 {
        match self {
            Self::S0 => 0,
            Self::S1 => 1,
            Self::S2 => 2,
            Self::S3 => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::S0),
            1 => Some(Self::S1),
            2 => Some(Self::S2),
            3 => Some(Self::S3),
            _ => None,
        }
    }
}
impl CanonicalOrder for AttemptBindingV2SandboxTier {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptBindingV2Type {
    #[serde(rename = "AttemptBindingV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AttemptBindingV2Fields>"
)]
pub struct AttemptBindingV2 {
    pub r#type: AttemptBindingV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub attempt_id: Uuid,
    pub job_id: Uuid,
    pub evaluation_id: Uuid,
    pub evaluation_sequence: U64Str,
    pub workflow_digest: Hex<32>,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub capability_digest: Hex<32>,
    pub runner_id: Uuid,
    pub runner_session_id: Uuid,
    pub lease_generation: U64Str,
    pub runner_credential_generation: U64Str,
    pub sandbox_tier: AttemptBindingV2SandboxTier,
    pub resource_envelope_digest: Hex<32>,
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub issued_at: I64Str,
    pub lease_expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AttemptBindingV2Fields {
    r#type: AttemptBindingV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    attempt_id: Uuid,
    job_id: Uuid,
    evaluation_id: Uuid,
    evaluation_sequence: U64Str,
    workflow_digest: Hex<32>,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    capability_digest: Hex<32>,
    runner_id: Uuid,
    runner_session_id: Uuid,
    lease_generation: U64Str,
    runner_credential_generation: U64Str,
    sandbox_tier: AttemptBindingV2SandboxTier,
    resource_envelope_digest: Hex<32>,
    protocol_major: u16,
    protocol_minor: u16,
    issued_at: I64Str,
    lease_expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<AttemptBindingV2Fields>> for AttemptBindingV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AttemptBindingV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            attempt_id: fields.attempt_id,
            job_id: fields.job_id,
            evaluation_id: fields.evaluation_id,
            evaluation_sequence: fields.evaluation_sequence,
            workflow_digest: fields.workflow_digest,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            capability_digest: fields.capability_digest,
            runner_id: fields.runner_id,
            runner_session_id: fields.runner_session_id,
            lease_generation: fields.lease_generation,
            runner_credential_generation: fields.runner_credential_generation,
            sandbox_tier: fields.sandbox_tier,
            resource_envelope_digest: fields.resource_envelope_digest,
            protocol_major: fields.protocol_major,
            protocol_minor: fields.protocol_minor,
            issued_at: fields.issued_at,
            lease_expires_at: fields.lease_expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AttemptBindingV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 21)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("attempt_id", &self.attempt_id)?;
        state.serialize_field("job_id", &self.job_id)?;
        state.serialize_field("evaluation_id", &self.evaluation_id)?;
        state.serialize_field("evaluation_sequence", &self.evaluation_sequence)?;
        state.serialize_field("workflow_digest", &self.workflow_digest)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.serialize_field("runner_id", &self.runner_id)?;
        state.serialize_field("runner_session_id", &self.runner_session_id)?;
        state.serialize_field("lease_generation", &self.lease_generation)?;
        state.serialize_field(
            "runner_credential_generation",
            &self.runner_credential_generation,
        )?;
        state.serialize_field("sandbox_tier", &self.sandbox_tier)?;
        state.serialize_field("resource_envelope_digest", &self.resource_envelope_digest)?;
        state.serialize_field("protocol_major", &self.protocol_major)?;
        state.serialize_field("protocol_minor", &self.protocol_minor)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("lease_expires_at", &self.lease_expires_at)?;
        state.end()
    }
}
impl AttemptBindingV2 {
    pub const TYPE_NAME: &'static str = "AttemptBindingV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "attempt_id",
        "job_id",
        "evaluation_id",
        "evaluation_sequence",
        "workflow_digest",
        "subject_digest",
        "policy_digest",
        "capability_digest",
        "runner_id",
        "runner_session_id",
        "lease_generation",
        "runner_credential_generation",
        "sandbox_tier",
        "resource_envelope_digest",
        "protocol_major",
        "protocol_minor",
        "issued_at",
        "lease_expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AttemptBindingV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.attempt_id.canonical_cmp(&other.attempt_id))
            .then_with(|| self.job_id.canonical_cmp(&other.job_id))
            .then_with(|| self.evaluation_id.canonical_cmp(&other.evaluation_id))
            .then_with(|| {
                self.evaluation_sequence
                    .canonical_cmp(&other.evaluation_sequence)
            })
            .then_with(|| self.workflow_digest.canonical_cmp(&other.workflow_digest))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
            .then_with(|| self.runner_id.canonical_cmp(&other.runner_id))
            .then_with(|| {
                self.runner_session_id
                    .canonical_cmp(&other.runner_session_id)
            })
            .then_with(|| self.lease_generation.canonical_cmp(&other.lease_generation))
            .then_with(|| {
                self.runner_credential_generation
                    .canonical_cmp(&other.runner_credential_generation)
            })
            .then_with(|| self.sandbox_tier.canonical_cmp(&other.sandbox_tier))
            .then_with(|| {
                self.resource_envelope_digest
                    .canonical_cmp(&other.resource_envelope_digest)
            })
            .then_with(|| self.protocol_major.canonical_cmp(&other.protocol_major))
            .then_with(|| self.protocol_minor.canonical_cmp(&other.protocol_minor))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.lease_expires_at.canonical_cmp(&other.lease_expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(21u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.attempt_id.canonical_size())
            .saturating_add(self.job_id.canonical_size())
            .saturating_add(self.evaluation_id.canonical_size())
            .saturating_add(self.evaluation_sequence.canonical_size())
            .saturating_add(self.workflow_digest.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
            .saturating_add(self.runner_id.canonical_size())
            .saturating_add(self.runner_session_id.canonical_size())
            .saturating_add(self.lease_generation.canonical_size())
            .saturating_add(self.runner_credential_generation.canonical_size())
            .saturating_add(self.sandbox_tier.canonical_size())
            .saturating_add(self.resource_envelope_digest.canonical_size())
            .saturating_add(self.protocol_major.canonical_size())
            .saturating_add(self.protocol_minor.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.lease_expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.attempt_id.canonical_nesting())
                .max(self.job_id.canonical_nesting())
                .max(self.evaluation_id.canonical_nesting())
                .max(self.evaluation_sequence.canonical_nesting())
                .max(self.workflow_digest.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting())
                .max(self.runner_id.canonical_nesting())
                .max(self.runner_session_id.canonical_nesting())
                .max(self.lease_generation.canonical_nesting())
                .max(self.runner_credential_generation.canonical_nesting())
                .max(self.sandbox_tier.canonical_nesting())
                .max(self.resource_envelope_digest.canonical_nesting())
                .max(self.protocol_major.canonical_nesting())
                .max(self.protocol_minor.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.lease_expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptEventV2Type {
    #[serde(rename = "AttemptEventV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AttemptEventV2Fields>"
)]
pub struct AttemptEventV2 {
    pub r#type: AttemptEventV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub event_id: Uuid,
    pub producer_id: Uuid,
    pub producer_session_id: Uuid,
    pub producer_generation: U64Str,
    pub attempt_binding_digest: Hex<32>,
    pub sequence: U64Str,
    pub occurred_at: I64Str,
    pub received_at: I64Str,
    pub payload_type: Identifier,
    pub payload_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AttemptEventV2Fields {
    r#type: AttemptEventV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    event_id: Uuid,
    producer_id: Uuid,
    producer_session_id: Uuid,
    producer_generation: U64Str,
    attempt_binding_digest: Hex<32>,
    sequence: U64Str,
    occurred_at: I64Str,
    received_at: I64Str,
    payload_type: Identifier,
    payload_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<AttemptEventV2Fields>> for AttemptEventV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AttemptEventV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            event_id: fields.event_id,
            producer_id: fields.producer_id,
            producer_session_id: fields.producer_session_id,
            producer_generation: fields.producer_generation,
            attempt_binding_digest: fields.attempt_binding_digest,
            sequence: fields.sequence,
            occurred_at: fields.occurred_at,
            received_at: fields.received_at,
            payload_type: fields.payload_type,
            payload_digest: fields.payload_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AttemptEventV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("producer_session_id", &self.producer_session_id)?;
        state.serialize_field("producer_generation", &self.producer_generation)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("occurred_at", &self.occurred_at)?;
        state.serialize_field("received_at", &self.received_at)?;
        state.serialize_field("payload_type", &self.payload_type)?;
        state.serialize_field("payload_digest", &self.payload_digest)?;
        state.end()
    }
}
impl AttemptEventV2 {
    pub const TYPE_NAME: &'static str = "AttemptEventV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "event_id",
        "producer_id",
        "producer_session_id",
        "producer_generation",
        "attempt_binding_digest",
        "sequence",
        "occurred_at",
        "received_at",
        "payload_type",
        "payload_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AttemptEventV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.event_id.canonical_cmp(&other.event_id))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| {
                self.producer_session_id
                    .canonical_cmp(&other.producer_session_id)
            })
            .then_with(|| {
                self.producer_generation
                    .canonical_cmp(&other.producer_generation)
            })
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.occurred_at.canonical_cmp(&other.occurred_at))
            .then_with(|| self.received_at.canonical_cmp(&other.received_at))
            .then_with(|| self.payload_type.canonical_cmp(&other.payload_type))
            .then_with(|| self.payload_digest.canonical_cmp(&other.payload_digest))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.event_id.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.producer_session_id.canonical_size())
            .saturating_add(self.producer_generation.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.occurred_at.canonical_size())
            .saturating_add(self.received_at.canonical_size())
            .saturating_add(self.payload_type.canonical_size())
            .saturating_add(self.payload_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.event_id.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.producer_session_id.canonical_nesting())
                .max(self.producer_generation.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.sequence.canonical_nesting())
                .max(self.occurred_at.canonical_nesting())
                .max(self.received_at.canonical_nesting())
                .max(self.payload_type.canonical_nesting())
                .max(self.payload_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEnvelopeV1Type {
    #[serde(rename = "AuditEnvelopeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AuditEnvelopeV1Fields>"
)]
pub struct AuditEnvelopeV1 {
    pub r#type: AuditEnvelopeV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub audit_id: Uuid,
    pub stream_id: Uuid,
    pub sequence: U64Str,
    pub previous_digest: Hex<32>,
    pub producer_id: Uuid,
    pub producer_generation: U64Str,
    pub event_type: Identifier,
    pub subject_digest: Hex<32>,
    pub outcome: Identifier,
    pub recorded_at: I64Str,
    pub subject_type: Identifier,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuditEnvelopeV1Fields {
    r#type: AuditEnvelopeV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    audit_id: Uuid,
    stream_id: Uuid,
    sequence: U64Str,
    previous_digest: Hex<32>,
    producer_id: Uuid,
    producer_generation: U64Str,
    event_type: Identifier,
    subject_digest: Hex<32>,
    outcome: Identifier,
    recorded_at: I64Str,
    subject_type: Identifier,
}
impl TryFrom<crate::primitives::ObjectOnly<AuditEnvelopeV1Fields>> for AuditEnvelopeV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AuditEnvelopeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            audit_id: fields.audit_id,
            stream_id: fields.stream_id,
            sequence: fields.sequence,
            previous_digest: fields.previous_digest,
            producer_id: fields.producer_id,
            producer_generation: fields.producer_generation,
            event_type: fields.event_type,
            subject_digest: fields.subject_digest,
            outcome: fields.outcome,
            recorded_at: fields.recorded_at,
            subject_type: fields.subject_type,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AuditEnvelopeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 14)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("audit_id", &self.audit_id)?;
        state.serialize_field("stream_id", &self.stream_id)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("previous_digest", &self.previous_digest)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("producer_generation", &self.producer_generation)?;
        state.serialize_field("event_type", &self.event_type)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("outcome", &self.outcome)?;
        state.serialize_field("recorded_at", &self.recorded_at)?;
        state.serialize_field("subject_type", &self.subject_type)?;
        state.end()
    }
}
impl AuditEnvelopeV1 {
    pub const TYPE_NAME: &'static str = "AuditEnvelopeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "audit_id",
        "stream_id",
        "sequence",
        "previous_digest",
        "producer_id",
        "producer_generation",
        "event_type",
        "subject_digest",
        "outcome",
        "recorded_at",
        "subject_type",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AuditEnvelopeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.audit_id.canonical_cmp(&other.audit_id))
            .then_with(|| self.stream_id.canonical_cmp(&other.stream_id))
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.previous_digest.canonical_cmp(&other.previous_digest))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| {
                self.producer_generation
                    .canonical_cmp(&other.producer_generation)
            })
            .then_with(|| self.event_type.canonical_cmp(&other.event_type))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.outcome.canonical_cmp(&other.outcome))
            .then_with(|| self.recorded_at.canonical_cmp(&other.recorded_at))
            .then_with(|| self.subject_type.canonical_cmp(&other.subject_type))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(14u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.audit_id.canonical_size())
            .saturating_add(self.stream_id.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.previous_digest.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.producer_generation.canonical_size())
            .saturating_add(self.event_type.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.outcome.canonical_size())
            .saturating_add(self.recorded_at.canonical_size())
            .saturating_add(self.subject_type.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.audit_id.canonical_nesting())
                .max(self.stream_id.canonical_nesting())
                .max(self.sequence.canonical_nesting())
                .max(self.previous_digest.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.producer_generation.canonical_nesting())
                .max(self.event_type.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.outcome.canonical_nesting())
                .max(self.recorded_at.canonical_nesting())
                .max(self.subject_type.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityScopeV1Type {
    #[serde(rename = "AuthorityScopeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<AuthorityScopeV1Fields>"
)]
pub struct AuthorityScopeV1 {
    pub r#type: AuthorityScopeV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub repository_ids: SetOf<U64Str>,
    pub permissions: SetOf<Identifier>,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityScopeV1Fields {
    r#type: AuthorityScopeV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    repository_ids: SetOf<U64Str>,
    permissions: SetOf<Identifier>,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<AuthorityScopeV1Fields>> for AuthorityScopeV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<AuthorityScopeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            repository_ids: fields.repository_ids,
            permissions: fields.permissions,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for AuthorityScopeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 6)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("repository_ids", &self.repository_ids)?;
        state.serialize_field("permissions", &self.permissions)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl AuthorityScopeV1 {
    pub const TYPE_NAME: &'static str = "AuthorityScopeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "repository_ids",
        "permissions",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for AuthorityScopeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.repository_ids.canonical_cmp(&other.repository_ids))
            .then_with(|| self.permissions.canonical_cmp(&other.permissions))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(6u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.repository_ids.canonical_size())
            .saturating_add(self.permissions.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.repository_ids.canonical_nesting())
                .max(self.permissions.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityProfileV1Mode {
    #[serde(rename = "FAST")]
    Fast,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "HYBRID")]
    Hybrid,
    #[serde(rename = "DEEP")]
    Deep,
}
impl CapabilityProfileV1Mode {
    pub fn code(&self) -> u64 {
        match self {
            Self::Fast => 0,
            Self::Ai => 1,
            Self::Hybrid => 2,
            Self::Deep => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Fast),
            1 => Some(Self::Ai),
            2 => Some(Self::Hybrid),
            3 => Some(Self::Deep),
            _ => None,
        }
    }
}
impl CanonicalOrder for CapabilityProfileV1Mode {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityProfileV1SandboxTier {
    #[serde(rename = "S0")]
    S0,
    #[serde(rename = "S1")]
    S1,
    #[serde(rename = "S2")]
    S2,
    #[serde(rename = "S3")]
    S3,
}
impl CapabilityProfileV1SandboxTier {
    pub fn code(&self) -> u64 {
        match self {
            Self::S0 => 0,
            Self::S1 => 1,
            Self::S2 => 2,
            Self::S3 => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::S0),
            1 => Some(Self::S1),
            2 => Some(Self::S2),
            3 => Some(Self::S3),
            _ => None,
        }
    }
}
impl CanonicalOrder for CapabilityProfileV1SandboxTier {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityProfileV1MinimumIndependence {
    #[serde(rename = "C0")]
    C0,
    #[serde(rename = "C1")]
    C1,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "C4")]
    C4,
}
impl CapabilityProfileV1MinimumIndependence {
    pub fn code(&self) -> u64 {
        match self {
            Self::C0 => 0,
            Self::C1 => 1,
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::C0),
            1 => Some(Self::C1),
            2 => Some(Self::C2),
            3 => Some(Self::C3),
            4 => Some(Self::C4),
            _ => None,
        }
    }
}
impl CanonicalOrder for CapabilityProfileV1MinimumIndependence {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityProfileV1Type {
    #[serde(rename = "CapabilityProfileV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CapabilityProfileV1Fields>"
)]
pub struct CapabilityProfileV1 {
    pub r#type: CapabilityProfileV1Type,
    pub wire_revision: WireRevision,
    pub profile_id: Identifier,
    pub generation: U64Str,
    pub protocol_registry_digest: Hex<32>,
    pub mode: CapabilityProfileV1Mode,
    pub sandbox_tier: CapabilityProfileV1SandboxTier,
    pub minimum_independence: CapabilityProfileV1MinimumIndependence,
    pub qualified_claim_evidence_digests: SetOf<Hex<32>>,
    pub analyzer_bundle_digests: SetOf<Hex<32>>,
    pub model_cell_digests: SetOf<Hex<32>>,
    pub resource_policy_digest: Hex<32>,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityProfileV1Fields {
    r#type: CapabilityProfileV1Type,
    wire_revision: WireRevision,
    profile_id: Identifier,
    generation: U64Str,
    protocol_registry_digest: Hex<32>,
    mode: CapabilityProfileV1Mode,
    sandbox_tier: CapabilityProfileV1SandboxTier,
    minimum_independence: CapabilityProfileV1MinimumIndependence,
    qualified_claim_evidence_digests: SetOf<Hex<32>>,
    analyzer_bundle_digests: SetOf<Hex<32>>,
    model_cell_digests: SetOf<Hex<32>>,
    resource_policy_digest: Hex<32>,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<CapabilityProfileV1Fields>> for CapabilityProfileV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CapabilityProfileV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            profile_id: fields.profile_id,
            generation: fields.generation,
            protocol_registry_digest: fields.protocol_registry_digest,
            mode: fields.mode,
            sandbox_tier: fields.sandbox_tier,
            minimum_independence: fields.minimum_independence,
            qualified_claim_evidence_digests: fields.qualified_claim_evidence_digests,
            analyzer_bundle_digests: fields.analyzer_bundle_digests,
            model_cell_digests: fields.model_cell_digests,
            resource_policy_digest: fields.resource_policy_digest,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CapabilityProfileV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("profile_id", &self.profile_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("protocol_registry_digest", &self.protocol_registry_digest)?;
        state.serialize_field("mode", &self.mode)?;
        state.serialize_field("sandbox_tier", &self.sandbox_tier)?;
        state.serialize_field("minimum_independence", &self.minimum_independence)?;
        state.serialize_field(
            "qualified_claim_evidence_digests",
            &self.qualified_claim_evidence_digests,
        )?;
        state.serialize_field("analyzer_bundle_digests", &self.analyzer_bundle_digests)?;
        state.serialize_field("model_cell_digests", &self.model_cell_digests)?;
        state.serialize_field("resource_policy_digest", &self.resource_policy_digest)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl CapabilityProfileV1 {
    pub const TYPE_NAME: &'static str = "CapabilityProfileV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "profile_id",
        "generation",
        "protocol_registry_digest",
        "mode",
        "sandbox_tier",
        "minimum_independence",
        "qualified_claim_evidence_digests",
        "analyzer_bundle_digests",
        "model_cell_digests",
        "resource_policy_digest",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CapabilityProfileV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.profile_id.canonical_cmp(&other.profile_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| {
                self.protocol_registry_digest
                    .canonical_cmp(&other.protocol_registry_digest)
            })
            .then_with(|| self.mode.canonical_cmp(&other.mode))
            .then_with(|| self.sandbox_tier.canonical_cmp(&other.sandbox_tier))
            .then_with(|| {
                self.minimum_independence
                    .canonical_cmp(&other.minimum_independence)
            })
            .then_with(|| {
                self.qualified_claim_evidence_digests
                    .canonical_cmp(&other.qualified_claim_evidence_digests)
            })
            .then_with(|| {
                self.analyzer_bundle_digests
                    .canonical_cmp(&other.analyzer_bundle_digests)
            })
            .then_with(|| {
                self.model_cell_digests
                    .canonical_cmp(&other.model_cell_digests)
            })
            .then_with(|| {
                self.resource_policy_digest
                    .canonical_cmp(&other.resource_policy_digest)
            })
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.profile_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.protocol_registry_digest.canonical_size())
            .saturating_add(self.mode.canonical_size())
            .saturating_add(self.sandbox_tier.canonical_size())
            .saturating_add(self.minimum_independence.canonical_size())
            .saturating_add(self.qualified_claim_evidence_digests.canonical_size())
            .saturating_add(self.analyzer_bundle_digests.canonical_size())
            .saturating_add(self.model_cell_digests.canonical_size())
            .saturating_add(self.resource_policy_digest.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.profile_id
                .canonical_nesting()
                .max(self.generation.canonical_nesting())
                .max(self.protocol_registry_digest.canonical_nesting())
                .max(self.mode.canonical_nesting())
                .max(self.sandbox_tier.canonical_nesting())
                .max(self.minimum_independence.canonical_nesting())
                .max(self.qualified_claim_evidence_digests.canonical_nesting())
                .max(self.analyzer_bundle_digests.canonical_nesting())
                .max(self.model_cell_digests.canonical_nesting())
                .max(self.resource_policy_digest.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangedObjectV1Mode {
    #[serde(rename = "REGULAR")]
    Regular,
    #[serde(rename = "EXECUTABLE")]
    Executable,
    #[serde(rename = "SYMLINK")]
    Symlink,
    #[serde(rename = "DELETE")]
    Delete,
}
impl ChangedObjectV1Mode {
    pub fn code(&self) -> u64 {
        match self {
            Self::Regular => 0,
            Self::Executable => 1,
            Self::Symlink => 2,
            Self::Delete => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Regular),
            1 => Some(Self::Executable),
            2 => Some(Self::Symlink),
            3 => Some(Self::Delete),
            _ => None,
        }
    }
}
impl CanonicalOrder for ChangedObjectV1Mode {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangedObjectV1Type {
    #[serde(rename = "ChangedObjectV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ChangedObjectV1Fields>"
)]
pub struct ChangedObjectV1 {
    pub r#type: ChangedObjectV1Type,
    pub wire_revision: WireRevision,
    pub path: GitPath,
    pub mode: ChangedObjectV1Mode,
    pub blob_digest: Opt<Hex<32>>,
    pub blob_oid: Opt<GitOidV1>,
    pub byte_length: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChangedObjectV1Fields {
    r#type: ChangedObjectV1Type,
    wire_revision: WireRevision,
    path: GitPath,
    mode: ChangedObjectV1Mode,
    blob_digest: Opt<Hex<32>>,
    blob_oid: Opt<GitOidV1>,
    byte_length: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ChangedObjectV1Fields>> for ChangedObjectV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ChangedObjectV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            path: fields.path,
            mode: fields.mode,
            blob_digest: fields.blob_digest,
            blob_oid: fields.blob_oid,
            byte_length: fields.byte_length,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ChangedObjectV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("mode", &self.mode)?;
        state.serialize_field("blob_digest", &self.blob_digest)?;
        state.serialize_field("blob_oid", &self.blob_oid)?;
        state.serialize_field("byte_length", &self.byte_length)?;
        state.end()
    }
}
impl ChangedObjectV1 {
    pub const TYPE_NAME: &'static str = "ChangedObjectV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "path",
        "mode",
        "blob_digest",
        "blob_oid",
        "byte_length",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ChangedObjectV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.path.canonical_cmp(&other.path))
            .then_with(|| self.mode.canonical_cmp(&other.mode))
            .then_with(|| self.blob_digest.canonical_cmp(&other.blob_digest))
            .then_with(|| self.blob_oid.canonical_cmp(&other.blob_oid))
            .then_with(|| self.byte_length.canonical_cmp(&other.byte_length))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.path.canonical_size())
            .saturating_add(self.mode.canonical_size())
            .saturating_add(self.blob_digest.canonical_size())
            .saturating_add(self.blob_oid.canonical_size())
            .saturating_add(self.byte_length.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.path
                .canonical_nesting()
                .max(self.mode.canonical_nesting())
                .max(self.blob_digest.canonical_nesting())
                .max(self.blob_oid.canonical_nesting())
                .max(self.byte_length.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.path.0.clone())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckOutputV1Type {
    #[serde(rename = "CheckOutputV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CheckOutputV1Fields>"
)]
pub struct CheckOutputV1 {
    pub r#type: CheckOutputV1Type,
    pub wire_revision: WireRevision,
    pub title: Identifier,
    pub summary_code: Identifier,
    pub evaluation_identity_digest: Opt<Hex<32>>,
    pub projection_id: Uuid,
    pub generation: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckOutputV1Fields {
    r#type: CheckOutputV1Type,
    wire_revision: WireRevision,
    title: Identifier,
    summary_code: Identifier,
    evaluation_identity_digest: Opt<Hex<32>>,
    projection_id: Uuid,
    generation: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<CheckOutputV1Fields>> for CheckOutputV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CheckOutputV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            title: fields.title,
            summary_code: fields.summary_code,
            evaluation_identity_digest: fields.evaluation_identity_digest,
            projection_id: fields.projection_id,
            generation: fields.generation,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CheckOutputV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("summary_code", &self.summary_code)?;
        state.serialize_field(
            "evaluation_identity_digest",
            &self.evaluation_identity_digest,
        )?;
        state.serialize_field("projection_id", &self.projection_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.end()
    }
}
impl CheckOutputV1 {
    pub const TYPE_NAME: &'static str = "CheckOutputV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "title",
        "summary_code",
        "evaluation_identity_digest",
        "projection_id",
        "generation",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CheckOutputV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.title.canonical_cmp(&other.title))
            .then_with(|| self.summary_code.canonical_cmp(&other.summary_code))
            .then_with(|| {
                self.evaluation_identity_digest
                    .canonical_cmp(&other.evaluation_identity_digest)
            })
            .then_with(|| self.projection_id.canonical_cmp(&other.projection_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.title.canonical_size())
            .saturating_add(self.summary_code.canonical_size())
            .saturating_add(self.evaluation_identity_digest.canonical_size())
            .saturating_add(self.projection_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.title
                .canonical_nesting()
                .max(self.summary_code.canonical_nesting())
                .max(self.evaluation_identity_digest.canonical_nesting())
                .max(self.projection_id.canonical_nesting())
                .max(self.generation.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckProjectionV2ProjectedConclusion {
    #[serde(rename = "UNCONFIRMED")]
    Unconfirmed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ACTION_REQUIRED")]
    ActionRequired,
}
impl CheckProjectionV2ProjectedConclusion {
    pub fn code(&self) -> u64 {
        match self {
            Self::Unconfirmed => 0,
            Self::InProgress => 1,
            Self::Success => 2,
            Self::Failure => 3,
            Self::Cancelled => 4,
            Self::ActionRequired => 5,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Unconfirmed),
            1 => Some(Self::InProgress),
            2 => Some(Self::Success),
            3 => Some(Self::Failure),
            4 => Some(Self::Cancelled),
            5 => Some(Self::ActionRequired),
            _ => None,
        }
    }
}
impl CanonicalOrder for CheckProjectionV2ProjectedConclusion {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckProjectionV2DesiredConclusion {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ACTION_REQUIRED")]
    ActionRequired,
}
impl CheckProjectionV2DesiredConclusion {
    pub fn code(&self) -> u64 {
        match self {
            Self::InProgress => 0,
            Self::Success => 1,
            Self::Failure => 2,
            Self::Cancelled => 3,
            Self::ActionRequired => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::InProgress),
            1 => Some(Self::Success),
            2 => Some(Self::Failure),
            3 => Some(Self::Cancelled),
            4 => Some(Self::ActionRequired),
            _ => None,
        }
    }
}
impl CanonicalOrder for CheckProjectionV2DesiredConclusion {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckProjectionV2State {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "WRITING")]
    Writing,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "WITHDRAWAL_PENDING")]
    WithdrawalPending,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MANUAL_RECONCILIATION")]
    ManualReconciliation,
}
impl CheckProjectionV2State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Pending => 0,
            Self::Writing => 1,
            Self::Confirmed => 2,
            Self::WithdrawalPending => 3,
            Self::Unknown => 4,
            Self::ManualReconciliation => 5,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Pending),
            1 => Some(Self::Writing),
            2 => Some(Self::Confirmed),
            3 => Some(Self::WithdrawalPending),
            4 => Some(Self::Unknown),
            5 => Some(Self::ManualReconciliation),
            _ => None,
        }
    }
}
impl CanonicalOrder for CheckProjectionV2State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckProjectionV2FreezeState {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "FAILED")]
    Failed,
}
impl CheckProjectionV2FreezeState {
    pub fn code(&self) -> u64 {
        match self {
            Self::None => 0,
            Self::Requested => 1,
            Self::Confirmed => 2,
            Self::Failed => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::None),
            1 => Some(Self::Requested),
            2 => Some(Self::Confirmed),
            3 => Some(Self::Failed),
            _ => None,
        }
    }
}
impl CanonicalOrder for CheckProjectionV2FreezeState {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckProjectionV2Type {
    #[serde(rename = "CheckProjectionV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CheckProjectionV2Fields>"
)]
pub struct CheckProjectionV2 {
    pub r#type: CheckProjectionV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub projection_id: Uuid,
    pub evaluation_id: Uuid,
    pub provider: Provider,
    pub pr_number: U64Str,
    pub evaluation_identity_digest: Opt<Hex<32>>,
    pub head_commit: GitOidV1,
    pub candidate_commit: GitOidV1,
    pub candidate_tree: GitOidV1,
    pub policy_digest: Hex<32>,
    pub trust_set_digest: Hex<32>,
    pub provider_app_id: U64Str,
    pub check_run_id: Opt<U64Str>,
    pub projected_conclusion: CheckProjectionV2ProjectedConclusion,
    pub desired_conclusion: CheckProjectionV2DesiredConclusion,
    pub generation: U64Str,
    pub state: CheckProjectionV2State,
    pub gate_proof_digest: Opt<Hex<32>>,
    pub last_confirmed_at: Opt<I64Str>,
    pub effect_intent_id: Opt<Uuid>,
    pub freeze_state: CheckProjectionV2FreezeState,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckProjectionV2Fields {
    r#type: CheckProjectionV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    projection_id: Uuid,
    evaluation_id: Uuid,
    provider: Provider,
    pr_number: U64Str,
    evaluation_identity_digest: Opt<Hex<32>>,
    head_commit: GitOidV1,
    candidate_commit: GitOidV1,
    candidate_tree: GitOidV1,
    policy_digest: Hex<32>,
    trust_set_digest: Hex<32>,
    provider_app_id: U64Str,
    check_run_id: Opt<U64Str>,
    projected_conclusion: CheckProjectionV2ProjectedConclusion,
    desired_conclusion: CheckProjectionV2DesiredConclusion,
    generation: U64Str,
    state: CheckProjectionV2State,
    gate_proof_digest: Opt<Hex<32>>,
    last_confirmed_at: Opt<I64Str>,
    effect_intent_id: Opt<Uuid>,
    freeze_state: CheckProjectionV2FreezeState,
}
impl TryFrom<crate::primitives::ObjectOnly<CheckProjectionV2Fields>> for CheckProjectionV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CheckProjectionV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            projection_id: fields.projection_id,
            evaluation_id: fields.evaluation_id,
            provider: fields.provider,
            pr_number: fields.pr_number,
            evaluation_identity_digest: fields.evaluation_identity_digest,
            head_commit: fields.head_commit,
            candidate_commit: fields.candidate_commit,
            candidate_tree: fields.candidate_tree,
            policy_digest: fields.policy_digest,
            trust_set_digest: fields.trust_set_digest,
            provider_app_id: fields.provider_app_id,
            check_run_id: fields.check_run_id,
            projected_conclusion: fields.projected_conclusion,
            desired_conclusion: fields.desired_conclusion,
            generation: fields.generation,
            state: fields.state,
            gate_proof_digest: fields.gate_proof_digest,
            last_confirmed_at: fields.last_confirmed_at,
            effect_intent_id: fields.effect_intent_id,
            freeze_state: fields.freeze_state,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CheckProjectionV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 23)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("projection_id", &self.projection_id)?;
        state.serialize_field("evaluation_id", &self.evaluation_id)?;
        state.serialize_field("provider", &self.provider)?;
        state.serialize_field("pr_number", &self.pr_number)?;
        state.serialize_field(
            "evaluation_identity_digest",
            &self.evaluation_identity_digest,
        )?;
        state.serialize_field("head_commit", &self.head_commit)?;
        state.serialize_field("candidate_commit", &self.candidate_commit)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("trust_set_digest", &self.trust_set_digest)?;
        state.serialize_field("provider_app_id", &self.provider_app_id)?;
        state.serialize_field("check_run_id", &self.check_run_id)?;
        state.serialize_field("projected_conclusion", &self.projected_conclusion)?;
        state.serialize_field("desired_conclusion", &self.desired_conclusion)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("gate_proof_digest", &self.gate_proof_digest)?;
        state.serialize_field("last_confirmed_at", &self.last_confirmed_at)?;
        state.serialize_field("effect_intent_id", &self.effect_intent_id)?;
        state.serialize_field("freeze_state", &self.freeze_state)?;
        state.end()
    }
}
impl CheckProjectionV2 {
    pub const TYPE_NAME: &'static str = "CheckProjectionV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "projection_id",
        "evaluation_id",
        "provider",
        "pr_number",
        "evaluation_identity_digest",
        "head_commit",
        "candidate_commit",
        "candidate_tree",
        "policy_digest",
        "trust_set_digest",
        "provider_app_id",
        "check_run_id",
        "projected_conclusion",
        "desired_conclusion",
        "generation",
        "state",
        "gate_proof_digest",
        "last_confirmed_at",
        "effect_intent_id",
        "freeze_state",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CheckProjectionV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.projection_id.canonical_cmp(&other.projection_id))
            .then_with(|| self.evaluation_id.canonical_cmp(&other.evaluation_id))
            .then_with(|| self.provider.canonical_cmp(&other.provider))
            .then_with(|| self.pr_number.canonical_cmp(&other.pr_number))
            .then_with(|| {
                self.evaluation_identity_digest
                    .canonical_cmp(&other.evaluation_identity_digest)
            })
            .then_with(|| self.head_commit.canonical_cmp(&other.head_commit))
            .then_with(|| self.candidate_commit.canonical_cmp(&other.candidate_commit))
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.trust_set_digest.canonical_cmp(&other.trust_set_digest))
            .then_with(|| self.provider_app_id.canonical_cmp(&other.provider_app_id))
            .then_with(|| self.check_run_id.canonical_cmp(&other.check_run_id))
            .then_with(|| {
                self.projected_conclusion
                    .canonical_cmp(&other.projected_conclusion)
            })
            .then_with(|| {
                self.desired_conclusion
                    .canonical_cmp(&other.desired_conclusion)
            })
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| {
                self.gate_proof_digest
                    .canonical_cmp(&other.gate_proof_digest)
            })
            .then_with(|| {
                self.last_confirmed_at
                    .canonical_cmp(&other.last_confirmed_at)
            })
            .then_with(|| self.effect_intent_id.canonical_cmp(&other.effect_intent_id))
            .then_with(|| self.freeze_state.canonical_cmp(&other.freeze_state))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(23u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.projection_id.canonical_size())
            .saturating_add(self.evaluation_id.canonical_size())
            .saturating_add(self.provider.canonical_size())
            .saturating_add(self.pr_number.canonical_size())
            .saturating_add(self.evaluation_identity_digest.canonical_size())
            .saturating_add(self.head_commit.canonical_size())
            .saturating_add(self.candidate_commit.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.trust_set_digest.canonical_size())
            .saturating_add(self.provider_app_id.canonical_size())
            .saturating_add(self.check_run_id.canonical_size())
            .saturating_add(self.projected_conclusion.canonical_size())
            .saturating_add(self.desired_conclusion.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.gate_proof_digest.canonical_size())
            .saturating_add(self.last_confirmed_at.canonical_size())
            .saturating_add(self.effect_intent_id.canonical_size())
            .saturating_add(self.freeze_state.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.projection_id.canonical_nesting())
                .max(self.evaluation_id.canonical_nesting())
                .max(self.provider.canonical_nesting())
                .max(self.pr_number.canonical_nesting())
                .max(self.evaluation_identity_digest.canonical_nesting())
                .max(self.head_commit.canonical_nesting())
                .max(self.candidate_commit.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.trust_set_digest.canonical_nesting())
                .max(self.provider_app_id.canonical_nesting())
                .max(self.check_run_id.canonical_nesting())
                .max(self.projected_conclusion.canonical_nesting())
                .max(self.desired_conclusion.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.gate_proof_digest.canonical_nesting())
                .max(self.last_confirmed_at.canonical_nesting())
                .max(self.effect_intent_id.canonical_nesting())
                .max(self.freeze_state.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckWriteRequestV1Conclusion {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ACTION_REQUIRED")]
    ActionRequired,
}
impl CheckWriteRequestV1Conclusion {
    pub fn code(&self) -> u64 {
        match self {
            Self::InProgress => 0,
            Self::Success => 1,
            Self::Failure => 2,
            Self::Cancelled => 3,
            Self::ActionRequired => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::InProgress),
            1 => Some(Self::Success),
            2 => Some(Self::Failure),
            3 => Some(Self::Cancelled),
            4 => Some(Self::ActionRequired),
            _ => None,
        }
    }
}
impl CanonicalOrder for CheckWriteRequestV1Conclusion {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckWriteRequestV1Type {
    #[serde(rename = "CheckWriteRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CheckWriteRequestV1Fields>"
)]
pub struct CheckWriteRequestV1 {
    pub r#type: CheckWriteRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub projection_id: Uuid,
    pub generation: U64Str,
    pub provider_app_id: U64Str,
    pub check_run_id: Opt<U64Str>,
    pub name: Identifier,
    pub head_commit: GitOidV1,
    pub external_id: Identifier,
    pub conclusion: CheckWriteRequestV1Conclusion,
    pub gate_proof_digest: Opt<Hex<32>>,
    pub output_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckWriteRequestV1Fields {
    r#type: CheckWriteRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    projection_id: Uuid,
    generation: U64Str,
    provider_app_id: U64Str,
    check_run_id: Opt<U64Str>,
    name: Identifier,
    head_commit: GitOidV1,
    external_id: Identifier,
    conclusion: CheckWriteRequestV1Conclusion,
    gate_proof_digest: Opt<Hex<32>>,
    output_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<CheckWriteRequestV1Fields>> for CheckWriteRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CheckWriteRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            projection_id: fields.projection_id,
            generation: fields.generation,
            provider_app_id: fields.provider_app_id,
            check_run_id: fields.check_run_id,
            name: fields.name,
            head_commit: fields.head_commit,
            external_id: fields.external_id,
            conclusion: fields.conclusion,
            gate_proof_digest: fields.gate_proof_digest,
            output_digest: fields.output_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CheckWriteRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("projection_id", &self.projection_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("provider_app_id", &self.provider_app_id)?;
        state.serialize_field("check_run_id", &self.check_run_id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("head_commit", &self.head_commit)?;
        state.serialize_field("external_id", &self.external_id)?;
        state.serialize_field("conclusion", &self.conclusion)?;
        state.serialize_field("gate_proof_digest", &self.gate_proof_digest)?;
        state.serialize_field("output_digest", &self.output_digest)?;
        state.end()
    }
}
impl CheckWriteRequestV1 {
    pub const TYPE_NAME: &'static str = "CheckWriteRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "projection_id",
        "generation",
        "provider_app_id",
        "check_run_id",
        "name",
        "head_commit",
        "external_id",
        "conclusion",
        "gate_proof_digest",
        "output_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CheckWriteRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.projection_id.canonical_cmp(&other.projection_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.provider_app_id.canonical_cmp(&other.provider_app_id))
            .then_with(|| self.check_run_id.canonical_cmp(&other.check_run_id))
            .then_with(|| self.name.canonical_cmp(&other.name))
            .then_with(|| self.head_commit.canonical_cmp(&other.head_commit))
            .then_with(|| self.external_id.canonical_cmp(&other.external_id))
            .then_with(|| self.conclusion.canonical_cmp(&other.conclusion))
            .then_with(|| {
                self.gate_proof_digest
                    .canonical_cmp(&other.gate_proof_digest)
            })
            .then_with(|| self.output_digest.canonical_cmp(&other.output_digest))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.projection_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.provider_app_id.canonical_size())
            .saturating_add(self.check_run_id.canonical_size())
            .saturating_add(self.name.canonical_size())
            .saturating_add(self.head_commit.canonical_size())
            .saturating_add(self.external_id.canonical_size())
            .saturating_add(self.conclusion.canonical_size())
            .saturating_add(self.gate_proof_digest.canonical_size())
            .saturating_add(self.output_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.projection_id.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.provider_app_id.canonical_nesting())
                .max(self.check_run_id.canonical_nesting())
                .max(self.name.canonical_nesting())
                .max(self.head_commit.canonical_nesting())
                .max(self.external_id.canonical_nesting())
                .max(self.conclusion.canonical_nesting())
                .max(self.gate_proof_digest.canonical_nesting())
                .max(self.output_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckoutGrantV2Type {
    #[serde(rename = "CheckoutGrantV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CheckoutGrantV2Fields>"
)]
pub struct CheckoutGrantV2 {
    pub r#type: CheckoutGrantV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub grant_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub allowed_oids: SetOf<GitOidV1>,
    pub maximum_bytes: U64Str,
    pub maximum_objects: U64Str,
    pub maximum_calls: u32,
    pub deadline: I64Str,
    pub broker_generation: U64Str,
    pub nonce: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckoutGrantV2Fields {
    r#type: CheckoutGrantV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    grant_id: Uuid,
    attempt_binding_digest: Hex<32>,
    allowed_oids: SetOf<GitOidV1>,
    maximum_bytes: U64Str,
    maximum_objects: U64Str,
    maximum_calls: u32,
    deadline: I64Str,
    broker_generation: U64Str,
    nonce: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<CheckoutGrantV2Fields>> for CheckoutGrantV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CheckoutGrantV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            grant_id: fields.grant_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            allowed_oids: fields.allowed_oids,
            maximum_bytes: fields.maximum_bytes,
            maximum_objects: fields.maximum_objects,
            maximum_calls: fields.maximum_calls,
            deadline: fields.deadline,
            broker_generation: fields.broker_generation,
            nonce: fields.nonce,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CheckoutGrantV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("grant_id", &self.grant_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("allowed_oids", &self.allowed_oids)?;
        state.serialize_field("maximum_bytes", &self.maximum_bytes)?;
        state.serialize_field("maximum_objects", &self.maximum_objects)?;
        state.serialize_field("maximum_calls", &self.maximum_calls)?;
        state.serialize_field("deadline", &self.deadline)?;
        state.serialize_field("broker_generation", &self.broker_generation)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.end()
    }
}
impl CheckoutGrantV2 {
    pub const TYPE_NAME: &'static str = "CheckoutGrantV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "grant_id",
        "attempt_binding_digest",
        "allowed_oids",
        "maximum_bytes",
        "maximum_objects",
        "maximum_calls",
        "deadline",
        "broker_generation",
        "nonce",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CheckoutGrantV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.grant_id.canonical_cmp(&other.grant_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.allowed_oids.canonical_cmp(&other.allowed_oids))
            .then_with(|| self.maximum_bytes.canonical_cmp(&other.maximum_bytes))
            .then_with(|| self.maximum_objects.canonical_cmp(&other.maximum_objects))
            .then_with(|| self.maximum_calls.canonical_cmp(&other.maximum_calls))
            .then_with(|| self.deadline.canonical_cmp(&other.deadline))
            .then_with(|| {
                self.broker_generation
                    .canonical_cmp(&other.broker_generation)
            })
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.grant_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.allowed_oids.canonical_size())
            .saturating_add(self.maximum_bytes.canonical_size())
            .saturating_add(self.maximum_objects.canonical_size())
            .saturating_add(self.maximum_calls.canonical_size())
            .saturating_add(self.deadline.canonical_size())
            .saturating_add(self.broker_generation.canonical_size())
            .saturating_add(self.nonce.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.grant_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.allowed_oids.canonical_nesting())
                .max(self.maximum_bytes.canonical_nesting())
                .max(self.maximum_objects.canonical_nesting())
                .max(self.maximum_calls.canonical_nesting())
                .max(self.deadline.canonical_nesting())
                .max(self.broker_generation.canonical_nesting())
                .max(self.nonce.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitIdentityV1Type {
    #[serde(rename = "CommitIdentityV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CommitIdentityV1Fields>"
)]
pub struct CommitIdentityV1 {
    pub r#type: CommitIdentityV1Type,
    pub wire_revision: WireRevision,
    pub name: Bytes,
    pub email: Bytes,
    pub timestamp_seconds: I64Str,
    pub timezone_minutes: i16,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CommitIdentityV1Fields {
    r#type: CommitIdentityV1Type,
    wire_revision: WireRevision,
    name: Bytes,
    email: Bytes,
    timestamp_seconds: I64Str,
    timezone_minutes: i16,
}
impl TryFrom<crate::primitives::ObjectOnly<CommitIdentityV1Fields>> for CommitIdentityV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CommitIdentityV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            name: fields.name,
            email: fields.email,
            timestamp_seconds: fields.timestamp_seconds,
            timezone_minutes: fields.timezone_minutes,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CommitIdentityV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 6)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("timestamp_seconds", &self.timestamp_seconds)?;
        state.serialize_field("timezone_minutes", &self.timezone_minutes)?;
        state.end()
    }
}
impl CommitIdentityV1 {
    pub const TYPE_NAME: &'static str = "CommitIdentityV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "name",
        "email",
        "timestamp_seconds",
        "timezone_minutes",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CommitIdentityV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.name.canonical_cmp(&other.name))
            .then_with(|| self.email.canonical_cmp(&other.email))
            .then_with(|| {
                self.timestamp_seconds
                    .canonical_cmp(&other.timestamp_seconds)
            })
            .then_with(|| self.timezone_minutes.canonical_cmp(&other.timezone_minutes))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(6u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.name.canonical_size())
            .saturating_add(self.email.canonical_size())
            .saturating_add(self.timestamp_seconds.canonical_size())
            .saturating_add(self.timezone_minutes.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.name
                .canonical_nesting()
                .max(self.email.canonical_nesting())
                .max(self.timestamp_seconds.canonical_nesting())
                .max(self.timezone_minutes.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateBlobRequestV1Type {
    #[serde(rename = "CreateBlobRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CreateBlobRequestV1Fields>"
)]
pub struct CreateBlobRequestV1 {
    pub r#type: CreateBlobRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub content_custody_digest: Hex<32>,
    pub content_digest: Hex<32>,
    pub byte_length: U64Str,
    pub expected_blob_oid: GitOidV1,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateBlobRequestV1Fields {
    r#type: CreateBlobRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    content_custody_digest: Hex<32>,
    content_digest: Hex<32>,
    byte_length: U64Str,
    expected_blob_oid: GitOidV1,
}
impl TryFrom<crate::primitives::ObjectOnly<CreateBlobRequestV1Fields>> for CreateBlobRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CreateBlobRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            content_custody_digest: fields.content_custody_digest,
            content_digest: fields.content_digest,
            byte_length: fields.byte_length,
            expected_blob_oid: fields.expected_blob_oid,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CreateBlobRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("content_custody_digest", &self.content_custody_digest)?;
        state.serialize_field("content_digest", &self.content_digest)?;
        state.serialize_field("byte_length", &self.byte_length)?;
        state.serialize_field("expected_blob_oid", &self.expected_blob_oid)?;
        state.end()
    }
}
impl CreateBlobRequestV1 {
    pub const TYPE_NAME: &'static str = "CreateBlobRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "content_custody_digest",
        "content_digest",
        "byte_length",
        "expected_blob_oid",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CreateBlobRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| {
                self.content_custody_digest
                    .canonical_cmp(&other.content_custody_digest)
            })
            .then_with(|| self.content_digest.canonical_cmp(&other.content_digest))
            .then_with(|| self.byte_length.canonical_cmp(&other.byte_length))
            .then_with(|| {
                self.expected_blob_oid
                    .canonical_cmp(&other.expected_blob_oid)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.content_custody_digest.canonical_size())
            .saturating_add(self.content_digest.canonical_size())
            .saturating_add(self.byte_length.canonical_size())
            .saturating_add(self.expected_blob_oid.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.content_custody_digest.canonical_nesting())
                .max(self.content_digest.canonical_nesting())
                .max(self.byte_length.canonical_nesting())
                .max(self.expected_blob_oid.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateCommitRequestV1Type {
    #[serde(rename = "CreateCommitRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CreateCommitRequestV1Fields>"
)]
pub struct CreateCommitRequestV1 {
    pub r#type: CreateCommitRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub tree: GitOidV1,
    pub parent: GitOidV1,
    pub author: CommitIdentityV1,
    pub committer: CommitIdentityV1,
    pub message: Bytes,
    pub expected_commit: GitOidV1,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateCommitRequestV1Fields {
    r#type: CreateCommitRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    tree: GitOidV1,
    parent: GitOidV1,
    author: CommitIdentityV1,
    committer: CommitIdentityV1,
    message: Bytes,
    expected_commit: GitOidV1,
}
impl TryFrom<crate::primitives::ObjectOnly<CreateCommitRequestV1Fields>> for CreateCommitRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CreateCommitRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            tree: fields.tree,
            parent: fields.parent,
            author: fields.author,
            committer: fields.committer,
            message: fields.message,
            expected_commit: fields.expected_commit,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CreateCommitRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("tree", &self.tree)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("committer", &self.committer)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("expected_commit", &self.expected_commit)?;
        state.end()
    }
}
impl CreateCommitRequestV1 {
    pub const TYPE_NAME: &'static str = "CreateCommitRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "tree",
        "parent",
        "author",
        "committer",
        "message",
        "expected_commit",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CreateCommitRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.tree.canonical_cmp(&other.tree))
            .then_with(|| self.parent.canonical_cmp(&other.parent))
            .then_with(|| self.author.canonical_cmp(&other.author))
            .then_with(|| self.committer.canonical_cmp(&other.committer))
            .then_with(|| self.message.canonical_cmp(&other.message))
            .then_with(|| self.expected_commit.canonical_cmp(&other.expected_commit))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.tree.canonical_size())
            .saturating_add(self.parent.canonical_size())
            .saturating_add(self.author.canonical_size())
            .saturating_add(self.committer.canonical_size())
            .saturating_add(self.message.canonical_size())
            .saturating_add(self.expected_commit.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.tree.canonical_nesting())
                .max(self.parent.canonical_nesting())
                .max(self.author.canonical_nesting())
                .max(self.committer.canonical_nesting())
                .max(self.message.canonical_nesting())
                .max(self.expected_commit.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateFixPrRequestV1Type {
    #[serde(rename = "CreateFixPrRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CreateFixPrRequestV1Fields>"
)]
pub struct CreateFixPrRequestV1 {
    pub r#type: CreateFixPrRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub head_ref: GitRef,
    pub base_ref: GitRef,
    pub title: Bytes,
    pub body_custody_digest: Hex<32>,
    pub body_digest: Hex<32>,
    pub draft: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateFixPrRequestV1Fields {
    r#type: CreateFixPrRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    head_ref: GitRef,
    base_ref: GitRef,
    title: Bytes,
    body_custody_digest: Hex<32>,
    body_digest: Hex<32>,
    draft: bool,
}
impl TryFrom<crate::primitives::ObjectOnly<CreateFixPrRequestV1Fields>> for CreateFixPrRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CreateFixPrRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            head_ref: fields.head_ref,
            base_ref: fields.base_ref,
            title: fields.title,
            body_custody_digest: fields.body_custody_digest,
            body_digest: fields.body_digest,
            draft: fields.draft,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CreateFixPrRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("head_ref", &self.head_ref)?;
        state.serialize_field("base_ref", &self.base_ref)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body_custody_digest", &self.body_custody_digest)?;
        state.serialize_field("body_digest", &self.body_digest)?;
        state.serialize_field("draft", &self.draft)?;
        state.end()
    }
}
impl CreateFixPrRequestV1 {
    pub const TYPE_NAME: &'static str = "CreateFixPrRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "head_ref",
        "base_ref",
        "title",
        "body_custody_digest",
        "body_digest",
        "draft",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CreateFixPrRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.head_ref.canonical_cmp(&other.head_ref))
            .then_with(|| self.base_ref.canonical_cmp(&other.base_ref))
            .then_with(|| self.title.canonical_cmp(&other.title))
            .then_with(|| {
                self.body_custody_digest
                    .canonical_cmp(&other.body_custody_digest)
            })
            .then_with(|| self.body_digest.canonical_cmp(&other.body_digest))
            .then_with(|| self.draft.canonical_cmp(&other.draft))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.head_ref.canonical_size())
            .saturating_add(self.base_ref.canonical_size())
            .saturating_add(self.title.canonical_size())
            .saturating_add(self.body_custody_digest.canonical_size())
            .saturating_add(self.body_digest.canonical_size())
            .saturating_add(self.draft.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.head_ref.canonical_nesting())
                .max(self.base_ref.canonical_nesting())
                .max(self.title.canonical_nesting())
                .max(self.body_custody_digest.canonical_nesting())
                .max(self.body_digest.canonical_nesting())
                .max(self.draft.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateRefRequestV1Type {
    #[serde(rename = "CreateRefRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CreateRefRequestV1Fields>"
)]
pub struct CreateRefRequestV1 {
    pub r#type: CreateRefRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub destination_ref: GitRef,
    pub commit: GitOidV1,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateRefRequestV1Fields {
    r#type: CreateRefRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    destination_ref: GitRef,
    commit: GitOidV1,
}
impl TryFrom<crate::primitives::ObjectOnly<CreateRefRequestV1Fields>> for CreateRefRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CreateRefRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            destination_ref: fields.destination_ref,
            commit: fields.commit,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CreateRefRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 6)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("destination_ref", &self.destination_ref)?;
        state.serialize_field("commit", &self.commit)?;
        state.end()
    }
}
impl CreateRefRequestV1 {
    pub const TYPE_NAME: &'static str = "CreateRefRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "destination_ref",
        "commit",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CreateRefRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.destination_ref.canonical_cmp(&other.destination_ref))
            .then_with(|| self.commit.canonical_cmp(&other.commit))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(6u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.destination_ref.canonical_size())
            .saturating_add(self.commit.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.destination_ref.canonical_nesting())
                .max(self.commit.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateTreeRequestV1Type {
    #[serde(rename = "CreateTreeRequestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CreateTreeRequestV1Fields>"
)]
pub struct CreateTreeRequestV1 {
    pub r#type: CreateTreeRequestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub base_tree: GitOidV1,
    pub entries: SetOf<TreeEntryV1>,
    pub expected_tree: GitOidV1,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateTreeRequestV1Fields {
    r#type: CreateTreeRequestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    base_tree: GitOidV1,
    entries: SetOf<TreeEntryV1>,
    expected_tree: GitOidV1,
}
impl TryFrom<crate::primitives::ObjectOnly<CreateTreeRequestV1Fields>> for CreateTreeRequestV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CreateTreeRequestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            base_tree: fields.base_tree,
            entries: fields.entries,
            expected_tree: fields.expected_tree,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CreateTreeRequestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("base_tree", &self.base_tree)?;
        state.serialize_field("entries", &self.entries)?;
        state.serialize_field("expected_tree", &self.expected_tree)?;
        state.end()
    }
}
impl CreateTreeRequestV1 {
    pub const TYPE_NAME: &'static str = "CreateTreeRequestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "base_tree",
        "entries",
        "expected_tree",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CreateTreeRequestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.base_tree.canonical_cmp(&other.base_tree))
            .then_with(|| self.entries.canonical_cmp(&other.entries))
            .then_with(|| self.expected_tree.canonical_cmp(&other.expected_tree))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.base_tree.canonical_size())
            .saturating_add(self.entries.canonical_size())
            .saturating_add(self.expected_tree.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.base_tree.canonical_nesting())
                .max(self.entries.canonical_nesting())
                .max(self.expected_tree.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentCandidateV1Type {
    #[serde(rename = "CurrentCandidateV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<CurrentCandidateV1Fields>"
)]
pub struct CurrentCandidateV1 {
    pub r#type: CurrentCandidateV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub subject_digest: Hex<32>,
    pub provider_generation: U64Str,
    pub observation_digest: Hex<32>,
    pub writer_id: Uuid,
    pub observed_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentCandidateV1Fields {
    r#type: CurrentCandidateV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    subject_digest: Hex<32>,
    provider_generation: U64Str,
    observation_digest: Hex<32>,
    writer_id: Uuid,
    observed_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<CurrentCandidateV1Fields>> for CurrentCandidateV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<CurrentCandidateV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            subject_digest: fields.subject_digest,
            provider_generation: fields.provider_generation,
            observation_digest: fields.observation_digest,
            writer_id: fields.writer_id,
            observed_at: fields.observed_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for CurrentCandidateV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 9)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("provider_generation", &self.provider_generation)?;
        state.serialize_field("observation_digest", &self.observation_digest)?;
        state.serialize_field("writer_id", &self.writer_id)?;
        state.serialize_field("observed_at", &self.observed_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl CurrentCandidateV1 {
    pub const TYPE_NAME: &'static str = "CurrentCandidateV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "subject_digest",
        "provider_generation",
        "observation_digest",
        "writer_id",
        "observed_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for CurrentCandidateV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| {
                self.provider_generation
                    .canonical_cmp(&other.provider_generation)
            })
            .then_with(|| {
                self.observation_digest
                    .canonical_cmp(&other.observation_digest)
            })
            .then_with(|| self.writer_id.canonical_cmp(&other.writer_id))
            .then_with(|| self.observed_at.canonical_cmp(&other.observed_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(9u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.provider_generation.canonical_size())
            .saturating_add(self.observation_digest.canonical_size())
            .saturating_add(self.writer_id.canonical_size())
            .saturating_add(self.observed_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.subject_digest.canonical_nesting())
                .max(self.provider_generation.canonical_nesting())
                .max(self.observation_digest.canonical_nesting())
                .max(self.writer_id.canonical_nesting())
                .max(self.observed_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbStartCertificateV1Type {
    #[serde(rename = "DbStartCertificateV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<DbStartCertificateV1Fields>"
)]
pub struct DbStartCertificateV1 {
    pub r#type: DbStartCertificateV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub effect_id: Uuid,
    pub intent_digest: Hex<32>,
    pub state_revision: U64Str,
    pub dispatcher_session_id: Uuid,
    pub committed_at: I64Str,
    pub dispatch_deadline: I64Str,
    pub request_deadline: I64Str,
    pub authorization_snapshot_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DbStartCertificateV1Fields {
    r#type: DbStartCertificateV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    effect_id: Uuid,
    intent_digest: Hex<32>,
    state_revision: U64Str,
    dispatcher_session_id: Uuid,
    committed_at: I64Str,
    dispatch_deadline: I64Str,
    request_deadline: I64Str,
    authorization_snapshot_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<DbStartCertificateV1Fields>> for DbStartCertificateV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<DbStartCertificateV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            effect_id: fields.effect_id,
            intent_digest: fields.intent_digest,
            state_revision: fields.state_revision,
            dispatcher_session_id: fields.dispatcher_session_id,
            committed_at: fields.committed_at,
            dispatch_deadline: fields.dispatch_deadline,
            request_deadline: fields.request_deadline,
            authorization_snapshot_digest: fields.authorization_snapshot_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for DbStartCertificateV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("effect_id", &self.effect_id)?;
        state.serialize_field("intent_digest", &self.intent_digest)?;
        state.serialize_field("state_revision", &self.state_revision)?;
        state.serialize_field("dispatcher_session_id", &self.dispatcher_session_id)?;
        state.serialize_field("committed_at", &self.committed_at)?;
        state.serialize_field("dispatch_deadline", &self.dispatch_deadline)?;
        state.serialize_field("request_deadline", &self.request_deadline)?;
        state.serialize_field(
            "authorization_snapshot_digest",
            &self.authorization_snapshot_digest,
        )?;
        state.end()
    }
}
impl DbStartCertificateV1 {
    pub const TYPE_NAME: &'static str = "DbStartCertificateV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "effect_id",
        "intent_digest",
        "state_revision",
        "dispatcher_session_id",
        "committed_at",
        "dispatch_deadline",
        "request_deadline",
        "authorization_snapshot_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for DbStartCertificateV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.effect_id.canonical_cmp(&other.effect_id))
            .then_with(|| self.intent_digest.canonical_cmp(&other.intent_digest))
            .then_with(|| self.state_revision.canonical_cmp(&other.state_revision))
            .then_with(|| {
                self.dispatcher_session_id
                    .canonical_cmp(&other.dispatcher_session_id)
            })
            .then_with(|| self.committed_at.canonical_cmp(&other.committed_at))
            .then_with(|| {
                self.dispatch_deadline
                    .canonical_cmp(&other.dispatch_deadline)
            })
            .then_with(|| self.request_deadline.canonical_cmp(&other.request_deadline))
            .then_with(|| {
                self.authorization_snapshot_digest
                    .canonical_cmp(&other.authorization_snapshot_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.effect_id.canonical_size())
            .saturating_add(self.intent_digest.canonical_size())
            .saturating_add(self.state_revision.canonical_size())
            .saturating_add(self.dispatcher_session_id.canonical_size())
            .saturating_add(self.committed_at.canonical_size())
            .saturating_add(self.dispatch_deadline.canonical_size())
            .saturating_add(self.request_deadline.canonical_size())
            .saturating_add(self.authorization_snapshot_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.effect_id.canonical_nesting())
                .max(self.intent_digest.canonical_nesting())
                .max(self.state_revision.canonical_nesting())
                .max(self.dispatcher_session_id.canonical_nesting())
                .max(self.committed_at.canonical_nesting())
                .max(self.dispatch_deadline.canonical_nesting())
                .max(self.request_deadline.canonical_nesting())
                .max(self.authorization_snapshot_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyAcquisitionGrantV1Type {
    #[serde(rename = "DependencyAcquisitionGrantV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<DependencyAcquisitionGrantV1Fields>"
)]
pub struct DependencyAcquisitionGrantV1 {
    pub r#type: DependencyAcquisitionGrantV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub grant_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub lockfile_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub packages: SetOf<DependencyPackageV1>,
    pub endpoint_allowlist_digest: Hex<32>,
    pub credential_authority_id: Uuid,
    pub cache_scope_digest: Hex<32>,
    pub maximum_bytes: U64Str,
    pub maximum_packages: u32,
    pub maximum_requests: u32,
    pub maximum_duration_ms: U64Str,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
    pub nonce: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencyAcquisitionGrantV1Fields {
    r#type: DependencyAcquisitionGrantV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    grant_id: Uuid,
    attempt_binding_digest: Hex<32>,
    lockfile_digest: Hex<32>,
    policy_digest: Hex<32>,
    packages: SetOf<DependencyPackageV1>,
    endpoint_allowlist_digest: Hex<32>,
    credential_authority_id: Uuid,
    cache_scope_digest: Hex<32>,
    maximum_bytes: U64Str,
    maximum_packages: u32,
    maximum_requests: u32,
    maximum_duration_ms: U64Str,
    issued_at: I64Str,
    expires_at: I64Str,
    nonce: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<DependencyAcquisitionGrantV1Fields>>
    for DependencyAcquisitionGrantV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<DependencyAcquisitionGrantV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            grant_id: fields.grant_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            lockfile_digest: fields.lockfile_digest,
            policy_digest: fields.policy_digest,
            packages: fields.packages,
            endpoint_allowlist_digest: fields.endpoint_allowlist_digest,
            credential_authority_id: fields.credential_authority_id,
            cache_scope_digest: fields.cache_scope_digest,
            maximum_bytes: fields.maximum_bytes,
            maximum_packages: fields.maximum_packages,
            maximum_requests: fields.maximum_requests,
            maximum_duration_ms: fields.maximum_duration_ms,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
            nonce: fields.nonce,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for DependencyAcquisitionGrantV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 18)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("grant_id", &self.grant_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("lockfile_digest", &self.lockfile_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("packages", &self.packages)?;
        state.serialize_field("endpoint_allowlist_digest", &self.endpoint_allowlist_digest)?;
        state.serialize_field("credential_authority_id", &self.credential_authority_id)?;
        state.serialize_field("cache_scope_digest", &self.cache_scope_digest)?;
        state.serialize_field("maximum_bytes", &self.maximum_bytes)?;
        state.serialize_field("maximum_packages", &self.maximum_packages)?;
        state.serialize_field("maximum_requests", &self.maximum_requests)?;
        state.serialize_field("maximum_duration_ms", &self.maximum_duration_ms)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.end()
    }
}
impl DependencyAcquisitionGrantV1 {
    pub const TYPE_NAME: &'static str = "DependencyAcquisitionGrantV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "grant_id",
        "attempt_binding_digest",
        "lockfile_digest",
        "policy_digest",
        "packages",
        "endpoint_allowlist_digest",
        "credential_authority_id",
        "cache_scope_digest",
        "maximum_bytes",
        "maximum_packages",
        "maximum_requests",
        "maximum_duration_ms",
        "issued_at",
        "expires_at",
        "nonce",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for DependencyAcquisitionGrantV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.grant_id.canonical_cmp(&other.grant_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.lockfile_digest.canonical_cmp(&other.lockfile_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.packages.canonical_cmp(&other.packages))
            .then_with(|| {
                self.endpoint_allowlist_digest
                    .canonical_cmp(&other.endpoint_allowlist_digest)
            })
            .then_with(|| {
                self.credential_authority_id
                    .canonical_cmp(&other.credential_authority_id)
            })
            .then_with(|| {
                self.cache_scope_digest
                    .canonical_cmp(&other.cache_scope_digest)
            })
            .then_with(|| self.maximum_bytes.canonical_cmp(&other.maximum_bytes))
            .then_with(|| self.maximum_packages.canonical_cmp(&other.maximum_packages))
            .then_with(|| self.maximum_requests.canonical_cmp(&other.maximum_requests))
            .then_with(|| {
                self.maximum_duration_ms
                    .canonical_cmp(&other.maximum_duration_ms)
            })
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(18u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.grant_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.lockfile_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.packages.canonical_size())
            .saturating_add(self.endpoint_allowlist_digest.canonical_size())
            .saturating_add(self.credential_authority_id.canonical_size())
            .saturating_add(self.cache_scope_digest.canonical_size())
            .saturating_add(self.maximum_bytes.canonical_size())
            .saturating_add(self.maximum_packages.canonical_size())
            .saturating_add(self.maximum_requests.canonical_size())
            .saturating_add(self.maximum_duration_ms.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.nonce.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.grant_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.lockfile_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.packages.canonical_nesting())
                .max(self.endpoint_allowlist_digest.canonical_nesting())
                .max(self.credential_authority_id.canonical_nesting())
                .max(self.cache_scope_digest.canonical_nesting())
                .max(self.maximum_bytes.canonical_nesting())
                .max(self.maximum_packages.canonical_nesting())
                .max(self.maximum_requests.canonical_nesting())
                .max(self.maximum_duration_ms.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.nonce.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyPackageV1Type {
    #[serde(rename = "DependencyPackageV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<DependencyPackageV1Fields>"
)]
pub struct DependencyPackageV1 {
    pub r#type: DependencyPackageV1Type,
    pub wire_revision: WireRevision,
    pub ecosystem: Identifier,
    pub registry_origin: Origin,
    pub package_name: Identifier,
    pub version: Identifier,
    pub content_digest: Hex<32>,
    pub lockfile_entry_digest: Hex<32>,
    pub provenance_digest: Hex<32>,
    pub git_commit: Opt<GitOidV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencyPackageV1Fields {
    r#type: DependencyPackageV1Type,
    wire_revision: WireRevision,
    ecosystem: Identifier,
    registry_origin: Origin,
    package_name: Identifier,
    version: Identifier,
    content_digest: Hex<32>,
    lockfile_entry_digest: Hex<32>,
    provenance_digest: Hex<32>,
    git_commit: Opt<GitOidV1>,
}
impl TryFrom<crate::primitives::ObjectOnly<DependencyPackageV1Fields>> for DependencyPackageV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<DependencyPackageV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            ecosystem: fields.ecosystem,
            registry_origin: fields.registry_origin,
            package_name: fields.package_name,
            version: fields.version,
            content_digest: fields.content_digest,
            lockfile_entry_digest: fields.lockfile_entry_digest,
            provenance_digest: fields.provenance_digest,
            git_commit: fields.git_commit,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for DependencyPackageV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("ecosystem", &self.ecosystem)?;
        state.serialize_field("registry_origin", &self.registry_origin)?;
        state.serialize_field("package_name", &self.package_name)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("content_digest", &self.content_digest)?;
        state.serialize_field("lockfile_entry_digest", &self.lockfile_entry_digest)?;
        state.serialize_field("provenance_digest", &self.provenance_digest)?;
        state.serialize_field("git_commit", &self.git_commit)?;
        state.end()
    }
}
impl DependencyPackageV1 {
    pub const TYPE_NAME: &'static str = "DependencyPackageV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "ecosystem",
        "registry_origin",
        "package_name",
        "version",
        "content_digest",
        "lockfile_entry_digest",
        "provenance_digest",
        "git_commit",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for DependencyPackageV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.ecosystem.canonical_cmp(&other.ecosystem))
            .then_with(|| self.registry_origin.canonical_cmp(&other.registry_origin))
            .then_with(|| self.package_name.canonical_cmp(&other.package_name))
            .then_with(|| self.version.canonical_cmp(&other.version))
            .then_with(|| self.content_digest.canonical_cmp(&other.content_digest))
            .then_with(|| {
                self.lockfile_entry_digest
                    .canonical_cmp(&other.lockfile_entry_digest)
            })
            .then_with(|| {
                self.provenance_digest
                    .canonical_cmp(&other.provenance_digest)
            })
            .then_with(|| self.git_commit.canonical_cmp(&other.git_commit))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.ecosystem.canonical_size())
            .saturating_add(self.registry_origin.canonical_size())
            .saturating_add(self.package_name.canonical_size())
            .saturating_add(self.version.canonical_size())
            .saturating_add(self.content_digest.canonical_size())
            .saturating_add(self.lockfile_entry_digest.canonical_size())
            .saturating_add(self.provenance_digest.canonical_size())
            .saturating_add(self.git_commit.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.ecosystem
                .canonical_nesting()
                .max(self.registry_origin.canonical_nesting())
                .max(self.package_name.canonical_nesting())
                .max(self.version.canonical_nesting())
                .max(self.content_digest.canonical_nesting())
                .max(self.lockfile_entry_digest.canonical_nesting())
                .max(self.provenance_digest.canonical_nesting())
                .max(self.git_commit.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DescriptorEntryV1Type {
    #[serde(rename = "DescriptorEntryV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<DescriptorEntryV1Fields>"
)]
pub struct DescriptorEntryV1 {
    pub r#type: DescriptorEntryV1Type,
    pub wire_revision: WireRevision,
    pub key: Identifier,
    pub value: Bytes,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DescriptorEntryV1Fields {
    r#type: DescriptorEntryV1Type,
    wire_revision: WireRevision,
    key: Identifier,
    value: Bytes,
}
impl TryFrom<crate::primitives::ObjectOnly<DescriptorEntryV1Fields>> for DescriptorEntryV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<DescriptorEntryV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            key: fields.key,
            value: fields.value,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for DescriptorEntryV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 4)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}
impl DescriptorEntryV1 {
    pub const TYPE_NAME: &'static str = "DescriptorEntryV1";
    pub const FIELD_ORDER: &'static [&'static str] = &["type", "wire_revision", "key", "value"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for DescriptorEntryV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.key.canonical_cmp(&other.key))
            .then_with(|| self.value.canonical_cmp(&other.value))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(4u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.key.canonical_size())
            .saturating_add(self.value.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.key
                .canonical_nesting()
                .max(self.value.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectIntentV2State {
    #[serde(rename = "PENDING")]
    Pending,
}
impl EffectIntentV2State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Pending => 0,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Pending),
            _ => None,
        }
    }
}
impl CanonicalOrder for EffectIntentV2State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectIntentV2Type {
    #[serde(rename = "EffectIntentV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EffectIntentV2Fields>"
)]
pub struct EffectIntentV2 {
    pub r#type: EffectIntentV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub effect_id: Uuid,
    pub logical_operation_id: Uuid,
    pub parent_operation_id: Uuid,
    pub predecessor_effect_ids: SetOf<Uuid>,
    pub call_attempt: u32,
    pub effect_kind: EffectKind,
    pub provider: Provider,
    pub account_id: Identifier,
    pub canonical_request_digest: Hex<32>,
    pub request_custody_digest: Hex<32>,
    pub authorization_proof_digest: Hex<32>,
    pub created_at: I64Str,
    pub dispatch_deadline: I64Str,
    pub request_deadline: I64Str,
    pub idempotency_token: Opt<Identifier>,
    pub state: EffectIntentV2State,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectIntentV2Fields {
    r#type: EffectIntentV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    effect_id: Uuid,
    logical_operation_id: Uuid,
    parent_operation_id: Uuid,
    predecessor_effect_ids: SetOf<Uuid>,
    call_attempt: u32,
    effect_kind: EffectKind,
    provider: Provider,
    account_id: Identifier,
    canonical_request_digest: Hex<32>,
    request_custody_digest: Hex<32>,
    authorization_proof_digest: Hex<32>,
    created_at: I64Str,
    dispatch_deadline: I64Str,
    request_deadline: I64Str,
    idempotency_token: Opt<Identifier>,
    state: EffectIntentV2State,
}
impl TryFrom<crate::primitives::ObjectOnly<EffectIntentV2Fields>> for EffectIntentV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EffectIntentV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            effect_id: fields.effect_id,
            logical_operation_id: fields.logical_operation_id,
            parent_operation_id: fields.parent_operation_id,
            predecessor_effect_ids: fields.predecessor_effect_ids,
            call_attempt: fields.call_attempt,
            effect_kind: fields.effect_kind,
            provider: fields.provider,
            account_id: fields.account_id,
            canonical_request_digest: fields.canonical_request_digest,
            request_custody_digest: fields.request_custody_digest,
            authorization_proof_digest: fields.authorization_proof_digest,
            created_at: fields.created_at,
            dispatch_deadline: fields.dispatch_deadline,
            request_deadline: fields.request_deadline,
            idempotency_token: fields.idempotency_token,
            state: fields.state,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EffectIntentV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 19)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("effect_id", &self.effect_id)?;
        state.serialize_field("logical_operation_id", &self.logical_operation_id)?;
        state.serialize_field("parent_operation_id", &self.parent_operation_id)?;
        state.serialize_field("predecessor_effect_ids", &self.predecessor_effect_ids)?;
        state.serialize_field("call_attempt", &self.call_attempt)?;
        state.serialize_field("effect_kind", &self.effect_kind)?;
        state.serialize_field("provider", &self.provider)?;
        state.serialize_field("account_id", &self.account_id)?;
        state.serialize_field("canonical_request_digest", &self.canonical_request_digest)?;
        state.serialize_field("request_custody_digest", &self.request_custody_digest)?;
        state.serialize_field(
            "authorization_proof_digest",
            &self.authorization_proof_digest,
        )?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("dispatch_deadline", &self.dispatch_deadline)?;
        state.serialize_field("request_deadline", &self.request_deadline)?;
        state.serialize_field("idempotency_token", &self.idempotency_token)?;
        state.serialize_field("state", &self.state)?;
        state.end()
    }
}
impl EffectIntentV2 {
    pub const TYPE_NAME: &'static str = "EffectIntentV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "effect_id",
        "logical_operation_id",
        "parent_operation_id",
        "predecessor_effect_ids",
        "call_attempt",
        "effect_kind",
        "provider",
        "account_id",
        "canonical_request_digest",
        "request_custody_digest",
        "authorization_proof_digest",
        "created_at",
        "dispatch_deadline",
        "request_deadline",
        "idempotency_token",
        "state",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EffectIntentV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.effect_id.canonical_cmp(&other.effect_id))
            .then_with(|| {
                self.logical_operation_id
                    .canonical_cmp(&other.logical_operation_id)
            })
            .then_with(|| {
                self.parent_operation_id
                    .canonical_cmp(&other.parent_operation_id)
            })
            .then_with(|| {
                self.predecessor_effect_ids
                    .canonical_cmp(&other.predecessor_effect_ids)
            })
            .then_with(|| self.call_attempt.canonical_cmp(&other.call_attempt))
            .then_with(|| self.effect_kind.canonical_cmp(&other.effect_kind))
            .then_with(|| self.provider.canonical_cmp(&other.provider))
            .then_with(|| self.account_id.canonical_cmp(&other.account_id))
            .then_with(|| {
                self.canonical_request_digest
                    .canonical_cmp(&other.canonical_request_digest)
            })
            .then_with(|| {
                self.request_custody_digest
                    .canonical_cmp(&other.request_custody_digest)
            })
            .then_with(|| {
                self.authorization_proof_digest
                    .canonical_cmp(&other.authorization_proof_digest)
            })
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| {
                self.dispatch_deadline
                    .canonical_cmp(&other.dispatch_deadline)
            })
            .then_with(|| self.request_deadline.canonical_cmp(&other.request_deadline))
            .then_with(|| {
                self.idempotency_token
                    .canonical_cmp(&other.idempotency_token)
            })
            .then_with(|| self.state.canonical_cmp(&other.state))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(19u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.effect_id.canonical_size())
            .saturating_add(self.logical_operation_id.canonical_size())
            .saturating_add(self.parent_operation_id.canonical_size())
            .saturating_add(self.predecessor_effect_ids.canonical_size())
            .saturating_add(self.call_attempt.canonical_size())
            .saturating_add(self.effect_kind.canonical_size())
            .saturating_add(self.provider.canonical_size())
            .saturating_add(self.account_id.canonical_size())
            .saturating_add(self.canonical_request_digest.canonical_size())
            .saturating_add(self.request_custody_digest.canonical_size())
            .saturating_add(self.authorization_proof_digest.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.dispatch_deadline.canonical_size())
            .saturating_add(self.request_deadline.canonical_size())
            .saturating_add(self.idempotency_token.canonical_size())
            .saturating_add(self.state.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.effect_id.canonical_nesting())
                .max(self.logical_operation_id.canonical_nesting())
                .max(self.parent_operation_id.canonical_nesting())
                .max(self.predecessor_effect_ids.canonical_nesting())
                .max(self.call_attempt.canonical_nesting())
                .max(self.effect_kind.canonical_nesting())
                .max(self.provider.canonical_nesting())
                .max(self.account_id.canonical_nesting())
                .max(self.canonical_request_digest.canonical_nesting())
                .max(self.request_custody_digest.canonical_nesting())
                .max(self.authorization_proof_digest.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.dispatch_deadline.canonical_nesting())
                .max(self.request_deadline.canonical_nesting())
                .max(self.idempotency_token.canonical_nesting())
                .max(self.state.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectPlanEntryV1Type {
    #[serde(rename = "EffectPlanEntryV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EffectPlanEntryV1Fields>"
)]
pub struct EffectPlanEntryV1 {
    pub r#type: EffectPlanEntryV1Type,
    pub wire_revision: WireRevision,
    pub step_id: Identifier,
    pub request_type: Identifier,
    pub request_digest: Hex<32>,
    pub predecessor_step_ids: SetOf<Identifier>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectPlanEntryV1Fields {
    r#type: EffectPlanEntryV1Type,
    wire_revision: WireRevision,
    step_id: Identifier,
    request_type: Identifier,
    request_digest: Hex<32>,
    predecessor_step_ids: SetOf<Identifier>,
}
impl TryFrom<crate::primitives::ObjectOnly<EffectPlanEntryV1Fields>> for EffectPlanEntryV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EffectPlanEntryV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            step_id: fields.step_id,
            request_type: fields.request_type,
            request_digest: fields.request_digest,
            predecessor_step_ids: fields.predecessor_step_ids,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EffectPlanEntryV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 6)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("step_id", &self.step_id)?;
        state.serialize_field("request_type", &self.request_type)?;
        state.serialize_field("request_digest", &self.request_digest)?;
        state.serialize_field("predecessor_step_ids", &self.predecessor_step_ids)?;
        state.end()
    }
}
impl EffectPlanEntryV1 {
    pub const TYPE_NAME: &'static str = "EffectPlanEntryV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "step_id",
        "request_type",
        "request_digest",
        "predecessor_step_ids",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EffectPlanEntryV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.step_id.canonical_cmp(&other.step_id))
            .then_with(|| self.request_type.canonical_cmp(&other.request_type))
            .then_with(|| self.request_digest.canonical_cmp(&other.request_digest))
            .then_with(|| {
                self.predecessor_step_ids
                    .canonical_cmp(&other.predecessor_step_ids)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(6u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.step_id.canonical_size())
            .saturating_add(self.request_type.canonical_size())
            .saturating_add(self.request_digest.canonical_size())
            .saturating_add(self.predecessor_step_ids.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.step_id
                .canonical_nesting()
                .max(self.request_type.canonical_nesting())
                .max(self.request_digest.canonical_nesting())
                .max(self.predecessor_step_ids.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.step_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectStateV1State {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "FAILED_TERMINAL")]
    FailedTerminal,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MANUAL_RECONCILIATION")]
    ManualReconciliation,
}
impl EffectStateV1State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Pending => 0,
            Self::Started => 1,
            Self::Confirmed => 2,
            Self::FailedTerminal => 3,
            Self::Unknown => 4,
            Self::ManualReconciliation => 5,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Pending),
            1 => Some(Self::Started),
            2 => Some(Self::Confirmed),
            3 => Some(Self::FailedTerminal),
            4 => Some(Self::Unknown),
            5 => Some(Self::ManualReconciliation),
            _ => None,
        }
    }
}
impl CanonicalOrder for EffectStateV1State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectStateV1Type {
    #[serde(rename = "EffectStateV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EffectStateV1Fields>"
)]
pub struct EffectStateV1 {
    pub r#type: EffectStateV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub effect_id: Uuid,
    pub intent_digest: Hex<32>,
    pub revision: U64Str,
    pub state: EffectStateV1State,
    pub prior_state_digest: Opt<Hex<32>>,
    pub started_at: Opt<I64Str>,
    pub completed_at: Opt<I64Str>,
    pub provider_result_digest: Opt<Hex<32>>,
    pub reconciliation_evidence_digests: SetOf<Hex<32>>,
    pub recorded_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectStateV1Fields {
    r#type: EffectStateV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    effect_id: Uuid,
    intent_digest: Hex<32>,
    revision: U64Str,
    state: EffectStateV1State,
    prior_state_digest: Opt<Hex<32>>,
    started_at: Opt<I64Str>,
    completed_at: Opt<I64Str>,
    provider_result_digest: Opt<Hex<32>>,
    reconciliation_evidence_digests: SetOf<Hex<32>>,
    recorded_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<EffectStateV1Fields>> for EffectStateV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EffectStateV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            effect_id: fields.effect_id,
            intent_digest: fields.intent_digest,
            revision: fields.revision,
            state: fields.state,
            prior_state_digest: fields.prior_state_digest,
            started_at: fields.started_at,
            completed_at: fields.completed_at,
            provider_result_digest: fields.provider_result_digest,
            reconciliation_evidence_digests: fields.reconciliation_evidence_digests,
            recorded_at: fields.recorded_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EffectStateV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("effect_id", &self.effect_id)?;
        state.serialize_field("intent_digest", &self.intent_digest)?;
        state.serialize_field("revision", &self.revision)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("prior_state_digest", &self.prior_state_digest)?;
        state.serialize_field("started_at", &self.started_at)?;
        state.serialize_field("completed_at", &self.completed_at)?;
        state.serialize_field("provider_result_digest", &self.provider_result_digest)?;
        state.serialize_field(
            "reconciliation_evidence_digests",
            &self.reconciliation_evidence_digests,
        )?;
        state.serialize_field("recorded_at", &self.recorded_at)?;
        state.end()
    }
}
impl EffectStateV1 {
    pub const TYPE_NAME: &'static str = "EffectStateV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "effect_id",
        "intent_digest",
        "revision",
        "state",
        "prior_state_digest",
        "started_at",
        "completed_at",
        "provider_result_digest",
        "reconciliation_evidence_digests",
        "recorded_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EffectStateV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.effect_id.canonical_cmp(&other.effect_id))
            .then_with(|| self.intent_digest.canonical_cmp(&other.intent_digest))
            .then_with(|| self.revision.canonical_cmp(&other.revision))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| {
                self.prior_state_digest
                    .canonical_cmp(&other.prior_state_digest)
            })
            .then_with(|| self.started_at.canonical_cmp(&other.started_at))
            .then_with(|| self.completed_at.canonical_cmp(&other.completed_at))
            .then_with(|| {
                self.provider_result_digest
                    .canonical_cmp(&other.provider_result_digest)
            })
            .then_with(|| {
                self.reconciliation_evidence_digests
                    .canonical_cmp(&other.reconciliation_evidence_digests)
            })
            .then_with(|| self.recorded_at.canonical_cmp(&other.recorded_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.effect_id.canonical_size())
            .saturating_add(self.intent_digest.canonical_size())
            .saturating_add(self.revision.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.prior_state_digest.canonical_size())
            .saturating_add(self.started_at.canonical_size())
            .saturating_add(self.completed_at.canonical_size())
            .saturating_add(self.provider_result_digest.canonical_size())
            .saturating_add(self.reconciliation_evidence_digests.canonical_size())
            .saturating_add(self.recorded_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.effect_id.canonical_nesting())
                .max(self.intent_digest.canonical_nesting())
                .max(self.revision.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.prior_state_digest.canonical_nesting())
                .max(self.started_at.canonical_nesting())
                .max(self.completed_at.canonical_nesting())
                .max(self.provider_result_digest.canonical_nesting())
                .max(self.reconciliation_evidence_digests.canonical_nesting())
                .max(self.recorded_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EligibilityProofV2Type {
    #[serde(rename = "EligibilityProofV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EligibilityProofV2Fields>"
)]
pub struct EligibilityProofV2 {
    pub r#type: EligibilityProofV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub proof_id: Uuid,
    pub provider: Provider,
    pub installation_id: U64Str,
    pub pr_number: U64Str,
    pub provider_actor_id: U64Str,
    pub linked_user_id: Uuid,
    pub membership_generation: U64Str,
    pub policy_digest: Hex<32>,
    pub observation_digest: Hex<32>,
    pub issuer_id: Uuid,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
    pub eligible: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EligibilityProofV2Fields {
    r#type: EligibilityProofV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    proof_id: Uuid,
    provider: Provider,
    installation_id: U64Str,
    pr_number: U64Str,
    provider_actor_id: U64Str,
    linked_user_id: Uuid,
    membership_generation: U64Str,
    policy_digest: Hex<32>,
    observation_digest: Hex<32>,
    issuer_id: Uuid,
    issued_at: I64Str,
    expires_at: I64Str,
    eligible: bool,
}
impl TryFrom<crate::primitives::ObjectOnly<EligibilityProofV2Fields>> for EligibilityProofV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EligibilityProofV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            proof_id: fields.proof_id,
            provider: fields.provider,
            installation_id: fields.installation_id,
            pr_number: fields.pr_number,
            provider_actor_id: fields.provider_actor_id,
            linked_user_id: fields.linked_user_id,
            membership_generation: fields.membership_generation,
            policy_digest: fields.policy_digest,
            observation_digest: fields.observation_digest,
            issuer_id: fields.issuer_id,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
            eligible: fields.eligible,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EligibilityProofV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 16)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("proof_id", &self.proof_id)?;
        state.serialize_field("provider", &self.provider)?;
        state.serialize_field("installation_id", &self.installation_id)?;
        state.serialize_field("pr_number", &self.pr_number)?;
        state.serialize_field("provider_actor_id", &self.provider_actor_id)?;
        state.serialize_field("linked_user_id", &self.linked_user_id)?;
        state.serialize_field("membership_generation", &self.membership_generation)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("observation_digest", &self.observation_digest)?;
        state.serialize_field("issuer_id", &self.issuer_id)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("eligible", &self.eligible)?;
        state.end()
    }
}
impl EligibilityProofV2 {
    pub const TYPE_NAME: &'static str = "EligibilityProofV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "proof_id",
        "provider",
        "installation_id",
        "pr_number",
        "provider_actor_id",
        "linked_user_id",
        "membership_generation",
        "policy_digest",
        "observation_digest",
        "issuer_id",
        "issued_at",
        "expires_at",
        "eligible",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EligibilityProofV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.proof_id.canonical_cmp(&other.proof_id))
            .then_with(|| self.provider.canonical_cmp(&other.provider))
            .then_with(|| self.installation_id.canonical_cmp(&other.installation_id))
            .then_with(|| self.pr_number.canonical_cmp(&other.pr_number))
            .then_with(|| {
                self.provider_actor_id
                    .canonical_cmp(&other.provider_actor_id)
            })
            .then_with(|| self.linked_user_id.canonical_cmp(&other.linked_user_id))
            .then_with(|| {
                self.membership_generation
                    .canonical_cmp(&other.membership_generation)
            })
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.observation_digest
                    .canonical_cmp(&other.observation_digest)
            })
            .then_with(|| self.issuer_id.canonical_cmp(&other.issuer_id))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| self.eligible.canonical_cmp(&other.eligible))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(16u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.proof_id.canonical_size())
            .saturating_add(self.provider.canonical_size())
            .saturating_add(self.installation_id.canonical_size())
            .saturating_add(self.pr_number.canonical_size())
            .saturating_add(self.provider_actor_id.canonical_size())
            .saturating_add(self.linked_user_id.canonical_size())
            .saturating_add(self.membership_generation.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.observation_digest.canonical_size())
            .saturating_add(self.issuer_id.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.eligible.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.proof_id.canonical_nesting())
                .max(self.provider.canonical_nesting())
                .max(self.installation_id.canonical_nesting())
                .max(self.pr_number.canonical_nesting())
                .max(self.provider_actor_id.canonical_nesting())
                .max(self.linked_user_id.canonical_nesting())
                .max(self.membership_generation.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.observation_digest.canonical_nesting())
                .max(self.issuer_id.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.eligible.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationIdentityV2Type {
    #[serde(rename = "EvaluationIdentityV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EvaluationIdentityV2Fields>"
)]
pub struct EvaluationIdentityV2 {
    pub r#type: EvaluationIdentityV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub evaluation_id: Uuid,
    pub evaluation_sequence: U64Str,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub capability_digest: Hex<32>,
    pub trust_set_digest: Hex<32>,
    pub manifest_digest: Hex<32>,
    pub verification_set_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationIdentityV2Fields {
    r#type: EvaluationIdentityV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    evaluation_id: Uuid,
    evaluation_sequence: U64Str,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    capability_digest: Hex<32>,
    trust_set_digest: Hex<32>,
    manifest_digest: Hex<32>,
    verification_set_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<EvaluationIdentityV2Fields>> for EvaluationIdentityV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EvaluationIdentityV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            evaluation_id: fields.evaluation_id,
            evaluation_sequence: fields.evaluation_sequence,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            capability_digest: fields.capability_digest,
            trust_set_digest: fields.trust_set_digest,
            manifest_digest: fields.manifest_digest,
            verification_set_digest: fields.verification_set_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EvaluationIdentityV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("evaluation_id", &self.evaluation_id)?;
        state.serialize_field("evaluation_sequence", &self.evaluation_sequence)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.serialize_field("trust_set_digest", &self.trust_set_digest)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("verification_set_digest", &self.verification_set_digest)?;
        state.end()
    }
}
impl EvaluationIdentityV2 {
    pub const TYPE_NAME: &'static str = "EvaluationIdentityV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "evaluation_id",
        "evaluation_sequence",
        "subject_digest",
        "policy_digest",
        "capability_digest",
        "trust_set_digest",
        "manifest_digest",
        "verification_set_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EvaluationIdentityV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.evaluation_id.canonical_cmp(&other.evaluation_id))
            .then_with(|| {
                self.evaluation_sequence
                    .canonical_cmp(&other.evaluation_sequence)
            })
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
            .then_with(|| self.trust_set_digest.canonical_cmp(&other.trust_set_digest))
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| {
                self.verification_set_digest
                    .canonical_cmp(&other.verification_set_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.evaluation_id.canonical_size())
            .saturating_add(self.evaluation_sequence.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
            .saturating_add(self.trust_set_digest.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.verification_set_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.evaluation_id.canonical_nesting())
                .max(self.evaluation_sequence.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting())
                .max(self.trust_set_digest.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.verification_set_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceReceiptV2Grade {
    #[serde(rename = "E0")]
    E0,
    #[serde(rename = "E1")]
    E1,
    #[serde(rename = "E2")]
    E2,
    #[serde(rename = "E3")]
    E3,
    #[serde(rename = "E4")]
    E4,
}
impl EvidenceReceiptV2Grade {
    pub fn code(&self) -> u64 {
        match self {
            Self::E0 => 0,
            Self::E1 => 1,
            Self::E2 => 2,
            Self::E3 => 3,
            Self::E4 => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::E0),
            1 => Some(Self::E1),
            2 => Some(Self::E2),
            3 => Some(Self::E3),
            4 => Some(Self::E4),
            _ => None,
        }
    }
}
impl CanonicalOrder for EvidenceReceiptV2Grade {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceReceiptV2Type {
    #[serde(rename = "EvidenceReceiptV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<EvidenceReceiptV2Fields>"
)]
pub struct EvidenceReceiptV2 {
    pub r#type: EvidenceReceiptV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub receipt_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub manifest_digest: Hex<32>,
    pub unit_id: Identifier,
    pub producer_id: Uuid,
    pub producer_generation: U64Str,
    pub input_digest: Hex<32>,
    pub environment_digest: Hex<32>,
    pub output_digest: Hex<32>,
    pub oracle: OracleTrustClassV1,
    pub grade: EvidenceReceiptV2Grade,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceReceiptV2Fields {
    r#type: EvidenceReceiptV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    receipt_id: Uuid,
    attempt_binding_digest: Hex<32>,
    manifest_digest: Hex<32>,
    unit_id: Identifier,
    producer_id: Uuid,
    producer_generation: U64Str,
    input_digest: Hex<32>,
    environment_digest: Hex<32>,
    output_digest: Hex<32>,
    oracle: OracleTrustClassV1,
    grade: EvidenceReceiptV2Grade,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<EvidenceReceiptV2Fields>> for EvidenceReceiptV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<EvidenceReceiptV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            receipt_id: fields.receipt_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            manifest_digest: fields.manifest_digest,
            unit_id: fields.unit_id,
            producer_id: fields.producer_id,
            producer_generation: fields.producer_generation,
            input_digest: fields.input_digest,
            environment_digest: fields.environment_digest,
            output_digest: fields.output_digest,
            oracle: fields.oracle,
            grade: fields.grade,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for EvidenceReceiptV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 15)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("receipt_id", &self.receipt_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("unit_id", &self.unit_id)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("producer_generation", &self.producer_generation)?;
        state.serialize_field("input_digest", &self.input_digest)?;
        state.serialize_field("environment_digest", &self.environment_digest)?;
        state.serialize_field("output_digest", &self.output_digest)?;
        state.serialize_field("oracle", &self.oracle)?;
        state.serialize_field("grade", &self.grade)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl EvidenceReceiptV2 {
    pub const TYPE_NAME: &'static str = "EvidenceReceiptV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "receipt_id",
        "attempt_binding_digest",
        "manifest_digest",
        "unit_id",
        "producer_id",
        "producer_generation",
        "input_digest",
        "environment_digest",
        "output_digest",
        "oracle",
        "grade",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for EvidenceReceiptV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.receipt_id.canonical_cmp(&other.receipt_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| self.unit_id.canonical_cmp(&other.unit_id))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| {
                self.producer_generation
                    .canonical_cmp(&other.producer_generation)
            })
            .then_with(|| self.input_digest.canonical_cmp(&other.input_digest))
            .then_with(|| {
                self.environment_digest
                    .canonical_cmp(&other.environment_digest)
            })
            .then_with(|| self.output_digest.canonical_cmp(&other.output_digest))
            .then_with(|| self.oracle.canonical_cmp(&other.oracle))
            .then_with(|| self.grade.canonical_cmp(&other.grade))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(15u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.receipt_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.unit_id.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.producer_generation.canonical_size())
            .saturating_add(self.input_digest.canonical_size())
            .saturating_add(self.environment_digest.canonical_size())
            .saturating_add(self.output_digest.canonical_size())
            .saturating_add(self.oracle.canonical_size())
            .saturating_add(self.grade.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.receipt_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.unit_id.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.producer_generation.canonical_nesting())
                .max(self.input_digest.canonical_nesting())
                .max(self.environment_digest.canonical_nesting())
                .max(self.output_digest.canonical_nesting())
                .max(self.oracle.canonical_nesting())
                .max(self.grade.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.unit_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpectedAnalysisManifestV1Type {
    #[serde(rename = "ExpectedAnalysisManifestV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ExpectedAnalysisManifestV1Fields>"
)]
pub struct ExpectedAnalysisManifestV1 {
    pub r#type: ExpectedAnalysisManifestV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub manifest_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub candidate_tree: GitOidV1,
    pub policy_digest: Hex<32>,
    pub analysis_plan_digest: Hex<32>,
    pub required_units: SetOf<AnalysisUnitV1>,
    pub issuer_id: Uuid,
    pub issued_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectedAnalysisManifestV1Fields {
    r#type: ExpectedAnalysisManifestV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    manifest_id: Uuid,
    attempt_binding_digest: Hex<32>,
    candidate_tree: GitOidV1,
    policy_digest: Hex<32>,
    analysis_plan_digest: Hex<32>,
    required_units: SetOf<AnalysisUnitV1>,
    issuer_id: Uuid,
    issued_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ExpectedAnalysisManifestV1Fields>>
    for ExpectedAnalysisManifestV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ExpectedAnalysisManifestV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            manifest_id: fields.manifest_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            candidate_tree: fields.candidate_tree,
            policy_digest: fields.policy_digest,
            analysis_plan_digest: fields.analysis_plan_digest,
            required_units: fields.required_units,
            issuer_id: fields.issuer_id,
            issued_at: fields.issued_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ExpectedAnalysisManifestV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("manifest_id", &self.manifest_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("analysis_plan_digest", &self.analysis_plan_digest)?;
        state.serialize_field("required_units", &self.required_units)?;
        state.serialize_field("issuer_id", &self.issuer_id)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.end()
    }
}
impl ExpectedAnalysisManifestV1 {
    pub const TYPE_NAME: &'static str = "ExpectedAnalysisManifestV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "manifest_id",
        "attempt_binding_digest",
        "candidate_tree",
        "policy_digest",
        "analysis_plan_digest",
        "required_units",
        "issuer_id",
        "issued_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ExpectedAnalysisManifestV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.manifest_id.canonical_cmp(&other.manifest_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.analysis_plan_digest
                    .canonical_cmp(&other.analysis_plan_digest)
            })
            .then_with(|| self.required_units.canonical_cmp(&other.required_units))
            .then_with(|| self.issuer_id.canonical_cmp(&other.issuer_id))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.manifest_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.analysis_plan_digest.canonical_size())
            .saturating_add(self.required_units.canonical_size())
            .saturating_add(self.issuer_id.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.manifest_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.analysis_plan_digest.canonical_nesting())
                .max(self.required_units.canonical_nesting())
                .max(self.issuer_id.canonical_nesting())
                .max(self.issued_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalEffectJournalV1EntryKind {
    #[serde(rename = "PREPARED")]
    Prepared,
    #[serde(rename = "SEND_ARMED")]
    SendArmed,
    #[serde(rename = "OBSERVED")]
    Observed,
    #[serde(rename = "RESOLVED")]
    Resolved,
}
impl ExternalEffectJournalV1EntryKind {
    pub fn code(&self) -> u64 {
        match self {
            Self::Prepared => 0,
            Self::SendArmed => 1,
            Self::Observed => 2,
            Self::Resolved => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Prepared),
            1 => Some(Self::SendArmed),
            2 => Some(Self::Observed),
            3 => Some(Self::Resolved),
            _ => None,
        }
    }
}
impl CanonicalOrder for ExternalEffectJournalV1EntryKind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalEffectJournalV1Type {
    #[serde(rename = "ExternalEffectJournalV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ExternalEffectJournalV1Fields>"
)]
pub struct ExternalEffectJournalV1 {
    pub r#type: ExternalEffectJournalV1Type,
    pub wire_revision: WireRevision,
    pub journal_id: Uuid,
    pub sequence: U64Str,
    pub previous_digest: Hex<32>,
    pub scope: ScopeV1,
    pub effect_id: Uuid,
    pub intent_digest: Hex<32>,
    pub request_custody_digest: Hex<32>,
    pub state_digest: Hex<32>,
    pub entry_kind: ExternalEffectJournalV1EntryKind,
    pub entry_time: I64Str,
    pub db_start_certificate_digest: Opt<Hex<32>>,
    pub dispatcher_session_id: Opt<Uuid>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExternalEffectJournalV1Fields {
    r#type: ExternalEffectJournalV1Type,
    wire_revision: WireRevision,
    journal_id: Uuid,
    sequence: U64Str,
    previous_digest: Hex<32>,
    scope: ScopeV1,
    effect_id: Uuid,
    intent_digest: Hex<32>,
    request_custody_digest: Hex<32>,
    state_digest: Hex<32>,
    entry_kind: ExternalEffectJournalV1EntryKind,
    entry_time: I64Str,
    db_start_certificate_digest: Opt<Hex<32>>,
    dispatcher_session_id: Opt<Uuid>,
}
impl TryFrom<crate::primitives::ObjectOnly<ExternalEffectJournalV1Fields>>
    for ExternalEffectJournalV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ExternalEffectJournalV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            journal_id: fields.journal_id,
            sequence: fields.sequence,
            previous_digest: fields.previous_digest,
            scope: fields.scope,
            effect_id: fields.effect_id,
            intent_digest: fields.intent_digest,
            request_custody_digest: fields.request_custody_digest,
            state_digest: fields.state_digest,
            entry_kind: fields.entry_kind,
            entry_time: fields.entry_time,
            db_start_certificate_digest: fields.db_start_certificate_digest,
            dispatcher_session_id: fields.dispatcher_session_id,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ExternalEffectJournalV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 14)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("journal_id", &self.journal_id)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("previous_digest", &self.previous_digest)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("effect_id", &self.effect_id)?;
        state.serialize_field("intent_digest", &self.intent_digest)?;
        state.serialize_field("request_custody_digest", &self.request_custody_digest)?;
        state.serialize_field("state_digest", &self.state_digest)?;
        state.serialize_field("entry_kind", &self.entry_kind)?;
        state.serialize_field("entry_time", &self.entry_time)?;
        state.serialize_field(
            "db_start_certificate_digest",
            &self.db_start_certificate_digest,
        )?;
        state.serialize_field("dispatcher_session_id", &self.dispatcher_session_id)?;
        state.end()
    }
}
impl ExternalEffectJournalV1 {
    pub const TYPE_NAME: &'static str = "ExternalEffectJournalV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "journal_id",
        "sequence",
        "previous_digest",
        "scope",
        "effect_id",
        "intent_digest",
        "request_custody_digest",
        "state_digest",
        "entry_kind",
        "entry_time",
        "db_start_certificate_digest",
        "dispatcher_session_id",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ExternalEffectJournalV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.journal_id.canonical_cmp(&other.journal_id))
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.previous_digest.canonical_cmp(&other.previous_digest))
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.effect_id.canonical_cmp(&other.effect_id))
            .then_with(|| self.intent_digest.canonical_cmp(&other.intent_digest))
            .then_with(|| {
                self.request_custody_digest
                    .canonical_cmp(&other.request_custody_digest)
            })
            .then_with(|| self.state_digest.canonical_cmp(&other.state_digest))
            .then_with(|| self.entry_kind.canonical_cmp(&other.entry_kind))
            .then_with(|| self.entry_time.canonical_cmp(&other.entry_time))
            .then_with(|| {
                self.db_start_certificate_digest
                    .canonical_cmp(&other.db_start_certificate_digest)
            })
            .then_with(|| {
                self.dispatcher_session_id
                    .canonical_cmp(&other.dispatcher_session_id)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(14u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.journal_id.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.previous_digest.canonical_size())
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.effect_id.canonical_size())
            .saturating_add(self.intent_digest.canonical_size())
            .saturating_add(self.request_custody_digest.canonical_size())
            .saturating_add(self.state_digest.canonical_size())
            .saturating_add(self.entry_kind.canonical_size())
            .saturating_add(self.entry_time.canonical_size())
            .saturating_add(self.db_start_certificate_digest.canonical_size())
            .saturating_add(self.dispatcher_session_id.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.journal_id
                .canonical_nesting()
                .max(self.sequence.canonical_nesting())
                .max(self.previous_digest.canonical_nesting())
                .max(self.scope.canonical_nesting())
                .max(self.effect_id.canonical_nesting())
                .max(self.intent_digest.canonical_nesting())
                .max(self.request_custody_digest.canonical_nesting())
                .max(self.state_digest.canonical_nesting())
                .max(self.entry_kind.canonical_nesting())
                .max(self.entry_time.canonical_nesting())
                .max(self.db_start_certificate_digest.canonical_nesting())
                .max(self.dispatcher_session_id.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingCandidateV2Type {
    #[serde(rename = "FindingCandidateV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<FindingCandidateV2Fields>"
)]
pub struct FindingCandidateV2 {
    pub r#type: FindingCandidateV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub finding_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub unit_id: Identifier,
    pub claim_digest: Hex<32>,
    pub oracle: OracleTrustClassV1,
    pub evidence_digests: SetOf<Hex<32>>,
    pub producer_id: Uuid,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FindingCandidateV2Fields {
    r#type: FindingCandidateV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    finding_id: Uuid,
    attempt_binding_digest: Hex<32>,
    unit_id: Identifier,
    claim_digest: Hex<32>,
    oracle: OracleTrustClassV1,
    evidence_digests: SetOf<Hex<32>>,
    producer_id: Uuid,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<FindingCandidateV2Fields>> for FindingCandidateV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<FindingCandidateV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            finding_id: fields.finding_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            unit_id: fields.unit_id,
            claim_digest: fields.claim_digest,
            oracle: fields.oracle,
            evidence_digests: fields.evidence_digests,
            producer_id: fields.producer_id,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for FindingCandidateV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("finding_id", &self.finding_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("unit_id", &self.unit_id)?;
        state.serialize_field("claim_digest", &self.claim_digest)?;
        state.serialize_field("oracle", &self.oracle)?;
        state.serialize_field("evidence_digests", &self.evidence_digests)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl FindingCandidateV2 {
    pub const TYPE_NAME: &'static str = "FindingCandidateV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "finding_id",
        "attempt_binding_digest",
        "unit_id",
        "claim_digest",
        "oracle",
        "evidence_digests",
        "producer_id",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for FindingCandidateV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.finding_id.canonical_cmp(&other.finding_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.unit_id.canonical_cmp(&other.unit_id))
            .then_with(|| self.claim_digest.canonical_cmp(&other.claim_digest))
            .then_with(|| self.oracle.canonical_cmp(&other.oracle))
            .then_with(|| self.evidence_digests.canonical_cmp(&other.evidence_digests))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.finding_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.unit_id.canonical_size())
            .saturating_add(self.claim_digest.canonical_size())
            .saturating_add(self.oracle.canonical_size())
            .saturating_add(self.evidence_digests.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.finding_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.unit_id.canonical_nesting())
                .max(self.claim_digest.canonical_nesting())
                .max(self.oracle.canonical_nesting())
                .max(self.evidence_digests.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.unit_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateProofV2Conclusion {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "HUMAN_REVIEW_REQUIRED")]
    HumanReviewRequired,
    #[serde(rename = "PARTIALLY_AVAILABLE")]
    PartiallyAvailable,
    #[serde(rename = "STALE")]
    Stale,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "INVALID")]
    Invalid,
}
impl GateProofV2Conclusion {
    pub fn code(&self) -> u64 {
        match self {
            Self::Pass => 0,
            Self::Fail => 1,
            Self::Blocked => 2,
            Self::HumanReviewRequired => 3,
            Self::PartiallyAvailable => 4,
            Self::Stale => 5,
            Self::Revoked => 6,
            Self::Invalid => 7,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Pass),
            1 => Some(Self::Fail),
            2 => Some(Self::Blocked),
            3 => Some(Self::HumanReviewRequired),
            4 => Some(Self::PartiallyAvailable),
            5 => Some(Self::Stale),
            6 => Some(Self::Revoked),
            7 => Some(Self::Invalid),
            _ => None,
        }
    }
}
impl CanonicalOrder for GateProofV2Conclusion {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateProofV2Type {
    #[serde(rename = "GateProofV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<GateProofV2Fields>"
)]
pub struct GateProofV2 {
    pub r#type: GateProofV2Type,
    pub wire_revision: WireRevision,
    pub identity: EvaluationIdentityV2,
    pub conclusion: GateProofV2Conclusion,
    pub completion_receipt_digests: SetOf<Hex<32>>,
    pub reason_codes: SetOf<Identifier>,
    pub gate_authority_id: Uuid,
    pub gate_generation: U64Str,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GateProofV2Fields {
    r#type: GateProofV2Type,
    wire_revision: WireRevision,
    identity: EvaluationIdentityV2,
    conclusion: GateProofV2Conclusion,
    completion_receipt_digests: SetOf<Hex<32>>,
    reason_codes: SetOf<Identifier>,
    gate_authority_id: Uuid,
    gate_generation: U64Str,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<GateProofV2Fields>> for GateProofV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<GateProofV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            identity: fields.identity,
            conclusion: fields.conclusion,
            completion_receipt_digests: fields.completion_receipt_digests,
            reason_codes: fields.reason_codes,
            gate_authority_id: fields.gate_authority_id,
            gate_generation: fields.gate_generation,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for GateProofV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("identity", &self.identity)?;
        state.serialize_field("conclusion", &self.conclusion)?;
        state.serialize_field(
            "completion_receipt_digests",
            &self.completion_receipt_digests,
        )?;
        state.serialize_field("reason_codes", &self.reason_codes)?;
        state.serialize_field("gate_authority_id", &self.gate_authority_id)?;
        state.serialize_field("gate_generation", &self.gate_generation)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl GateProofV2 {
    pub const TYPE_NAME: &'static str = "GateProofV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "identity",
        "conclusion",
        "completion_receipt_digests",
        "reason_codes",
        "gate_authority_id",
        "gate_generation",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for GateProofV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.identity.canonical_cmp(&other.identity))
            .then_with(|| self.conclusion.canonical_cmp(&other.conclusion))
            .then_with(|| {
                self.completion_receipt_digests
                    .canonical_cmp(&other.completion_receipt_digests)
            })
            .then_with(|| self.reason_codes.canonical_cmp(&other.reason_codes))
            .then_with(|| {
                self.gate_authority_id
                    .canonical_cmp(&other.gate_authority_id)
            })
            .then_with(|| self.gate_generation.canonical_cmp(&other.gate_generation))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.identity.canonical_size())
            .saturating_add(self.conclusion.canonical_size())
            .saturating_add(self.completion_receipt_digests.canonical_size())
            .saturating_add(self.reason_codes.canonical_size())
            .saturating_add(self.gate_authority_id.canonical_size())
            .saturating_add(self.gate_generation.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.identity
                .canonical_nesting()
                .max(self.conclusion.canonical_nesting())
                .max(self.completion_receipt_digests.canonical_nesting())
                .max(self.reason_codes.canonical_nesting())
                .max(self.gate_authority_id.canonical_nesting())
                .max(self.gate_generation.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitOidV1Algorithm {
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA256")]
    Sha256,
}
impl GitOidV1Algorithm {
    pub fn code(&self) -> u64 {
        match self {
            Self::Sha1 => 0,
            Self::Sha256 => 1,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Sha1),
            1 => Some(Self::Sha256),
            _ => None,
        }
    }
}
impl CanonicalOrder for GitOidV1Algorithm {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitOidV1Type {
    #[serde(rename = "GitOidV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<GitOidV1Fields>"
)]
pub struct GitOidV1 {
    pub r#type: GitOidV1Type,
    pub wire_revision: WireRevision,
    pub algorithm: GitOidV1Algorithm,
    pub oid: Bytes,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GitOidV1Fields {
    r#type: GitOidV1Type,
    wire_revision: WireRevision,
    algorithm: GitOidV1Algorithm,
    oid: Bytes,
}
impl TryFrom<crate::primitives::ObjectOnly<GitOidV1Fields>> for GitOidV1 {
    type Error = ProjectionError;
    fn try_from(input: crate::primitives::ObjectOnly<GitOidV1Fields>) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            algorithm: fields.algorithm,
            oid: fields.oid,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for GitOidV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 4)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("algorithm", &self.algorithm)?;
        state.serialize_field("oid", &self.oid)?;
        state.end()
    }
}
impl GitOidV1 {
    pub const TYPE_NAME: &'static str = "GitOidV1";
    pub const FIELD_ORDER: &'static [&'static str] = &["type", "wire_revision", "algorithm", "oid"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for GitOidV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.algorithm.canonical_cmp(&other.algorithm))
            .then_with(|| self.oid.canonical_cmp(&other.oid))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(4u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.algorithm.canonical_size())
            .saturating_add(self.oid.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.algorithm
                .canonical_nesting()
                .max(self.oid.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitPublicationCapabilityV2EffectKind {
    #[serde(rename = "CREATE_BLOB")]
    CreateBlob,
    #[serde(rename = "CREATE_TREE")]
    CreateTree,
    #[serde(rename = "CREATE_COMMIT")]
    CreateCommit,
    #[serde(rename = "CREATE_REF")]
    CreateRef,
    #[serde(rename = "CREATE_FIX_PR")]
    CreateFixPr,
}
impl GitPublicationCapabilityV2EffectKind {
    pub fn code(&self) -> u64 {
        match self {
            Self::CreateBlob => 0,
            Self::CreateTree => 1,
            Self::CreateCommit => 2,
            Self::CreateRef => 3,
            Self::CreateFixPr => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::CreateBlob),
            1 => Some(Self::CreateTree),
            2 => Some(Self::CreateCommit),
            3 => Some(Self::CreateRef),
            4 => Some(Self::CreateFixPr),
            _ => None,
        }
    }
}
impl CanonicalOrder for GitPublicationCapabilityV2EffectKind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitPublicationCapabilityV2Type {
    #[serde(rename = "GitPublicationCapabilityV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<GitPublicationCapabilityV2Fields>"
)]
pub struct GitPublicationCapabilityV2 {
    pub r#type: GitPublicationCapabilityV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub capability_id: Uuid,
    pub publication_id: Uuid,
    pub effect_intent_id: Uuid,
    pub plan_digest: Hex<32>,
    pub approval_assertion_digest: Hex<32>,
    pub custody_digest: Hex<32>,
    pub gateway_identity: Uuid,
    pub gateway_generation: U64Str,
    pub provider_app_id: U64Str,
    pub installation_id: U64Str,
    pub effect_kind: GitPublicationCapabilityV2EffectKind,
    pub request_digest: Hex<32>,
    pub destination_ref: GitRef,
    pub expires_at: I64Str,
    pub nonce: Hex<32>,
    pub eligibility_proof_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GitPublicationCapabilityV2Fields {
    r#type: GitPublicationCapabilityV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    capability_id: Uuid,
    publication_id: Uuid,
    effect_intent_id: Uuid,
    plan_digest: Hex<32>,
    approval_assertion_digest: Hex<32>,
    custody_digest: Hex<32>,
    gateway_identity: Uuid,
    gateway_generation: U64Str,
    provider_app_id: U64Str,
    installation_id: U64Str,
    effect_kind: GitPublicationCapabilityV2EffectKind,
    request_digest: Hex<32>,
    destination_ref: GitRef,
    expires_at: I64Str,
    nonce: Hex<32>,
    eligibility_proof_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<GitPublicationCapabilityV2Fields>>
    for GitPublicationCapabilityV2
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<GitPublicationCapabilityV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            capability_id: fields.capability_id,
            publication_id: fields.publication_id,
            effect_intent_id: fields.effect_intent_id,
            plan_digest: fields.plan_digest,
            approval_assertion_digest: fields.approval_assertion_digest,
            custody_digest: fields.custody_digest,
            gateway_identity: fields.gateway_identity,
            gateway_generation: fields.gateway_generation,
            provider_app_id: fields.provider_app_id,
            installation_id: fields.installation_id,
            effect_kind: fields.effect_kind,
            request_digest: fields.request_digest,
            destination_ref: fields.destination_ref,
            expires_at: fields.expires_at,
            nonce: fields.nonce,
            eligibility_proof_digest: fields.eligibility_proof_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for GitPublicationCapabilityV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 19)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("capability_id", &self.capability_id)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("effect_intent_id", &self.effect_intent_id)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("approval_assertion_digest", &self.approval_assertion_digest)?;
        state.serialize_field("custody_digest", &self.custody_digest)?;
        state.serialize_field("gateway_identity", &self.gateway_identity)?;
        state.serialize_field("gateway_generation", &self.gateway_generation)?;
        state.serialize_field("provider_app_id", &self.provider_app_id)?;
        state.serialize_field("installation_id", &self.installation_id)?;
        state.serialize_field("effect_kind", &self.effect_kind)?;
        state.serialize_field("request_digest", &self.request_digest)?;
        state.serialize_field("destination_ref", &self.destination_ref)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("eligibility_proof_digest", &self.eligibility_proof_digest)?;
        state.end()
    }
}
impl GitPublicationCapabilityV2 {
    pub const TYPE_NAME: &'static str = "GitPublicationCapabilityV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "capability_id",
        "publication_id",
        "effect_intent_id",
        "plan_digest",
        "approval_assertion_digest",
        "custody_digest",
        "gateway_identity",
        "gateway_generation",
        "provider_app_id",
        "installation_id",
        "effect_kind",
        "request_digest",
        "destination_ref",
        "expires_at",
        "nonce",
        "eligibility_proof_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for GitPublicationCapabilityV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.capability_id.canonical_cmp(&other.capability_id))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.effect_intent_id.canonical_cmp(&other.effect_intent_id))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| {
                self.approval_assertion_digest
                    .canonical_cmp(&other.approval_assertion_digest)
            })
            .then_with(|| self.custody_digest.canonical_cmp(&other.custody_digest))
            .then_with(|| self.gateway_identity.canonical_cmp(&other.gateway_identity))
            .then_with(|| {
                self.gateway_generation
                    .canonical_cmp(&other.gateway_generation)
            })
            .then_with(|| self.provider_app_id.canonical_cmp(&other.provider_app_id))
            .then_with(|| self.installation_id.canonical_cmp(&other.installation_id))
            .then_with(|| self.effect_kind.canonical_cmp(&other.effect_kind))
            .then_with(|| self.request_digest.canonical_cmp(&other.request_digest))
            .then_with(|| self.destination_ref.canonical_cmp(&other.destination_ref))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| {
                self.eligibility_proof_digest
                    .canonical_cmp(&other.eligibility_proof_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(19u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.capability_id.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.effect_intent_id.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.approval_assertion_digest.canonical_size())
            .saturating_add(self.custody_digest.canonical_size())
            .saturating_add(self.gateway_identity.canonical_size())
            .saturating_add(self.gateway_generation.canonical_size())
            .saturating_add(self.provider_app_id.canonical_size())
            .saturating_add(self.installation_id.canonical_size())
            .saturating_add(self.effect_kind.canonical_size())
            .saturating_add(self.request_digest.canonical_size())
            .saturating_add(self.destination_ref.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.eligibility_proof_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.capability_id.canonical_nesting())
                .max(self.publication_id.canonical_nesting())
                .max(self.effect_intent_id.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.approval_assertion_digest.canonical_nesting())
                .max(self.custody_digest.canonical_nesting())
                .max(self.gateway_identity.canonical_nesting())
                .max(self.gateway_generation.canonical_nesting())
                .max(self.provider_app_id.canonical_nesting())
                .max(self.installation_id.canonical_nesting())
                .max(self.effect_kind.canonical_nesting())
                .max(self.request_digest.canonical_nesting())
                .max(self.destination_ref.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.eligibility_proof_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanApprovalAssertionV2Type {
    #[serde(rename = "HumanApprovalAssertionV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<HumanApprovalAssertionV2Fields>"
)]
pub struct HumanApprovalAssertionV2 {
    pub r#type: HumanApprovalAssertionV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub approval_id: Uuid,
    pub publication_id: Uuid,
    pub plan_digest: Hex<32>,
    pub custody_digest: Hex<32>,
    pub verification_set_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub candidate_tree: GitOidV1,
    pub destination_ref: GitRef,
    pub effects_digest: Hex<32>,
    pub user_id: Uuid,
    pub credential_binding_id: Uuid,
    pub credential_generation: U64Str,
    pub display_receipt_digest: Hex<32>,
    pub challenge_digest: Hex<32>,
    pub challenge_nonce: Hex<32>,
    pub ceremony_digest: Hex<32>,
    pub approval_authority_id: Uuid,
    pub approval_authority_generation: U64Str,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanApprovalAssertionV2Fields {
    r#type: HumanApprovalAssertionV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    approval_id: Uuid,
    publication_id: Uuid,
    plan_digest: Hex<32>,
    custody_digest: Hex<32>,
    verification_set_digest: Hex<32>,
    policy_digest: Hex<32>,
    candidate_tree: GitOidV1,
    destination_ref: GitRef,
    effects_digest: Hex<32>,
    user_id: Uuid,
    credential_binding_id: Uuid,
    credential_generation: U64Str,
    display_receipt_digest: Hex<32>,
    challenge_digest: Hex<32>,
    challenge_nonce: Hex<32>,
    ceremony_digest: Hex<32>,
    approval_authority_id: Uuid,
    approval_authority_generation: U64Str,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<HumanApprovalAssertionV2Fields>>
    for HumanApprovalAssertionV2
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<HumanApprovalAssertionV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            approval_id: fields.approval_id,
            publication_id: fields.publication_id,
            plan_digest: fields.plan_digest,
            custody_digest: fields.custody_digest,
            verification_set_digest: fields.verification_set_digest,
            policy_digest: fields.policy_digest,
            candidate_tree: fields.candidate_tree,
            destination_ref: fields.destination_ref,
            effects_digest: fields.effects_digest,
            user_id: fields.user_id,
            credential_binding_id: fields.credential_binding_id,
            credential_generation: fields.credential_generation,
            display_receipt_digest: fields.display_receipt_digest,
            challenge_digest: fields.challenge_digest,
            challenge_nonce: fields.challenge_nonce,
            ceremony_digest: fields.ceremony_digest,
            approval_authority_id: fields.approval_authority_id,
            approval_authority_generation: fields.approval_authority_generation,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for HumanApprovalAssertionV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 23)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("approval_id", &self.approval_id)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("custody_digest", &self.custody_digest)?;
        state.serialize_field("verification_set_digest", &self.verification_set_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("destination_ref", &self.destination_ref)?;
        state.serialize_field("effects_digest", &self.effects_digest)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("credential_binding_id", &self.credential_binding_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("display_receipt_digest", &self.display_receipt_digest)?;
        state.serialize_field("challenge_digest", &self.challenge_digest)?;
        state.serialize_field("challenge_nonce", &self.challenge_nonce)?;
        state.serialize_field("ceremony_digest", &self.ceremony_digest)?;
        state.serialize_field("approval_authority_id", &self.approval_authority_id)?;
        state.serialize_field(
            "approval_authority_generation",
            &self.approval_authority_generation,
        )?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl HumanApprovalAssertionV2 {
    pub const TYPE_NAME: &'static str = "HumanApprovalAssertionV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "approval_id",
        "publication_id",
        "plan_digest",
        "custody_digest",
        "verification_set_digest",
        "policy_digest",
        "candidate_tree",
        "destination_ref",
        "effects_digest",
        "user_id",
        "credential_binding_id",
        "credential_generation",
        "display_receipt_digest",
        "challenge_digest",
        "challenge_nonce",
        "ceremony_digest",
        "approval_authority_id",
        "approval_authority_generation",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for HumanApprovalAssertionV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.approval_id.canonical_cmp(&other.approval_id))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| self.custody_digest.canonical_cmp(&other.custody_digest))
            .then_with(|| {
                self.verification_set_digest
                    .canonical_cmp(&other.verification_set_digest)
            })
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.destination_ref.canonical_cmp(&other.destination_ref))
            .then_with(|| self.effects_digest.canonical_cmp(&other.effects_digest))
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| {
                self.credential_binding_id
                    .canonical_cmp(&other.credential_binding_id)
            })
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| {
                self.display_receipt_digest
                    .canonical_cmp(&other.display_receipt_digest)
            })
            .then_with(|| self.challenge_digest.canonical_cmp(&other.challenge_digest))
            .then_with(|| self.challenge_nonce.canonical_cmp(&other.challenge_nonce))
            .then_with(|| self.ceremony_digest.canonical_cmp(&other.ceremony_digest))
            .then_with(|| {
                self.approval_authority_id
                    .canonical_cmp(&other.approval_authority_id)
            })
            .then_with(|| {
                self.approval_authority_generation
                    .canonical_cmp(&other.approval_authority_generation)
            })
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(23u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.approval_id.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.custody_digest.canonical_size())
            .saturating_add(self.verification_set_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.destination_ref.canonical_size())
            .saturating_add(self.effects_digest.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.credential_binding_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.display_receipt_digest.canonical_size())
            .saturating_add(self.challenge_digest.canonical_size())
            .saturating_add(self.challenge_nonce.canonical_size())
            .saturating_add(self.ceremony_digest.canonical_size())
            .saturating_add(self.approval_authority_id.canonical_size())
            .saturating_add(self.approval_authority_generation.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.approval_id.canonical_nesting())
                .max(self.publication_id.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.custody_digest.canonical_nesting())
                .max(self.verification_set_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.destination_ref.canonical_nesting())
                .max(self.effects_digest.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.credential_binding_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.display_receipt_digest.canonical_nesting())
                .max(self.challenge_digest.canonical_nesting())
                .max(self.challenge_nonce.canonical_nesting())
                .max(self.ceremony_digest.canonical_nesting())
                .max(self.approval_authority_id.canonical_nesting())
                .max(self.approval_authority_generation.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalEpochFenceV1Type {
    #[serde(rename = "JournalEpochFenceV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<JournalEpochFenceV1Fields>"
)]
pub struct JournalEpochFenceV1 {
    pub r#type: JournalEpochFenceV1Type,
    pub wire_revision: WireRevision,
    pub journal_id: Uuid,
    pub sequence: U64Str,
    pub previous_digest: Hex<32>,
    pub tenant_id: Uuid,
    pub previous_epoch: Uuid,
    pub current_epoch: Uuid,
    pub old_sender_fencing_evidence_digest: Hex<32>,
    pub recorded_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalEpochFenceV1Fields {
    r#type: JournalEpochFenceV1Type,
    wire_revision: WireRevision,
    journal_id: Uuid,
    sequence: U64Str,
    previous_digest: Hex<32>,
    tenant_id: Uuid,
    previous_epoch: Uuid,
    current_epoch: Uuid,
    old_sender_fencing_evidence_digest: Hex<32>,
    recorded_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<JournalEpochFenceV1Fields>> for JournalEpochFenceV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<JournalEpochFenceV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            journal_id: fields.journal_id,
            sequence: fields.sequence,
            previous_digest: fields.previous_digest,
            tenant_id: fields.tenant_id,
            previous_epoch: fields.previous_epoch,
            current_epoch: fields.current_epoch,
            old_sender_fencing_evidence_digest: fields.old_sender_fencing_evidence_digest,
            recorded_at: fields.recorded_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for JournalEpochFenceV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("journal_id", &self.journal_id)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("previous_digest", &self.previous_digest)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("previous_epoch", &self.previous_epoch)?;
        state.serialize_field("current_epoch", &self.current_epoch)?;
        state.serialize_field(
            "old_sender_fencing_evidence_digest",
            &self.old_sender_fencing_evidence_digest,
        )?;
        state.serialize_field("recorded_at", &self.recorded_at)?;
        state.end()
    }
}
impl JournalEpochFenceV1 {
    pub const TYPE_NAME: &'static str = "JournalEpochFenceV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "journal_id",
        "sequence",
        "previous_digest",
        "tenant_id",
        "previous_epoch",
        "current_epoch",
        "old_sender_fencing_evidence_digest",
        "recorded_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for JournalEpochFenceV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.journal_id.canonical_cmp(&other.journal_id))
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.previous_digest.canonical_cmp(&other.previous_digest))
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.previous_epoch.canonical_cmp(&other.previous_epoch))
            .then_with(|| self.current_epoch.canonical_cmp(&other.current_epoch))
            .then_with(|| {
                self.old_sender_fencing_evidence_digest
                    .canonical_cmp(&other.old_sender_fencing_evidence_digest)
            })
            .then_with(|| self.recorded_at.canonical_cmp(&other.recorded_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.journal_id.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.previous_digest.canonical_size())
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.previous_epoch.canonical_size())
            .saturating_add(self.current_epoch.canonical_size())
            .saturating_add(self.old_sender_fencing_evidence_digest.canonical_size())
            .saturating_add(self.recorded_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.journal_id
                .canonical_nesting()
                .max(self.sequence.canonical_nesting())
                .max(self.previous_digest.canonical_nesting())
                .max(self.tenant_id.canonical_nesting())
                .max(self.previous_epoch.canonical_nesting())
                .max(self.current_epoch.canonical_nesting())
                .max(self.old_sender_fencing_evidence_digest.canonical_nesting())
                .max(self.recorded_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalWitnessReceiptV1Type {
    #[serde(rename = "JournalWitnessReceiptV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<JournalWitnessReceiptV1Fields>"
)]
pub struct JournalWitnessReceiptV1 {
    pub r#type: JournalWitnessReceiptV1Type,
    pub wire_revision: WireRevision,
    pub journal_id: Uuid,
    pub entry_digest: Hex<32>,
    pub sequence: U64Str,
    pub authority_epoch: Uuid,
    pub witness_id: Uuid,
    pub witness_generation: U64Str,
    pub durable_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalWitnessReceiptV1Fields {
    r#type: JournalWitnessReceiptV1Type,
    wire_revision: WireRevision,
    journal_id: Uuid,
    entry_digest: Hex<32>,
    sequence: U64Str,
    authority_epoch: Uuid,
    witness_id: Uuid,
    witness_generation: U64Str,
    durable_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<JournalWitnessReceiptV1Fields>>
    for JournalWitnessReceiptV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<JournalWitnessReceiptV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            journal_id: fields.journal_id,
            entry_digest: fields.entry_digest,
            sequence: fields.sequence,
            authority_epoch: fields.authority_epoch,
            witness_id: fields.witness_id,
            witness_generation: fields.witness_generation,
            durable_at: fields.durable_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for JournalWitnessReceiptV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 9)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("journal_id", &self.journal_id)?;
        state.serialize_field("entry_digest", &self.entry_digest)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.serialize_field("witness_id", &self.witness_id)?;
        state.serialize_field("witness_generation", &self.witness_generation)?;
        state.serialize_field("durable_at", &self.durable_at)?;
        state.end()
    }
}
impl JournalWitnessReceiptV1 {
    pub const TYPE_NAME: &'static str = "JournalWitnessReceiptV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "journal_id",
        "entry_digest",
        "sequence",
        "authority_epoch",
        "witness_id",
        "witness_generation",
        "durable_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for JournalWitnessReceiptV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.journal_id.canonical_cmp(&other.journal_id))
            .then_with(|| self.entry_digest.canonical_cmp(&other.entry_digest))
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
            .then_with(|| self.witness_id.canonical_cmp(&other.witness_id))
            .then_with(|| {
                self.witness_generation
                    .canonical_cmp(&other.witness_generation)
            })
            .then_with(|| self.durable_at.canonical_cmp(&other.durable_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(9u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.journal_id.canonical_size())
            .saturating_add(self.entry_digest.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
            .saturating_add(self.witness_id.canonical_size())
            .saturating_add(self.witness_generation.canonical_size())
            .saturating_add(self.durable_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.journal_id
                .canonical_nesting()
                .max(self.entry_digest.canonical_nesting())
                .max(self.sequence.canonical_nesting())
                .max(self.authority_epoch.canonical_nesting())
                .max(self.witness_id.canonical_nesting())
                .max(self.witness_generation.canonical_nesting())
                .max(self.durable_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestDescriptorV1Kind {
    #[serde(rename = "SCOPES")]
    Scopes,
    #[serde(rename = "FILES_SYMBOLS")]
    FilesSymbols,
    #[serde(rename = "EXCLUSIONS")]
    Exclusions,
    #[serde(rename = "SANDBOX_REQUIREMENTS")]
    SandboxRequirements,
    #[serde(rename = "COVERAGE_EXPECTATIONS")]
    CoverageExpectations,
    #[serde(rename = "ANALYZER_CONFIGURATION")]
    AnalyzerConfiguration,
}
impl ManifestDescriptorV1Kind {
    pub fn code(&self) -> u64 {
        match self {
            Self::Scopes => 0,
            Self::FilesSymbols => 1,
            Self::Exclusions => 2,
            Self::SandboxRequirements => 3,
            Self::CoverageExpectations => 4,
            Self::AnalyzerConfiguration => 5,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Scopes),
            1 => Some(Self::FilesSymbols),
            2 => Some(Self::Exclusions),
            3 => Some(Self::SandboxRequirements),
            4 => Some(Self::CoverageExpectations),
            5 => Some(Self::AnalyzerConfiguration),
            _ => None,
        }
    }
}
impl CanonicalOrder for ManifestDescriptorV1Kind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestDescriptorV1Type {
    #[serde(rename = "ManifestDescriptorV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ManifestDescriptorV1Fields>"
)]
pub struct ManifestDescriptorV1 {
    pub r#type: ManifestDescriptorV1Type,
    pub wire_revision: WireRevision,
    pub kind: ManifestDescriptorV1Kind,
    pub entries: SetOf<DescriptorEntryV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestDescriptorV1Fields {
    r#type: ManifestDescriptorV1Type,
    wire_revision: WireRevision,
    kind: ManifestDescriptorV1Kind,
    entries: SetOf<DescriptorEntryV1>,
}
impl TryFrom<crate::primitives::ObjectOnly<ManifestDescriptorV1Fields>> for ManifestDescriptorV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ManifestDescriptorV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            kind: fields.kind,
            entries: fields.entries,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ManifestDescriptorV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 4)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("entries", &self.entries)?;
        state.end()
    }
}
impl ManifestDescriptorV1 {
    pub const TYPE_NAME: &'static str = "ManifestDescriptorV1";
    pub const FIELD_ORDER: &'static [&'static str] = &["type", "wire_revision", "kind", "entries"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ManifestDescriptorV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.kind.canonical_cmp(&other.kind))
            .then_with(|| self.entries.canonical_cmp(&other.entries))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(4u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.kind.canonical_size())
            .saturating_add(self.entries.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.kind
                .canonical_nesting()
                .max(self.entries.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualCaseStateV1State {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "EVIDENCE_PENDING")]
    EvidencePending,
    #[serde(rename = "RESOLVED_CONFIRMED")]
    ResolvedConfirmed,
    #[serde(rename = "RESOLVED_NOT_EXECUTED")]
    ResolvedNotExecuted,
    #[serde(rename = "RESOLVED_REEVALUATE")]
    ResolvedReevaluate,
    #[serde(rename = "ACCEPTED_RISK")]
    AcceptedRisk,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "CLOSED_UNRESOLVED")]
    ClosedUnresolved,
    #[serde(rename = "EXPIRED")]
    Expired,
}
impl ManualCaseStateV1State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Open => 0,
            Self::EvidencePending => 1,
            Self::ResolvedConfirmed => 2,
            Self::ResolvedNotExecuted => 3,
            Self::ResolvedReevaluate => 4,
            Self::AcceptedRisk => 5,
            Self::Rejected => 6,
            Self::ClosedUnresolved => 7,
            Self::Expired => 8,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Open),
            1 => Some(Self::EvidencePending),
            2 => Some(Self::ResolvedConfirmed),
            3 => Some(Self::ResolvedNotExecuted),
            4 => Some(Self::ResolvedReevaluate),
            5 => Some(Self::AcceptedRisk),
            6 => Some(Self::Rejected),
            7 => Some(Self::ClosedUnresolved),
            8 => Some(Self::Expired),
            _ => None,
        }
    }
}
impl CanonicalOrder for ManualCaseStateV1State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualCaseStateV1Type {
    #[serde(rename = "ManualCaseStateV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ManualCaseStateV1Fields>"
)]
pub struct ManualCaseStateV1 {
    pub r#type: ManualCaseStateV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub case_id: Uuid,
    pub subject_type: Identifier,
    pub subject_digest: Hex<32>,
    pub revision: U64Str,
    pub state: ManualCaseStateV1State,
    pub recorded_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManualCaseStateV1Fields {
    r#type: ManualCaseStateV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    case_id: Uuid,
    subject_type: Identifier,
    subject_digest: Hex<32>,
    revision: U64Str,
    state: ManualCaseStateV1State,
    recorded_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ManualCaseStateV1Fields>> for ManualCaseStateV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ManualCaseStateV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            case_id: fields.case_id,
            subject_type: fields.subject_type,
            subject_digest: fields.subject_digest,
            revision: fields.revision,
            state: fields.state,
            recorded_at: fields.recorded_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ManualCaseStateV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("case_id", &self.case_id)?;
        state.serialize_field("subject_type", &self.subject_type)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("revision", &self.revision)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("recorded_at", &self.recorded_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ManualCaseStateV1 {
    pub const TYPE_NAME: &'static str = "ManualCaseStateV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "case_id",
        "subject_type",
        "subject_digest",
        "revision",
        "state",
        "recorded_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ManualCaseStateV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.case_id.canonical_cmp(&other.case_id))
            .then_with(|| self.subject_type.canonical_cmp(&other.subject_type))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.revision.canonical_cmp(&other.revision))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| self.recorded_at.canonical_cmp(&other.recorded_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.case_id.canonical_size())
            .saturating_add(self.subject_type.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.revision.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.recorded_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.case_id.canonical_nesting())
                .max(self.subject_type.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.revision.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.recorded_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualResolutionActionV1Kind {
    #[serde(rename = "DISPUTED_FINDING")]
    DisputedFinding,
    #[serde(rename = "FALSE_POSITIVE")]
    FalsePositive,
    #[serde(rename = "UNSUPPORTED_VERIFICATION")]
    UnsupportedVerification,
    #[serde(rename = "EXTERNAL_EFFECT_UNCERTAINTY")]
    ExternalEffectUncertainty,
    #[serde(rename = "APPROVAL_RECOVERY")]
    ApprovalRecovery,
    #[serde(rename = "UNTRUSTED_ORACLE")]
    UntrustedOracle,
}
impl ManualResolutionActionV1Kind {
    pub fn code(&self) -> u64 {
        match self {
            Self::DisputedFinding => 0,
            Self::FalsePositive => 1,
            Self::UnsupportedVerification => 2,
            Self::ExternalEffectUncertainty => 3,
            Self::ApprovalRecovery => 4,
            Self::UntrustedOracle => 5,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::DisputedFinding),
            1 => Some(Self::FalsePositive),
            2 => Some(Self::UnsupportedVerification),
            3 => Some(Self::ExternalEffectUncertainty),
            4 => Some(Self::ApprovalRecovery),
            5 => Some(Self::UntrustedOracle),
            _ => None,
        }
    }
}
impl CanonicalOrder for ManualResolutionActionV1Kind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualResolutionActionV1Decision {
    #[serde(rename = "REQUEST_FRESH_EVALUATION")]
    RequestFreshEvaluation,
    #[serde(rename = "PROVIDE_ORACLE")]
    ProvideOracle,
    #[serde(rename = "CONFIRM_EFFECT")]
    ConfirmEffect,
    #[serde(rename = "PROVE_TERMINAL_NONEXECUTION")]
    ProveTerminalNonexecution,
    #[serde(rename = "REMAIN_BLOCKED")]
    RemainBlocked,
    #[serde(rename = "REVOKE")]
    Revoke,
    #[serde(rename = "ACCEPT_RISK")]
    AcceptRisk,
    #[serde(rename = "CLOSE_UNRESOLVED")]
    CloseUnresolved,
    #[serde(rename = "CREATE_POLICY_EXCLUSION")]
    CreatePolicyExclusion,
}
impl ManualResolutionActionV1Decision {
    pub fn code(&self) -> u64 {
        match self {
            Self::RequestFreshEvaluation => 0,
            Self::ProvideOracle => 1,
            Self::ConfirmEffect => 2,
            Self::ProveTerminalNonexecution => 3,
            Self::RemainBlocked => 4,
            Self::Revoke => 5,
            Self::AcceptRisk => 6,
            Self::CloseUnresolved => 7,
            Self::CreatePolicyExclusion => 8,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::RequestFreshEvaluation),
            1 => Some(Self::ProvideOracle),
            2 => Some(Self::ConfirmEffect),
            3 => Some(Self::ProveTerminalNonexecution),
            4 => Some(Self::RemainBlocked),
            5 => Some(Self::Revoke),
            6 => Some(Self::AcceptRisk),
            7 => Some(Self::CloseUnresolved),
            8 => Some(Self::CreatePolicyExclusion),
            _ => None,
        }
    }
}
impl CanonicalOrder for ManualResolutionActionV1Decision {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualResolutionActionV1Type {
    #[serde(rename = "ManualResolutionActionV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ManualResolutionActionV1Fields>"
)]
pub struct ManualResolutionActionV1 {
    pub r#type: ManualResolutionActionV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub resolution_id: Uuid,
    pub case_id: Uuid,
    pub subject_digest: Hex<32>,
    pub prior_state_digest: Hex<32>,
    pub actor_id: Uuid,
    pub actor_scope_digest: Hex<32>,
    pub kind: ManualResolutionActionV1Kind,
    pub decision: ManualResolutionActionV1Decision,
    pub reason_digest: Hex<32>,
    pub evidence_digests: SetOf<Hex<32>>,
    pub created_at: I64Str,
    pub expires_at: I64Str,
    pub audit_record_id: Uuid,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManualResolutionActionV1Fields {
    r#type: ManualResolutionActionV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    resolution_id: Uuid,
    case_id: Uuid,
    subject_digest: Hex<32>,
    prior_state_digest: Hex<32>,
    actor_id: Uuid,
    actor_scope_digest: Hex<32>,
    kind: ManualResolutionActionV1Kind,
    decision: ManualResolutionActionV1Decision,
    reason_digest: Hex<32>,
    evidence_digests: SetOf<Hex<32>>,
    created_at: I64Str,
    expires_at: I64Str,
    audit_record_id: Uuid,
}
impl TryFrom<crate::primitives::ObjectOnly<ManualResolutionActionV1Fields>>
    for ManualResolutionActionV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ManualResolutionActionV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            resolution_id: fields.resolution_id,
            case_id: fields.case_id,
            subject_digest: fields.subject_digest,
            prior_state_digest: fields.prior_state_digest,
            actor_id: fields.actor_id,
            actor_scope_digest: fields.actor_scope_digest,
            kind: fields.kind,
            decision: fields.decision,
            reason_digest: fields.reason_digest,
            evidence_digests: fields.evidence_digests,
            created_at: fields.created_at,
            expires_at: fields.expires_at,
            audit_record_id: fields.audit_record_id,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ManualResolutionActionV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 16)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("resolution_id", &self.resolution_id)?;
        state.serialize_field("case_id", &self.case_id)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("prior_state_digest", &self.prior_state_digest)?;
        state.serialize_field("actor_id", &self.actor_id)?;
        state.serialize_field("actor_scope_digest", &self.actor_scope_digest)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("decision", &self.decision)?;
        state.serialize_field("reason_digest", &self.reason_digest)?;
        state.serialize_field("evidence_digests", &self.evidence_digests)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("audit_record_id", &self.audit_record_id)?;
        state.end()
    }
}
impl ManualResolutionActionV1 {
    pub const TYPE_NAME: &'static str = "ManualResolutionActionV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "resolution_id",
        "case_id",
        "subject_digest",
        "prior_state_digest",
        "actor_id",
        "actor_scope_digest",
        "kind",
        "decision",
        "reason_digest",
        "evidence_digests",
        "created_at",
        "expires_at",
        "audit_record_id",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ManualResolutionActionV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.resolution_id.canonical_cmp(&other.resolution_id))
            .then_with(|| self.case_id.canonical_cmp(&other.case_id))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| {
                self.prior_state_digest
                    .canonical_cmp(&other.prior_state_digest)
            })
            .then_with(|| self.actor_id.canonical_cmp(&other.actor_id))
            .then_with(|| {
                self.actor_scope_digest
                    .canonical_cmp(&other.actor_scope_digest)
            })
            .then_with(|| self.kind.canonical_cmp(&other.kind))
            .then_with(|| self.decision.canonical_cmp(&other.decision))
            .then_with(|| self.reason_digest.canonical_cmp(&other.reason_digest))
            .then_with(|| self.evidence_digests.canonical_cmp(&other.evidence_digests))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| self.audit_record_id.canonical_cmp(&other.audit_record_id))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(16u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.resolution_id.canonical_size())
            .saturating_add(self.case_id.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.prior_state_digest.canonical_size())
            .saturating_add(self.actor_id.canonical_size())
            .saturating_add(self.actor_scope_digest.canonical_size())
            .saturating_add(self.kind.canonical_size())
            .saturating_add(self.decision.canonical_size())
            .saturating_add(self.reason_digest.canonical_size())
            .saturating_add(self.evidence_digests.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.audit_record_id.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.resolution_id.canonical_nesting())
                .max(self.case_id.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.prior_state_digest.canonical_nesting())
                .max(self.actor_id.canonical_nesting())
                .max(self.actor_scope_digest.canonical_nesting())
                .max(self.kind.canonical_nesting())
                .max(self.decision.canonical_nesting())
                .max(self.reason_digest.canonical_nesting())
                .max(self.evidence_digests.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.audit_record_id.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualResolutionV1Type {
    #[serde(rename = "ManualResolutionV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ManualResolutionV1Fields>"
)]
pub struct ManualResolutionV1 {
    pub r#type: ManualResolutionV1Type,
    pub wire_revision: WireRevision,
    pub action: ManualResolutionActionV1,
    pub authentication_ceremony_digest: Hex<32>,
    pub challenge_nonce: Hex<32>,
    pub resolution_authority_id: Uuid,
    pub resolution_authority_generation: U64Str,
    pub committed_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManualResolutionV1Fields {
    r#type: ManualResolutionV1Type,
    wire_revision: WireRevision,
    action: ManualResolutionActionV1,
    authentication_ceremony_digest: Hex<32>,
    challenge_nonce: Hex<32>,
    resolution_authority_id: Uuid,
    resolution_authority_generation: U64Str,
    committed_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ManualResolutionV1Fields>> for ManualResolutionV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ManualResolutionV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            action: fields.action,
            authentication_ceremony_digest: fields.authentication_ceremony_digest,
            challenge_nonce: fields.challenge_nonce,
            resolution_authority_id: fields.resolution_authority_id,
            resolution_authority_generation: fields.resolution_authority_generation,
            committed_at: fields.committed_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ManualResolutionV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("action", &self.action)?;
        state.serialize_field(
            "authentication_ceremony_digest",
            &self.authentication_ceremony_digest,
        )?;
        state.serialize_field("challenge_nonce", &self.challenge_nonce)?;
        state.serialize_field("resolution_authority_id", &self.resolution_authority_id)?;
        state.serialize_field(
            "resolution_authority_generation",
            &self.resolution_authority_generation,
        )?;
        state.serialize_field("committed_at", &self.committed_at)?;
        state.end()
    }
}
impl ManualResolutionV1 {
    pub const TYPE_NAME: &'static str = "ManualResolutionV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "action",
        "authentication_ceremony_digest",
        "challenge_nonce",
        "resolution_authority_id",
        "resolution_authority_generation",
        "committed_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ManualResolutionV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.action.canonical_cmp(&other.action))
            .then_with(|| {
                self.authentication_ceremony_digest
                    .canonical_cmp(&other.authentication_ceremony_digest)
            })
            .then_with(|| self.challenge_nonce.canonical_cmp(&other.challenge_nonce))
            .then_with(|| {
                self.resolution_authority_id
                    .canonical_cmp(&other.resolution_authority_id)
            })
            .then_with(|| {
                self.resolution_authority_generation
                    .canonical_cmp(&other.resolution_authority_generation)
            })
            .then_with(|| self.committed_at.canonical_cmp(&other.committed_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.action.canonical_size())
            .saturating_add(self.authentication_ceremony_digest.canonical_size())
            .saturating_add(self.challenge_nonce.canonical_size())
            .saturating_add(self.resolution_authority_id.canonical_size())
            .saturating_add(self.resolution_authority_generation.canonical_size())
            .saturating_add(self.committed_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.action
                .canonical_nesting()
                .max(self.authentication_ceremony_digest.canonical_nesting())
                .max(self.challenge_nonce.canonical_nesting())
                .max(self.resolution_authority_id.canonical_nesting())
                .max(self.resolution_authority_generation.canonical_nesting())
                .max(self.committed_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeSubjectV2CandidateKind {
    #[serde(rename = "HEAD_ONLY_NONMERGE")]
    HeadOnlyNonmerge,
    #[serde(rename = "TEST_MERGE")]
    TestMerge,
    #[serde(rename = "MERGE_GROUP")]
    MergeGroup,
    #[serde(rename = "PROVIDER_EQUIVALENT")]
    ProviderEquivalent,
}
impl MergeSubjectV2CandidateKind {
    pub fn code(&self) -> u64 {
        match self {
            Self::HeadOnlyNonmerge => 0,
            Self::TestMerge => 1,
            Self::MergeGroup => 2,
            Self::ProviderEquivalent => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::HeadOnlyNonmerge),
            1 => Some(Self::TestMerge),
            2 => Some(Self::MergeGroup),
            3 => Some(Self::ProviderEquivalent),
            _ => None,
        }
    }
}
impl CanonicalOrder for MergeSubjectV2CandidateKind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeSubjectV2MergeMethod {
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "SQUASH")]
    Squash,
    #[serde(rename = "REBASE")]
    Rebase,
    #[serde(rename = "QUEUE")]
    Queue,
    #[serde(rename = "ADVISORY")]
    Advisory,
}
impl MergeSubjectV2MergeMethod {
    pub fn code(&self) -> u64 {
        match self {
            Self::Merge => 0,
            Self::Squash => 1,
            Self::Rebase => 2,
            Self::Queue => 3,
            Self::Advisory => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Merge),
            1 => Some(Self::Squash),
            2 => Some(Self::Rebase),
            3 => Some(Self::Queue),
            4 => Some(Self::Advisory),
            _ => None,
        }
    }
}
impl CanonicalOrder for MergeSubjectV2MergeMethod {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeSubjectV2Type {
    #[serde(rename = "MergeSubjectV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<MergeSubjectV2Fields>"
)]
pub struct MergeSubjectV2 {
    pub r#type: MergeSubjectV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub provider: Provider,
    pub pr_number: U64Str,
    pub head_commit: GitOidV1,
    pub head_tree: GitOidV1,
    pub base_ref: GitRef,
    pub base_commit: GitOidV1,
    pub base_tree: GitOidV1,
    pub candidate_kind: MergeSubjectV2CandidateKind,
    pub candidate_commit: GitOidV1,
    pub candidate_tree: GitOidV1,
    pub merge_method: MergeSubjectV2MergeMethod,
    pub provider_generation: U64Str,
    pub observed_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MergeSubjectV2Fields {
    r#type: MergeSubjectV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    provider: Provider,
    pr_number: U64Str,
    head_commit: GitOidV1,
    head_tree: GitOidV1,
    base_ref: GitRef,
    base_commit: GitOidV1,
    base_tree: GitOidV1,
    candidate_kind: MergeSubjectV2CandidateKind,
    candidate_commit: GitOidV1,
    candidate_tree: GitOidV1,
    merge_method: MergeSubjectV2MergeMethod,
    provider_generation: U64Str,
    observed_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<MergeSubjectV2Fields>> for MergeSubjectV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<MergeSubjectV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            provider: fields.provider,
            pr_number: fields.pr_number,
            head_commit: fields.head_commit,
            head_tree: fields.head_tree,
            base_ref: fields.base_ref,
            base_commit: fields.base_commit,
            base_tree: fields.base_tree,
            candidate_kind: fields.candidate_kind,
            candidate_commit: fields.candidate_commit,
            candidate_tree: fields.candidate_tree,
            merge_method: fields.merge_method,
            provider_generation: fields.provider_generation,
            observed_at: fields.observed_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for MergeSubjectV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 17)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("provider", &self.provider)?;
        state.serialize_field("pr_number", &self.pr_number)?;
        state.serialize_field("head_commit", &self.head_commit)?;
        state.serialize_field("head_tree", &self.head_tree)?;
        state.serialize_field("base_ref", &self.base_ref)?;
        state.serialize_field("base_commit", &self.base_commit)?;
        state.serialize_field("base_tree", &self.base_tree)?;
        state.serialize_field("candidate_kind", &self.candidate_kind)?;
        state.serialize_field("candidate_commit", &self.candidate_commit)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("merge_method", &self.merge_method)?;
        state.serialize_field("provider_generation", &self.provider_generation)?;
        state.serialize_field("observed_at", &self.observed_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl MergeSubjectV2 {
    pub const TYPE_NAME: &'static str = "MergeSubjectV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "provider",
        "pr_number",
        "head_commit",
        "head_tree",
        "base_ref",
        "base_commit",
        "base_tree",
        "candidate_kind",
        "candidate_commit",
        "candidate_tree",
        "merge_method",
        "provider_generation",
        "observed_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for MergeSubjectV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.provider.canonical_cmp(&other.provider))
            .then_with(|| self.pr_number.canonical_cmp(&other.pr_number))
            .then_with(|| self.head_commit.canonical_cmp(&other.head_commit))
            .then_with(|| self.head_tree.canonical_cmp(&other.head_tree))
            .then_with(|| self.base_ref.canonical_cmp(&other.base_ref))
            .then_with(|| self.base_commit.canonical_cmp(&other.base_commit))
            .then_with(|| self.base_tree.canonical_cmp(&other.base_tree))
            .then_with(|| self.candidate_kind.canonical_cmp(&other.candidate_kind))
            .then_with(|| self.candidate_commit.canonical_cmp(&other.candidate_commit))
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.merge_method.canonical_cmp(&other.merge_method))
            .then_with(|| {
                self.provider_generation
                    .canonical_cmp(&other.provider_generation)
            })
            .then_with(|| self.observed_at.canonical_cmp(&other.observed_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(17u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.provider.canonical_size())
            .saturating_add(self.pr_number.canonical_size())
            .saturating_add(self.head_commit.canonical_size())
            .saturating_add(self.head_tree.canonical_size())
            .saturating_add(self.base_ref.canonical_size())
            .saturating_add(self.base_commit.canonical_size())
            .saturating_add(self.base_tree.canonical_size())
            .saturating_add(self.candidate_kind.canonical_size())
            .saturating_add(self.candidate_commit.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.merge_method.canonical_size())
            .saturating_add(self.provider_generation.canonical_size())
            .saturating_add(self.observed_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.provider.canonical_nesting())
                .max(self.pr_number.canonical_nesting())
                .max(self.head_commit.canonical_nesting())
                .max(self.head_tree.canonical_nesting())
                .max(self.base_ref.canonical_nesting())
                .max(self.base_commit.canonical_nesting())
                .max(self.base_tree.canonical_nesting())
                .max(self.candidate_kind.canonical_nesting())
                .max(self.candidate_commit.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.merge_method.canonical_nesting())
                .max(self.provider_generation.canonical_nesting())
                .max(self.observed_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelContentBlockV1Kind {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "TOOL_USE")]
    ToolUse,
    #[serde(rename = "TOOL_RESULT")]
    ToolResult,
}
impl ModelContentBlockV1Kind {
    pub fn code(&self) -> u64 {
        match self {
            Self::Text => 0,
            Self::ToolUse => 1,
            Self::ToolResult => 2,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Text),
            1 => Some(Self::ToolUse),
            2 => Some(Self::ToolResult),
            _ => None,
        }
    }
}
impl CanonicalOrder for ModelContentBlockV1Kind {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelContentBlockV1Type {
    #[serde(rename = "ModelContentBlockV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelContentBlockV1Fields>"
)]
pub struct ModelContentBlockV1 {
    pub r#type: ModelContentBlockV1Type,
    pub wire_revision: WireRevision,
    pub kind: ModelContentBlockV1Kind,
    pub tool_call_id: Opt<Identifier>,
    pub tool_id: Opt<Identifier>,
    pub content_digest: Opt<Hex<32>>,
    pub arguments_digest: Opt<Hex<32>>,
    pub is_error: Opt<bool>,
    pub truncated: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelContentBlockV1Fields {
    r#type: ModelContentBlockV1Type,
    wire_revision: WireRevision,
    kind: ModelContentBlockV1Kind,
    tool_call_id: Opt<Identifier>,
    tool_id: Opt<Identifier>,
    content_digest: Opt<Hex<32>>,
    arguments_digest: Opt<Hex<32>>,
    is_error: Opt<bool>,
    truncated: bool,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelContentBlockV1Fields>> for ModelContentBlockV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelContentBlockV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            kind: fields.kind,
            tool_call_id: fields.tool_call_id,
            tool_id: fields.tool_id,
            content_digest: fields.content_digest,
            arguments_digest: fields.arguments_digest,
            is_error: fields.is_error,
            truncated: fields.truncated,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelContentBlockV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 9)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("tool_call_id", &self.tool_call_id)?;
        state.serialize_field("tool_id", &self.tool_id)?;
        state.serialize_field("content_digest", &self.content_digest)?;
        state.serialize_field("arguments_digest", &self.arguments_digest)?;
        state.serialize_field("is_error", &self.is_error)?;
        state.serialize_field("truncated", &self.truncated)?;
        state.end()
    }
}
impl ModelContentBlockV1 {
    pub const TYPE_NAME: &'static str = "ModelContentBlockV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "kind",
        "tool_call_id",
        "tool_id",
        "content_digest",
        "arguments_digest",
        "is_error",
        "truncated",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelContentBlockV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.kind.canonical_cmp(&other.kind))
            .then_with(|| self.tool_call_id.canonical_cmp(&other.tool_call_id))
            .then_with(|| self.tool_id.canonical_cmp(&other.tool_id))
            .then_with(|| self.content_digest.canonical_cmp(&other.content_digest))
            .then_with(|| self.arguments_digest.canonical_cmp(&other.arguments_digest))
            .then_with(|| self.is_error.canonical_cmp(&other.is_error))
            .then_with(|| self.truncated.canonical_cmp(&other.truncated))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(9u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.kind.canonical_size())
            .saturating_add(self.tool_call_id.canonical_size())
            .saturating_add(self.tool_id.canonical_size())
            .saturating_add(self.content_digest.canonical_size())
            .saturating_add(self.arguments_digest.canonical_size())
            .saturating_add(self.is_error.canonical_size())
            .saturating_add(self.truncated.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.kind
                .canonical_nesting()
                .max(self.tool_call_id.canonical_nesting())
                .max(self.tool_id.canonical_nesting())
                .max(self.content_digest.canonical_nesting())
                .max(self.arguments_digest.canonical_nesting())
                .max(self.is_error.canonical_nesting())
                .max(self.truncated.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelContentV1Type {
    #[serde(rename = "ModelContentV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelContentV1Fields>"
)]
pub struct ModelContentV1 {
    pub r#type: ModelContentV1Type,
    pub wire_revision: WireRevision,
    pub blocks: Vec<ModelContentBlockV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelContentV1Fields {
    r#type: ModelContentV1Type,
    wire_revision: WireRevision,
    #[serde(deserialize_with = "crate::primitives::bounded_array")]
    blocks: Vec<ModelContentBlockV1>,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelContentV1Fields>> for ModelContentV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelContentV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            blocks: fields.blocks,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelContentV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 3)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("blocks", &crate::primitives::BoundedArrayRef(&self.blocks))?;
        state.end()
    }
}
impl ModelContentV1 {
    pub const TYPE_NAME: &'static str = "ModelContentV1";
    pub const FIELD_ORDER: &'static [&'static str] = &["type", "wire_revision", "blocks"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelContentV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal.then_with(|| self.blocks.canonical_cmp(&other.blocks))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(3u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.blocks.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(self.blocks.canonical_nesting())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelIntentV2Type {
    #[serde(rename = "ModelIntentV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelIntentV2Fields>"
)]
pub struct ModelIntentV2 {
    pub r#type: ModelIntentV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub intent_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub root_envelope_digest: Hex<32>,
    pub turn_id: Uuid,
    pub model_cell_digest: Hex<32>,
    pub maximum_input_tokens: U64Str,
    pub maximum_output_tokens: U64Str,
    pub maximum_spend_microunits: U64Str,
    pub request_digest: Hex<32>,
    pub broker_generation: U64Str,
    pub deadline: I64Str,
    pub nonce: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelIntentV2Fields {
    r#type: ModelIntentV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    intent_id: Uuid,
    attempt_binding_digest: Hex<32>,
    root_envelope_digest: Hex<32>,
    turn_id: Uuid,
    model_cell_digest: Hex<32>,
    maximum_input_tokens: U64Str,
    maximum_output_tokens: U64Str,
    maximum_spend_microunits: U64Str,
    request_digest: Hex<32>,
    broker_generation: U64Str,
    deadline: I64Str,
    nonce: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelIntentV2Fields>> for ModelIntentV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelIntentV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            intent_id: fields.intent_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            root_envelope_digest: fields.root_envelope_digest,
            turn_id: fields.turn_id,
            model_cell_digest: fields.model_cell_digest,
            maximum_input_tokens: fields.maximum_input_tokens,
            maximum_output_tokens: fields.maximum_output_tokens,
            maximum_spend_microunits: fields.maximum_spend_microunits,
            request_digest: fields.request_digest,
            broker_generation: fields.broker_generation,
            deadline: fields.deadline,
            nonce: fields.nonce,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelIntentV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 15)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("intent_id", &self.intent_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("root_envelope_digest", &self.root_envelope_digest)?;
        state.serialize_field("turn_id", &self.turn_id)?;
        state.serialize_field("model_cell_digest", &self.model_cell_digest)?;
        state.serialize_field("maximum_input_tokens", &self.maximum_input_tokens)?;
        state.serialize_field("maximum_output_tokens", &self.maximum_output_tokens)?;
        state.serialize_field("maximum_spend_microunits", &self.maximum_spend_microunits)?;
        state.serialize_field("request_digest", &self.request_digest)?;
        state.serialize_field("broker_generation", &self.broker_generation)?;
        state.serialize_field("deadline", &self.deadline)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.end()
    }
}
impl ModelIntentV2 {
    pub const TYPE_NAME: &'static str = "ModelIntentV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "intent_id",
        "attempt_binding_digest",
        "root_envelope_digest",
        "turn_id",
        "model_cell_digest",
        "maximum_input_tokens",
        "maximum_output_tokens",
        "maximum_spend_microunits",
        "request_digest",
        "broker_generation",
        "deadline",
        "nonce",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelIntentV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.intent_id.canonical_cmp(&other.intent_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| {
                self.root_envelope_digest
                    .canonical_cmp(&other.root_envelope_digest)
            })
            .then_with(|| self.turn_id.canonical_cmp(&other.turn_id))
            .then_with(|| {
                self.model_cell_digest
                    .canonical_cmp(&other.model_cell_digest)
            })
            .then_with(|| {
                self.maximum_input_tokens
                    .canonical_cmp(&other.maximum_input_tokens)
            })
            .then_with(|| {
                self.maximum_output_tokens
                    .canonical_cmp(&other.maximum_output_tokens)
            })
            .then_with(|| {
                self.maximum_spend_microunits
                    .canonical_cmp(&other.maximum_spend_microunits)
            })
            .then_with(|| self.request_digest.canonical_cmp(&other.request_digest))
            .then_with(|| {
                self.broker_generation
                    .canonical_cmp(&other.broker_generation)
            })
            .then_with(|| self.deadline.canonical_cmp(&other.deadline))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(15u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.intent_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.root_envelope_digest.canonical_size())
            .saturating_add(self.turn_id.canonical_size())
            .saturating_add(self.model_cell_digest.canonical_size())
            .saturating_add(self.maximum_input_tokens.canonical_size())
            .saturating_add(self.maximum_output_tokens.canonical_size())
            .saturating_add(self.maximum_spend_microunits.canonical_size())
            .saturating_add(self.request_digest.canonical_size())
            .saturating_add(self.broker_generation.canonical_size())
            .saturating_add(self.deadline.canonical_size())
            .saturating_add(self.nonce.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.intent_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.root_envelope_digest.canonical_nesting())
                .max(self.turn_id.canonical_nesting())
                .max(self.model_cell_digest.canonical_nesting())
                .max(self.maximum_input_tokens.canonical_nesting())
                .max(self.maximum_output_tokens.canonical_nesting())
                .max(self.maximum_spend_microunits.canonical_nesting())
                .max(self.request_digest.canonical_nesting())
                .max(self.broker_generation.canonical_nesting())
                .max(self.deadline.canonical_nesting())
                .max(self.nonce.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelMessageV1Role {
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "ASSISTANT")]
    Assistant,
    #[serde(rename = "TOOL")]
    Tool,
}
impl ModelMessageV1Role {
    pub fn code(&self) -> u64 {
        match self {
            Self::System => 0,
            Self::User => 1,
            Self::Assistant => 2,
            Self::Tool => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::System),
            1 => Some(Self::User),
            2 => Some(Self::Assistant),
            3 => Some(Self::Tool),
            _ => None,
        }
    }
}
impl CanonicalOrder for ModelMessageV1Role {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelMessageV1Type {
    #[serde(rename = "ModelMessageV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelMessageV1Fields>"
)]
pub struct ModelMessageV1 {
    pub r#type: ModelMessageV1Type,
    pub wire_revision: WireRevision,
    pub role: ModelMessageV1Role,
    pub content_custody_digest: Hex<32>,
    pub tool_call_id: Opt<Identifier>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelMessageV1Fields {
    r#type: ModelMessageV1Type,
    wire_revision: WireRevision,
    role: ModelMessageV1Role,
    content_custody_digest: Hex<32>,
    tool_call_id: Opt<Identifier>,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelMessageV1Fields>> for ModelMessageV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelMessageV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            role: fields.role,
            content_custody_digest: fields.content_custody_digest,
            tool_call_id: fields.tool_call_id,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelMessageV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 5)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("role", &self.role)?;
        state.serialize_field("content_custody_digest", &self.content_custody_digest)?;
        state.serialize_field("tool_call_id", &self.tool_call_id)?;
        state.end()
    }
}
impl ModelMessageV1 {
    pub const TYPE_NAME: &'static str = "ModelMessageV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "role",
        "content_custody_digest",
        "tool_call_id",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelMessageV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.role.canonical_cmp(&other.role))
            .then_with(|| {
                self.content_custody_digest
                    .canonical_cmp(&other.content_custody_digest)
            })
            .then_with(|| self.tool_call_id.canonical_cmp(&other.tool_call_id))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(5u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.role.canonical_size())
            .saturating_add(self.content_custody_digest.canonical_size())
            .saturating_add(self.tool_call_id.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.role
                .canonical_nesting()
                .max(self.content_custody_digest.canonical_nesting())
                .max(self.tool_call_id.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelToolV1Type {
    #[serde(rename = "ModelToolV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelToolV1Fields>"
)]
pub struct ModelToolV1 {
    pub r#type: ModelToolV1Type,
    pub wire_revision: WireRevision,
    pub tool_id: Identifier,
    pub input_schema_digest: Hex<32>,
    pub capability_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelToolV1Fields {
    r#type: ModelToolV1Type,
    wire_revision: WireRevision,
    tool_id: Identifier,
    input_schema_digest: Hex<32>,
    capability_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelToolV1Fields>> for ModelToolV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelToolV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tool_id: fields.tool_id,
            input_schema_digest: fields.input_schema_digest,
            capability_digest: fields.capability_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelToolV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 5)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tool_id", &self.tool_id)?;
        state.serialize_field("input_schema_digest", &self.input_schema_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.end()
    }
}
impl ModelToolV1 {
    pub const TYPE_NAME: &'static str = "ModelToolV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tool_id",
        "input_schema_digest",
        "capability_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelToolV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tool_id.canonical_cmp(&other.tool_id))
            .then_with(|| {
                self.input_schema_digest
                    .canonical_cmp(&other.input_schema_digest)
            })
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(5u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tool_id.canonical_size())
            .saturating_add(self.input_schema_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tool_id
                .canonical_nesting()
                .max(self.input_schema_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.tool_id.0.as_bytes().to_vec())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelTurnV2Type {
    #[serde(rename = "ModelTurnV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ModelTurnV2Fields>"
)]
pub struct ModelTurnV2 {
    pub r#type: ModelTurnV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub turn_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub model_intent_id: Uuid,
    pub model_cell_digest: Hex<32>,
    pub messages: Vec<ModelMessageV1>,
    pub tools: SetOf<ModelToolV1>,
    pub maximum_output_tokens: U64Str,
    pub sampling_parameters_digest: Hex<32>,
    pub deadline: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelTurnV2Fields {
    r#type: ModelTurnV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    turn_id: Uuid,
    attempt_binding_digest: Hex<32>,
    model_intent_id: Uuid,
    model_cell_digest: Hex<32>,
    #[serde(deserialize_with = "crate::primitives::bounded_array")]
    messages: Vec<ModelMessageV1>,
    tools: SetOf<ModelToolV1>,
    maximum_output_tokens: U64Str,
    sampling_parameters_digest: Hex<32>,
    deadline: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ModelTurnV2Fields>> for ModelTurnV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ModelTurnV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            turn_id: fields.turn_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            model_intent_id: fields.model_intent_id,
            model_cell_digest: fields.model_cell_digest,
            messages: fields.messages,
            tools: fields.tools,
            maximum_output_tokens: fields.maximum_output_tokens,
            sampling_parameters_digest: fields.sampling_parameters_digest,
            deadline: fields.deadline,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ModelTurnV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("turn_id", &self.turn_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("model_intent_id", &self.model_intent_id)?;
        state.serialize_field("model_cell_digest", &self.model_cell_digest)?;
        state.serialize_field(
            "messages",
            &crate::primitives::BoundedArrayRef(&self.messages),
        )?;
        state.serialize_field("tools", &self.tools)?;
        state.serialize_field("maximum_output_tokens", &self.maximum_output_tokens)?;
        state.serialize_field(
            "sampling_parameters_digest",
            &self.sampling_parameters_digest,
        )?;
        state.serialize_field("deadline", &self.deadline)?;
        state.end()
    }
}
impl ModelTurnV2 {
    pub const TYPE_NAME: &'static str = "ModelTurnV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "turn_id",
        "attempt_binding_digest",
        "model_intent_id",
        "model_cell_digest",
        "messages",
        "tools",
        "maximum_output_tokens",
        "sampling_parameters_digest",
        "deadline",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ModelTurnV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.turn_id.canonical_cmp(&other.turn_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.model_intent_id.canonical_cmp(&other.model_intent_id))
            .then_with(|| {
                self.model_cell_digest
                    .canonical_cmp(&other.model_cell_digest)
            })
            .then_with(|| self.messages.canonical_cmp(&other.messages))
            .then_with(|| self.tools.canonical_cmp(&other.tools))
            .then_with(|| {
                self.maximum_output_tokens
                    .canonical_cmp(&other.maximum_output_tokens)
            })
            .then_with(|| {
                self.sampling_parameters_digest
                    .canonical_cmp(&other.sampling_parameters_digest)
            })
            .then_with(|| self.deadline.canonical_cmp(&other.deadline))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.turn_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.model_intent_id.canonical_size())
            .saturating_add(self.model_cell_digest.canonical_size())
            .saturating_add(self.messages.canonical_size())
            .saturating_add(self.tools.canonical_size())
            .saturating_add(self.maximum_output_tokens.canonical_size())
            .saturating_add(self.sampling_parameters_digest.canonical_size())
            .saturating_add(self.deadline.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.turn_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.model_intent_id.canonical_nesting())
                .max(self.model_cell_digest.canonical_nesting())
                .max(self.messages.canonical_nesting())
                .max(self.tools.canonical_nesting())
                .max(self.maximum_output_tokens.canonical_nesting())
                .max(self.sampling_parameters_digest.canonical_nesting())
                .max(self.deadline.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleTrustClassV1Origin {
    #[serde(rename = "TRUSTED_REPOSITORY_TEST")]
    TrustedRepositoryTest,
    #[serde(rename = "TRUSTED_SPECIFICATION")]
    TrustedSpecification,
    #[serde(rename = "SECURITY_INVARIANT")]
    SecurityInvariant,
    #[serde(rename = "STANDARDS_REQUIREMENT")]
    StandardsRequirement,
    #[serde(rename = "MAINTAINER_APPROVED")]
    MaintainerApproved,
    #[serde(rename = "DETERMINISTIC_EXPLOIT")]
    DeterministicExploit,
    #[serde(rename = "UNTRUSTED_AGENT_EXPECTATION")]
    UntrustedAgentExpectation,
}
impl OracleTrustClassV1Origin {
    pub fn code(&self) -> u64 {
        match self {
            Self::TrustedRepositoryTest => 0,
            Self::TrustedSpecification => 1,
            Self::SecurityInvariant => 2,
            Self::StandardsRequirement => 3,
            Self::MaintainerApproved => 4,
            Self::DeterministicExploit => 5,
            Self::UntrustedAgentExpectation => 6,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::TrustedRepositoryTest),
            1 => Some(Self::TrustedSpecification),
            2 => Some(Self::SecurityInvariant),
            3 => Some(Self::StandardsRequirement),
            4 => Some(Self::MaintainerApproved),
            5 => Some(Self::DeterministicExploit),
            6 => Some(Self::UntrustedAgentExpectation),
            _ => None,
        }
    }
}
impl CanonicalOrder for OracleTrustClassV1Origin {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleTrustClassV1Type {
    #[serde(rename = "OracleTrustClassV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<OracleTrustClassV1Fields>"
)]
pub struct OracleTrustClassV1 {
    pub r#type: OracleTrustClassV1Type,
    pub wire_revision: WireRevision,
    pub origin: OracleTrustClassV1Origin,
    pub source_digest: Hex<32>,
    pub applicability_digest: Hex<32>,
    pub approval_authority_id: Opt<Uuid>,
    pub approval_record_digest: Opt<Hex<32>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleTrustClassV1Fields {
    r#type: OracleTrustClassV1Type,
    wire_revision: WireRevision,
    origin: OracleTrustClassV1Origin,
    source_digest: Hex<32>,
    applicability_digest: Hex<32>,
    approval_authority_id: Opt<Uuid>,
    approval_record_digest: Opt<Hex<32>>,
}
impl TryFrom<crate::primitives::ObjectOnly<OracleTrustClassV1Fields>> for OracleTrustClassV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<OracleTrustClassV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            origin: fields.origin,
            source_digest: fields.source_digest,
            applicability_digest: fields.applicability_digest,
            approval_authority_id: fields.approval_authority_id,
            approval_record_digest: fields.approval_record_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for OracleTrustClassV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("origin", &self.origin)?;
        state.serialize_field("source_digest", &self.source_digest)?;
        state.serialize_field("applicability_digest", &self.applicability_digest)?;
        state.serialize_field("approval_authority_id", &self.approval_authority_id)?;
        state.serialize_field("approval_record_digest", &self.approval_record_digest)?;
        state.end()
    }
}
impl OracleTrustClassV1 {
    pub const TYPE_NAME: &'static str = "OracleTrustClassV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "origin",
        "source_digest",
        "applicability_digest",
        "approval_authority_id",
        "approval_record_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for OracleTrustClassV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.origin.canonical_cmp(&other.origin))
            .then_with(|| self.source_digest.canonical_cmp(&other.source_digest))
            .then_with(|| {
                self.applicability_digest
                    .canonical_cmp(&other.applicability_digest)
            })
            .then_with(|| {
                self.approval_authority_id
                    .canonical_cmp(&other.approval_authority_id)
            })
            .then_with(|| {
                self.approval_record_digest
                    .canonical_cmp(&other.approval_record_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.origin.canonical_size())
            .saturating_add(self.source_digest.canonical_size())
            .saturating_add(self.applicability_digest.canonical_size())
            .saturating_add(self.approval_authority_id.canonical_size())
            .saturating_add(self.approval_record_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.origin
                .canonical_nesting()
                .max(self.source_digest.canonical_nesting())
                .max(self.applicability_digest.canonical_nesting())
                .max(self.approval_authority_id.canonical_nesting())
                .max(self.approval_record_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchCandidateV2Type {
    #[serde(rename = "PatchCandidateV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<PatchCandidateV2Fields>"
)]
pub struct PatchCandidateV2 {
    pub r#type: PatchCandidateV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub patch_revision_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub base_commit: GitOidV1,
    pub base_tree: GitOidV1,
    pub patch_digest: Hex<32>,
    pub manifest_digest: Hex<32>,
    pub producer_id: Uuid,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PatchCandidateV2Fields {
    r#type: PatchCandidateV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    patch_revision_id: Uuid,
    attempt_binding_digest: Hex<32>,
    base_commit: GitOidV1,
    base_tree: GitOidV1,
    patch_digest: Hex<32>,
    manifest_digest: Hex<32>,
    producer_id: Uuid,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<PatchCandidateV2Fields>> for PatchCandidateV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<PatchCandidateV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            patch_revision_id: fields.patch_revision_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            base_commit: fields.base_commit,
            base_tree: fields.base_tree,
            patch_digest: fields.patch_digest,
            manifest_digest: fields.manifest_digest,
            producer_id: fields.producer_id,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for PatchCandidateV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("patch_revision_id", &self.patch_revision_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("base_commit", &self.base_commit)?;
        state.serialize_field("base_tree", &self.base_tree)?;
        state.serialize_field("patch_digest", &self.patch_digest)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl PatchCandidateV2 {
    pub const TYPE_NAME: &'static str = "PatchCandidateV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "patch_revision_id",
        "attempt_binding_digest",
        "base_commit",
        "base_tree",
        "patch_digest",
        "manifest_digest",
        "producer_id",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for PatchCandidateV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| {
                self.patch_revision_id
                    .canonical_cmp(&other.patch_revision_id)
            })
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.base_commit.canonical_cmp(&other.base_commit))
            .then_with(|| self.base_tree.canonical_cmp(&other.base_tree))
            .then_with(|| self.patch_digest.canonical_cmp(&other.patch_digest))
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.patch_revision_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.base_commit.canonical_size())
            .saturating_add(self.base_tree.canonical_size())
            .saturating_add(self.patch_digest.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.patch_revision_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.base_commit.canonical_nesting())
                .max(self.base_tree.canonical_nesting())
                .max(self.patch_digest.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchCustodyV1State {
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "LOST")]
    Lost,
    #[serde(rename = "DELETION_PENDING")]
    DeletionPending,
    #[serde(rename = "DELETED")]
    Deleted,
}
impl PatchCustodyV1State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Available => 0,
            Self::Lost => 1,
            Self::DeletionPending => 2,
            Self::Deleted => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Available),
            1 => Some(Self::Lost),
            2 => Some(Self::DeletionPending),
            3 => Some(Self::Deleted),
            _ => None,
        }
    }
}
impl CanonicalOrder for PatchCustodyV1State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchCustodyV1Type {
    #[serde(rename = "PatchCustodyV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<PatchCustodyV1Fields>"
)]
pub struct PatchCustodyV1 {
    pub r#type: PatchCustodyV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub custody_id: Uuid,
    pub patch_revision_id: Uuid,
    pub patch_digest: Hex<32>,
    pub manifest_digest: Hex<32>,
    pub candidate_tree: GitOidV1,
    pub plan_digest: Hex<32>,
    pub verification_set_digest: Hex<32>,
    pub artifacts: SetOf<ArtifactCustodyEntryV1>,
    pub access_policy_digest: Hex<32>,
    pub retention_policy_digest: Hex<32>,
    pub retention_deadline: I64Str,
    pub legal_hold: bool,
    pub state: PatchCustodyV1State,
    pub pinned_publication_ids: SetOf<Uuid>,
    pub generation: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PatchCustodyV1Fields {
    r#type: PatchCustodyV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    custody_id: Uuid,
    patch_revision_id: Uuid,
    patch_digest: Hex<32>,
    manifest_digest: Hex<32>,
    candidate_tree: GitOidV1,
    plan_digest: Hex<32>,
    verification_set_digest: Hex<32>,
    artifacts: SetOf<ArtifactCustodyEntryV1>,
    access_policy_digest: Hex<32>,
    retention_policy_digest: Hex<32>,
    retention_deadline: I64Str,
    legal_hold: bool,
    state: PatchCustodyV1State,
    pinned_publication_ids: SetOf<Uuid>,
    generation: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<PatchCustodyV1Fields>> for PatchCustodyV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<PatchCustodyV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            custody_id: fields.custody_id,
            patch_revision_id: fields.patch_revision_id,
            patch_digest: fields.patch_digest,
            manifest_digest: fields.manifest_digest,
            candidate_tree: fields.candidate_tree,
            plan_digest: fields.plan_digest,
            verification_set_digest: fields.verification_set_digest,
            artifacts: fields.artifacts,
            access_policy_digest: fields.access_policy_digest,
            retention_policy_digest: fields.retention_policy_digest,
            retention_deadline: fields.retention_deadline,
            legal_hold: fields.legal_hold,
            state: fields.state,
            pinned_publication_ids: fields.pinned_publication_ids,
            generation: fields.generation,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for PatchCustodyV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 18)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("custody_id", &self.custody_id)?;
        state.serialize_field("patch_revision_id", &self.patch_revision_id)?;
        state.serialize_field("patch_digest", &self.patch_digest)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("verification_set_digest", &self.verification_set_digest)?;
        state.serialize_field("artifacts", &self.artifacts)?;
        state.serialize_field("access_policy_digest", &self.access_policy_digest)?;
        state.serialize_field("retention_policy_digest", &self.retention_policy_digest)?;
        state.serialize_field("retention_deadline", &self.retention_deadline)?;
        state.serialize_field("legal_hold", &self.legal_hold)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("pinned_publication_ids", &self.pinned_publication_ids)?;
        state.serialize_field("generation", &self.generation)?;
        state.end()
    }
}
impl PatchCustodyV1 {
    pub const TYPE_NAME: &'static str = "PatchCustodyV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "custody_id",
        "patch_revision_id",
        "patch_digest",
        "manifest_digest",
        "candidate_tree",
        "plan_digest",
        "verification_set_digest",
        "artifacts",
        "access_policy_digest",
        "retention_policy_digest",
        "retention_deadline",
        "legal_hold",
        "state",
        "pinned_publication_ids",
        "generation",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for PatchCustodyV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.custody_id.canonical_cmp(&other.custody_id))
            .then_with(|| {
                self.patch_revision_id
                    .canonical_cmp(&other.patch_revision_id)
            })
            .then_with(|| self.patch_digest.canonical_cmp(&other.patch_digest))
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| {
                self.verification_set_digest
                    .canonical_cmp(&other.verification_set_digest)
            })
            .then_with(|| self.artifacts.canonical_cmp(&other.artifacts))
            .then_with(|| {
                self.access_policy_digest
                    .canonical_cmp(&other.access_policy_digest)
            })
            .then_with(|| {
                self.retention_policy_digest
                    .canonical_cmp(&other.retention_policy_digest)
            })
            .then_with(|| {
                self.retention_deadline
                    .canonical_cmp(&other.retention_deadline)
            })
            .then_with(|| self.legal_hold.canonical_cmp(&other.legal_hold))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| {
                self.pinned_publication_ids
                    .canonical_cmp(&other.pinned_publication_ids)
            })
            .then_with(|| self.generation.canonical_cmp(&other.generation))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(18u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.custody_id.canonical_size())
            .saturating_add(self.patch_revision_id.canonical_size())
            .saturating_add(self.patch_digest.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.verification_set_digest.canonical_size())
            .saturating_add(self.artifacts.canonical_size())
            .saturating_add(self.access_policy_digest.canonical_size())
            .saturating_add(self.retention_policy_digest.canonical_size())
            .saturating_add(self.retention_deadline.canonical_size())
            .saturating_add(self.legal_hold.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.pinned_publication_ids.canonical_size())
            .saturating_add(self.generation.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.custody_id.canonical_nesting())
                .max(self.patch_revision_id.canonical_nesting())
                .max(self.patch_digest.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.verification_set_digest.canonical_nesting())
                .max(self.artifacts.canonical_nesting())
                .max(self.access_policy_digest.canonical_nesting())
                .max(self.retention_policy_digest.canonical_nesting())
                .max(self.retention_deadline.canonical_nesting())
                .max(self.legal_hold.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.pinned_publication_ids.canonical_nesting())
                .max(self.generation.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyVersionV1Type {
    #[serde(rename = "PolicyVersionV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<PolicyVersionV1Fields>"
)]
pub struct PolicyVersionV1 {
    pub r#type: PolicyVersionV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub policy_id: Uuid,
    pub generation: U64Str,
    pub previous_digest: Opt<Hex<32>>,
    pub requirements_digest: Hex<32>,
    pub analyzer_configuration_schema_digest: Hex<32>,
    pub trust_set_digest: Hex<32>,
    pub capability_digest: Hex<32>,
    pub approver_scope_digest: Hex<32>,
    pub created_at: I64Str,
    pub activated_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyVersionV1Fields {
    r#type: PolicyVersionV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    policy_id: Uuid,
    generation: U64Str,
    previous_digest: Opt<Hex<32>>,
    requirements_digest: Hex<32>,
    analyzer_configuration_schema_digest: Hex<32>,
    trust_set_digest: Hex<32>,
    capability_digest: Hex<32>,
    approver_scope_digest: Hex<32>,
    created_at: I64Str,
    activated_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<PolicyVersionV1Fields>> for PolicyVersionV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<PolicyVersionV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            policy_id: fields.policy_id,
            generation: fields.generation,
            previous_digest: fields.previous_digest,
            requirements_digest: fields.requirements_digest,
            analyzer_configuration_schema_digest: fields.analyzer_configuration_schema_digest,
            trust_set_digest: fields.trust_set_digest,
            capability_digest: fields.capability_digest,
            approver_scope_digest: fields.approver_scope_digest,
            created_at: fields.created_at,
            activated_at: fields.activated_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for PolicyVersionV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("policy_id", &self.policy_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("previous_digest", &self.previous_digest)?;
        state.serialize_field("requirements_digest", &self.requirements_digest)?;
        state.serialize_field(
            "analyzer_configuration_schema_digest",
            &self.analyzer_configuration_schema_digest,
        )?;
        state.serialize_field("trust_set_digest", &self.trust_set_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.serialize_field("approver_scope_digest", &self.approver_scope_digest)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("activated_at", &self.activated_at)?;
        state.end()
    }
}
impl PolicyVersionV1 {
    pub const TYPE_NAME: &'static str = "PolicyVersionV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "policy_id",
        "generation",
        "previous_digest",
        "requirements_digest",
        "analyzer_configuration_schema_digest",
        "trust_set_digest",
        "capability_digest",
        "approver_scope_digest",
        "created_at",
        "activated_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for PolicyVersionV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.policy_id.canonical_cmp(&other.policy_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.previous_digest.canonical_cmp(&other.previous_digest))
            .then_with(|| {
                self.requirements_digest
                    .canonical_cmp(&other.requirements_digest)
            })
            .then_with(|| {
                self.analyzer_configuration_schema_digest
                    .canonical_cmp(&other.analyzer_configuration_schema_digest)
            })
            .then_with(|| self.trust_set_digest.canonical_cmp(&other.trust_set_digest))
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
            .then_with(|| {
                self.approver_scope_digest
                    .canonical_cmp(&other.approver_scope_digest)
            })
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.activated_at.canonical_cmp(&other.activated_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.policy_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.previous_digest.canonical_size())
            .saturating_add(self.requirements_digest.canonical_size())
            .saturating_add(self.analyzer_configuration_schema_digest.canonical_size())
            .saturating_add(self.trust_set_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
            .saturating_add(self.approver_scope_digest.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.activated_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.policy_id.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.previous_digest.canonical_nesting())
                .max(self.requirements_digest.canonical_nesting())
                .max(
                    self.analyzer_configuration_schema_digest
                        .canonical_nesting(),
                )
                .max(self.trust_set_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting())
                .max(self.approver_scope_digest.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.activated_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProducerEventV2Type {
    #[serde(rename = "ProducerEventV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ProducerEventV2Fields>"
)]
pub struct ProducerEventV2 {
    pub r#type: ProducerEventV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub event_id: Uuid,
    pub producer_id: Uuid,
    pub producer_session_id: Uuid,
    pub producer_generation: U64Str,
    pub sequence: U64Str,
    pub occurred_at: I64Str,
    pub received_at: I64Str,
    pub payload_type: Identifier,
    pub payload_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProducerEventV2Fields {
    r#type: ProducerEventV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    event_id: Uuid,
    producer_id: Uuid,
    producer_session_id: Uuid,
    producer_generation: U64Str,
    sequence: U64Str,
    occurred_at: I64Str,
    received_at: I64Str,
    payload_type: Identifier,
    payload_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<ProducerEventV2Fields>> for ProducerEventV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ProducerEventV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            event_id: fields.event_id,
            producer_id: fields.producer_id,
            producer_session_id: fields.producer_session_id,
            producer_generation: fields.producer_generation,
            sequence: fields.sequence,
            occurred_at: fields.occurred_at,
            received_at: fields.received_at,
            payload_type: fields.payload_type,
            payload_digest: fields.payload_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ProducerEventV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.serialize_field("producer_id", &self.producer_id)?;
        state.serialize_field("producer_session_id", &self.producer_session_id)?;
        state.serialize_field("producer_generation", &self.producer_generation)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("occurred_at", &self.occurred_at)?;
        state.serialize_field("received_at", &self.received_at)?;
        state.serialize_field("payload_type", &self.payload_type)?;
        state.serialize_field("payload_digest", &self.payload_digest)?;
        state.end()
    }
}
impl ProducerEventV2 {
    pub const TYPE_NAME: &'static str = "ProducerEventV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "event_id",
        "producer_id",
        "producer_session_id",
        "producer_generation",
        "sequence",
        "occurred_at",
        "received_at",
        "payload_type",
        "payload_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ProducerEventV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.event_id.canonical_cmp(&other.event_id))
            .then_with(|| self.producer_id.canonical_cmp(&other.producer_id))
            .then_with(|| {
                self.producer_session_id
                    .canonical_cmp(&other.producer_session_id)
            })
            .then_with(|| {
                self.producer_generation
                    .canonical_cmp(&other.producer_generation)
            })
            .then_with(|| self.sequence.canonical_cmp(&other.sequence))
            .then_with(|| self.occurred_at.canonical_cmp(&other.occurred_at))
            .then_with(|| self.received_at.canonical_cmp(&other.received_at))
            .then_with(|| self.payload_type.canonical_cmp(&other.payload_type))
            .then_with(|| self.payload_digest.canonical_cmp(&other.payload_digest))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.event_id.canonical_size())
            .saturating_add(self.producer_id.canonical_size())
            .saturating_add(self.producer_session_id.canonical_size())
            .saturating_add(self.producer_generation.canonical_size())
            .saturating_add(self.sequence.canonical_size())
            .saturating_add(self.occurred_at.canonical_size())
            .saturating_add(self.received_at.canonical_size())
            .saturating_add(self.payload_type.canonical_size())
            .saturating_add(self.payload_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.event_id.canonical_nesting())
                .max(self.producer_id.canonical_nesting())
                .max(self.producer_session_id.canonical_nesting())
                .max(self.producer_generation.canonical_nesting())
                .max(self.sequence.canonical_nesting())
                .max(self.occurred_at.canonical_nesting())
                .max(self.received_at.canonical_nesting())
                .max(self.payload_type.canonical_nesting())
                .max(self.payload_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderEventV2Type {
    #[serde(rename = "ProviderEventV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ProviderEventV2Fields>"
)]
pub struct ProviderEventV2 {
    pub r#type: ProviderEventV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub event_id: Uuid,
    pub provider: Provider,
    pub installation_id: U64Str,
    pub delivery_id: Identifier,
    pub delivery_digest: Hex<32>,
    pub received_at: I64Str,
    pub normalized_readback_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderEventV2Fields {
    r#type: ProviderEventV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    event_id: Uuid,
    provider: Provider,
    installation_id: U64Str,
    delivery_id: Identifier,
    delivery_digest: Hex<32>,
    received_at: I64Str,
    normalized_readback_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<ProviderEventV2Fields>> for ProviderEventV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ProviderEventV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            event_id: fields.event_id,
            provider: fields.provider,
            installation_id: fields.installation_id,
            delivery_id: fields.delivery_id,
            delivery_digest: fields.delivery_digest,
            received_at: fields.received_at,
            normalized_readback_digest: fields.normalized_readback_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ProviderEventV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.serialize_field("provider", &self.provider)?;
        state.serialize_field("installation_id", &self.installation_id)?;
        state.serialize_field("delivery_id", &self.delivery_id)?;
        state.serialize_field("delivery_digest", &self.delivery_digest)?;
        state.serialize_field("received_at", &self.received_at)?;
        state.serialize_field(
            "normalized_readback_digest",
            &self.normalized_readback_digest,
        )?;
        state.end()
    }
}
impl ProviderEventV2 {
    pub const TYPE_NAME: &'static str = "ProviderEventV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "event_id",
        "provider",
        "installation_id",
        "delivery_id",
        "delivery_digest",
        "received_at",
        "normalized_readback_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ProviderEventV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.event_id.canonical_cmp(&other.event_id))
            .then_with(|| self.provider.canonical_cmp(&other.provider))
            .then_with(|| self.installation_id.canonical_cmp(&other.installation_id))
            .then_with(|| self.delivery_id.canonical_cmp(&other.delivery_id))
            .then_with(|| self.delivery_digest.canonical_cmp(&other.delivery_digest))
            .then_with(|| self.received_at.canonical_cmp(&other.received_at))
            .then_with(|| {
                self.normalized_readback_digest
                    .canonical_cmp(&other.normalized_readback_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.event_id.canonical_size())
            .saturating_add(self.provider.canonical_size())
            .saturating_add(self.installation_id.canonical_size())
            .saturating_add(self.delivery_id.canonical_size())
            .saturating_add(self.delivery_digest.canonical_size())
            .saturating_add(self.received_at.canonical_size())
            .saturating_add(self.normalized_readback_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.event_id.canonical_nesting())
                .max(self.provider.canonical_nesting())
                .max(self.installation_id.canonical_nesting())
                .max(self.delivery_id.canonical_nesting())
                .max(self.delivery_digest.canonical_nesting())
                .max(self.received_at.canonical_nesting())
                .max(self.normalized_readback_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderResultV1Type {
    #[serde(rename = "ProviderResultV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ProviderResultV1Fields>"
)]
pub struct ProviderResultV1 {
    pub r#type: ProviderResultV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub effect_id: Uuid,
    pub request_digest: Hex<32>,
    pub provider_request_id: Identifier,
    pub observed_at: I64Str,
    pub result_oid: Opt<GitOidV1>,
    pub result_numeric_id: Opt<U64Str>,
    pub result_digest: Opt<Hex<32>>,
    pub readback_evidence_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderResultV1Fields {
    r#type: ProviderResultV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    effect_id: Uuid,
    request_digest: Hex<32>,
    provider_request_id: Identifier,
    observed_at: I64Str,
    result_oid: Opt<GitOidV1>,
    result_numeric_id: Opt<U64Str>,
    result_digest: Opt<Hex<32>>,
    readback_evidence_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<ProviderResultV1Fields>> for ProviderResultV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ProviderResultV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            effect_id: fields.effect_id,
            request_digest: fields.request_digest,
            provider_request_id: fields.provider_request_id,
            observed_at: fields.observed_at,
            result_oid: fields.result_oid,
            result_numeric_id: fields.result_numeric_id,
            result_digest: fields.result_digest,
            readback_evidence_digest: fields.readback_evidence_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ProviderResultV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("effect_id", &self.effect_id)?;
        state.serialize_field("request_digest", &self.request_digest)?;
        state.serialize_field("provider_request_id", &self.provider_request_id)?;
        state.serialize_field("observed_at", &self.observed_at)?;
        state.serialize_field("result_oid", &self.result_oid)?;
        state.serialize_field("result_numeric_id", &self.result_numeric_id)?;
        state.serialize_field("result_digest", &self.result_digest)?;
        state.serialize_field("readback_evidence_digest", &self.readback_evidence_digest)?;
        state.end()
    }
}
impl ProviderResultV1 {
    pub const TYPE_NAME: &'static str = "ProviderResultV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "effect_id",
        "request_digest",
        "provider_request_id",
        "observed_at",
        "result_oid",
        "result_numeric_id",
        "result_digest",
        "readback_evidence_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ProviderResultV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.effect_id.canonical_cmp(&other.effect_id))
            .then_with(|| self.request_digest.canonical_cmp(&other.request_digest))
            .then_with(|| {
                self.provider_request_id
                    .canonical_cmp(&other.provider_request_id)
            })
            .then_with(|| self.observed_at.canonical_cmp(&other.observed_at))
            .then_with(|| self.result_oid.canonical_cmp(&other.result_oid))
            .then_with(|| {
                self.result_numeric_id
                    .canonical_cmp(&other.result_numeric_id)
            })
            .then_with(|| self.result_digest.canonical_cmp(&other.result_digest))
            .then_with(|| {
                self.readback_evidence_digest
                    .canonical_cmp(&other.readback_evidence_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.effect_id.canonical_size())
            .saturating_add(self.request_digest.canonical_size())
            .saturating_add(self.provider_request_id.canonical_size())
            .saturating_add(self.observed_at.canonical_size())
            .saturating_add(self.result_oid.canonical_size())
            .saturating_add(self.result_numeric_id.canonical_size())
            .saturating_add(self.result_digest.canonical_size())
            .saturating_add(self.readback_evidence_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.effect_id.canonical_nesting())
                .max(self.request_digest.canonical_nesting())
                .max(self.provider_request_id.canonical_nesting())
                .max(self.observed_at.canonical_nesting())
                .max(self.result_oid.canonical_nesting())
                .max(self.result_numeric_id.canonical_nesting())
                .max(self.result_digest.canonical_nesting())
                .max(self.readback_evidence_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationEffectsV1Type {
    #[serde(rename = "PublicationEffectsV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<PublicationEffectsV1Fields>"
)]
pub struct PublicationEffectsV1 {
    pub r#type: PublicationEffectsV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub steps: Vec<EffectPlanEntryV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicationEffectsV1Fields {
    r#type: PublicationEffectsV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    #[serde(deserialize_with = "crate::primitives::bounded_array")]
    steps: Vec<EffectPlanEntryV1>,
}
impl TryFrom<crate::primitives::ObjectOnly<PublicationEffectsV1Fields>> for PublicationEffectsV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<PublicationEffectsV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            steps: fields.steps,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for PublicationEffectsV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 5)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("steps", &crate::primitives::BoundedArrayRef(&self.steps))?;
        state.end()
    }
}
impl PublicationEffectsV1 {
    pub const TYPE_NAME: &'static str = "PublicationEffectsV1";
    pub const FIELD_ORDER: &'static [&'static str] =
        &["type", "wire_revision", "scope", "publication_id", "steps"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for PublicationEffectsV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.steps.canonical_cmp(&other.steps))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(5u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.steps.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.steps.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationOperationV1State {
    #[serde(rename = "PLANNED")]
    Planned,
    #[serde(rename = "AWAITING_APPROVAL")]
    AwaitingApproval,
    #[serde(rename = "AUTHORIZED")]
    Authorized,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "FAILED_TERMINAL")]
    FailedTerminal,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MANUAL_RECONCILIATION")]
    ManualReconciliation,
}
impl PublicationOperationV1State {
    pub fn code(&self) -> u64 {
        match self {
            Self::Planned => 0,
            Self::AwaitingApproval => 1,
            Self::Authorized => 2,
            Self::Running => 3,
            Self::Confirmed => 4,
            Self::FailedTerminal => 5,
            Self::Revoked => 6,
            Self::Expired => 7,
            Self::Unknown => 8,
            Self::ManualReconciliation => 9,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Planned),
            1 => Some(Self::AwaitingApproval),
            2 => Some(Self::Authorized),
            3 => Some(Self::Running),
            4 => Some(Self::Confirmed),
            5 => Some(Self::FailedTerminal),
            6 => Some(Self::Revoked),
            7 => Some(Self::Expired),
            8 => Some(Self::Unknown),
            9 => Some(Self::ManualReconciliation),
            _ => None,
        }
    }
}
impl CanonicalOrder for PublicationOperationV1State {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationOperationV1Type {
    #[serde(rename = "PublicationOperationV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<PublicationOperationV1Fields>"
)]
pub struct PublicationOperationV1 {
    pub r#type: PublicationOperationV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub plan_digest: Hex<32>,
    pub approval_digest: Hex<32>,
    pub effect_ids: Vec<Uuid>,
    pub state: PublicationOperationV1State,
    pub generation: U64Str,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicationOperationV1Fields {
    r#type: PublicationOperationV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    plan_digest: Hex<32>,
    approval_digest: Hex<32>,
    #[serde(deserialize_with = "crate::primitives::bounded_array")]
    effect_ids: Vec<Uuid>,
    state: PublicationOperationV1State,
    generation: U64Str,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<PublicationOperationV1Fields>>
    for PublicationOperationV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<PublicationOperationV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            plan_digest: fields.plan_digest,
            approval_digest: fields.approval_digest,
            effect_ids: fields.effect_ids,
            state: fields.state,
            generation: fields.generation,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for PublicationOperationV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("approval_digest", &self.approval_digest)?;
        state.serialize_field(
            "effect_ids",
            &crate::primitives::BoundedArrayRef(&self.effect_ids),
        )?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl PublicationOperationV1 {
    pub const TYPE_NAME: &'static str = "PublicationOperationV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "plan_digest",
        "approval_digest",
        "effect_ids",
        "state",
        "generation",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for PublicationOperationV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| self.approval_digest.canonical_cmp(&other.approval_digest))
            .then_with(|| self.effect_ids.canonical_cmp(&other.effect_ids))
            .then_with(|| self.state.canonical_cmp(&other.state))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.approval_digest.canonical_size())
            .saturating_add(self.effect_ids.canonical_size())
            .saturating_add(self.state.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.approval_digest.canonical_nesting())
                .max(self.effect_ids.canonical_nesting())
                .max(self.state.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionChallengeV1Type {
    #[serde(rename = "ResolutionChallengeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ResolutionChallengeV1Fields>"
)]
pub struct ResolutionChallengeV1 {
    pub r#type: ResolutionChallengeV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub challenge_id: Uuid,
    pub action_digest: Hex<32>,
    pub user_id: Uuid,
    pub credential_binding_id: Uuid,
    pub credential_generation: U64Str,
    pub session_id: Uuid,
    pub nonce: Hex<32>,
    pub issued_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolutionChallengeV1Fields {
    r#type: ResolutionChallengeV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    challenge_id: Uuid,
    action_digest: Hex<32>,
    user_id: Uuid,
    credential_binding_id: Uuid,
    credential_generation: U64Str,
    session_id: Uuid,
    nonce: Hex<32>,
    issued_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ResolutionChallengeV1Fields>> for ResolutionChallengeV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ResolutionChallengeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            challenge_id: fields.challenge_id,
            action_digest: fields.action_digest,
            user_id: fields.user_id,
            credential_binding_id: fields.credential_binding_id,
            credential_generation: fields.credential_generation,
            session_id: fields.session_id,
            nonce: fields.nonce,
            issued_at: fields.issued_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ResolutionChallengeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("challenge_id", &self.challenge_id)?;
        state.serialize_field("action_digest", &self.action_digest)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("credential_binding_id", &self.credential_binding_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl ResolutionChallengeV1 {
    pub const TYPE_NAME: &'static str = "ResolutionChallengeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "challenge_id",
        "action_digest",
        "user_id",
        "credential_binding_id",
        "credential_generation",
        "session_id",
        "nonce",
        "issued_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ResolutionChallengeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.challenge_id.canonical_cmp(&other.challenge_id))
            .then_with(|| self.action_digest.canonical_cmp(&other.action_digest))
            .then_with(|| self.user_id.canonical_cmp(&other.user_id))
            .then_with(|| {
                self.credential_binding_id
                    .canonical_cmp(&other.credential_binding_id)
            })
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.nonce.canonical_cmp(&other.nonce))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.challenge_id.canonical_size())
            .saturating_add(self.action_digest.canonical_size())
            .saturating_add(self.user_id.canonical_size())
            .saturating_add(self.credential_binding_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.nonce.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.challenge_id.canonical_nesting())
                .max(self.action_digest.canonical_nesting())
                .max(self.user_id.canonical_nesting())
                .max(self.credential_binding_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.nonce.canonical_nesting())
                .max(self.issued_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceEnvelopeV2Type {
    #[serde(rename = "ResourceEnvelopeV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ResourceEnvelopeV2Fields>"
)]
pub struct ResourceEnvelopeV2 {
    pub r#type: ResourceEnvelopeV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub root_id: Uuid,
    pub maximum_cpu_ms: U64Str,
    pub maximum_memory_bytes: U64Str,
    pub maximum_wall_ms: U64Str,
    pub maximum_model_tokens: U64Str,
    pub maximum_spend_microunits: U64Str,
    pub currency: Currency,
    pub maximum_nodes: u32,
    pub maximum_depth: u16,
    pub maximum_concurrency: u32,
    pub maximum_artifact_bytes: U64Str,
    pub expires_at: I64Str,
    pub eligibility_proof_digest: Hex<32>,
    pub maximum_model_calls: U64Str,
    pub maximum_events: U64Str,
    pub maximum_dependency_packages: U64Str,
    pub maximum_dependency_bytes: U64Str,
    pub maximum_dependency_requests: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceEnvelopeV2Fields {
    r#type: ResourceEnvelopeV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    root_id: Uuid,
    maximum_cpu_ms: U64Str,
    maximum_memory_bytes: U64Str,
    maximum_wall_ms: U64Str,
    maximum_model_tokens: U64Str,
    maximum_spend_microunits: U64Str,
    currency: Currency,
    maximum_nodes: u32,
    maximum_depth: u16,
    maximum_concurrency: u32,
    maximum_artifact_bytes: U64Str,
    expires_at: I64Str,
    eligibility_proof_digest: Hex<32>,
    maximum_model_calls: U64Str,
    maximum_events: U64Str,
    maximum_dependency_packages: U64Str,
    maximum_dependency_bytes: U64Str,
    maximum_dependency_requests: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<ResourceEnvelopeV2Fields>> for ResourceEnvelopeV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<ResourceEnvelopeV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            root_id: fields.root_id,
            maximum_cpu_ms: fields.maximum_cpu_ms,
            maximum_memory_bytes: fields.maximum_memory_bytes,
            maximum_wall_ms: fields.maximum_wall_ms,
            maximum_model_tokens: fields.maximum_model_tokens,
            maximum_spend_microunits: fields.maximum_spend_microunits,
            currency: fields.currency,
            maximum_nodes: fields.maximum_nodes,
            maximum_depth: fields.maximum_depth,
            maximum_concurrency: fields.maximum_concurrency,
            maximum_artifact_bytes: fields.maximum_artifact_bytes,
            expires_at: fields.expires_at,
            eligibility_proof_digest: fields.eligibility_proof_digest,
            maximum_model_calls: fields.maximum_model_calls,
            maximum_events: fields.maximum_events,
            maximum_dependency_packages: fields.maximum_dependency_packages,
            maximum_dependency_bytes: fields.maximum_dependency_bytes,
            maximum_dependency_requests: fields.maximum_dependency_requests,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ResourceEnvelopeV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 21)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("root_id", &self.root_id)?;
        state.serialize_field("maximum_cpu_ms", &self.maximum_cpu_ms)?;
        state.serialize_field("maximum_memory_bytes", &self.maximum_memory_bytes)?;
        state.serialize_field("maximum_wall_ms", &self.maximum_wall_ms)?;
        state.serialize_field("maximum_model_tokens", &self.maximum_model_tokens)?;
        state.serialize_field("maximum_spend_microunits", &self.maximum_spend_microunits)?;
        state.serialize_field("currency", &self.currency)?;
        state.serialize_field("maximum_nodes", &self.maximum_nodes)?;
        state.serialize_field("maximum_depth", &self.maximum_depth)?;
        state.serialize_field("maximum_concurrency", &self.maximum_concurrency)?;
        state.serialize_field("maximum_artifact_bytes", &self.maximum_artifact_bytes)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.serialize_field("eligibility_proof_digest", &self.eligibility_proof_digest)?;
        state.serialize_field("maximum_model_calls", &self.maximum_model_calls)?;
        state.serialize_field("maximum_events", &self.maximum_events)?;
        state.serialize_field(
            "maximum_dependency_packages",
            &self.maximum_dependency_packages,
        )?;
        state.serialize_field("maximum_dependency_bytes", &self.maximum_dependency_bytes)?;
        state.serialize_field(
            "maximum_dependency_requests",
            &self.maximum_dependency_requests,
        )?;
        state.end()
    }
}
impl ResourceEnvelopeV2 {
    pub const TYPE_NAME: &'static str = "ResourceEnvelopeV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "root_id",
        "maximum_cpu_ms",
        "maximum_memory_bytes",
        "maximum_wall_ms",
        "maximum_model_tokens",
        "maximum_spend_microunits",
        "currency",
        "maximum_nodes",
        "maximum_depth",
        "maximum_concurrency",
        "maximum_artifact_bytes",
        "expires_at",
        "eligibility_proof_digest",
        "maximum_model_calls",
        "maximum_events",
        "maximum_dependency_packages",
        "maximum_dependency_bytes",
        "maximum_dependency_requests",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ResourceEnvelopeV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.root_id.canonical_cmp(&other.root_id))
            .then_with(|| self.maximum_cpu_ms.canonical_cmp(&other.maximum_cpu_ms))
            .then_with(|| {
                self.maximum_memory_bytes
                    .canonical_cmp(&other.maximum_memory_bytes)
            })
            .then_with(|| self.maximum_wall_ms.canonical_cmp(&other.maximum_wall_ms))
            .then_with(|| {
                self.maximum_model_tokens
                    .canonical_cmp(&other.maximum_model_tokens)
            })
            .then_with(|| {
                self.maximum_spend_microunits
                    .canonical_cmp(&other.maximum_spend_microunits)
            })
            .then_with(|| self.currency.canonical_cmp(&other.currency))
            .then_with(|| self.maximum_nodes.canonical_cmp(&other.maximum_nodes))
            .then_with(|| self.maximum_depth.canonical_cmp(&other.maximum_depth))
            .then_with(|| {
                self.maximum_concurrency
                    .canonical_cmp(&other.maximum_concurrency)
            })
            .then_with(|| {
                self.maximum_artifact_bytes
                    .canonical_cmp(&other.maximum_artifact_bytes)
            })
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
            .then_with(|| {
                self.eligibility_proof_digest
                    .canonical_cmp(&other.eligibility_proof_digest)
            })
            .then_with(|| {
                self.maximum_model_calls
                    .canonical_cmp(&other.maximum_model_calls)
            })
            .then_with(|| self.maximum_events.canonical_cmp(&other.maximum_events))
            .then_with(|| {
                self.maximum_dependency_packages
                    .canonical_cmp(&other.maximum_dependency_packages)
            })
            .then_with(|| {
                self.maximum_dependency_bytes
                    .canonical_cmp(&other.maximum_dependency_bytes)
            })
            .then_with(|| {
                self.maximum_dependency_requests
                    .canonical_cmp(&other.maximum_dependency_requests)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(21u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.root_id.canonical_size())
            .saturating_add(self.maximum_cpu_ms.canonical_size())
            .saturating_add(self.maximum_memory_bytes.canonical_size())
            .saturating_add(self.maximum_wall_ms.canonical_size())
            .saturating_add(self.maximum_model_tokens.canonical_size())
            .saturating_add(self.maximum_spend_microunits.canonical_size())
            .saturating_add(self.currency.canonical_size())
            .saturating_add(self.maximum_nodes.canonical_size())
            .saturating_add(self.maximum_depth.canonical_size())
            .saturating_add(self.maximum_concurrency.canonical_size())
            .saturating_add(self.maximum_artifact_bytes.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
            .saturating_add(self.eligibility_proof_digest.canonical_size())
            .saturating_add(self.maximum_model_calls.canonical_size())
            .saturating_add(self.maximum_events.canonical_size())
            .saturating_add(self.maximum_dependency_packages.canonical_size())
            .saturating_add(self.maximum_dependency_bytes.canonical_size())
            .saturating_add(self.maximum_dependency_requests.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.root_id.canonical_nesting())
                .max(self.maximum_cpu_ms.canonical_nesting())
                .max(self.maximum_memory_bytes.canonical_nesting())
                .max(self.maximum_wall_ms.canonical_nesting())
                .max(self.maximum_model_tokens.canonical_nesting())
                .max(self.maximum_spend_microunits.canonical_nesting())
                .max(self.currency.canonical_nesting())
                .max(self.maximum_nodes.canonical_nesting())
                .max(self.maximum_depth.canonical_nesting())
                .max(self.maximum_concurrency.canonical_nesting())
                .max(self.maximum_artifact_bytes.canonical_nesting())
                .max(self.expires_at.canonical_nesting())
                .max(self.eligibility_proof_digest.canonical_nesting())
                .max(self.maximum_model_calls.canonical_nesting())
                .max(self.maximum_events.canonical_nesting())
                .max(self.maximum_dependency_packages.canonical_nesting())
                .max(self.maximum_dependency_bytes.canonical_nesting())
                .max(self.maximum_dependency_requests.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationRecordV1Type {
    #[serde(rename = "RevocationRecordV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<RevocationRecordV1Fields>"
)]
pub struct RevocationRecordV1 {
    pub r#type: RevocationRecordV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub revocation_id: Uuid,
    pub subject_digest: Hex<32>,
    pub revoked_generation: U64Str,
    pub reason: Identifier,
    pub effective_at: I64Str,
    pub authority_id: Uuid,
    pub subject_type: Identifier,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RevocationRecordV1Fields {
    r#type: RevocationRecordV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    revocation_id: Uuid,
    subject_digest: Hex<32>,
    revoked_generation: U64Str,
    reason: Identifier,
    effective_at: I64Str,
    authority_id: Uuid,
    subject_type: Identifier,
}
impl TryFrom<crate::primitives::ObjectOnly<RevocationRecordV1Fields>> for RevocationRecordV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<RevocationRecordV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            revocation_id: fields.revocation_id,
            subject_digest: fields.subject_digest,
            revoked_generation: fields.revoked_generation,
            reason: fields.reason,
            effective_at: fields.effective_at,
            authority_id: fields.authority_id,
            subject_type: fields.subject_type,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for RevocationRecordV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 10)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("revocation_id", &self.revocation_id)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("revoked_generation", &self.revoked_generation)?;
        state.serialize_field("reason", &self.reason)?;
        state.serialize_field("effective_at", &self.effective_at)?;
        state.serialize_field("authority_id", &self.authority_id)?;
        state.serialize_field("subject_type", &self.subject_type)?;
        state.end()
    }
}
impl RevocationRecordV1 {
    pub const TYPE_NAME: &'static str = "RevocationRecordV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "revocation_id",
        "subject_digest",
        "revoked_generation",
        "reason",
        "effective_at",
        "authority_id",
        "subject_type",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for RevocationRecordV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.revocation_id.canonical_cmp(&other.revocation_id))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| {
                self.revoked_generation
                    .canonical_cmp(&other.revoked_generation)
            })
            .then_with(|| self.reason.canonical_cmp(&other.reason))
            .then_with(|| self.effective_at.canonical_cmp(&other.effective_at))
            .then_with(|| self.authority_id.canonical_cmp(&other.authority_id))
            .then_with(|| self.subject_type.canonical_cmp(&other.subject_type))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(10u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.revocation_id.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.revoked_generation.canonical_size())
            .saturating_add(self.reason.canonical_size())
            .saturating_add(self.effective_at.canonical_size())
            .saturating_add(self.authority_id.canonical_size())
            .saturating_add(self.subject_type.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.revocation_id.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.revoked_generation.canonical_nesting())
                .max(self.reason.canonical_nesting())
                .max(self.effective_at.canonical_nesting())
                .max(self.authority_id.canonical_nesting())
                .max(self.subject_type.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeV1Type {
    #[serde(rename = "ScopeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<ScopeV1Fields>"
)]
pub struct ScopeV1 {
    pub r#type: ScopeV1Type,
    pub wire_revision: WireRevision,
    pub tenant_id: Uuid,
    pub repository_id: U64Str,
    pub repository_tenant_epoch: U64Str,
    pub authority_epoch: Uuid,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScopeV1Fields {
    r#type: ScopeV1Type,
    wire_revision: WireRevision,
    tenant_id: Uuid,
    repository_id: U64Str,
    repository_tenant_epoch: U64Str,
    authority_epoch: Uuid,
}
impl TryFrom<crate::primitives::ObjectOnly<ScopeV1Fields>> for ScopeV1 {
    type Error = ProjectionError;
    fn try_from(input: crate::primitives::ObjectOnly<ScopeV1Fields>) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            tenant_id: fields.tenant_id,
            repository_id: fields.repository_id,
            repository_tenant_epoch: fields.repository_tenant_epoch,
            authority_epoch: fields.authority_epoch,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for ScopeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 6)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("repository_id", &self.repository_id)?;
        state.serialize_field("repository_tenant_epoch", &self.repository_tenant_epoch)?;
        state.serialize_field("authority_epoch", &self.authority_epoch)?;
        state.end()
    }
}
impl ScopeV1 {
    pub const TYPE_NAME: &'static str = "ScopeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "tenant_id",
        "repository_id",
        "repository_tenant_epoch",
        "authority_epoch",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for ScopeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.tenant_id.canonical_cmp(&other.tenant_id))
            .then_with(|| self.repository_id.canonical_cmp(&other.repository_id))
            .then_with(|| {
                self.repository_tenant_epoch
                    .canonical_cmp(&other.repository_tenant_epoch)
            })
            .then_with(|| self.authority_epoch.canonical_cmp(&other.authority_epoch))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(6u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.tenant_id.canonical_size())
            .saturating_add(self.repository_id.canonical_size())
            .saturating_add(self.repository_tenant_epoch.canonical_size())
            .saturating_add(self.authority_epoch.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.tenant_id
                .canonical_nesting()
                .max(self.repository_id.canonical_nesting())
                .max(self.repository_tenant_epoch.canonical_nesting())
                .max(self.authority_epoch.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignedEnvelopeV1Type {
    #[serde(rename = "SignedEnvelopeV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<SignedEnvelopeV1Fields>"
)]
pub struct SignedEnvelopeV1 {
    pub r#type: SignedEnvelopeV1Type,
    pub wire_revision: WireRevision,
    pub contract_type: Identifier,
    pub payload_digest: Hex<32>,
    pub signer_id: Uuid,
    pub key_id: Uuid,
    pub credential_generation: U64Str,
    pub signature: Hex<64>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SignedEnvelopeV1Fields {
    r#type: SignedEnvelopeV1Type,
    wire_revision: WireRevision,
    contract_type: Identifier,
    payload_digest: Hex<32>,
    signer_id: Uuid,
    key_id: Uuid,
    credential_generation: U64Str,
    signature: Hex<64>,
}
impl TryFrom<crate::primitives::ObjectOnly<SignedEnvelopeV1Fields>> for SignedEnvelopeV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<SignedEnvelopeV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            contract_type: fields.contract_type,
            payload_digest: fields.payload_digest,
            signer_id: fields.signer_id,
            key_id: fields.key_id,
            credential_generation: fields.credential_generation,
            signature: fields.signature,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for SignedEnvelopeV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("contract_type", &self.contract_type)?;
        state.serialize_field("payload_digest", &self.payload_digest)?;
        state.serialize_field("signer_id", &self.signer_id)?;
        state.serialize_field("key_id", &self.key_id)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("signature", &self.signature)?;
        state.end()
    }
}
impl SignedEnvelopeV1 {
    pub const TYPE_NAME: &'static str = "SignedEnvelopeV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "contract_type",
        "payload_digest",
        "signer_id",
        "key_id",
        "credential_generation",
        "signature",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for SignedEnvelopeV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.contract_type.canonical_cmp(&other.contract_type))
            .then_with(|| self.payload_digest.canonical_cmp(&other.payload_digest))
            .then_with(|| self.signer_id.canonical_cmp(&other.signer_id))
            .then_with(|| self.key_id.canonical_cmp(&other.key_id))
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| self.signature.canonical_cmp(&other.signature))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.contract_type.canonical_size())
            .saturating_add(self.payload_digest.canonical_size())
            .saturating_add(self.signer_id.canonical_size())
            .saturating_add(self.key_id.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.signature.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.contract_type
                .canonical_nesting()
                .max(self.payload_digest.canonical_nesting())
                .max(self.signer_id.canonical_nesting())
                .max(self.key_id.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.signature.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreeEntryV1Type {
    #[serde(rename = "TreeEntryV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<TreeEntryV1Fields>"
)]
pub struct TreeEntryV1 {
    pub r#type: TreeEntryV1Type,
    pub wire_revision: WireRevision,
    pub path: GitPath,
    pub mode: Identifier,
    pub oid: Opt<GitOidV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TreeEntryV1Fields {
    r#type: TreeEntryV1Type,
    wire_revision: WireRevision,
    path: GitPath,
    mode: Identifier,
    oid: Opt<GitOidV1>,
}
impl TryFrom<crate::primitives::ObjectOnly<TreeEntryV1Fields>> for TreeEntryV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<TreeEntryV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            path: fields.path,
            mode: fields.mode,
            oid: fields.oid,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for TreeEntryV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 5)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("mode", &self.mode)?;
        state.serialize_field("oid", &self.oid)?;
        state.end()
    }
}
impl TreeEntryV1 {
    pub const TYPE_NAME: &'static str = "TreeEntryV1";
    pub const FIELD_ORDER: &'static [&'static str] =
        &["type", "wire_revision", "path", "mode", "oid"];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for TreeEntryV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.path.canonical_cmp(&other.path))
            .then_with(|| self.mode.canonical_cmp(&other.mode))
            .then_with(|| self.oid.canonical_cmp(&other.oid))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(5u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.path.canonical_size())
            .saturating_add(self.mode.canonical_size())
            .saturating_add(self.oid.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.path
                .canonical_nesting()
                .max(self.mode.canonical_nesting())
                .max(self.oid.canonical_nesting()),
        )
    }
    fn identity_key(&self) -> Option<Vec<u8>> {
        Some(self.path.0.clone())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustBundleV1Type {
    #[serde(rename = "TrustBundleV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<TrustBundleV1Fields>"
)]
pub struct TrustBundleV1 {
    pub r#type: TrustBundleV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub trust_bundle_id: Uuid,
    pub generation: U64Str,
    pub members: SetOf<TrustMemberV1>,
    pub created_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrustBundleV1Fields {
    r#type: TrustBundleV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    trust_bundle_id: Uuid,
    generation: U64Str,
    members: SetOf<TrustMemberV1>,
    created_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<TrustBundleV1Fields>> for TrustBundleV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<TrustBundleV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            trust_bundle_id: fields.trust_bundle_id,
            generation: fields.generation,
            members: fields.members,
            created_at: fields.created_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for TrustBundleV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("trust_bundle_id", &self.trust_bundle_id)?;
        state.serialize_field("generation", &self.generation)?;
        state.serialize_field("members", &self.members)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl TrustBundleV1 {
    pub const TYPE_NAME: &'static str = "TrustBundleV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "trust_bundle_id",
        "generation",
        "members",
        "created_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for TrustBundleV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.trust_bundle_id.canonical_cmp(&other.trust_bundle_id))
            .then_with(|| self.generation.canonical_cmp(&other.generation))
            .then_with(|| self.members.canonical_cmp(&other.members))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.trust_bundle_id.canonical_size())
            .saturating_add(self.generation.canonical_size())
            .saturating_add(self.members.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.trust_bundle_id.canonical_nesting())
                .max(self.generation.canonical_nesting())
                .max(self.members.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustMemberV1Type {
    #[serde(rename = "TrustMemberV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<TrustMemberV1Fields>"
)]
pub struct TrustMemberV1 {
    pub r#type: TrustMemberV1Type,
    pub wire_revision: WireRevision,
    pub component_id: Uuid,
    pub role: Identifier,
    pub key_id: Uuid,
    pub public_key: Hex<32>,
    pub credential_generation: U64Str,
    pub administrative_domain_id: Uuid,
    pub capability_digest: Hex<32>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrustMemberV1Fields {
    r#type: TrustMemberV1Type,
    wire_revision: WireRevision,
    component_id: Uuid,
    role: Identifier,
    key_id: Uuid,
    public_key: Hex<32>,
    credential_generation: U64Str,
    administrative_domain_id: Uuid,
    capability_digest: Hex<32>,
}
impl TryFrom<crate::primitives::ObjectOnly<TrustMemberV1Fields>> for TrustMemberV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<TrustMemberV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            component_id: fields.component_id,
            role: fields.role,
            key_id: fields.key_id,
            public_key: fields.public_key,
            credential_generation: fields.credential_generation,
            administrative_domain_id: fields.administrative_domain_id,
            capability_digest: fields.capability_digest,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for TrustMemberV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 9)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("component_id", &self.component_id)?;
        state.serialize_field("role", &self.role)?;
        state.serialize_field("key_id", &self.key_id)?;
        state.serialize_field("public_key", &self.public_key)?;
        state.serialize_field("credential_generation", &self.credential_generation)?;
        state.serialize_field("administrative_domain_id", &self.administrative_domain_id)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.end()
    }
}
impl TrustMemberV1 {
    pub const TYPE_NAME: &'static str = "TrustMemberV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "component_id",
        "role",
        "key_id",
        "public_key",
        "credential_generation",
        "administrative_domain_id",
        "capability_digest",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for TrustMemberV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.component_id.canonical_cmp(&other.component_id))
            .then_with(|| self.role.canonical_cmp(&other.role))
            .then_with(|| self.key_id.canonical_cmp(&other.key_id))
            .then_with(|| self.public_key.canonical_cmp(&other.public_key))
            .then_with(|| {
                self.credential_generation
                    .canonical_cmp(&other.credential_generation)
            })
            .then_with(|| {
                self.administrative_domain_id
                    .canonical_cmp(&other.administrative_domain_id)
            })
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(9u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.component_id.canonical_size())
            .saturating_add(self.role.canonical_size())
            .saturating_add(self.key_id.canonical_size())
            .saturating_add(self.public_key.canonical_size())
            .saturating_add(self.credential_generation.canonical_size())
            .saturating_add(self.administrative_domain_id.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.component_id
                .canonical_nesting()
                .max(self.role.canonical_nesting())
                .max(self.key_id.canonical_nesting())
                .max(self.public_key.canonical_nesting())
                .max(self.credential_generation.canonical_nesting())
                .max(self.administrative_domain_id.canonical_nesting())
                .max(self.capability_digest.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustedDisplayReceiptV1Type {
    #[serde(rename = "TrustedDisplayReceiptV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<TrustedDisplayReceiptV1Fields>"
)]
pub struct TrustedDisplayReceiptV1 {
    pub r#type: TrustedDisplayReceiptV1Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub receipt_id: Uuid,
    pub plan_digest: Hex<32>,
    pub custody_digest: Hex<32>,
    pub displayed_effects_digest: Hex<32>,
    pub candidate_tree: GitOidV1,
    pub display_origin: Origin,
    pub renderer_bundle_digest: Hex<32>,
    pub session_id: Uuid,
    pub displayed_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrustedDisplayReceiptV1Fields {
    r#type: TrustedDisplayReceiptV1Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    receipt_id: Uuid,
    plan_digest: Hex<32>,
    custody_digest: Hex<32>,
    displayed_effects_digest: Hex<32>,
    candidate_tree: GitOidV1,
    display_origin: Origin,
    renderer_bundle_digest: Hex<32>,
    session_id: Uuid,
    displayed_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<TrustedDisplayReceiptV1Fields>>
    for TrustedDisplayReceiptV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<TrustedDisplayReceiptV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            receipt_id: fields.receipt_id,
            plan_digest: fields.plan_digest,
            custody_digest: fields.custody_digest,
            displayed_effects_digest: fields.displayed_effects_digest,
            candidate_tree: fields.candidate_tree,
            display_origin: fields.display_origin,
            renderer_bundle_digest: fields.renderer_bundle_digest,
            session_id: fields.session_id,
            displayed_at: fields.displayed_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for TrustedDisplayReceiptV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 13)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("receipt_id", &self.receipt_id)?;
        state.serialize_field("plan_digest", &self.plan_digest)?;
        state.serialize_field("custody_digest", &self.custody_digest)?;
        state.serialize_field("displayed_effects_digest", &self.displayed_effects_digest)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("display_origin", &self.display_origin)?;
        state.serialize_field("renderer_bundle_digest", &self.renderer_bundle_digest)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("displayed_at", &self.displayed_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl TrustedDisplayReceiptV1 {
    pub const TYPE_NAME: &'static str = "TrustedDisplayReceiptV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "receipt_id",
        "plan_digest",
        "custody_digest",
        "displayed_effects_digest",
        "candidate_tree",
        "display_origin",
        "renderer_bundle_digest",
        "session_id",
        "displayed_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for TrustedDisplayReceiptV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.receipt_id.canonical_cmp(&other.receipt_id))
            .then_with(|| self.plan_digest.canonical_cmp(&other.plan_digest))
            .then_with(|| self.custody_digest.canonical_cmp(&other.custody_digest))
            .then_with(|| {
                self.displayed_effects_digest
                    .canonical_cmp(&other.displayed_effects_digest)
            })
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.display_origin.canonical_cmp(&other.display_origin))
            .then_with(|| {
                self.renderer_bundle_digest
                    .canonical_cmp(&other.renderer_bundle_digest)
            })
            .then_with(|| self.session_id.canonical_cmp(&other.session_id))
            .then_with(|| self.displayed_at.canonical_cmp(&other.displayed_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(13u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.receipt_id.canonical_size())
            .saturating_add(self.plan_digest.canonical_size())
            .saturating_add(self.custody_digest.canonical_size())
            .saturating_add(self.displayed_effects_digest.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.display_origin.canonical_size())
            .saturating_add(self.renderer_bundle_digest.canonical_size())
            .saturating_add(self.session_id.canonical_size())
            .saturating_add(self.displayed_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.receipt_id.canonical_nesting())
                .max(self.plan_digest.canonical_nesting())
                .max(self.custody_digest.canonical_nesting())
                .max(self.displayed_effects_digest.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.display_origin.canonical_nesting())
                .max(self.renderer_bundle_digest.canonical_nesting())
                .max(self.session_id.canonical_nesting())
                .max(self.displayed_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationReceiptV2IndependenceClass {
    #[serde(rename = "C0")]
    C0,
    #[serde(rename = "C1")]
    C1,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "C4")]
    C4,
}
impl VerificationReceiptV2IndependenceClass {
    pub fn code(&self) -> u64 {
        match self {
            Self::C0 => 0,
            Self::C1 => 1,
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::C0),
            1 => Some(Self::C1),
            2 => Some(Self::C2),
            3 => Some(Self::C3),
            4 => Some(Self::C4),
            _ => None,
        }
    }
}
impl CanonicalOrder for VerificationReceiptV2IndependenceClass {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationReceiptV2Outcome {
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "UNSUPPORTED")]
    Unsupported,
    #[serde(rename = "INCONCLUSIVE")]
    Inconclusive,
}
impl VerificationReceiptV2Outcome {
    pub fn code(&self) -> u64 {
        match self {
            Self::Confirmed => 0,
            Self::Rejected => 1,
            Self::Unsupported => 2,
            Self::Inconclusive => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Confirmed),
            1 => Some(Self::Rejected),
            2 => Some(Self::Unsupported),
            3 => Some(Self::Inconclusive),
            _ => None,
        }
    }
}
impl CanonicalOrder for VerificationReceiptV2Outcome {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationReceiptV2Type {
    #[serde(rename = "VerificationReceiptV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<VerificationReceiptV2Fields>"
)]
pub struct VerificationReceiptV2 {
    pub r#type: VerificationReceiptV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub receipt_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub evidence_digest: Hex<32>,
    pub verifier_id: Uuid,
    pub verifier_generation: U64Str,
    pub administrative_domain_id: Uuid,
    pub environment_digest: Hex<32>,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub oracle: OracleTrustClassV1,
    pub independence_class: VerificationReceiptV2IndependenceClass,
    pub outcome: VerificationReceiptV2Outcome,
    pub started_at: I64Str,
    pub completed_at: I64Str,
    pub lease_generation: U64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationReceiptV2Fields {
    r#type: VerificationReceiptV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    receipt_id: Uuid,
    attempt_binding_digest: Hex<32>,
    evidence_digest: Hex<32>,
    verifier_id: Uuid,
    verifier_generation: U64Str,
    administrative_domain_id: Uuid,
    environment_digest: Hex<32>,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    oracle: OracleTrustClassV1,
    independence_class: VerificationReceiptV2IndependenceClass,
    outcome: VerificationReceiptV2Outcome,
    started_at: I64Str,
    completed_at: I64Str,
    lease_generation: U64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<VerificationReceiptV2Fields>> for VerificationReceiptV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<VerificationReceiptV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            receipt_id: fields.receipt_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            evidence_digest: fields.evidence_digest,
            verifier_id: fields.verifier_id,
            verifier_generation: fields.verifier_generation,
            administrative_domain_id: fields.administrative_domain_id,
            environment_digest: fields.environment_digest,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            oracle: fields.oracle,
            independence_class: fields.independence_class,
            outcome: fields.outcome,
            started_at: fields.started_at,
            completed_at: fields.completed_at,
            lease_generation: fields.lease_generation,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for VerificationReceiptV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 18)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("receipt_id", &self.receipt_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("evidence_digest", &self.evidence_digest)?;
        state.serialize_field("verifier_id", &self.verifier_id)?;
        state.serialize_field("verifier_generation", &self.verifier_generation)?;
        state.serialize_field("administrative_domain_id", &self.administrative_domain_id)?;
        state.serialize_field("environment_digest", &self.environment_digest)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("oracle", &self.oracle)?;
        state.serialize_field("independence_class", &self.independence_class)?;
        state.serialize_field("outcome", &self.outcome)?;
        state.serialize_field("started_at", &self.started_at)?;
        state.serialize_field("completed_at", &self.completed_at)?;
        state.serialize_field("lease_generation", &self.lease_generation)?;
        state.end()
    }
}
impl VerificationReceiptV2 {
    pub const TYPE_NAME: &'static str = "VerificationReceiptV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "receipt_id",
        "attempt_binding_digest",
        "evidence_digest",
        "verifier_id",
        "verifier_generation",
        "administrative_domain_id",
        "environment_digest",
        "subject_digest",
        "policy_digest",
        "oracle",
        "independence_class",
        "outcome",
        "started_at",
        "completed_at",
        "lease_generation",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for VerificationReceiptV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.receipt_id.canonical_cmp(&other.receipt_id))
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.evidence_digest.canonical_cmp(&other.evidence_digest))
            .then_with(|| self.verifier_id.canonical_cmp(&other.verifier_id))
            .then_with(|| {
                self.verifier_generation
                    .canonical_cmp(&other.verifier_generation)
            })
            .then_with(|| {
                self.administrative_domain_id
                    .canonical_cmp(&other.administrative_domain_id)
            })
            .then_with(|| {
                self.environment_digest
                    .canonical_cmp(&other.environment_digest)
            })
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.oracle.canonical_cmp(&other.oracle))
            .then_with(|| {
                self.independence_class
                    .canonical_cmp(&other.independence_class)
            })
            .then_with(|| self.outcome.canonical_cmp(&other.outcome))
            .then_with(|| self.started_at.canonical_cmp(&other.started_at))
            .then_with(|| self.completed_at.canonical_cmp(&other.completed_at))
            .then_with(|| self.lease_generation.canonical_cmp(&other.lease_generation))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(18u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.receipt_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.evidence_digest.canonical_size())
            .saturating_add(self.verifier_id.canonical_size())
            .saturating_add(self.verifier_generation.canonical_size())
            .saturating_add(self.administrative_domain_id.canonical_size())
            .saturating_add(self.environment_digest.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.oracle.canonical_size())
            .saturating_add(self.independence_class.canonical_size())
            .saturating_add(self.outcome.canonical_size())
            .saturating_add(self.started_at.canonical_size())
            .saturating_add(self.completed_at.canonical_size())
            .saturating_add(self.lease_generation.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.receipt_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.evidence_digest.canonical_nesting())
                .max(self.verifier_id.canonical_nesting())
                .max(self.verifier_generation.canonical_nesting())
                .max(self.administrative_domain_id.canonical_nesting())
                .max(self.environment_digest.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.oracle.canonical_nesting())
                .max(self.independence_class.canonical_nesting())
                .max(self.outcome.canonical_nesting())
                .max(self.started_at.canonical_nesting())
                .max(self.completed_at.canonical_nesting())
                .max(self.lease_generation.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationSetV2Type {
    #[serde(rename = "VerificationSetV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<VerificationSetV2Fields>"
)]
pub struct VerificationSetV2 {
    pub r#type: VerificationSetV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub verification_set_id: Uuid,
    pub attempt_binding_digest: Hex<32>,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub trust_set_digest: Hex<32>,
    pub receipt_digests: SetOf<Hex<32>>,
    pub issuer_id: Uuid,
    pub issued_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationSetV2Fields {
    r#type: VerificationSetV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    verification_set_id: Uuid,
    attempt_binding_digest: Hex<32>,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    trust_set_digest: Hex<32>,
    receipt_digests: SetOf<Hex<32>>,
    issuer_id: Uuid,
    issued_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<VerificationSetV2Fields>> for VerificationSetV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<VerificationSetV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            verification_set_id: fields.verification_set_id,
            attempt_binding_digest: fields.attempt_binding_digest,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            trust_set_digest: fields.trust_set_digest,
            receipt_digests: fields.receipt_digests,
            issuer_id: fields.issuer_id,
            issued_at: fields.issued_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for VerificationSetV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 11)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("verification_set_id", &self.verification_set_id)?;
        state.serialize_field("attempt_binding_digest", &self.attempt_binding_digest)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("trust_set_digest", &self.trust_set_digest)?;
        state.serialize_field("receipt_digests", &self.receipt_digests)?;
        state.serialize_field("issuer_id", &self.issuer_id)?;
        state.serialize_field("issued_at", &self.issued_at)?;
        state.end()
    }
}
impl VerificationSetV2 {
    pub const TYPE_NAME: &'static str = "VerificationSetV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "verification_set_id",
        "attempt_binding_digest",
        "subject_digest",
        "policy_digest",
        "trust_set_digest",
        "receipt_digests",
        "issuer_id",
        "issued_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for VerificationSetV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| {
                self.verification_set_id
                    .canonical_cmp(&other.verification_set_id)
            })
            .then_with(|| {
                self.attempt_binding_digest
                    .canonical_cmp(&other.attempt_binding_digest)
            })
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.trust_set_digest.canonical_cmp(&other.trust_set_digest))
            .then_with(|| self.receipt_digests.canonical_cmp(&other.receipt_digests))
            .then_with(|| self.issuer_id.canonical_cmp(&other.issuer_id))
            .then_with(|| self.issued_at.canonical_cmp(&other.issued_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(11u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.verification_set_id.canonical_size())
            .saturating_add(self.attempt_binding_digest.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.trust_set_digest.canonical_size())
            .saturating_add(self.receipt_digests.canonical_size())
            .saturating_add(self.issuer_id.canonical_size())
            .saturating_add(self.issued_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.verification_set_id.canonical_nesting())
                .max(self.attempt_binding_digest.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.trust_set_digest.canonical_nesting())
                .max(self.receipt_digests.canonical_nesting())
                .max(self.issuer_id.canonical_nesting())
                .max(self.issued_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerifiedPublicationPlanV2Type {
    #[serde(rename = "VerifiedPublicationPlanV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<VerifiedPublicationPlanV2Fields>"
)]
pub struct VerifiedPublicationPlanV2 {
    pub r#type: VerifiedPublicationPlanV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub publication_id: Uuid,
    pub patch_revision_id: Uuid,
    pub subject_digest: Hex<32>,
    pub base_commit: GitOidV1,
    pub base_tree: GitOidV1,
    pub candidate_tree: GitOidV1,
    pub patch_digest: Hex<32>,
    pub manifest_digest: Hex<32>,
    pub verification_set_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub trust_set_digest: Hex<32>,
    pub changed_objects: SetOf<ChangedObjectV1>,
    pub destination_ref: GitRef,
    pub target_base_ref: GitRef,
    pub commit_metadata_digest: Hex<32>,
    pub fix_pr_metadata_digest: Hex<32>,
    pub custody_id: Uuid,
    pub effects_digest: Hex<32>,
    pub verifier_id: Uuid,
    pub created_at: I64Str,
    pub expires_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerifiedPublicationPlanV2Fields {
    r#type: VerifiedPublicationPlanV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    publication_id: Uuid,
    patch_revision_id: Uuid,
    subject_digest: Hex<32>,
    base_commit: GitOidV1,
    base_tree: GitOidV1,
    candidate_tree: GitOidV1,
    patch_digest: Hex<32>,
    manifest_digest: Hex<32>,
    verification_set_digest: Hex<32>,
    policy_digest: Hex<32>,
    trust_set_digest: Hex<32>,
    changed_objects: SetOf<ChangedObjectV1>,
    destination_ref: GitRef,
    target_base_ref: GitRef,
    commit_metadata_digest: Hex<32>,
    fix_pr_metadata_digest: Hex<32>,
    custody_id: Uuid,
    effects_digest: Hex<32>,
    verifier_id: Uuid,
    created_at: I64Str,
    expires_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<VerifiedPublicationPlanV2Fields>>
    for VerifiedPublicationPlanV2
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<VerifiedPublicationPlanV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            publication_id: fields.publication_id,
            patch_revision_id: fields.patch_revision_id,
            subject_digest: fields.subject_digest,
            base_commit: fields.base_commit,
            base_tree: fields.base_tree,
            candidate_tree: fields.candidate_tree,
            patch_digest: fields.patch_digest,
            manifest_digest: fields.manifest_digest,
            verification_set_digest: fields.verification_set_digest,
            policy_digest: fields.policy_digest,
            trust_set_digest: fields.trust_set_digest,
            changed_objects: fields.changed_objects,
            destination_ref: fields.destination_ref,
            target_base_ref: fields.target_base_ref,
            commit_metadata_digest: fields.commit_metadata_digest,
            fix_pr_metadata_digest: fields.fix_pr_metadata_digest,
            custody_id: fields.custody_id,
            effects_digest: fields.effects_digest,
            verifier_id: fields.verifier_id,
            created_at: fields.created_at,
            expires_at: fields.expires_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for VerifiedPublicationPlanV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 24)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("publication_id", &self.publication_id)?;
        state.serialize_field("patch_revision_id", &self.patch_revision_id)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("base_commit", &self.base_commit)?;
        state.serialize_field("base_tree", &self.base_tree)?;
        state.serialize_field("candidate_tree", &self.candidate_tree)?;
        state.serialize_field("patch_digest", &self.patch_digest)?;
        state.serialize_field("manifest_digest", &self.manifest_digest)?;
        state.serialize_field("verification_set_digest", &self.verification_set_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("trust_set_digest", &self.trust_set_digest)?;
        state.serialize_field("changed_objects", &self.changed_objects)?;
        state.serialize_field("destination_ref", &self.destination_ref)?;
        state.serialize_field("target_base_ref", &self.target_base_ref)?;
        state.serialize_field("commit_metadata_digest", &self.commit_metadata_digest)?;
        state.serialize_field("fix_pr_metadata_digest", &self.fix_pr_metadata_digest)?;
        state.serialize_field("custody_id", &self.custody_id)?;
        state.serialize_field("effects_digest", &self.effects_digest)?;
        state.serialize_field("verifier_id", &self.verifier_id)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires_at", &self.expires_at)?;
        state.end()
    }
}
impl VerifiedPublicationPlanV2 {
    pub const TYPE_NAME: &'static str = "VerifiedPublicationPlanV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "publication_id",
        "patch_revision_id",
        "subject_digest",
        "base_commit",
        "base_tree",
        "candidate_tree",
        "patch_digest",
        "manifest_digest",
        "verification_set_digest",
        "policy_digest",
        "trust_set_digest",
        "changed_objects",
        "destination_ref",
        "target_base_ref",
        "commit_metadata_digest",
        "fix_pr_metadata_digest",
        "custody_id",
        "effects_digest",
        "verifier_id",
        "created_at",
        "expires_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for VerifiedPublicationPlanV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.publication_id.canonical_cmp(&other.publication_id))
            .then_with(|| {
                self.patch_revision_id
                    .canonical_cmp(&other.patch_revision_id)
            })
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.base_commit.canonical_cmp(&other.base_commit))
            .then_with(|| self.base_tree.canonical_cmp(&other.base_tree))
            .then_with(|| self.candidate_tree.canonical_cmp(&other.candidate_tree))
            .then_with(|| self.patch_digest.canonical_cmp(&other.patch_digest))
            .then_with(|| self.manifest_digest.canonical_cmp(&other.manifest_digest))
            .then_with(|| {
                self.verification_set_digest
                    .canonical_cmp(&other.verification_set_digest)
            })
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| self.trust_set_digest.canonical_cmp(&other.trust_set_digest))
            .then_with(|| self.changed_objects.canonical_cmp(&other.changed_objects))
            .then_with(|| self.destination_ref.canonical_cmp(&other.destination_ref))
            .then_with(|| self.target_base_ref.canonical_cmp(&other.target_base_ref))
            .then_with(|| {
                self.commit_metadata_digest
                    .canonical_cmp(&other.commit_metadata_digest)
            })
            .then_with(|| {
                self.fix_pr_metadata_digest
                    .canonical_cmp(&other.fix_pr_metadata_digest)
            })
            .then_with(|| self.custody_id.canonical_cmp(&other.custody_id))
            .then_with(|| self.effects_digest.canonical_cmp(&other.effects_digest))
            .then_with(|| self.verifier_id.canonical_cmp(&other.verifier_id))
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
            .then_with(|| self.expires_at.canonical_cmp(&other.expires_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(24u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.publication_id.canonical_size())
            .saturating_add(self.patch_revision_id.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.base_commit.canonical_size())
            .saturating_add(self.base_tree.canonical_size())
            .saturating_add(self.candidate_tree.canonical_size())
            .saturating_add(self.patch_digest.canonical_size())
            .saturating_add(self.manifest_digest.canonical_size())
            .saturating_add(self.verification_set_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.trust_set_digest.canonical_size())
            .saturating_add(self.changed_objects.canonical_size())
            .saturating_add(self.destination_ref.canonical_size())
            .saturating_add(self.target_base_ref.canonical_size())
            .saturating_add(self.commit_metadata_digest.canonical_size())
            .saturating_add(self.fix_pr_metadata_digest.canonical_size())
            .saturating_add(self.custody_id.canonical_size())
            .saturating_add(self.effects_digest.canonical_size())
            .saturating_add(self.verifier_id.canonical_size())
            .saturating_add(self.created_at.canonical_size())
            .saturating_add(self.expires_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.publication_id.canonical_nesting())
                .max(self.patch_revision_id.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.base_commit.canonical_nesting())
                .max(self.base_tree.canonical_nesting())
                .max(self.candidate_tree.canonical_nesting())
                .max(self.patch_digest.canonical_nesting())
                .max(self.manifest_digest.canonical_nesting())
                .max(self.verification_set_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.trust_set_digest.canonical_nesting())
                .max(self.changed_objects.canonical_nesting())
                .max(self.destination_ref.canonical_nesting())
                .max(self.target_base_ref.canonical_nesting())
                .max(self.commit_metadata_digest.canonical_nesting())
                .max(self.fix_pr_metadata_digest.canonical_nesting())
                .max(self.custody_id.canonical_nesting())
                .max(self.effects_digest.canonical_nesting())
                .max(self.verifier_id.canonical_nesting())
                .max(self.created_at.canonical_nesting())
                .max(self.expires_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebAuthnCeremonyV1Type {
    #[serde(rename = "WebAuthnCeremonyV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<WebAuthnCeremonyV1Fields>"
)]
pub struct WebAuthnCeremonyV1 {
    pub r#type: WebAuthnCeremonyV1Type,
    pub wire_revision: WireRevision,
    pub credential_id: Bytes,
    pub client_data_json: Bytes,
    pub authenticator_data: Bytes,
    pub signature: Bytes,
    pub challenge_digest: Hex<32>,
    pub received_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WebAuthnCeremonyV1Fields {
    r#type: WebAuthnCeremonyV1Type,
    wire_revision: WireRevision,
    credential_id: Bytes,
    client_data_json: Bytes,
    authenticator_data: Bytes,
    signature: Bytes,
    challenge_digest: Hex<32>,
    received_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<WebAuthnCeremonyV1Fields>> for WebAuthnCeremonyV1 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<WebAuthnCeremonyV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            credential_id: fields.credential_id,
            client_data_json: fields.client_data_json,
            authenticator_data: fields.authenticator_data,
            signature: fields.signature,
            challenge_digest: fields.challenge_digest,
            received_at: fields.received_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for WebAuthnCeremonyV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 8)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("credential_id", &self.credential_id)?;
        state.serialize_field("client_data_json", &self.client_data_json)?;
        state.serialize_field("authenticator_data", &self.authenticator_data)?;
        state.serialize_field("signature", &self.signature)?;
        state.serialize_field("challenge_digest", &self.challenge_digest)?;
        state.serialize_field("received_at", &self.received_at)?;
        state.end()
    }
}
impl WebAuthnCeremonyV1 {
    pub const TYPE_NAME: &'static str = "WebAuthnCeremonyV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "credential_id",
        "client_data_json",
        "authenticator_data",
        "signature",
        "challenge_digest",
        "received_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for WebAuthnCeremonyV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.credential_id.canonical_cmp(&other.credential_id))
            .then_with(|| self.client_data_json.canonical_cmp(&other.client_data_json))
            .then_with(|| {
                self.authenticator_data
                    .canonical_cmp(&other.authenticator_data)
            })
            .then_with(|| self.signature.canonical_cmp(&other.signature))
            .then_with(|| self.challenge_digest.canonical_cmp(&other.challenge_digest))
            .then_with(|| self.received_at.canonical_cmp(&other.received_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(8u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.credential_id.canonical_size())
            .saturating_add(self.client_data_json.canonical_size())
            .saturating_add(self.authenticator_data.canonical_size())
            .saturating_add(self.signature.canonical_size())
            .saturating_add(self.challenge_digest.canonical_size())
            .saturating_add(self.received_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.credential_id
                .canonical_nesting()
                .max(self.client_data_json.canonical_nesting())
                .max(self.authenticator_data.canonical_nesting())
                .max(self.signature.canonical_nesting())
                .max(self.challenge_digest.canonical_nesting())
                .max(self.received_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebAuthnRegistrationCeremonyV1Type {
    #[serde(rename = "WebAuthnRegistrationCeremonyV1")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<WebAuthnRegistrationCeremonyV1Fields>"
)]
pub struct WebAuthnRegistrationCeremonyV1 {
    pub r#type: WebAuthnRegistrationCeremonyV1Type,
    pub wire_revision: WireRevision,
    pub credential_id: Bytes,
    pub client_data_json: Bytes,
    pub attestation_object: Bytes,
    pub challenge_digest: Hex<32>,
    pub received_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WebAuthnRegistrationCeremonyV1Fields {
    r#type: WebAuthnRegistrationCeremonyV1Type,
    wire_revision: WireRevision,
    credential_id: Bytes,
    client_data_json: Bytes,
    attestation_object: Bytes,
    challenge_digest: Hex<32>,
    received_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<WebAuthnRegistrationCeremonyV1Fields>>
    for WebAuthnRegistrationCeremonyV1
{
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<WebAuthnRegistrationCeremonyV1Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            credential_id: fields.credential_id,
            client_data_json: fields.client_data_json,
            attestation_object: fields.attestation_object,
            challenge_digest: fields.challenge_digest,
            received_at: fields.received_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for WebAuthnRegistrationCeremonyV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 7)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("credential_id", &self.credential_id)?;
        state.serialize_field("client_data_json", &self.client_data_json)?;
        state.serialize_field("attestation_object", &self.attestation_object)?;
        state.serialize_field("challenge_digest", &self.challenge_digest)?;
        state.serialize_field("received_at", &self.received_at)?;
        state.end()
    }
}
impl WebAuthnRegistrationCeremonyV1 {
    pub const TYPE_NAME: &'static str = "WebAuthnRegistrationCeremonyV1";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "credential_id",
        "client_data_json",
        "attestation_object",
        "challenge_digest",
        "received_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for WebAuthnRegistrationCeremonyV1 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.credential_id.canonical_cmp(&other.credential_id))
            .then_with(|| self.client_data_json.canonical_cmp(&other.client_data_json))
            .then_with(|| {
                self.attestation_object
                    .canonical_cmp(&other.attestation_object)
            })
            .then_with(|| self.challenge_digest.canonical_cmp(&other.challenge_digest))
            .then_with(|| self.received_at.canonical_cmp(&other.received_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(7u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.credential_id.canonical_size())
            .saturating_add(self.client_data_json.canonical_size())
            .saturating_add(self.attestation_object.canonical_size())
            .saturating_add(self.challenge_digest.canonical_size())
            .saturating_add(self.received_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.credential_id
                .canonical_nesting()
                .max(self.client_data_json.canonical_nesting())
                .max(self.attestation_object.canonical_nesting())
                .max(self.challenge_digest.canonical_nesting())
                .max(self.received_at.canonical_nesting()),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowSpecCoreV2Mode {
    #[serde(rename = "FAST")]
    Fast,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "HYBRID")]
    Hybrid,
    #[serde(rename = "DEEP")]
    Deep,
}
impl WorkflowSpecCoreV2Mode {
    pub fn code(&self) -> u64 {
        match self {
            Self::Fast => 0,
            Self::Ai => 1,
            Self::Hybrid => 2,
            Self::Deep => 3,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::Fast),
            1 => Some(Self::Ai),
            2 => Some(Self::Hybrid),
            3 => Some(Self::Deep),
            _ => None,
        }
    }
}
impl CanonicalOrder for WorkflowSpecCoreV2Mode {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(self.code())
    }
    fn canonical_nesting(&self) -> usize {
        0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowSpecCoreV2Type {
    #[serde(rename = "WorkflowSpecCoreV2")]
    Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(
    deny_unknown_fields,
    try_from = "crate::primitives::ObjectOnly<WorkflowSpecCoreV2Fields>"
)]
pub struct WorkflowSpecCoreV2 {
    pub r#type: WorkflowSpecCoreV2Type,
    pub wire_revision: WireRevision,
    pub scope: ScopeV1,
    pub workflow_id: Uuid,
    pub subject_digest: Hex<32>,
    pub policy_digest: Hex<32>,
    pub analysis_plan_digest: Hex<32>,
    pub capability_digest: Hex<32>,
    pub resource_envelope_digest: Hex<32>,
    pub mode: WorkflowSpecCoreV2Mode,
    pub required_unit_ids: SetOf<Identifier>,
    pub created_at: I64Str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkflowSpecCoreV2Fields {
    r#type: WorkflowSpecCoreV2Type,
    wire_revision: WireRevision,
    scope: ScopeV1,
    workflow_id: Uuid,
    subject_digest: Hex<32>,
    policy_digest: Hex<32>,
    analysis_plan_digest: Hex<32>,
    capability_digest: Hex<32>,
    resource_envelope_digest: Hex<32>,
    mode: WorkflowSpecCoreV2Mode,
    required_unit_ids: SetOf<Identifier>,
    created_at: I64Str,
}
impl TryFrom<crate::primitives::ObjectOnly<WorkflowSpecCoreV2Fields>> for WorkflowSpecCoreV2 {
    type Error = ProjectionError;
    fn try_from(
        input: crate::primitives::ObjectOnly<WorkflowSpecCoreV2Fields>,
    ) -> Result<Self, Self::Error> {
        let fields = input.0;
        let value = Self {
            r#type: fields.r#type,
            wire_revision: fields.wire_revision,
            scope: fields.scope,
            workflow_id: fields.workflow_id,
            subject_digest: fields.subject_digest,
            policy_digest: fields.policy_digest,
            analysis_plan_digest: fields.analysis_plan_digest,
            capability_digest: fields.capability_digest,
            resource_envelope_digest: fields.resource_envelope_digest,
            mode: fields.mode,
            required_unit_ids: fields.required_unit_ids,
            created_at: fields.created_at,
        };
        value.validate_projection_limits()?;
        Ok(value)
    }
}
impl Serialize for WorkflowSpecCoreV2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.validate_projection_limits()
            .map_err(serde::ser::Error::custom)?;
        let mut state = serializer.serialize_struct(Self::TYPE_NAME, 12)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("wire_revision", &self.wire_revision)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("workflow_id", &self.workflow_id)?;
        state.serialize_field("subject_digest", &self.subject_digest)?;
        state.serialize_field("policy_digest", &self.policy_digest)?;
        state.serialize_field("analysis_plan_digest", &self.analysis_plan_digest)?;
        state.serialize_field("capability_digest", &self.capability_digest)?;
        state.serialize_field("resource_envelope_digest", &self.resource_envelope_digest)?;
        state.serialize_field("mode", &self.mode)?;
        state.serialize_field("required_unit_ids", &self.required_unit_ids)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}
impl WorkflowSpecCoreV2 {
    pub const TYPE_NAME: &'static str = "WorkflowSpecCoreV2";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "type",
        "wire_revision",
        "scope",
        "workflow_id",
        "subject_digest",
        "policy_digest",
        "analysis_plan_digest",
        "capability_digest",
        "resource_envelope_digest",
        "mode",
        "required_unit_ids",
        "created_at",
    ];
    pub fn validate_projection_limits(&self) -> Result<(), ProjectionError> {
        crate::primitives::validate_projection_limits(
            self.canonical_size(),
            self.canonical_nesting(),
        )
    }
}
impl CanonicalOrder for WorkflowSpecCoreV2 {
    fn canonical_cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.scope.canonical_cmp(&other.scope))
            .then_with(|| self.workflow_id.canonical_cmp(&other.workflow_id))
            .then_with(|| self.subject_digest.canonical_cmp(&other.subject_digest))
            .then_with(|| self.policy_digest.canonical_cmp(&other.policy_digest))
            .then_with(|| {
                self.analysis_plan_digest
                    .canonical_cmp(&other.analysis_plan_digest)
            })
            .then_with(|| {
                self.capability_digest
                    .canonical_cmp(&other.capability_digest)
            })
            .then_with(|| {
                self.resource_envelope_digest
                    .canonical_cmp(&other.resource_envelope_digest)
            })
            .then_with(|| self.mode.canonical_cmp(&other.mode))
            .then_with(|| {
                self.required_unit_ids
                    .canonical_cmp(&other.required_unit_ids)
            })
            .then_with(|| self.created_at.canonical_cmp(&other.created_at))
    }
    fn canonical_size(&self) -> usize {
        crate::primitives::canonical_header_len(12u64)
            .saturating_add(crate::primitives::canonical_header_len(
                Self::TYPE_NAME.len() as u64,
            ))
            .saturating_add(Self::TYPE_NAME.len())
            .saturating_add(1)
            .saturating_add(self.scope.canonical_size())
            .saturating_add(self.workflow_id.canonical_size())
            .saturating_add(self.subject_digest.canonical_size())
            .saturating_add(self.policy_digest.canonical_size())
            .saturating_add(self.analysis_plan_digest.canonical_size())
            .saturating_add(self.capability_digest.canonical_size())
            .saturating_add(self.resource_envelope_digest.canonical_size())
            .saturating_add(self.mode.canonical_size())
            .saturating_add(self.required_unit_ids.canonical_size())
            .saturating_add(self.created_at.canonical_size())
    }
    fn canonical_nesting(&self) -> usize {
        1usize.saturating_add(
            self.scope
                .canonical_nesting()
                .max(self.workflow_id.canonical_nesting())
                .max(self.subject_digest.canonical_nesting())
                .max(self.policy_digest.canonical_nesting())
                .max(self.analysis_plan_digest.canonical_nesting())
                .max(self.capability_digest.canonical_nesting())
                .max(self.resource_envelope_digest.canonical_nesting())
                .max(self.mode.canonical_nesting())
                .max(self.required_unit_ids.canonical_nesting())
                .max(self.created_at.canonical_nesting()),
        )
    }
}
pub const ALL_TYPE_NAMES: &[&str] = &[
    "AcquisitionRequestV1",
    "AnalysisCompletionReceiptV1",
    "AnalysisPlanV1",
    "AnalysisUnitV1",
    "ApprovalChallengeV2",
    "ApprovalCredentialBindingV1",
    "ApprovalEnrollmentActionV1",
    "ApprovalEnrollmentAuthorizationV1",
    "ApprovalEnrollmentChallengeV1",
    "ApprovalEnrollmentIntentV1",
    "ApprovalLifecycleChallengeV1",
    "ApprovalLifecycleConsentV1",
    "ApprovalRecoveryPolicyV1",
    "ArtifactCustodyEntryV1",
    "AttemptBindingV2",
    "AttemptEventV2",
    "AuditEnvelopeV1",
    "AuthorityScopeV1",
    "CapabilityProfileV1",
    "ChangedObjectV1",
    "CheckOutputV1",
    "CheckProjectionV2",
    "CheckWriteRequestV1",
    "CheckoutGrantV2",
    "CommitIdentityV1",
    "CreateBlobRequestV1",
    "CreateCommitRequestV1",
    "CreateFixPrRequestV1",
    "CreateRefRequestV1",
    "CreateTreeRequestV1",
    "CurrentCandidateV1",
    "DbStartCertificateV1",
    "DependencyAcquisitionGrantV1",
    "DependencyPackageV1",
    "DescriptorEntryV1",
    "EffectIntentV2",
    "EffectPlanEntryV1",
    "EffectStateV1",
    "EligibilityProofV2",
    "EvaluationIdentityV2",
    "EvidenceReceiptV2",
    "ExpectedAnalysisManifestV1",
    "ExternalEffectJournalV1",
    "FindingCandidateV2",
    "GateProofV2",
    "GitOidV1",
    "GitPublicationCapabilityV2",
    "HumanApprovalAssertionV2",
    "JournalEpochFenceV1",
    "JournalWitnessReceiptV1",
    "ManifestDescriptorV1",
    "ManualCaseStateV1",
    "ManualResolutionActionV1",
    "ManualResolutionV1",
    "MergeSubjectV2",
    "ModelContentBlockV1",
    "ModelContentV1",
    "ModelIntentV2",
    "ModelMessageV1",
    "ModelToolV1",
    "ModelTurnV2",
    "OracleTrustClassV1",
    "PatchCandidateV2",
    "PatchCustodyV1",
    "PolicyVersionV1",
    "ProducerEventV2",
    "ProviderEventV2",
    "ProviderResultV1",
    "PublicationEffectsV1",
    "PublicationOperationV1",
    "ResolutionChallengeV1",
    "ResourceEnvelopeV2",
    "RevocationRecordV1",
    "ScopeV1",
    "SignedEnvelopeV1",
    "TreeEntryV1",
    "TrustBundleV1",
    "TrustMemberV1",
    "TrustedDisplayReceiptV1",
    "VerificationReceiptV2",
    "VerificationSetV2",
    "VerifiedPublicationPlanV2",
    "WebAuthnCeremonyV1",
    "WebAuthnRegistrationCeremonyV1",
    "WorkflowSpecCoreV2",
];
pub fn field_order(type_name: &str) -> Option<&'static [&'static str]> {
    match type_name {
        "AcquisitionRequestV1" => Some(AcquisitionRequestV1::FIELD_ORDER),
        "AnalysisCompletionReceiptV1" => Some(AnalysisCompletionReceiptV1::FIELD_ORDER),
        "AnalysisPlanV1" => Some(AnalysisPlanV1::FIELD_ORDER),
        "AnalysisUnitV1" => Some(AnalysisUnitV1::FIELD_ORDER),
        "ApprovalChallengeV2" => Some(ApprovalChallengeV2::FIELD_ORDER),
        "ApprovalCredentialBindingV1" => Some(ApprovalCredentialBindingV1::FIELD_ORDER),
        "ApprovalEnrollmentActionV1" => Some(ApprovalEnrollmentActionV1::FIELD_ORDER),
        "ApprovalEnrollmentAuthorizationV1" => Some(ApprovalEnrollmentAuthorizationV1::FIELD_ORDER),
        "ApprovalEnrollmentChallengeV1" => Some(ApprovalEnrollmentChallengeV1::FIELD_ORDER),
        "ApprovalEnrollmentIntentV1" => Some(ApprovalEnrollmentIntentV1::FIELD_ORDER),
        "ApprovalLifecycleChallengeV1" => Some(ApprovalLifecycleChallengeV1::FIELD_ORDER),
        "ApprovalLifecycleConsentV1" => Some(ApprovalLifecycleConsentV1::FIELD_ORDER),
        "ApprovalRecoveryPolicyV1" => Some(ApprovalRecoveryPolicyV1::FIELD_ORDER),
        "ArtifactCustodyEntryV1" => Some(ArtifactCustodyEntryV1::FIELD_ORDER),
        "AttemptBindingV2" => Some(AttemptBindingV2::FIELD_ORDER),
        "AttemptEventV2" => Some(AttemptEventV2::FIELD_ORDER),
        "AuditEnvelopeV1" => Some(AuditEnvelopeV1::FIELD_ORDER),
        "AuthorityScopeV1" => Some(AuthorityScopeV1::FIELD_ORDER),
        "CapabilityProfileV1" => Some(CapabilityProfileV1::FIELD_ORDER),
        "ChangedObjectV1" => Some(ChangedObjectV1::FIELD_ORDER),
        "CheckOutputV1" => Some(CheckOutputV1::FIELD_ORDER),
        "CheckProjectionV2" => Some(CheckProjectionV2::FIELD_ORDER),
        "CheckWriteRequestV1" => Some(CheckWriteRequestV1::FIELD_ORDER),
        "CheckoutGrantV2" => Some(CheckoutGrantV2::FIELD_ORDER),
        "CommitIdentityV1" => Some(CommitIdentityV1::FIELD_ORDER),
        "CreateBlobRequestV1" => Some(CreateBlobRequestV1::FIELD_ORDER),
        "CreateCommitRequestV1" => Some(CreateCommitRequestV1::FIELD_ORDER),
        "CreateFixPrRequestV1" => Some(CreateFixPrRequestV1::FIELD_ORDER),
        "CreateRefRequestV1" => Some(CreateRefRequestV1::FIELD_ORDER),
        "CreateTreeRequestV1" => Some(CreateTreeRequestV1::FIELD_ORDER),
        "CurrentCandidateV1" => Some(CurrentCandidateV1::FIELD_ORDER),
        "DbStartCertificateV1" => Some(DbStartCertificateV1::FIELD_ORDER),
        "DependencyAcquisitionGrantV1" => Some(DependencyAcquisitionGrantV1::FIELD_ORDER),
        "DependencyPackageV1" => Some(DependencyPackageV1::FIELD_ORDER),
        "DescriptorEntryV1" => Some(DescriptorEntryV1::FIELD_ORDER),
        "EffectIntentV2" => Some(EffectIntentV2::FIELD_ORDER),
        "EffectPlanEntryV1" => Some(EffectPlanEntryV1::FIELD_ORDER),
        "EffectStateV1" => Some(EffectStateV1::FIELD_ORDER),
        "EligibilityProofV2" => Some(EligibilityProofV2::FIELD_ORDER),
        "EvaluationIdentityV2" => Some(EvaluationIdentityV2::FIELD_ORDER),
        "EvidenceReceiptV2" => Some(EvidenceReceiptV2::FIELD_ORDER),
        "ExpectedAnalysisManifestV1" => Some(ExpectedAnalysisManifestV1::FIELD_ORDER),
        "ExternalEffectJournalV1" => Some(ExternalEffectJournalV1::FIELD_ORDER),
        "FindingCandidateV2" => Some(FindingCandidateV2::FIELD_ORDER),
        "GateProofV2" => Some(GateProofV2::FIELD_ORDER),
        "GitOidV1" => Some(GitOidV1::FIELD_ORDER),
        "GitPublicationCapabilityV2" => Some(GitPublicationCapabilityV2::FIELD_ORDER),
        "HumanApprovalAssertionV2" => Some(HumanApprovalAssertionV2::FIELD_ORDER),
        "JournalEpochFenceV1" => Some(JournalEpochFenceV1::FIELD_ORDER),
        "JournalWitnessReceiptV1" => Some(JournalWitnessReceiptV1::FIELD_ORDER),
        "ManifestDescriptorV1" => Some(ManifestDescriptorV1::FIELD_ORDER),
        "ManualCaseStateV1" => Some(ManualCaseStateV1::FIELD_ORDER),
        "ManualResolutionActionV1" => Some(ManualResolutionActionV1::FIELD_ORDER),
        "ManualResolutionV1" => Some(ManualResolutionV1::FIELD_ORDER),
        "MergeSubjectV2" => Some(MergeSubjectV2::FIELD_ORDER),
        "ModelContentBlockV1" => Some(ModelContentBlockV1::FIELD_ORDER),
        "ModelContentV1" => Some(ModelContentV1::FIELD_ORDER),
        "ModelIntentV2" => Some(ModelIntentV2::FIELD_ORDER),
        "ModelMessageV1" => Some(ModelMessageV1::FIELD_ORDER),
        "ModelToolV1" => Some(ModelToolV1::FIELD_ORDER),
        "ModelTurnV2" => Some(ModelTurnV2::FIELD_ORDER),
        "OracleTrustClassV1" => Some(OracleTrustClassV1::FIELD_ORDER),
        "PatchCandidateV2" => Some(PatchCandidateV2::FIELD_ORDER),
        "PatchCustodyV1" => Some(PatchCustodyV1::FIELD_ORDER),
        "PolicyVersionV1" => Some(PolicyVersionV1::FIELD_ORDER),
        "ProducerEventV2" => Some(ProducerEventV2::FIELD_ORDER),
        "ProviderEventV2" => Some(ProviderEventV2::FIELD_ORDER),
        "ProviderResultV1" => Some(ProviderResultV1::FIELD_ORDER),
        "PublicationEffectsV1" => Some(PublicationEffectsV1::FIELD_ORDER),
        "PublicationOperationV1" => Some(PublicationOperationV1::FIELD_ORDER),
        "ResolutionChallengeV1" => Some(ResolutionChallengeV1::FIELD_ORDER),
        "ResourceEnvelopeV2" => Some(ResourceEnvelopeV2::FIELD_ORDER),
        "RevocationRecordV1" => Some(RevocationRecordV1::FIELD_ORDER),
        "ScopeV1" => Some(ScopeV1::FIELD_ORDER),
        "SignedEnvelopeV1" => Some(SignedEnvelopeV1::FIELD_ORDER),
        "TreeEntryV1" => Some(TreeEntryV1::FIELD_ORDER),
        "TrustBundleV1" => Some(TrustBundleV1::FIELD_ORDER),
        "TrustMemberV1" => Some(TrustMemberV1::FIELD_ORDER),
        "TrustedDisplayReceiptV1" => Some(TrustedDisplayReceiptV1::FIELD_ORDER),
        "VerificationReceiptV2" => Some(VerificationReceiptV2::FIELD_ORDER),
        "VerificationSetV2" => Some(VerificationSetV2::FIELD_ORDER),
        "VerifiedPublicationPlanV2" => Some(VerifiedPublicationPlanV2::FIELD_ORDER),
        "WebAuthnCeremonyV1" => Some(WebAuthnCeremonyV1::FIELD_ORDER),
        "WebAuthnRegistrationCeremonyV1" => Some(WebAuthnRegistrationCeremonyV1::FIELD_ORDER),
        "WorkflowSpecCoreV2" => Some(WorkflowSpecCoreV2::FIELD_ORDER),
        _ => None,
    }
}
pub fn round_trip_projection(
    value: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Error> {
    let name = value
        .get("type")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            <serde_json::Error as serde::de::Error>::custom("missing or invalid contract type")
        })?
        .to_owned();
    match name.as_str() {
        "AcquisitionRequestV1" => {
            serde_json::to_value(serde_json::from_value::<AcquisitionRequestV1>(value)?)
        }
        "AnalysisCompletionReceiptV1" => serde_json::to_value(serde_json::from_value::<
            AnalysisCompletionReceiptV1,
        >(value)?),
        "AnalysisPlanV1" => serde_json::to_value(serde_json::from_value::<AnalysisPlanV1>(value)?),
        "AnalysisUnitV1" => serde_json::to_value(serde_json::from_value::<AnalysisUnitV1>(value)?),
        "ApprovalChallengeV2" => {
            serde_json::to_value(serde_json::from_value::<ApprovalChallengeV2>(value)?)
        }
        "ApprovalCredentialBindingV1" => serde_json::to_value(serde_json::from_value::<
            ApprovalCredentialBindingV1,
        >(value)?),
        "ApprovalEnrollmentActionV1" => {
            serde_json::to_value(serde_json::from_value::<ApprovalEnrollmentActionV1>(value)?)
        }
        "ApprovalEnrollmentAuthorizationV1" => serde_json::to_value(serde_json::from_value::<
            ApprovalEnrollmentAuthorizationV1,
        >(value)?),
        "ApprovalEnrollmentChallengeV1" => serde_json::to_value(serde_json::from_value::<
            ApprovalEnrollmentChallengeV1,
        >(value)?),
        "ApprovalEnrollmentIntentV1" => {
            serde_json::to_value(serde_json::from_value::<ApprovalEnrollmentIntentV1>(value)?)
        }
        "ApprovalLifecycleChallengeV1" => serde_json::to_value(serde_json::from_value::<
            ApprovalLifecycleChallengeV1,
        >(value)?),
        "ApprovalLifecycleConsentV1" => {
            serde_json::to_value(serde_json::from_value::<ApprovalLifecycleConsentV1>(value)?)
        }
        "ApprovalRecoveryPolicyV1" => {
            serde_json::to_value(serde_json::from_value::<ApprovalRecoveryPolicyV1>(value)?)
        }
        "ArtifactCustodyEntryV1" => {
            serde_json::to_value(serde_json::from_value::<ArtifactCustodyEntryV1>(value)?)
        }
        "AttemptBindingV2" => {
            serde_json::to_value(serde_json::from_value::<AttemptBindingV2>(value)?)
        }
        "AttemptEventV2" => serde_json::to_value(serde_json::from_value::<AttemptEventV2>(value)?),
        "AuditEnvelopeV1" => {
            serde_json::to_value(serde_json::from_value::<AuditEnvelopeV1>(value)?)
        }
        "AuthorityScopeV1" => {
            serde_json::to_value(serde_json::from_value::<AuthorityScopeV1>(value)?)
        }
        "CapabilityProfileV1" => {
            serde_json::to_value(serde_json::from_value::<CapabilityProfileV1>(value)?)
        }
        "ChangedObjectV1" => {
            serde_json::to_value(serde_json::from_value::<ChangedObjectV1>(value)?)
        }
        "CheckOutputV1" => serde_json::to_value(serde_json::from_value::<CheckOutputV1>(value)?),
        "CheckProjectionV2" => {
            serde_json::to_value(serde_json::from_value::<CheckProjectionV2>(value)?)
        }
        "CheckWriteRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CheckWriteRequestV1>(value)?)
        }
        "CheckoutGrantV2" => {
            serde_json::to_value(serde_json::from_value::<CheckoutGrantV2>(value)?)
        }
        "CommitIdentityV1" => {
            serde_json::to_value(serde_json::from_value::<CommitIdentityV1>(value)?)
        }
        "CreateBlobRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CreateBlobRequestV1>(value)?)
        }
        "CreateCommitRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CreateCommitRequestV1>(value)?)
        }
        "CreateFixPrRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CreateFixPrRequestV1>(value)?)
        }
        "CreateRefRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CreateRefRequestV1>(value)?)
        }
        "CreateTreeRequestV1" => {
            serde_json::to_value(serde_json::from_value::<CreateTreeRequestV1>(value)?)
        }
        "CurrentCandidateV1" => {
            serde_json::to_value(serde_json::from_value::<CurrentCandidateV1>(value)?)
        }
        "DbStartCertificateV1" => {
            serde_json::to_value(serde_json::from_value::<DbStartCertificateV1>(value)?)
        }
        "DependencyAcquisitionGrantV1" => serde_json::to_value(serde_json::from_value::<
            DependencyAcquisitionGrantV1,
        >(value)?),
        "DependencyPackageV1" => {
            serde_json::to_value(serde_json::from_value::<DependencyPackageV1>(value)?)
        }
        "DescriptorEntryV1" => {
            serde_json::to_value(serde_json::from_value::<DescriptorEntryV1>(value)?)
        }
        "EffectIntentV2" => serde_json::to_value(serde_json::from_value::<EffectIntentV2>(value)?),
        "EffectPlanEntryV1" => {
            serde_json::to_value(serde_json::from_value::<EffectPlanEntryV1>(value)?)
        }
        "EffectStateV1" => serde_json::to_value(serde_json::from_value::<EffectStateV1>(value)?),
        "EligibilityProofV2" => {
            serde_json::to_value(serde_json::from_value::<EligibilityProofV2>(value)?)
        }
        "EvaluationIdentityV2" => {
            serde_json::to_value(serde_json::from_value::<EvaluationIdentityV2>(value)?)
        }
        "EvidenceReceiptV2" => {
            serde_json::to_value(serde_json::from_value::<EvidenceReceiptV2>(value)?)
        }
        "ExpectedAnalysisManifestV1" => {
            serde_json::to_value(serde_json::from_value::<ExpectedAnalysisManifestV1>(value)?)
        }
        "ExternalEffectJournalV1" => {
            serde_json::to_value(serde_json::from_value::<ExternalEffectJournalV1>(value)?)
        }
        "FindingCandidateV2" => {
            serde_json::to_value(serde_json::from_value::<FindingCandidateV2>(value)?)
        }
        "GateProofV2" => serde_json::to_value(serde_json::from_value::<GateProofV2>(value)?),
        "GitOidV1" => serde_json::to_value(serde_json::from_value::<GitOidV1>(value)?),
        "GitPublicationCapabilityV2" => {
            serde_json::to_value(serde_json::from_value::<GitPublicationCapabilityV2>(value)?)
        }
        "HumanApprovalAssertionV2" => {
            serde_json::to_value(serde_json::from_value::<HumanApprovalAssertionV2>(value)?)
        }
        "JournalEpochFenceV1" => {
            serde_json::to_value(serde_json::from_value::<JournalEpochFenceV1>(value)?)
        }
        "JournalWitnessReceiptV1" => {
            serde_json::to_value(serde_json::from_value::<JournalWitnessReceiptV1>(value)?)
        }
        "ManifestDescriptorV1" => {
            serde_json::to_value(serde_json::from_value::<ManifestDescriptorV1>(value)?)
        }
        "ManualCaseStateV1" => {
            serde_json::to_value(serde_json::from_value::<ManualCaseStateV1>(value)?)
        }
        "ManualResolutionActionV1" => {
            serde_json::to_value(serde_json::from_value::<ManualResolutionActionV1>(value)?)
        }
        "ManualResolutionV1" => {
            serde_json::to_value(serde_json::from_value::<ManualResolutionV1>(value)?)
        }
        "MergeSubjectV2" => serde_json::to_value(serde_json::from_value::<MergeSubjectV2>(value)?),
        "ModelContentBlockV1" => {
            serde_json::to_value(serde_json::from_value::<ModelContentBlockV1>(value)?)
        }
        "ModelContentV1" => serde_json::to_value(serde_json::from_value::<ModelContentV1>(value)?),
        "ModelIntentV2" => serde_json::to_value(serde_json::from_value::<ModelIntentV2>(value)?),
        "ModelMessageV1" => serde_json::to_value(serde_json::from_value::<ModelMessageV1>(value)?),
        "ModelToolV1" => serde_json::to_value(serde_json::from_value::<ModelToolV1>(value)?),
        "ModelTurnV2" => serde_json::to_value(serde_json::from_value::<ModelTurnV2>(value)?),
        "OracleTrustClassV1" => {
            serde_json::to_value(serde_json::from_value::<OracleTrustClassV1>(value)?)
        }
        "PatchCandidateV2" => {
            serde_json::to_value(serde_json::from_value::<PatchCandidateV2>(value)?)
        }
        "PatchCustodyV1" => serde_json::to_value(serde_json::from_value::<PatchCustodyV1>(value)?),
        "PolicyVersionV1" => {
            serde_json::to_value(serde_json::from_value::<PolicyVersionV1>(value)?)
        }
        "ProducerEventV2" => {
            serde_json::to_value(serde_json::from_value::<ProducerEventV2>(value)?)
        }
        "ProviderEventV2" => {
            serde_json::to_value(serde_json::from_value::<ProviderEventV2>(value)?)
        }
        "ProviderResultV1" => {
            serde_json::to_value(serde_json::from_value::<ProviderResultV1>(value)?)
        }
        "PublicationEffectsV1" => {
            serde_json::to_value(serde_json::from_value::<PublicationEffectsV1>(value)?)
        }
        "PublicationOperationV1" => {
            serde_json::to_value(serde_json::from_value::<PublicationOperationV1>(value)?)
        }
        "ResolutionChallengeV1" => {
            serde_json::to_value(serde_json::from_value::<ResolutionChallengeV1>(value)?)
        }
        "ResourceEnvelopeV2" => {
            serde_json::to_value(serde_json::from_value::<ResourceEnvelopeV2>(value)?)
        }
        "RevocationRecordV1" => {
            serde_json::to_value(serde_json::from_value::<RevocationRecordV1>(value)?)
        }
        "ScopeV1" => serde_json::to_value(serde_json::from_value::<ScopeV1>(value)?),
        "SignedEnvelopeV1" => {
            serde_json::to_value(serde_json::from_value::<SignedEnvelopeV1>(value)?)
        }
        "TreeEntryV1" => serde_json::to_value(serde_json::from_value::<TreeEntryV1>(value)?),
        "TrustBundleV1" => serde_json::to_value(serde_json::from_value::<TrustBundleV1>(value)?),
        "TrustMemberV1" => serde_json::to_value(serde_json::from_value::<TrustMemberV1>(value)?),
        "TrustedDisplayReceiptV1" => {
            serde_json::to_value(serde_json::from_value::<TrustedDisplayReceiptV1>(value)?)
        }
        "VerificationReceiptV2" => {
            serde_json::to_value(serde_json::from_value::<VerificationReceiptV2>(value)?)
        }
        "VerificationSetV2" => {
            serde_json::to_value(serde_json::from_value::<VerificationSetV2>(value)?)
        }
        "VerifiedPublicationPlanV2" => {
            serde_json::to_value(serde_json::from_value::<VerifiedPublicationPlanV2>(value)?)
        }
        "WebAuthnCeremonyV1" => {
            serde_json::to_value(serde_json::from_value::<WebAuthnCeremonyV1>(value)?)
        }
        "WebAuthnRegistrationCeremonyV1" => serde_json::to_value(serde_json::from_value::<
            WebAuthnRegistrationCeremonyV1,
        >(value)?),
        "WorkflowSpecCoreV2" => {
            serde_json::to_value(serde_json::from_value::<WorkflowSpecCoreV2>(value)?)
        }
        _ => Err(<serde_json::Error as serde::de::Error>::custom(
            "unregistered contract type",
        )),
    }
}
