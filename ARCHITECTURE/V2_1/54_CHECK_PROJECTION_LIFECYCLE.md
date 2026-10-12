# Check Projection Lifecycle

## Identity and authority

CheckProjectionV2 separates an immutable internal evaluation from a mutable provider check. It binds provider/stable repository/PR, tenant and ownership epoch, review/evaluation identity, head/candidate commit and tree, policy and analyzer/trust-set digests, authority epoch, gate-proof digest, expected Review App ID, actual check-run ID (absent option `[]` only before creation), desired and observed conclusion, projection generation, effect identity and confirmation time. SCHEMAS is normative. Gate signer establishes internal truth; Review Broker alone projects it with a distinct Checks-write App. The ordinary API and Publication Gateway have no check authority.

One current projection slot is unique by `(tenant, repository_epoch, provider, repository_id, gate_name, required_subject_oid, review_app_id)`. Provider checks can be shared across PRs with the same commit; therefore the broker also indexes every associated PR/evaluation. It may publish SUCCESS only if every currently eligible association has an identical supported effective merge subject/policy and qualified current PASS. Different base/policy/candidate associations on the same checked SHA force FAILURE/GATE_UNVERIFIABLE until a provider-supported distinct candidate SHA separates them. PR metadata alone cannot disambiguate GitHub's commit-scoped required check.

Association rows use `(projection_id, pr_number, evaluation_identity_digest)` uniqueness and the trusted Review Broker write role; adding/removing an association locks the same projection slot, bumps generation and recomputes qualification. They are local relational projection indexes, not an additional signed wire type.

The canonical name is `Mastercoder Review`, and required-check rules pin the expected App identity. Provider check ID/external_id is discovery metadata, not a provider idempotency or compare-and-swap guarantee. On discovery require matching repository, App, name, head/candidate mapping and full recorded generation; enumerate all matching runs with complete pagination, including duplicates/orphans from recovery.

## Lifecycle and concurrency

| State | Entry condition / next action |
|---|---|
| PENDING | Current internal evaluation exists; desired conclusion initially non-permitting |
| WRITING | One journaled CREATE_CHECK or UPDATE_CHECK effect dispatched |
| CONFIRMED | Read-back matches required identity, latest generation and desired conclusion |
| WITHDRAWAL_PENDING | A confirmed/possible success becomes STALE, REVOKED or INVALID; desired conclusion FAILURE |
| UNKNOWN | Write outcome, competing run or late request is uncertain; no success dispatch |
| CONFIRMED with projected_conclusion FAILURE | Withdrawal complete: read-back proves failure on every potentially authoritative prior success and all old writes terminal |
| MANUAL_RECONCILIATION | Operator must resolve missing terminality/identity proof; no permitting dispatch |

Generation increments in the same transaction that changes evaluation currentness and writes the outbox invalidation event. Lock the projection slot and current authority rows; compare old generation, repository epoch, authority epoch, active policy/trust-set and database-time live writer lease. Unique `(projection_id, generation, effect_kind)` provides internal idempotency. Only a single credentialed Review Broker writer for a slot may dispatch; gateway enforces the fencing generation. DB fencing alone cannot cancel a previously sent provider write.

Each check mutation is a separate durable EffectIntentV2. Before success STARTED, verify current signed qualified PASS and every mandatory analysis receipt through gate authority, reread provider candidate and ruleset posture, compare current slot generation, and ensure no older unresolved mutation. After provider response, independently read the actual check and store confirmation only if generation remains current. A delayed response never restores an old evaluation or marks a superseded generation CONFIRMED. Duplicate webhooks merely trigger idempotent current-state observation. If a previous writer has an ambiguous sent success, request and confirm gate freeze, then reconcile; do not issue a newer success that assumes arrival order.

Freeze is an orthogonal `freeze_state` (NONE, REQUESTED, CONFIRMED, FAILED), not another projection-state enum. A projection can be UNKNOWN while freeze is CONFIRMED. EXTERNAL_GATE_UNSAFE is a derived product/incident label, not a wire state.

## Invalidation protocol

Head/base movement, candidate/merge-group/queue generation change, repository transfer, policy activation, analyzer/plugin/trust-set revocation, authority epoch rotation, manual security revocation and evaluation expiry all invalidate the old projection. Transactionally record reason/time, bump generation, mark affected internal evaluation STALE/REVOKED/INVALID, and enqueue withdrawal. The broker identifies all potentially authoritative success runs, writes explicit FAILURE and reads each back. A new evaluation is a new generation and requires a new gate proof; old evidence is retained, never edited to appear current.

Mandatory incomplete/partial/unsupported/canceled/stale/ambiguous work maps to FAILURE, never SUCCESS, NEUTRAL or SKIPPED. Ordinary PENDING execution may appear in-progress only in a proven provider cell; withholding success is required throughout. Completed withdrawal requires provider confirmation and resolution of all prior sent writes. A read showing failure while an earlier success request might still arrive is insufficient. An unbounded provider timeout leads to UNKNOWN with freeze requested and manual provider investigation, not invented terminality.

## Normal policy change versus emergency

Normal policy change is staged: keep old policy explicitly effective while preparing the new version; acquire repository security administrator's provider gate freeze, verify effective freeze, withdraw all old successes and settle old writes, activate new policy/epoch transactionally, evaluate fresh candidates, confirm fresh projections, then release freeze after reconciliation. If freeze capability is unavailable, defer policy activation for enforcement mode; display the pending version and use advisory-only mode rather than claiming the new policy is enforced.

Emergency security revocation is immediate internally: stop success and publication dispatch, invalidate affected authority, request gate freeze through the separately enrolled repository security administrator and withdraw success in parallel. There is an unavoidable interval from internal invalidation until provider-enforced freeze or check withdrawal is effective. During that interval GitHub may still permit a merge using a stale success. Expose `EXTERNAL_GATE_UNSAFE`, page the security operator, record start/end and any observed merges, and never claim instantaneous revocation. Freeze is a provider branch-update/merge-denial posture applying to every relevant target branch and bypass actor, or an independently proven equivalent; merely disabling auto-merge or asking users to stop is not sufficient. Review and Publication Apps cannot modify this posture. If provider access is unavailable, freeze_state remains REQUESTED and enforcement cannot be claimed restored.

Repository registration for enforcement mode requires a tested freeze procedure, administrator ownership and recovery contact; repositories without it retain advisory review and the clear stale-check residual. Normal head/base/provider changes cannot be pre-frozen atomically by this service; use strict freshness/merge-queue semantics only in P0-proven cells, and apply the same unsafe-window reconciliation when assumptions fail. Freeze release requires independent security administrator acknowledgment, all ambiguous old writes resolved, current ruleset/App identity verified, fresh policy/authority evaluation confirmed, and immutable audit. Manual resolution never asserts old writes cannot arrive without provider-specific terminal proof.

## Prototypes and provider facts

P0-03 must prove required-check App pinning, commit/PR/shared-SHA behavior, merge modes/queue mapping, failure replacement, duplicate runs, delayed/reordered success and withdrawal, unknown writes, base/head changes and actual freeze enforcement including bypass identities. Record exact provider/API version and ruleset fixture. P0-10 restore testing must replay durable check intents and discover orphan provider checks before releasing freeze. Until evidence passes, these are proposed protocol guarantees, not observed GitHub behavior.

GitHub checks are created for commits and support read/update by check-run ID: [Checks REST API](https://docs.github.com/en/rest/checks/runs). Required-check acceptance behavior includes conclusions that this protocol intentionally avoids for incomplete work: [Troubleshooting required checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks). Repository bypass settings must be considered in freeze evidence: [Ruleset creation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository).

## Pre-verdict writes

CheckWriteRequestV1.gate_proof_digest is an explicit option: initial IN_PROGRESS and safety withdrawal FAILURE may carry no proof and instead require the protected current projection authorization. SUCCESS always requires a present qualified current PASS proof; absent proof can never be interpreted as success. CheckOutputV1 is the metadata-only output preimage.

Before final verification inputs exist, CheckProjectionV2 binds the allocated evaluation_id and an absent evaluation_identity_digest. Once finalized, the identity is present and mandatory for SUCCESS; no zero-digest placeholder is used.
