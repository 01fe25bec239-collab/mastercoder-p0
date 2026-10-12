# Astra finding resolution matrix

Status is architecture disposition only; all required prototype evidence remains NOT_RUN. Every accepted finding is retained. Finding14 clarifies an existing product limitation. Actual independent-review closure is recorded separately in REVIEWS; no author self-approval is implied.

## Finding 1 — CRITICAL — Publisher create-only authority

- V2 defect: 08 Git identities and ADR-003 call publisher create-only although provider Contents credential permits broader ref mutation.
- V2.1 repair: Separate credential-free planner and independently administered typed gateway; per-effect exact body/ref/capability validation, no-bypass provider posture; gateway/token residual risk stated.
- Documents: [08_GIT_INTEGRATION.md](../08_GIT_INTEGRATION.md), [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [15_IDENTITY_SECRETS.md](../15_IDENTITY_SECRETS.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md](../27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md), [53_GIT_PUBLICATION_CAPABILITY.md](../53_GIT_PUBLICATION_CAPABILITY.md)
- ADRs: [ADRs/ADR-016-GIT-PUBLICATION-CAPABILITY.md](../ADRs/ADR-016-GIT-PUBLICATION-CAPABILITY.md)
- Contracts: GitPublicationCapabilityV2 VerifiedPublicationPlanV2 EffectIntentV2
- State machines: PublicationOperation ExternalEffect
- Atomic prototype requirements: P0-09D P0-09E (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 2 — CRITICAL — Stale provider success

- V2 defect: 08 pre-write check and30 freshness omit withdrawal of already successful provider checks.
- V2.1 repair: Generation-fenced projection/readback/withdrawal; serialize unresolved old writes; shared-SHA association guard; staged normal changes versus emergency unsafe interval and confirmed freeze.
- Documents: [08_GIT_INTEGRATION.md](../08_GIT_INTEGRATION.md), [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [17_PLATFORM_RELIABILITY.md](../17_PLATFORM_RELIABILITY.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [25_PR_SEQUENCE_DIAGRAMS.md](../25_PR_SEQUENCE_DIAGRAMS.md), [30_EXACT_GIT_LINEAGE.md](../30_EXACT_GIT_LINEAGE.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [48_COMPONENT_REVOCATION_POLICY.md](../48_COMPONENT_REVOCATION_POLICY.md), [54_CHECK_PROJECTION_LIFECYCLE.md](../54_CHECK_PROJECTION_LIFECYCLE.md)
- ADRs: [ADRs/ADR-017-CHECK-PROJECTION-REVOCATION.md](../ADRs/ADR-017-CHECK-PROJECTION-REVOCATION.md)
- Contracts: CheckProjectionV2 CheckWriteRequestV1 GateProofV2
- State machines: Evaluation GitCheckProjection ExternalEffect
- Atomic prototype requirements: P0-03B P0-03C P0-03D P0-03E (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 3 — HIGH — Trusted approval display

- V2 defect: 14/16 ordinary UI display plus WebAuthn does not establish displayed patch equals signed plan.
- V2.1 repair: Independent AA origin/session fetches and renders canonical pinned diff/effects; purpose-bound challenge; browser/OS residual explicit.
- Documents: [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [15_IDENTITY_SECRETS.md](../15_IDENTITY_SECRETS.md), [16_NEXTJS_REACT_WEB_UX.md](../16_NEXTJS_REACT_WEB_UX.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [24_PRIVACY_DATA_CLASSIFICATION.md](../24_PRIVACY_DATA_CLASSIFICATION.md), [27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md](../27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md), [52_APPROVAL_AUTHORITY_CONTRACT.md](../52_APPROVAL_AUTHORITY_CONTRACT.md)
- ADRs: [ADRs/ADR-018-TRUSTED-APPROVAL-AUTHORITY.md](../ADRs/ADR-018-TRUSTED-APPROVAL-AUTHORITY.md)
- Contracts: TrustedDisplayReceiptV1 ApprovalChallengeV2 HumanApprovalAssertionV2
- State machines: Patch Approval
- Atomic prototype requirements: P0-09A P0-09C (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 4 — HIGH — Authenticator lifecycle authority

- V2 defect: 15 has no independent add/replace/recovery authority and registration proof contract.
- V2.1 repair: Constructible intent-create-action-authority-signature enrollment chain; scoped credential generations, dual-admin or offline bootstrap, ordinary recovery excluded.
- Documents: [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [15_IDENTITY_SECRETS.md](../15_IDENTITY_SECRETS.md), [16_NEXTJS_REACT_WEB_UX.md](../16_NEXTJS_REACT_WEB_UX.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [48_COMPONENT_REVOCATION_POLICY.md](../48_COMPONENT_REVOCATION_POLICY.md), [52_APPROVAL_AUTHORITY_CONTRACT.md](../52_APPROVAL_AUTHORITY_CONTRACT.md)
- ADRs: [ADRs/ADR-018-TRUSTED-APPROVAL-AUTHORITY.md](../ADRs/ADR-018-TRUSTED-APPROVAL-AUTHORITY.md)
- Contracts: ApprovalCredentialBindingV1 ApprovalEnrollmentIntentV1 ApprovalEnrollmentAuthorizationV1
- State machines: Approval Credential Recovery
- Atomic prototype requirements: P0-09B P0-10I (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 5 — HIGH — Signer input authority

- V2 defect: 06/18 name verified records without protected producer/write/consumer matrix; ordinary API boundary underspecified.
- V2.1 repair: Nine gate-input rows plus manifest/eligibility have authenticated producer, immutable signature, dedicated DB writer, tenant/attempt/epoch binding and consumer revalidation.
- Documents: [06_RUST_CONTROL_PLANE.md](../06_RUST_CONTROL_PLANE.md), [07_POSTGRES_PERSISTENCE_WORKFLOW.md](../07_POSTGRES_PERSISTENCE_WORKFLOW.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [21_EVENT_CONTRACTS.md](../21_EVENT_CONTRACTS.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [29_CONCURRENCY_FENCING.md](../29_CONCURRENCY_FENCING.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md)
- ADRs: [ADRs/ADR-019-GATE-INPUT-AUTHORITY-AND-DATABASE-WRITERS.md](../ADRs/ADR-019-GATE-INPUT-AUTHORITY-AND-DATABASE-WRITERS.md)
- Contracts: EvidenceReceiptV2 VerificationReceiptV2 VerificationSetV2 GateProofV2 PolicyVersionV1 TrustBundleV1
- State machines: Evaluation Verification JobAttempt
- Atomic prototype requirements: P0-07B P0-10A P0-10I (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 6 — HIGH — Complete required analysis

- V2 defect: 09/13 PASS can confuse no finding with analyzer did not run.
- V2.1 repair: Policy-derived logical plan, allocation then immutable attempt-bound expected manifest; mandatory matching completion and coverage including zero findings.
- Documents: [09_ANALYSIS_ENGINE.md](../09_ANALYSIS_ENGINE.md), [13_REVIEW_VERIFICATION.md](../13_REVIEW_VERIFICATION.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [21_EVENT_CONTRACTS.md](../21_EVENT_CONTRACTS.md), [25_PR_SEQUENCE_DIAGRAMS.md](../25_PR_SEQUENCE_DIAGRAMS.md), [26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md](../26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md), [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md), [51_ANALYSIS_COVERAGE_CONTRACT.md](../51_ANALYSIS_COVERAGE_CONTRACT.md)
- ADRs: [ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md](../ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md)
- Contracts: AnalysisPlanV1 ExpectedAnalysisManifestV1 AnalysisCompletionReceiptV1
- State machines: Evaluation JobAttempt Verification
- Atomic prototype requirements: P0-07A P0-10E (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 7 — HIGH — Agent oracle validity

- V2 defect: 09 final paragraph/13 E3 allow causal reproduction to stand in for requirement validity.
- V2.1 repair: Independent oracle trust classification plus applicability and causal reproduction; untrusted agent expectations never block; typed maintainer resolution purpose.
- Documents: [09_ANALYSIS_ENGINE.md](../09_ANALYSIS_ENGINE.md), [13_REVIEW_VERIFICATION.md](../13_REVIEW_VERIFICATION.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [23_THREAT_MODEL.md](../23_THREAT_MODEL.md), [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md), [51_ANALYSIS_COVERAGE_CONTRACT.md](../51_ANALYSIS_COVERAGE_CONTRACT.md), [56_MANUAL_RESOLUTION.md](../56_MANUAL_RESOLUTION.md)
- ADRs: [ADRs/ADR-021-ORACLE-TRUST.md](../ADRs/ADR-021-ORACLE-TRUST.md)
- Contracts: OracleTrustClassV1 EvidenceReceiptV2 ManualResolutionActionV1
- State machines: Verification Finding Evaluation
- Atomic prototype requirements: P0-07C P0-10J (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 8 — HIGH — Actual lease expiry

- V2 defect: 07 Work acquisition and29 CAS predicates omit expiry; reaper timing controls acceptance.
- V2.1 repair: Lock then sample database clock_timestamp; owner/generation/epoch/state and strictly unexpired predicate on heartbeat/completion/artifact/verification/model authorization.
- Documents: [06_RUST_CONTROL_PLANE.md](../06_RUST_CONTROL_PLANE.md), [07_POSTGRES_PERSISTENCE_WORKFLOW.md](../07_POSTGRES_PERSISTENCE_WORKFLOW.md), [12_MODEL_ACCESS.md](../12_MODEL_ACCESS.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [25_PR_SEQUENCE_DIAGRAMS.md](../25_PR_SEQUENCE_DIAGRAMS.md), [26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md](../26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md), [28_FAILURE_RECOVERY.md](../28_FAILURE_RECOVERY.md), [29_CONCURRENCY_FENCING.md](../29_CONCURRENCY_FENCING.md), [50_EXTERNAL_EFFECT_RECOVERY.md](../50_EXTERNAL_EFFECT_RECOVERY.md), [53_GIT_PUBLICATION_CAPABILITY.md](../53_GIT_PUBLICATION_CAPABILITY.md)
- ADRs: [ADRs/ADR-022-ACTUAL-LEASE-EXPIRATION.md](../ADRs/ADR-022-ACTUAL-LEASE-EXPIRATION.md)
- Contracts: AttemptBindingV2 AnalysisCompletionReceiptV1 ModelIntentV2
- State machines: Job JobAttempt Verification ExternalEffect
- Atomic prototype requirements: P0-08A (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 9 — HIGH — Lost external intents after restore

- V2 defect: 35 scans surviving STARTED/UNKNOWN rows despite nonzero RPO, missing whole lost intents.
- V2.1 repair: Synchronous two-copy independent-restore-domain intent/send journal, fail-closed unconfirmed durability, complete restore enumeration; no restored entitlement reused without fresh security bootstrap.
- Documents: [07_POSTGRES_PERSISTENCE_WORKFLOW.md](../07_POSTGRES_PERSISTENCE_WORKFLOW.md), [17_PLATFORM_RELIABILITY.md](../17_PLATFORM_RELIABILITY.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [28_FAILURE_RECOVERY.md](../28_FAILURE_RECOVERY.md), [29_CONCURRENCY_FENCING.md](../29_CONCURRENCY_FENCING.md), [34_OBSERVABILITY.md](../34_OBSERVABILITY.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [48_COMPONENT_REVOCATION_POLICY.md](../48_COMPONENT_REVOCATION_POLICY.md), [50_EXTERNAL_EFFECT_RECOVERY.md](../50_EXTERNAL_EFFECT_RECOVERY.md)
- ADRs: [ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md](../ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md)
- Contracts: ExternalEffectJournalV1 JournalWitnessReceiptV1 JournalEpochFenceV1 EffectIntentV2
- State machines: ExternalEffect ManualReconciliation Recovery
- Atomic prototype requirements: P0-10G P0-10H P0-10I (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 10 — HIGH — Weak absence proof

- V2 defect: 19 and29 permit ABSENT_CONFIRMED without excluding late completion.
- V2.1 repair: No absence state; exact positive result or strong terminal nonexecution/qualified idempotency proof; otherwise UNKNOWN/manual tombstone with no conflicting retry.
- Documents: [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [28_FAILURE_RECOVERY.md](../28_FAILURE_RECOVERY.md), [29_CONCURRENCY_FENCING.md](../29_CONCURRENCY_FENCING.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [50_EXTERNAL_EFFECT_RECOVERY.md](../50_EXTERNAL_EFFECT_RECOVERY.md), [56_MANUAL_RESOLUTION.md](../56_MANUAL_RESOLUTION.md)
- ADRs: [ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md](../ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md), [ADRs/ADR-027-AUTHORIZED-MANUAL-RESOLUTION-WITHOUT-EVIDENCE-PROMOTION.md](../ADRs/ADR-027-AUTHORIZED-MANUAL-RESOLUTION-WITHOUT-EVIDENCE-PROMOTION.md)
- Contracts: EffectIntentV2 EffectStateV1 ManualResolutionV1 ProviderResultV1
- State machines: ExternalEffect ManualReconciliation
- Atomic prototype requirements: P0-09H P0-10J (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 11 — HIGH — Parent versus individual effect

- V2 defect: 14 post-STARTED rule and18 EffectIntent conflate entire publication with single send.
- V2.1 repair: Parent orchestration separate from immutable per-call intent and state revisions; every child freshly authorized, witnessed and deadline-bounded.
- Documents: [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md](../27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md), [28_FAILURE_RECOVERY.md](../28_FAILURE_RECOVERY.md), [29_CONCURRENCY_FENCING.md](../29_CONCURRENCY_FENCING.md), [50_EXTERNAL_EFFECT_RECOVERY.md](../50_EXTERNAL_EFFECT_RECOVERY.md), [53_GIT_PUBLICATION_CAPABILITY.md](../53_GIT_PUBLICATION_CAPABILITY.md)
- ADRs: [ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md](../ADRs/ADR-023-PER-EFFECT-WITNESSED-JOURNAL-AND-RECOVERY.md)
- Contracts: PublicationOperationV1 EffectIntentV2 EffectStateV1 DbStartCertificateV1
- State machines: PublicationOperation ExternalEffect
- Atomic prototype requirements: P0-09G P0-10H (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 12 — HIGH — Complete normative wire contracts

- V2 defect: 18/20/21/ADR-005 omit complete fixed-array layouts, domain/version/optional rules and vectors despite freeze claim.
- V2.1 repair: Machine-readable complete ordered registry, strict JSON projections, exact CBOR profile, detached signature inputs and positive/rejection vectors; implementation interoperability remains P0.
- Documents: [18_DATA_MODEL.md](../18_DATA_MODEL.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [21_EVENT_CONTRACTS.md](../21_EVENT_CONTRACTS.md), [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md), [47_COMPATIBILITY_MATRIX.md](../47_COMPATIBILITY_MATRIX.md), [49_CANONICAL_PROTOCOL_GLOSSARY.md](../49_CANONICAL_PROTOCOL_GLOSSARY.md), [50_EXTERNAL_EFFECT_RECOVERY.md](../50_EXTERNAL_EFFECT_RECOVERY.md), [51_ANALYSIS_COVERAGE_CONTRACT.md](../51_ANALYSIS_COVERAGE_CONTRACT.md), [52_APPROVAL_AUTHORITY_CONTRACT.md](../52_APPROVAL_AUTHORITY_CONTRACT.md), [53_GIT_PUBLICATION_CAPABILITY.md](../53_GIT_PUBLICATION_CAPABILITY.md), [54_CHECK_PROJECTION_LIFECYCLE.md](../54_CHECK_PROJECTION_LIFECYCLE.md), [55_DEPENDENCY_ACQUISITION.md](../55_DEPENDENCY_ACQUISITION.md), [56_MANUAL_RESOLUTION.md](../56_MANUAL_RESOLUTION.md)
- ADRs: [ADRs/ADR-024-NORMATIVE-WIRE-SCHEMA.md](../ADRs/ADR-024-NORMATIVE-WIRE-SCHEMA.md)
- Contracts: ALL_REGISTERED_CONTRACTS
- State machines: ALL_AFFECTED_STATE_MACHINES
- Atomic prototype requirements: P0-06A P0-06B P0-06C P0-06D (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 13 — MEDIUM — Patch-byte custody

- V2 defect: 24 ephemeral P3 versus14 later display/replay has no pin through approval/publication.
- V2.1 repair: Encrypted scoped exact-byte custody and pins across verification/display/publication/reconciliation; loss/expiry invalidates unstarted publication, no alternate reconstruction.
- Documents: [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [16_NEXTJS_REACT_WEB_UX.md](../16_NEXTJS_REACT_WEB_UX.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [24_PRIVACY_DATA_CLASSIFICATION.md](../24_PRIVACY_DATA_CLASSIFICATION.md), [27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md](../27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md), [30_EXACT_GIT_LINEAGE.md](../30_EXACT_GIT_LINEAGE.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [52_APPROVAL_AUTHORITY_CONTRACT.md](../52_APPROVAL_AUTHORITY_CONTRACT.md), [53_GIT_PUBLICATION_CAPABILITY.md](../53_GIT_PUBLICATION_CAPABILITY.md)
- ADRs: [ADRs/ADR-025-PATCH-BYTE-CUSTODY.md](../ADRs/ADR-025-PATCH-BYTE-CUSTODY.md)
- Contracts: PatchCustodyV1 ArtifactCustodyEntryV1 VerifiedPublicationPlanV2
- State machines: Patch Approval PublicationOperation ExternalEffect
- Atomic prototype requirements: P0-09A P0-09F P0-10C (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 14 — MEDIUM — C3 deployment availability

- V2 defect: 13 C3 already requires independent admin domain; small deployment limitation needs explicit product profile.
- V2.1 repair: Clarify individual/team/independent-pool achievable classes; single-domain at most C2; ordinary deterministic policies usable, C3-required category non-pass without independent pool.
- Documents: [10_RUNNER_SANDBOX.md](../10_RUNNER_SANDBOX.md), [13_REVIEW_VERIFICATION.md](../13_REVIEW_VERIFICATION.md), [31_CROSS_PLATFORM.md](../31_CROSS_PLATFORM.md), [32_DEPLOYMENT_TOPOLOGY.md](../32_DEPLOYMENT_TOPOLOGY.md), [37_V1_SCOPE.md](../37_V1_SCOPE.md), [47_COMPATIBILITY_MATRIX.md](../47_COMPATIBILITY_MATRIX.md)
- ADRs: [ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md](../ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md), [ADRs/ADR-021-ORACLE-TRUST.md](../ADRs/ADR-021-ORACLE-TRUST.md)
- Contracts: CapabilityProfileV1 VerificationReceiptV2 OracleTrustClassV1
- State machines: Verification Evaluation
- Atomic prototype requirements: P0-07D (NOT_RUN)
- Final status: **CLARIFIED_EXISTING_LIMITATION**

## Finding 15 — MEDIUM — Bounded authenticated dependencies

- V2 defect: 10 denies package credentials and36 pins locks but lacks realistic acquisition path.
- V2.1 repair: Credential-free resolution plus bounded credential-bearing acquisition, exact pinned closure/provenance, approved endpoints/cache scope, offline installation and sandbox-only scripts.
- Documents: [10_RUNNER_SANDBOX.md](../10_RUNNER_SANDBOX.md), [11_AGENT_HARNESS_GRAPH.md](../11_AGENT_HARNESS_GRAPH.md), [12_MODEL_ACCESS.md](../12_MODEL_ACCESS.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [24_PRIVACY_DATA_CLASSIFICATION.md](../24_PRIVACY_DATA_CLASSIFICATION.md), [31_CROSS_PLATFORM.md](../31_CROSS_PLATFORM.md), [32_DEPLOYMENT_TOPOLOGY.md](../32_DEPLOYMENT_TOPOLOGY.md), [36_SUPPLY_CHAIN.md](../36_SUPPLY_CHAIN.md), [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md), [55_DEPENDENCY_ACQUISITION.md](../55_DEPENDENCY_ACQUISITION.md)
- ADRs: [ADRs/ADR-026-DEPENDENCY-ACQUISITION.md](../ADRs/ADR-026-DEPENDENCY-ACQUISITION.md)
- Contracts: DependencyAcquisitionGrantV1 DependencyPackageV1
- State machines: JobAttempt ExternalEffect
- Atomic prototype requirements: P0-05E P0-05F P0-05G P0-05H (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 16 — MEDIUM — Authorized manual resolution

- V2 defect: 19/28 name human and manual states without complete scoped evidence/expiry/resolution flows.
- V2.1 repair: Trusted purpose-bound resolution, fresh UV, evidence and nonce; exact-result/nonexecution proof, unresolved tombstones; no direct PASS and new evaluation after policy/oracle changes.
- Documents: [14_PATCH_APPROVAL_PUBLICATION.md](../14_PATCH_APPROVAL_PUBLICATION.md), [16_NEXTJS_REACT_WEB_UX.md](../16_NEXTJS_REACT_WEB_UX.md), [18_DATA_MODEL.md](../18_DATA_MODEL.md), [19_STATE_MACHINES.md](../19_STATE_MACHINES.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [21_EVENT_CONTRACTS.md](../21_EVENT_CONTRACTS.md), [28_FAILURE_RECOVERY.md](../28_FAILURE_RECOVERY.md), [35_DISASTER_RECOVERY.md](../35_DISASTER_RECOVERY.md), [56_MANUAL_RESOLUTION.md](../56_MANUAL_RESOLUTION.md)
- ADRs: [ADRs/ADR-027-AUTHORIZED-MANUAL-RESOLUTION-WITHOUT-EVIDENCE-PROMOTION.md](../ADRs/ADR-027-AUTHORIZED-MANUAL-RESOLUTION-WITHOUT-EVIDENCE-PROMOTION.md)
- Contracts: ManualResolutionActionV1 ResolutionChallengeV1 ManualResolutionV1
- State machines: Evaluation Finding ManualReconciliation ExternalEffect
- Atomic prototype requirements: P0-10J (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 17 — MEDIUM — Deterministic work DAG

- V2 defect: 05 MODEL→ANALYSIS and39 MODEL→VER incorrectly block deterministic review on model integration.
- V2.1 repair: Separate deterministic FAST DAG from optional model-assisted AI/HYBRID/DEEP extension; retain required security/capability dependencies.
- Documents: [05_DEPENDENCY_DAG.md](../05_DEPENDENCY_DAG.md), [09_ANALYSIS_ENGINE.md](../09_ANALYSIS_ENGINE.md), [11_AGENT_HARNESS_GRAPH.md](../11_AGENT_HARNESS_GRAPH.md), [12_MODEL_ACCESS.md](../12_MODEL_ACCESS.md), [13_REVIEW_VERIFICATION.md](../13_REVIEW_VERIFICATION.md), [37_V1_SCOPE.md](../37_V1_SCOPE.md), [38_IMPLEMENTATION_DECOMPOSITION.md](../38_IMPLEMENTATION_DECOMPOSITION.md), [39_WORK_ITEM_DAG.md](../39_WORK_ITEM_DAG.md), [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md)
- ADRs: [ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md](../ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md)
- Contracts: WorkflowSpecCoreV2 AnalysisPlanV1
- State machines: Review Evaluation
- Atomic prototype requirements: P0-07A (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 18 — MEDIUM — Atomic reproducible P0 claims

- V2 defect: 42 umbrella groups omit individual environments/injections/observations/thresholds/review/expiry.
- V2.1 repair: Retain ten P0 groups;55 uniquely identified atomic claims with every required specification field; exact run-manifest seal is precondition, all NOT_RUN.
- Documents: [40_TEST_ARCHITECTURE.md](../40_TEST_ARCHITECTURE.md), [41_SECURITY_VALIDATION.md](../41_SECURITY_VALIDATION.md), [42_PROTOTYPE_GATES.md](../42_PROTOTYPE_GATES.md), [47_COMPATIBILITY_MATRIX.md](../47_COMPATIBILITY_MATRIX.md), [57_ATOMIC_PROTOTYPE_CLAIMS.md](../57_ATOMIC_PROTOTYPE_CLAIMS.md)
- ADRs: [ADRs/ADR-028-ATOMIC-PROTOTYPE-CONTRACTS.md](../ADRs/ADR-028-ATOMIC-PROTOTYPE-CONTRACTS.md)
- Contracts: AtomicPrototypeClaimV1
- State machines: PrototypeEvidence
- Atomic prototype requirements: P0-01A P0-10J (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 19 — LOW — Protocol terminology drift

- V2 defect: 00B/00C/21 alternate ProducerBinding/ProducerEvent and PublicationPlan/VerifiedPublicationPlan.
- V2.1 repair: One registry and explicit historical alias map; normative unknown-name/revision checks; historical docs isolated.
- Documents: [18_DATA_MODEL.md](../18_DATA_MODEL.md), [20_API_CONTRACTS.md](../20_API_CONTRACTS.md), [21_EVENT_CONTRACTS.md](../21_EVENT_CONTRACTS.md), [43_ADR_INDEX.md](../43_ADR_INDEX.md), [47_COMPATIBILITY_MATRIX.md](../47_COMPATIBILITY_MATRIX.md), [49_CANONICAL_PROTOCOL_GLOSSARY.md](../49_CANONICAL_PROTOCOL_GLOSSARY.md)
- ADRs: [ADRs/ADR-024-NORMATIVE-WIRE-SCHEMA.md](../ADRs/ADR-024-NORMATIVE-WIRE-SCHEMA.md)
- Contracts: ProducerEventV2 VerifiedPublicationPlanV2 HumanApprovalAssertionV2
- State machines: ProtocolNegotiation
- Atomic prototype requirements: P0-06B P0-06D (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**

## Finding 20 — LOW — Diagram trust arrows

- V2 defect: 25 W→P and26 W/S→P imply runner/sandbox direct PostgreSQL credentials.
- V2.1 repair: Route every runner/verification/agent submission through trusted control/ingress services; label actual DB owner and separate logical plan/allocation/manifest ordering.
- Documents: [01_EXECUTIVE_ARCHITECTURE.md](../01_EXECUTIVE_ARCHITECTURE.md), [22_TRUST_BOUNDARIES.md](../22_TRUST_BOUNDARIES.md), [25_PR_SEQUENCE_DIAGRAMS.md](../25_PR_SEQUENCE_DIAGRAMS.md), [26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md](../26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md), [27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md](../27_FIX_PUBLICATION_SEQUENCE_DIAGRAMS.md)
- ADRs: [ADRs/ADR-019-GATE-INPUT-AUTHORITY-AND-DATABASE-WRITERS.md](../ADRs/ADR-019-GATE-INPUT-AUTHORITY-AND-DATABASE-WRITERS.md), [ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md](../ADRs/ADR-020-EXPECTED-ANALYSIS-MANIFEST.md)
- Contracts: AttemptBindingV2 AttemptEventV2 ProducerEventV2
- State machines: JobAttempt Evaluation Verification
- Atomic prototype requirements: P0-02A P0-10A (NOT_RUN)
- Final status: **RESOLVED_IN_V2_1**
