# Contract authority and digest-reference catalog

This catalog is normative with REGISTRY.json/ENCODING.md. Every record is immutable once hashed. A state transition creates a new revision plus a protected current pointer. A signed row requires its producer key role, tenant/repository scope, epoch and current generation, not merely a mathematically valid signature. Embedded values inherit enclosing authority. Ordinary API/runner/agent has no trusted write or signing role. The complete per-input grant/lock matrix is07; security-sensitive administrative requests must use scoped trusted-origin UV and immutable audit, never an ordinary API database write.

## Producers and protected writers

| Contract | Authority |
|---|---|
| AnalysisCompletionReceiptV1 | Authenticated supervisor submits; evidence_ingest_writer validates; claim origin alone is not evidence authority |
| AnalysisPlanV1 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |
| AnalysisUnitV1 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |
| ApprovalChallengeV2 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalCredentialBindingV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalEnrollmentActionV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalEnrollmentAuthorizationV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalEnrollmentChallengeV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalEnrollmentIntentV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalLifecycleChallengeV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalLifecycleConsentV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ApprovalRecoveryPolicyV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| ArtifactCustodyEntryV1 | Custodian/verifier scoped receipt; custody_writer or untrusted patch intake; exact-byte checks mandatory |
| AttemptBindingV2 | Named authenticated producer in07; event/audit intake cannot promote caller role; Runner Control issues attempt binding |
| AttemptEventV2 | Named authenticated producer in07; event/audit intake cannot promote caller role; Runner Control issues attempt binding |
| AuditEnvelopeV1 | Named authenticated producer in07; event/audit intake cannot promote caller role; Runner Control issues attempt binding |
| AuthorityScopeV1 | Scoped Policy/Identity Authority; policy_writer or eligibility_writer; independently authorized actor ceremony |
| CapabilityProfileV1 | Security/Trust Authority; trust_writer or revocation_writer; independent role/domain admission |
| ChangedObjectV1 | Custodian/verifier scoped receipt; custody_writer or untrusted patch intake; exact-byte checks mandatory |
| AcquisitionRequestV1 | Checkout/Dependency Broker derives exact selected request; credential-free, admitted typed grant |
| CheckOutputV1 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| CheckProjectionV2 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| CheckWriteRequestV1 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| CheckoutGrantV2 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| CommitIdentityV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CreateBlobRequestV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CreateCommitRequestV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CreateFixPrRequestV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CreateRefRequestV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CreateTreeRequestV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| CurrentCandidateV1 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| DbStartCertificateV1 | Protected effect authority/gateway; effect_writer; no ordinary planner writes |
| DependencyAcquisitionGrantV1 | Dependency Broker; dependency_writer; exact grant/lock/policy and bounded credential dialer |
| DependencyPackageV1 | Dependency Broker; dependency_writer; exact grant/lock/policy and bounded credential dialer |
| DescriptorEntryV1 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |
| EffectIntentV2 | Protected effect authority/gateway; effect_writer; no ordinary planner writes |
| EffectPlanEntryV1 | Protected effect authority/gateway; effect_writer; no ordinary planner writes |
| EffectStateV1 | Protected effect authority/gateway; effect_writer; no ordinary planner writes |
| EligibilityProofV2 | Scoped Policy/Identity Authority; policy_writer or eligibility_writer; independently authorized actor ceremony |
| EvaluationIdentityV2 | Gate Authority; gate_writer; validates all upstream records and locked currentness |
| EvidenceReceiptV2 | Authenticated supervisor submits; evidence_ingest_writer validates; claim origin alone is not evidence authority |
| ExpectedAnalysisManifestV1 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |
| ExternalEffectJournalV1 | Journal sequencer and independent witnesses; journal-only append identities |
| FindingCandidateV2 | Authenticated supervisor submits; evidence_ingest_writer validates; claim origin alone is not evidence authority |
| GateProofV2 | Gate Authority; gate_writer; validates all upstream records and locked currentness |
| GitOidV1 | Embedded value or detached envelope; inherits enclosing record authority, never independently grants privilege |
| GitPublicationCapabilityV2 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| HumanApprovalAssertionV2 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| JournalEpochFenceV1 | Journal sequencer and independent witnesses; journal-only append identities |
| JournalWitnessReceiptV1 | Journal sequencer and independent witnesses; journal-only append identities |
| ManifestDescriptorV1 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |
| ManualResolutionActionV1 | Resolution Authority; resolution_writer; fresh scoped purpose-bound user verification |
| ManualCaseStateV1 | Resolution Authority; resolution_writer; immutable current-case snapshots |
| ManualResolutionV1 | Resolution Authority; resolution_writer; fresh scoped purpose-bound user verification |
| MergeSubjectV2 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| ModelContentBlockV1 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| ModelContentV1 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| ModelIntentV2 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| ModelMessageV1 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| ModelToolV1 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| ModelTurnV2 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| OracleTrustClassV1 | Authenticated supervisor submits; evidence_ingest_writer validates; claim origin alone is not evidence authority |
| PatchCandidateV2 | Custodian/verifier scoped receipt; custody_writer or untrusted patch intake; exact-byte checks mandatory |
| PatchCustodyV1 | Custodian/verifier scoped receipt; custody_writer or untrusted patch intake; exact-byte checks mandatory |
| PolicyVersionV1 | Scoped Policy/Identity Authority; policy_writer or eligibility_writer; independently authorized actor ceremony |
| ProducerEventV2 | Named authenticated producer in07; event/audit intake cannot promote caller role; Runner Control issues attempt binding |
| ProviderEventV2 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| ProviderResultV1 | Review/Provider Observation Authority; candidate_writer or projection_writer; read-back truth only |
| PublicationEffectsV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| PublicationOperationV1 | Protected effect authority/gateway; effect_writer; no ordinary planner writes |
| ResolutionChallengeV1 | Resolution Authority; resolution_writer; fresh scoped purpose-bound user verification |
| ResourceEnvelopeV2 | Model/Resource Broker; budget_writer; root accounting and current lease/eligibility |
| RevocationRecordV1 | Security/Trust Authority; trust_writer or revocation_writer; independent role/domain admission |
| ScopeV1 | Embedded value or detached envelope; inherits enclosing record authority, never independently grants privilege |
| SignedEnvelopeV1 | Embedded value or detached envelope; inherits enclosing record authority, never independently grants privilege |
| TreeEntryV1 | Verifier fixes request plan; gateway validates; publication_writer records exact per-effect authority |
| TrustBundleV1 | Security/Trust Authority; trust_writer or revocation_writer; independent role/domain admission |
| TrustMemberV1 | Security/Trust Authority; trust_writer or revocation_writer; independent role/domain admission |
| TrustedDisplayReceiptV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| VerificationReceiptV2 | Independent Verification Authority; verification_ingest_writer / verification_set_writer; plan_writer for verified plan |
| VerificationSetV2 | Independent Verification Authority; verification_ingest_writer / verification_set_writer; plan_writer for verified plan |
| VerifiedPublicationPlanV2 | Independent Verification Authority; verification_ingest_writer / verification_set_writer; plan_writer for verified plan |
| WebAuthnCeremonyV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| WebAuthnRegistrationCeremonyV1 | Approval Authority; approval_writer; offline keys only for bootstrap/recovery as52 |
| WorkflowSpecCoreV2 | Policy/Analysis Planning Authority; manifest_writer; exact policy-derived expected work |

## Exact digest references and resolver rules

For a named registered record, the digest is D(type,record) from ENCODING.md, not SHA-256 of its human JSON. A raw-artifact hash is plain SHA-256 of exact retained bytes, before any decoding/newline/Unicode conversion. Raw fields are identified below; they do not secretly encode an unsigned permission structure. Every reference resolver verifies type, byte size, tenant custody, producer role, currentness and hash. Missing/unavailable/unsupported reference rejects; it never becomes an empty/default permission. Cross-tenant references are forbidden even when bytes/hash happen to match.

| Referencing field(s) | Exact target / binding |
|---|---|
| subject_digest | MergeSubjectV2; RevocationRecordV1/AuditEnvelopeV1 subject_type explicitly names the registered revoked/affected record; ManualResolutionActionV1 subject names the exact finding/effect/credential selected by ManualCaseStateV1.subject_type, not necessarily a merge subject |
| workflow_digest | WorkflowSpecCoreV2 |
| attempt_binding_digest | AttemptBindingV2; immutable original allocation, current lease checked separately |
| analysis_plan_digest | AnalysisPlanV1; no back-reference to workflow/attempt/manifest |
| policy_digest / previous_digest on PolicyVersionV1 | PolicyVersionV1; previous is [] for genesis and [digest] otherwise |
| capability_digest | CapabilityProfileV1; TrustMember uses the admitted component profile |
| resource_envelope_digest / root_envelope_digest | ResourceEnvelopeV2 |
| eligibility_proof_digest | EligibilityProofV2; eligible=true AND linked_user_id equals action/admission actor, exact opener/policy restriction/current membership; permission to approve is not eligibility |
| trust_set_digest | TrustBundleV1; online revocation/current pointer still required |
| manifest_digest | ExpectedAnalysisManifestV1 in analysis receipts/gate/evaluation; in patch/plan/custody it is SHA-256 of exact canonical patch manifest bytes: UTF-8 LF-delimited lower-hex Git path, tab, mode (100644/100755/120000/DELETE), tab, lower-hex content SHA-256 or '-' for DELETE, tab, canonical decimal size, LF; sorted raw path bytes, unique path, no escaping/implicit normalization |
| scopes/required_files_symbols/allowed_exclusions/sandbox_requirements/coverage_expectations/effective_configuration_digest | ManifestDescriptorV1 of the matching closed kind; keys and exact value bytes are checked against policy-pinned configuration schema and analyzer profile before sealing; no arbitrary repository-defined key can disable a required unit |
| actual_configuration_digest / actual_scope_digest | Matching ManifestDescriptorV1 of ANALYZER_CONFIGURATION / SCOPES; byte-equality to sealed expected descriptor unless explicitly allowed by pinned coverage predicate |
| coverage_digest | ManifestDescriptorV1 kind COVERAGE_EXPECTATIONS containing observed counts/sets under the same pinned analyzer schema; verifier compares expected predicates, not merely receipt self-label |
| analyzer_bundle_digest / model_cell_digest / renderer_bundle_digest | SHA-256 of exact signed distribution manifest bytes pinned in the admitted capability; change invalidates capability evidence |
| environment_digest / input_digest / output_digest on evidence | SHA-256 of exact retained immutable execution environment manifest / input artifact / output artifact bytes; trusted supervisor independently verifies these against subject, analyzer bundle, configuration and coverage; a digest without those checks is unqualified |
| evidence_digest | EvidenceReceiptV2 |
| verification_set_digest | VerificationSetV2; member receipt_digests resolve VerificationReceiptV2; canonical set order, not arbitrary input order |
| gate_proof_digest | Optional/current GateProofV2 in check writes/projection; SUCCESS requires present PASS proof; absent never success |
| evaluation_identity_digest | EvaluationIdentityV2 only once inputs are complete; absent option allowed in pre-verdict CheckOutputV1 and CheckProjectionV2, with allocated evaluation_id |
| plan_digest | VerifiedPublicationPlanV2 |
| patch_digest / blob_digest / content_digest | SHA-256 of exact patch/blob/source-content bytes; ModelContentBlock arguments/content are raw exact UTF-8 JSON/text/tool result bytes in bounded custody, not audit |
| commit_metadata_digest / fix_pr_metadata_digest | CreateCommitRequestV1 / CreateFixPrRequestV1; exactly the same request digests listed in PublicationEffectsV1; no alternate metadata serializer |
| effects_digest / displayed_effects_digest | PublicationEffectsV1; steps are ordered topology and typed Create* request digests |
| custody_digest | PatchCustodyV1 immutable snapshot; referenced custody_id and plan/publication/patch/verification identities must agree |
| content_custody_digest / body_custody_digest / request_custody_digest | ArtifactCustodyEntryV1 identifying exact bytes/size/encrypted scoped handle; ModelMessage content_custody_digest directly hashes ModelContentV1 whose referenced content is separately pinned; no arbitrary remote URL |
| approval_digest / approval_assertion_digest | HumanApprovalAssertionV2 |
| display_receipt_digest | TrustedDisplayReceiptV1 |
| challenge_digest | ApprovalChallengeV2 for publication, ResolutionChallengeV1 for manual resolution, ApprovalLifecycleChallengeV1 for owner/admin consents, ApprovalEnrollmentChallengeV1 for create ceremony; consuming endpoint pins one expected type/domain, no cross-purpose fallback |
| ceremony_digest / authentication_ceremony_digest | WebAuthnCeremonyV1 exact authentication response; received transport bytes validated without JSON reserialization |
| registration_ceremony_digest | WebAuthnRegistrationCeremonyV1; absent only for lifecycle operations that create no key |
| action_digest | ApprovalEnrollmentActionV1 for lifecycle, ManualResolutionActionV1 for resolution; exact challenge/endpoint selects type |
| intent_digest on enrollment action | ApprovalEnrollmentIntentV1 |
| authorization_scope_digest / actor_scope_digest / approver_scope_digest | AuthorityScopeV1; repository/effect permission checked against actual action; no empty-set wildcard |
| recovery_policy_digest | ApprovalRecoveryPolicyV1; operation-specific matrix52, not a general two-admin requirement |
| consent_receipt_digests | Signed ApprovalLifecycleConsentV1 payloads, with matching detached AA envelopes retained |
| authority_signature_envelope_digests | SignedEnvelopeV1 of exact enrollment action by independently registered offline bootstrap/recovery keys; not ordinary account keys |
| source_digest / applicability_digest on oracle | Exact raw requirement/test/specification artifact SHA-256 / canonical ManifestDescriptorV1 SCOPES descriptor naming applicable candidate/rule; trusted evidence authority validates provenance from protected policy history, never candidate-only assertion |
| approval_record_digest on MAINTAINER_APPROVED | ManualResolutionV1 containing action kind UNTRUSTED_ORACLE and decision PROVIDE_ORACLE, exact candidate/requirement/test, active scoped resolver, unexpired applicability; publication assertion cannot substitute |
| canonical_request_digest / request_digest on effect, capability or result | Closed effect table below; same scoped exact bytes everywhere |
| authorization_proof_digest on EffectIntentV2 | GitPublicationCapabilityV2 for five Git publication effects; ModelIntentV2 for MODEL_REQUEST; CheckoutGrantV2 for CHECKOUT_FETCH; DependencyAcquisitionGrantV1 for DEPENDENCY_FETCH; signed desired CheckProjectionV2 for CREATE_CHECK/UPDATE_CHECK |
| intent_digest on effect state/journal/start certificate | EffectIntentV2 |
| state_digest / prior_state_digest | EffectStateV1 for effect/journal fields; prior [] only first revision; revision increments and pointer CAS verifies prior chain. ManualResolutionActionV1.prior_state_digest instead references ManualCaseStateV1 and its exact current case revision |
| provider_result_digest | ProviderResultV1 and retained authenticated exact readback evidence; positive matching identity required |
| db_start_certificate_digest | DbStartCertificateV1; only SEND_ARMED may reference a committed start certificate; required there |
| authorization_snapshot_digest | Same authorization proof as EffectIntentV2 plus currentness checked at recorded committed_at; certificate signed after DB commit, not a caller-provided snapshot |
| previous_digest / entry_digest on journal | D(actual registered chain-entry type,entry), ExternalEffectJournalV1 or JournalEpochFenceV1; zero32 previous digest only genesis; no chain gap permitted |
| previous_digest on audit | AuditEnvelopeV1; zero32 only genesis; immutable per-stream chain, no reuse of another stream |
| payload_digest | Registered payload named by event/envelope contract_type; producer role checked for that payload, not merely event envelope |
| observation/readback/old_sender_fencing_evidence/provider_ownership_evidence_digest | SHA-256 of exact retained signed evidence-bundle bytes supplied by protected provider/recovery authority; evidence collector and independent reviewer verify actual source identity, currentness, complete coverage and declared proof predicate in30/35/50/52; arbitrary bytes with a digest cannot satisfy predicate |
| requirements_digest / analyzer_configuration_schema_digest / access_policy_digest / retention_policy_digest / endpoint_allowlist_digest / cache_scope_digest / resource_policy_digest / sampling_parameters_digest | SHA-256 of exact UTF-8 JSON profile bytes using no duplicate keys and no numeric floating values; Policy Authority pins a schema and all semantic settings before use. Requirements include mandatory units, allowed exclusions, oracle/provenance and independence minima; retention includes fixed lifetime/region/deletion rules; endpoints enumerate exact registry origin/path/port/TLS identity and redirect/DNS restrictions; cache scope names tenant and registry; model sampling has closed adapter-supported keys only. Any absent or unsupported profile/schema fails UNSUPPORTED. These profiles cannot override invariant minima or closed wire fields |
| lockfile_digest / lockfile_entry_digest / provenance_digest | SHA-256 of exact original lockfile bytes / exact resolved lock entry bytes / retained authenticated provenance bytes, all validated by pinned ecosystem adapter before credential acquisition |
| reason_digest / evidence_digests on manual action | Exact raw rationale SHA-256 (protected custody, never audit text) / type-tagged artifacts in case evidence inventory with hashes and verifier identity; resolver must actually fetch/validate evidence before action challenge |

## Closed effect-request mapping

CREATE_BLOB→CreateBlobRequestV1, CREATE_TREE→CreateTreeRequestV1, CREATE_COMMIT→CreateCommitRequestV1, CREATE_REF→CreateRefRequestV1, CREATE_FIX_PR→CreateFixPrRequestV1, CREATE_CHECK/UPDATE_CHECK→CheckWriteRequestV1, MODEL_REQUEST→ModelTurnV2, CHECKOUT_FETCH/DEPENDENCY_FETCH→AcquisitionRequestV1, whose grant_digest names the separately signed CheckoutGrantV2/DependencyAcquisitionGrantV1. Each actual HTTP operation has its own exact typed origin/method/path/body/selection/request digest; an aggregate grant is not the canonical request digest and does not allow a request outside its set or multiple sends under one effect. Operational request material is retained in request_custody_digest; exact raw HTTP/provider JSON bytes hash must match the approved grant selection and adapter canonical construction before STARTED. A different endpoint/body/selected package requires a new effect; transparent redirect/retry is forbidden.

## Acyclic lifecycle order

Policy/trust/capability → subject and eligibility → root envelope and logical analysis plan → workflow → allocated attempt → expected manifest → completion/evidence/verification → finalized evaluation identity and gate proof. `evaluation_id` can be allocated early as a UUID; EvaluationIdentityV2 payload is finalized only after manifest/verification set exists, never filled with invented zero digests. Early CreateReviewIntent produces that UUID/workflow intent, not a precomputed final evaluation payload.

Publication request objects and PublicationEffectsV1 → verified plan (custody ID only) → immutable pinned custody snapshot → trusted display → approval challenge → WebAuthn ceremony → human assertion → operation/effect capability → effect intent → state/start certificate → send journal → provider observation. Check desired projection authorization excludes future effect ID (absent option) and observation timestamps; per-effect linkage is recorded in subsequent projection snapshots, avoiding a request↔projection digest cycle. Model turn references model_intent_id UUID, not authorization digest; its request digest can be computed before ModelIntentV2.

Registration: intent → create challenge → new-key ceremony → final action → owner/admin lifecycle challenge → consent receipts/offline signatures → authorization → binding/audit. Manual resolution: proposed action → challenge → ceremony → signed resolution. Audit IDs may be allocated before payloads but no digest points back recursively. Resolver must reject cyclic or unresolved reference graphs.

