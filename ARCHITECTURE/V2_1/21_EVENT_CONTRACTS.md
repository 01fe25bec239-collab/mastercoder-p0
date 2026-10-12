# Event contracts

AttemptEventV2 binds an authenticated producer/session/generation, ScopeV1, attempt binding, stream sequence, database receive time and registered payload type/digest. ProducerEventV2 serves authoritative non-attempt changes without fabricating an attempt. ProviderEventV2 records authenticated delivery plus protected read-back; webhook contents never establish current subject by themselves. Exact fields, order, widths and domains are in SCHEMAS.

All delivery is at least once. Deduplicate `(tenant, producer, session, event_id)`; reject same ID with different bytes. Sequence uniqueness is per producer session, and business idempotency remains the protected transition key. Receive time is DB time; producer occurred_at is audit evidence only. No event replay can resurrect a lease, assertion, send claim or closed repository epoch.

| Transition / emitter | Payload | Consumer obligation |
|---|---|---|
| ExpectedAnalysisSealed / plan authority | ExpectedAnalysisManifestV1 | Seal pre-execution, pinned descriptors |
| AnalysisCompleted / admitted analyzer | AnalysisCompletionReceiptV1 | Validate producer and all mandatory coverage, including zero findings |
| EvidenceAccepted / evidence authority | EvidenceReceiptV2 | Verify origin and qualified oracle |
| VerificationAccepted / verification authority | VerificationReceiptV2 / VerificationSetV2 | Verify independent domain and currentness |
| GateCommitted / gate authority | GateProofV2 | Recompute trusted-input qualification; no API verdict authority |
| SubjectObserved / Review Broker | CurrentCandidateV1 | Current authenticated provider read-back |
| PolicyActivated / policy authority | PolicyVersionV1 | Invalidate dependent evaluations/projections atomically |
| ComponentRevoked / trust authority | RevocationRecordV1 | Immediate internal invalidation; projection/freeze protocol |
| ProjectionDesired/Confirmed / Review Broker | CheckProjectionV2 | CAS generation; reconcile provider identity and old in-flight calls |
| ApprovalMinted/Revoked / AA | HumanApprovalAssertionV2 / RevocationRecordV1 | Current binding, nonce, exact displayed plan; no whole-sequence authority |
| EffectPrepared/Transitioned / effect authority | EffectIntentV2 / EffectStateV1 | Immutable identity; state snapshot cannot change request |
| JournalAppended / sequencer | ExternalEffectJournalV1 | Both signed witness receipts before send; contiguous authenticated chain |
| ManualResolutionRecorded / resolution authority | ManualResolutionV1 | Evidence-bound decision, fresh evaluation where needed |
| AuditAppended / authorized producer | AuditEnvelopeV1 | Metadata-only immutable chain and independent seal |

`payload_type` must match the referenced record's registered type and producer role; event name is routing metadata, never permission. Atomic DB state transition appends outbox and audit in the same transaction. Dispatcher may retry delivery, not repeat provider calls. Scope/generation mismatch returns quarantine/reject with metadata only. Unknown schema/field/revision rejects rather than opaque forward. Typed domain-specific endpoints can accept the same payload without an event wrapper; neither path weakens validation.
