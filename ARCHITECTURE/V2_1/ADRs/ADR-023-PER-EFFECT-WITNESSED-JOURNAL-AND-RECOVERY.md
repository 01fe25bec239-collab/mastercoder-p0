# ADR-023: Per-Effect Witnessed Journal and Recovery

Status: ACCEPTED_V2_1_ARCHITECTURE; capability validation remains pending.

V2 used an underspecified parent STARTED boundary, weak confirmed-absence retry and restore scans limited to surviving intent rows despite a positive RPO.

Decision: each actual provider call is an immutable EffectIntentV2 plus append-only EffectStateV1; PostgreSQL checks authorization before that call STARTED. A separately restored, synchronously witnessed ExternalEffectJournalV1 has PREPARED intent and unique SEND_ARMED claim durably recorded on both independent copies before send. Missing acknowledgment means DO NOT SEND. Unknown outcome cannot retry unless nonexecution or exact provider idempotency safety is proven. Not-found alone never qualifies.

Consequence: journal availability becomes a send prerequisite, while business state stays in PostgreSQL. Recover by full independent journal enumeration, original-epoch tombstones and provider reconciliation, not surviving DB rows. Frozen restore authority and fenced old credential-bearing egress are mandatory. Bounded client deadlines do not prove provider termination. Document 50 supplies crash/retention/replacement rules; P0 proves the declared one-domain-loss envelope. Amends ADR-002/014; findings 9,10,11.
