# ADR-022: Actual Lease Expiration

Status: ACCEPTED_V2_1_ARCHITECTURE; capability validation remains pending.

V2 heartbeat/completion predicates checked owner, generation and epoch but not actual expiry, letting an unreaped lease retain authority.

Decision: lock required rows, sample PostgreSQL clock_timestamp() after any wait, and atomically require lease_expires_at > that sample plus owner/generation/epoch/state for every lease-bound mutation. Reaper only records expiry. Equality is expired; expired heartbeat cannot revive the same generation. Reassignment creates a fresh attempt.

Consequence: transactions use the same predicate for completion, artifact authority, verification and model authorization. CURRENT_TIMESTAMP/statement_timestamp are insufficient after waits. Clock-health anomaly stops authority and demands fenced recovery. P0 must delay beyond expiry without a reaper and while blocked on a row lock. Amends ADR-002/014; finding 8.
