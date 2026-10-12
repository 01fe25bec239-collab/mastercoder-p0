# ADR-002: PostgreSQL Durable Workflow

Status: ACCEPTED_DIRECTION.

## Context

Jobs, budget, currentness, effects, and audit must commit atomically. A separate broker/cache cannot provide that transaction and is not required by the frozen stack.

## Decision

PostgreSQL is the sole authority and provides jobs through SKIP LOCKED, DB-time leases, generation fencing, inbox/outbox, idempotency, reservation/ledger, RLS, and immutable audit indexes.

## Consequences

The write primary is a critical dependency and sets a scaling ceiling. Stateless roles stop rather than improvise during database loss. Partition/index/connection tuning precedes new infrastructure; any future queue or cache is a rebuildable projection.

Gate: P0-08/P0-10.

## V2.1 amendment — findings 8–11

Business authority remains PostgreSQL. Lease acceptance now checks actual clock_timestamp() after locking, including expiry without a reaper (ADR-022). The sole-authority wording does not imply that an RPO-restorable database alone can preserve knowledge of external sends: ADR-023 adds a separately witnessed per-effect recovery journal, durably acknowledged before dispatch. The journal authorizes no business decisions and is not a required message broker. This amendment narrows the old statement that all effect safety fits within one database commit.
