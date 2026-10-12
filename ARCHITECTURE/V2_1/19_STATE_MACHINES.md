# State Machines

These are normative transition rules. No state field is a public CRUD surface. Every accepted transition verifies scope/epoch, old state/version, authenticated writer, idempotency key and audit append in one PostgreSQL transaction. Lease-dependent transitions additionally use the actual-expiry predicate in document 29. Signed identities remain immutable; state/currentness changes are append-only events.

## Review and Evaluation

Review: REQUESTED → ACTIVE → COMPLETED | ACTION_REQUIRED | CANCELED. ACTIVE may report PARTIALLY_AVAILABLE without claiming completion. ACTION_REQUIRED can request a fresh evaluation or close as CANCELED; it cannot directly acquire a verdict. A new candidate/policy creates a new evaluation under the same review, with a new sequence.

Evaluation verdict: COLLECTING → VERIFYING → DECIDING → PASS | FAIL | BLOCKED | HUMAN_REVIEW_REQUIRED | PARTIALLY_AVAILABLE. Every mandatory manifest unit must have a qualifying completion receipt before PASS; zero findings without that receipt is incomplete. Terminal verdicts are immutable. A fresh evaluation is required after disputed evidence, oracle acceptance, permitted policy change or additional verification.

Evaluation currentness: CURRENT → STALE | REVOKED | INVALID, monotonically. Subject/base/head/queue or policy movement gives STALE; security revocation gives REVOKED; discovered invalid receipt or proof gives INVALID. Any can supersede an earlier verdict, including PASS. Revalidation creates a new identity; no terminal resurrection. Every loss of currentness atomically emits projection withdrawal intent under document 54. Historical verdict and currentness must both be displayed.

## Job and JobAttempt

Job: READY → LEASED → RUNNING → SUCCEEDED | FAILED | CANCELED | EXPIRED. Expired/lost lease may return the job to READY only with a new generation and attempt. The prior JobAttempt is terminal EXPIRED/FENCED, never RUNNING again.

JobAttempt: ASSIGNED → RUNNING → SUCCEEDED | FAILED | CANCELED | EXPIRED | FENCED. Heartbeat and completion require owner+generation+epoch+allowed-state and lease_expires_at > clock_timestamp() sampled after locking. A packet that waited across expiry is rejected even if the reaper has not run. Duplicate valid completion returns the original accepted result; a conflicting payload is rejected. Late reports may be retained as quarantined facts and settle real spend, but cannot change job result, promote artifacts, create verification authority, or acquire new model work.

## Verification and finding resolution

Verification: REQUESTED → ASSIGNED → RUNNING → VERIFIED | REFUTED | INCONCLUSIVE | UNSUPPORTED | EXPIRED | REVOKED. The verification class and oracle class are independent fields. Independent reproduction of an untrusted oracle cannot make it a blocking trusted requirement. Revocation/staleness invalidates qualification without overwriting the signed receipt. Additional work creates a new assignment/set, then a fresh evaluation.

Finding: CANDIDATE_FINDING → QUALIFIED | NON_BLOCKING | HUMAN_REVIEW_REQUIRED | DISPUTED. Human dispositions are scoped under document 56: confirm trusted oracle and request new verification; refute with evidence and request fresh evaluation; allow an explicit policy exclusion with expiry; accept risk visibly; or close without PASS. None edits a previous deterministic result into PASS.

## Patch and Approval

Patch: PROPOSED → VERIFIED → AWAITING_APPROVAL → APPROVED → PUBLICATION_QUEUED → PUBLISHED. Before final confirmation, REJECTED | EXPIRED | REVOKED | STALE stops future effects. If an already-started effect remains uncertain, patch publication status is UNKNOWN/MANUAL_RECONCILIATION until resolved; a revoked patch may still have an observed immutable object, which is recorded without restoring approval. Missing/expired custody bytes invalidate approval and prevent new sends.

Approval: CHALLENGE_ISSUED → APPROVED | REJECTED | EXPIRED; APPROVED → CONSUMED | REVOKED | EXPIRED | STALE. Only Approval Authority performs the credential-bound, fresh user-verified ceremony over its own canonical display. CONSUMED binds exactly one PublicationOperation; it does not preauthorize every later effect. An append-only revocation/currentness record can disable remaining effects after consumption. Replay returns the same operation or conflicts; nonce consumption never creates a second operation. Credential recovery creates a new binding and generation, not reinstatement of old assertions.

## PublicationOperation and ExternalEffect

PublicationOperation: PLANNED → AWAITING_APPROVAL → AUTHORIZED → RUNNING → CONFIRMED | FAILED_TERMINAL | REVOKED | EXPIRED | UNKNOWN | MANUAL_RECONCILIATION. RUNNING means an effect has begun, not future-send authority. Parent confirmation requires every mandatory child effect confirmed and exact final tree/ref/PR identity observed. Revocation/expiry blocks unstarted children. Completed immutable objects may remain orphaned. Unknown children keep uncertainty visible even when future authority is revoked.

Each immutable EffectIntentV2 begins PENDING. EffectStateV1 snapshots follow PENDING → STARTED → CONFIRMED | FAILED_TERMINAL | UNKNOWN; PENDING may become FAILED_TERMINAL on cancellation/expiry/revocation before send. UNKNOWN → CONFIRMED | FAILED_TERMINAL | MANUAL_RECONCILIATION only with qualifying evidence. MANUAL_RECONCILIATION → CONFIRMED | FAILED_TERMINAL only under document 56; unresolved abandonment remains an uncertainty tombstone. No transition returns this effect to PENDING/STARTED. Replacement calls have fresh effect_id/call_attempt after document 50's proof and authorization rule.

STARTED is one individual call's authorization linearization, not a publication sequence. The exact request and authorization are fixed; SEND_ARMED durable witness acknowledgment is additionally required before network send. Crash/timeout after possible send yields UNKNOWN even when a provider read says not found. ABSENT_CONFIRMED is not a V2.1 state. FAILED_TERMINAL means proven terminal outcome, and records whether execution occurred; its name alone does not authorize a replacement.

## Git Check Projection

The projection lifecycle in document 54 is authoritative: each desired generation is pending until exact provider identity/conclusion is confirmed; stale or revoked internal authority creates a withdrawal/replacement generation. Concurrent writers use CAS and one active effect per check identity. An unresolved prior write cannot be bypassed by issuing a newer success. Delayed results reconcile to current desired generation; emergency merge freeze remains until the real provider state is safe. Provider mutation confirmation never changes the underlying evaluation verdict.

## Manual Reconciliation

Case: OPEN → EVIDENCE_PENDING → RESOLVED_CONFIRMED | RESOLVED_NOT_EXECUTED | CLOSED_UNRESOLVED; OPEN/EVIDENCE_PENDING may become EXPIRED. A new case references an expired case; expiry never proves absence. RESOLVED_CONFIRMED appends an exact observed result; RESOLVED_NOT_EXECUTED requires original request can no longer execute; CLOSED_UNRESOLVED preserves dedupe tombstones and blocked conflicting logical effects. Human review cases may instead end RESOLVED_REEVALUATE | ACCEPTED_RISK | REJECTED. Every decision has scope, actor authority, evidence, expiry and audit under ManualResolutionV1. No manual state emits PASS.

## Prototype Evidence

NOT_RUN → RUNNING → SUBMITTED → ACCEPTED | REJECTED; ACCEPTED → EXPIRED | INVALIDATED on version/environment/invariant scope change. Reviewer must be independent of executor/repair author for the claim it accepts. SUBMITTED is not PASS; capability stays disabled until required evidence is ACCEPTED and unexpired. This architecture package keeps every atomic P0 claim NOT_RUN.

## Resource, identity and ownership

Resource envelope: RESERVED → ACTIVE → SETTLING → SETTLED. Cancellation/expiry blocks new intents but still settles actual usage. OVERSPENT creates debt/admission freeze, never evaluation authority. Component/session: ENROLLED → ACTIVE → QUARANTINED → REVOKED; rotation creates new generation/session. Repository ownership: ACTIVE epoch → CLOSED; new association gets a new tenant epoch. History never follows mutable names or restores an old closed authority interval.

## Prototype lifecycle versus measured outcome

The specification status in AtomicPrototypeClaimV1 remains NOT_RUN here. In later authorized execution, an evidence submission follows NOT_RUN → RUNNING → SUBMITTED → ACCEPTED or REJECTED; an accepted submission can later EXPIRE or be INVALIDATED. Its measured outcome is a separate PASS/FAIL/INCONCLUSIVE value. Only an independently ACCEPTED, current submission whose outcome is PASS enables its exact capability cell. FAIL and INCONCLUSIVE never enable a cell, even when evidence is complete. No reviewer/repair author may self-accept its own execution.
