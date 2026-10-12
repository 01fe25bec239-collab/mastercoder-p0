# API contracts

Mutating transports authenticate workload/user, bind tenant and protocol registry digest, require request_id/idempotency_key and expected generation, and authorize a closed operation before opening a transaction. Human JSON uses the schema projection; signed records use SCHEMAS canonical CBOR. The ordinary API may request protected operations but cannot insert their authoritative outputs. A repeated idempotency key with a different canonical payload is CONFLICT.

| Endpoint operation / authorized caller | Canonical request or bound input | Success / authority boundary |
|---|---|---|
| CreateReviewIntent / scoped member | CurrentCandidateV1 + PolicyVersionV1 + ResourceEnvelopeV2 digests, mode | New evaluation UUID/workflow intent through trusted orchestration; EvaluationIdentityV2 finalized after manifest/verification inputs exist |
| Poll/Accept/Heartbeat/Complete / enrolled runner | AttemptBindingV2 + authenticated session/message sequence | Runner Control atomically checks actual DB-clock lease predicate; late results quarantine |
| SubmitAnalysis / analyzer | AnalysisCompletionReceiptV1 and artifact digests | Evidence Authority verifies signature/profile/manifest/current attempt before accepting |
| SubmitVerification / qualified verifier | VerificationReceiptV2 | Verification Authority constructs immutable VerificationSetV2; API cannot do so |
| DecideGate / scheduler signal | EvaluationIdentityV2 | Gate Authority fetches trusted inputs and creates GateProofV2; never signs caller verdict |
| PrepareApproval / browser directly at AA origin | VerifiedPublicationPlanV2 digest | AA independently fetches custody/plan, produces TrustedDisplayReceiptV1 and ApprovalChallengeV2 |
| FinishApproval / same AA browser session | WebAuthnCeremonyV1 | Exact fresh UV/challenge/binding transaction yields HumanApprovalAssertionV2 |
| Register/replace/recover/revoke approver / scoped AA authority | ApprovalCredentialBindingV1 ceremony and AuthorityScopeV1 | Dedicated lifecycle in 52; ordinary account recovery has no permission |
| StartPublication / approved user intent | VerifiedPublicationPlanV2 + HumanApprovalAssertionV2 digests | Consume assertion to one PublicationOperationV1; no blanket send authorization |
| DispatchGitEffect / credential-free planner | EffectIntentV2 + GitPublicationCapabilityV2 | Gateway derives closed registered Git creation request; fresh per-effect check, durable journal, one send |
| ProjectCheck / gate transition outbox | CheckProjectionV2 + optional GateProofV2 | Review Broker serialized exact CheckWriteRequestV1 + witness; confirmed provider read-back |
| ExecuteModelTurn / bounded harness | ModelIntentV2 + ModelTurnV2 | One-use budget/lease validation, approved dialer, actual usage later settles |
| AcquireDependencies / runner | DependencyAcquisitionGrantV1 | Exact locked artifacts; secret-free sandbox receives verified bytes only |
| ResolveCase / scoped authority | ManualResolutionV1 | Evidence-bound transition under 56; never direct PASS |
| ReconcileEffect / recovery authority | EffectIntentV2 / EffectStateV1 / journal IDs | Read-only provider observations and ProviderResultV1; fresh mutation is a separate authorized effect |

All transport wrapper fields (request ID, user session, mTLS peer and idempotency key) are server-validated transport metadata and not alternatives to bound payload scope. Protected state transitions authenticate against the matrix in 07, constraints in 18 and lock/time rules in 29. For signed protocols the complete wire payload is the named schema, not this routing table. Administrative ordinary CRUD of gate-critical records is forbidden.

Response outcomes: ACCEPTED(record digest, current generation), REPLAY(same accepted digest), CONFLICT, UNAUTHORIZED, EXPIRED, STALE_AUTHORITY, UNSUPPORTED, BLOCKED, UNKNOWN(effect ID). UNKNOWN is never generic retryable failure. Cross-tenant objects return indistinguishable unavailable/unauthorized responses. Retries revalidate current authority; replay of historical result is explicitly historical. Exact route spelling and status-code adapters are implementation details with no authority semantics; no unlisted privileged operation is allowed.

Version negotiation uses exact registered `(type,wire_revision,registry_digest)` under authenticated session policy. Unknown fields/revisions reject; there is no forwarding of security-relevant opaque extensions. See 47 and SCHEMAS/ENCODING.md.
