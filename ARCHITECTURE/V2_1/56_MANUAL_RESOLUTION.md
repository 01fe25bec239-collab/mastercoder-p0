# Human Review and Manual Reconciliation

## Scope and authority

ManualResolutionV1 wraps a pre-ceremony ManualResolutionActionV1 with the ceremony digest, nonce and resolution-authority identity; it is an immutable, signed, expiring decision over one case and exact tenant/repository/evaluation/finding/effect identity. It records actor and authorized role, authentication ceremony, decision kind, previous state/version, evidence digests, proposed next action, policy/authority epochs, creation/expiry and audit reference. It never contains a caller-selected PASS. The ordinary API opens a case or displays it; the Resolution Authority verifies the actor's current scope and performs the protected transition.

An assigned repository maintainer with review-resolution scope may resolve a disputed finding or provide an oracle. A tenant policy administrator may create an explicit bounded policy exclusion if the active policy permits it; they cannot waive non-waivable security invariants. A provider reconciliation operator with read-only provider access may collect evidence; a separately authorized resolution reviewer must approve terminal nonexecution or an uncertainty closure before a replacement can be considered. Approval/publication rights are distinct from case resolution: any new publication still requires Approval Authority and current per-effect capability.

Authentication requires a fresh authority-owned user-verification ceremony bound to the canonical ManualResolutionActionV1 digest through ResolutionChallengeV1 for security-sensitive actions (oracle trust promotion, policy exclusion, external-effect terminal classification or case closure). Ordinary account recovery does not grant these scopes; recovery of approval authority follows document 52. Automated exact-result reconciliation may confirm a provider result using a policy-approved verifier, without human adjudication, but it cannot invent absence.

## Required resolution flows

| Case | Evidence and authorized decision | Transition and next action |
|---|---|---|
| HUMAN_REVIEW_REQUIRED: untrusted agent oracle | Scoped maintainer views exact test/assertion, requirement source and affected candidate; explicitly accepts a requirement-backed oracle or rejects it | Append trusted-oracle decision with expiry/scope; schedule new verification and fresh evaluation, or keep finding NON_BLOCKING/rejected. Independent reproduction alone is insufficient |
| Disputed finding | Maintainer supplies exact counterexample, requirement reference or trusted reproduction artifact bound to candidate/policy | Mark finding DISPUTED then RESOLVED_REEVALUATE; retain old receipt/verdict and create new verification/evaluation |
| False-positive claim | Signed reviewed refutation or policy-allowed exclusion identifying analyzer/rule/file/symbol scope, rationale and expiry | New immutable policy version/exclusion if permitted, then fresh evaluation. Old FAIL is never rewritten; non-waivable units remain mandatory |
| Unsupported verification/C3 unavailable | Capability evidence and profile assessment; maintainer chooses supported profile, independent pool, defer or accept visible risk | New request/evaluation under valid profile, or PARTIALLY_AVAILABLE/HUMAN_REVIEW_REQUIRED/ACCEPTED_RISK. Never label a single-domain deployment C3 |
| UNKNOWN external effect with exact positive result | Exact provider/account/repository/request-result identity, tree/ref/PR or usage proof and journal intent match | RESOLVED_CONFIRMED; append EffectStateV1 CONFIRMED under original identity even if authority is now revoked; this grants no later effect |
| Claimed effect nonexecution | Complete journal proof of never SEND_ARMED plus old sender fencing, or provider request-correlated terminal guarantee that request cannot later execute | Independent reconciliation reviewer signs RESOLVED_NOT_EXECUTED; FAILED_TERMINAL snapshot records proof. Fresh attempt still needs current authorization and document 50 retry predicates |
| External uncertainty cannot be proved | Record searches, provider limitations and request age; operator may keep investigating or close business case as uncertain | CLOSED_UNRESOLVED/MANUAL_RECONCILIATION tombstone, no retry and no authoritative success. Escalate provider support through separately authorized communications |

Accepted risk is an explicit non-pass business disposition. If a repository supports a separately authorized manual merge process, that action occurs outside Mastercoder's deterministic PASS/publication authority and must not masquerade as a check success. No resolution may silently convert incomplete analysis, an unsupported verifier, missing patch bytes or untrusted oracle into deterministic PASS.

## Transactions, replay and expiry

Open case by (tenant, subject identity, case kind, idempotency key); return existing identical case on duplicate and reject conflicting request bytes. The Resolution Authority locks tenant/current repository epoch, case and affected subject in document 29 order, verifies actor scope, fresh resolution challenge/nonce, evidence digests, current epoch and case version, and checks expires_at > clock_timestamp() after locks. It consumes nonce and appends resolution/audit/outbox atomically; only resolution_writer can execute this transition. Ordinary API has no write grants on resolution/terminal effect tables.

A resolution challenge expires after 5 minutes; the signed decision must be committed within that interval and is one-use. Policy exclusions/oracle approvals carry explicit policy-bounded expires_at, no unlimited default; initial maximum is 30 days unless a stricter policy applies. They are additionally bound to exact candidate/requirement/analyzer scope as appropriate; expiry or relevant source change invalidates qualification and triggers fresh evaluation/projection withdrawal. Effect nonexecution evidence has a provider-contract-specific validity scope; an expired idempotency window cannot be extended by an operator. Terminal observations remain immutable history.

OPEN → EVIDENCE_PENDING → RESOLVED_CONFIRMED | RESOLVED_NOT_EXECUTED | RESOLVED_REEVALUATE | ACCEPTED_RISK | REJECTED | CLOSED_UNRESOLVED; unresolved case expiry emits escalation and EXPIRED. A new case may reference the old case, but reopening does not revive any old grant or clear an effect tombstone. Duplicate valid resolution returns the original result; conflicting decisions require a new case/evidence and cannot overwrite prior audit.

## Evidence and containment

The trusted resolution UI fetches the canonical case/evidence itself and renders exact scope and proposed action before signing. Browser/OS compromise remains an endpoint threat; ordinary app UI instructions alone cannot create a resolution. Resolvers cannot edit submitted artifacts; changed evidence gets a new digest and review. Preserve separation between collector and terminal-nonexecution reviewer, and between resolver and publication approver where tenant policy requires it. A reviewer who authored disputed evidence cannot independently validate that same evidence.

A CLOSED_UNRESOLVED case retains effect/logical-key tombstones, custody disposition and affected namespace/resource constraints. It never frees a deduplication key for a new mutation. Unrelated requests may proceed only when they cannot duplicate/conflict with the uncertain effect. Metadata retention outlives source-byte retention where necessary for safe recovery and is disclosed by privacy policy.

## Validation obligations

Atomic P0 claims cover unauthorized resolver, cross-tenant case, challenge/nonce replay, expired decision after lock wait, fake positive result, not-found masquerading as absence, acceptance of an untrusted oracle without requirement, false-positive waiver of a mandatory invariant, unsupported C3 selection, unresolved closure followed by retry, and fresh-evaluation identity after permitted resolution. Every claim remains NOT_RUN here.

ManualCaseStateV1 is the immutable case-state snapshot referenced by ManualResolutionActionV1.prior_state_digest. It fixes subject_type, subject_digest and revision; action commit checks matching case/scope/subject and CAS current revision, preventing decisions against stale or different case state.
