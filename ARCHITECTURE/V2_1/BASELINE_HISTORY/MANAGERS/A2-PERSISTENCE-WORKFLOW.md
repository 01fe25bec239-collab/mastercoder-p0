# A2 Persistence and Workflow

## Charter

Own PostgreSQL schema, RLS/composite keys, jobs/leases/fencing, inbox/outbox, resource ledger, audit facts, migrations, backup and restore correctness.

## Required properties

DB time for authority; compare-and-set generations; append-only actual settlement; no cross-tenant FK; deny-by-default RLS; local audit append in business transaction; ambiguous external effects preserved; safe expand/contract.

## Dependencies and gates

ADRs 002, 005, 009, 013, 014. P0-06, P0-08, P0-10. No Redis/Kafka/workflow engine is required. Implementation status: NOT_AUTHORIZED.

## Review handoff

Deliver schema invariants, query/acquisition plans, fault-injection ledger conservation, replay/idempotency, RLS/transfer tests, migration matrix, backup restore, and split-brain epoch evidence.

