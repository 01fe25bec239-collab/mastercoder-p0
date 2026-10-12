# ADR-014: Settlement, Revocation, and Protocol Skew

Status: ACCEPTED_DIRECTION.

## Context

Late provider spend disappeared from current-binding accounting, publication expiry/revocation races were vague, and seven boundaries lacked a skew contract.

## Decision

Always append actual usage settlement, even after fencing. Use generation-bound authority and a transactional EFFECT_STARTED linearization point; post-start work may finish and must reconcile. Negotiate supported protocol minor once, pin it, reject unknown critical fields/major mismatch, and deploy readers before writers.

## Consequences

Cancellation cannot promise recall of sent provider calls. Ledger debt/freeze represents overspend without reviving work. Compatibility matrices and impact analysis become release artifacts.

Gate: P0-06/P0-08/P0-09/P0-10.

## V2.1 amendment — findings 8–11 and 16

The EFFECT_STARTED linearization point applies to exactly one provider call, never its parent publication sequence. Every child requires fresh current authority before STARTED, then a durable witnessed send claim. Already-started calls have a bounded client lifetime but may complete later at the provider. UNKNOWN is never automatically retried based on not-found; ADR-023 governs proof/recovery. Expired leases cannot regain authority while waiting for a reaper (ADR-022). Human case resolution is explicit, expiring and audited under ADR-027; it cannot manufacture deterministic PASS. Late actual usage settlement is preserved unchanged.
