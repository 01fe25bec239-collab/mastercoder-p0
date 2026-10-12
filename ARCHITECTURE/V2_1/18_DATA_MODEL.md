# Canonical Data Model

Normative wire layouts live in SCHEMAS; this document specifies durable identity and constraints, not executable DDL. Existing tenant, repository, policy, workflow, ledger, artifact and inbox/outbox entities are preserved.

## Identity, policy and authority

Tenant includes authority_epoch/status and retention policy. User/Membership map stable actors and tenant roles. RepositoryTenantEpoch is an immutable ownership interval; display names never substitute for provider repository identity. ComponentIdentity binds role, workload subject, credential_generation, capability/config digest and revocation. PolicyVersion and TrustBundle are immutable signed bytes with separate current pointers. RevocationEvent is append-only with a monotonic watermark; restore cannot lower it. ApprovalCredentialBindingV1 is separate from ordinary account identity and recovery.

## Review and execution

Review is a user request grouping immutable Evaluation identities. MergeSubjectV2 records exact PR/base/head/candidate commit/tree and provider generation. An evaluation pins subject, policy, trust-set/analysis-plan digests, expected manifest, sequence, capability profile and authority epoch. Verdict and currentness are separate: old PASS remains historical evidence but STALE/REVOKED cannot authorize a current check.

Job stores kind, tenant, state, not_before, owner, lease_generation, lease_expires_at and authority_epoch. JobAttempt has one immutable AttemptBindingV2 and generation, runner/session, sandbox tier, resource envelope and terminal outcome. ExpectedAnalysisManifestV1 enumerates mandatory units before execution; AnalysisCompletionReceiptV1 represents every unit including zero findings. EvidenceReceiptV2, VerificationReceiptV2 and VerificationSet are immutable signed records accepted only by document 07's writer roles. GateProofV2 binds the complete decision input digest and current evaluation coordinate. FindingCandidate and ManualResolutionV1 never substitute for qualifying evidence.

## Approval, custody and publication

PatchCandidate pins base/patch/manifest bytes and provenance. PatchCustodyV1 pins exact tenant/repository/patch revision/digest/candidate-tree bytes through verification, trusted display, approval, publication and reconciliation. VerifiedPublicationPlanV2 pins all effects and destinations. HumanApprovalAssertionV2, ApprovalChallenge and ApprovalCredentialBindingV1 bind the authority-owned display/ceremony and credential generation; approval nonce consumption is unique and never reconstructed from an ordinary account recovery.

PublicationOperation groups the immutable plan, approval, custody pins and expected ordered effects. It has no provider-send authority of its own. EffectIntentV2 describes exactly one actual provider call: effect_id, stable logical operation/parent identity, predecessor IDs, call_attempt, exact kind/request digest, provider/account/repository, tenant and repository epoch, authority epoch, authorization proof, initial PENDING state and deadlines, and optional provider idempotency token. EffectStateV1 holds append-only state/time/result/reconciliation snapshots referencing the immutable intent digest; current state is a CAS projection. Distinct blob calls, tree, commit, ref, PR, check and model calls are distinct effects. No parent STARTED state authorizes children.

ExternalEffectJournalV1 is the separately witnessed append-only recovery record described in document 50. PostgreSQL stores its digest/sequence/receipts as a projection; the external journal retains enough exact immutable intent and observation data to recover rows missing from an RPO restore. CheckProjectionV2 binds provider check identity, exact candidate/policy/trust/epoch and monotonic desired/confirmed generations. ManualResolutionV1 records a scoped decision, authenticated actor, evidence, old/new state, expiry and audit; it is not an editable verdict.

## Required relational constraints

Tenant-owned primary and foreign keys include tenant_id. Repository-scoped foreign keys also bind repository_tenant_epoch. Unique constraints cover provider delivery, evaluation sequence, manifest/work-unit/attempt completion, verifier assignment/receipt, approval nonce, job generation completion, ledger settlement, and (tenant, logical_operation_id, effect_kind, call_attempt). Each effect_id has exactly one SEND_ARMED record. A pending/started/unknown/manual attempt blocks conflicting replacement. A fresh call may be created only under document 50's exact proof: proven terminal nonexecution, or an explicitly qualified provider idempotency guarantee covering the same request/token/account within its retention window. In the latter case the original UNKNOWN remains unresolved until its own outcome is proven; internal uniqueness includes fresh call_attempt, while logical identity and request digest remain fixed. Conflicting idempotency bodies are rejected, never silently merged.

Digest-bearing rows are immutable; corrections create new identities. Current pointers and state versions use compare-and-set under document 29 lock ordering. Gate-critical tables use dedicated writer roles and RLS, not generic CRUD. Append-only audit stream uniqueness is (tenant, stream, sequence), with previous-digest continuity. Artifact grants/pins, deletion state and per-tenant encryption remain authoritative; expiry cannot be undone by rebuilding bytes from an untrusted source.

## Encoding

Use only canonical names from document 49 and exact layouts from SCHEMAS. Signed/hashed records use deterministic CBOR with fixed positional arrays, explicit optional-value conventions, integer widths, enum registries and domain-separated digests/signatures. JSON is a typed transport presentation, not a substitute hash encoding. TEST_VECTORS contains the required positive and rejection cases.
