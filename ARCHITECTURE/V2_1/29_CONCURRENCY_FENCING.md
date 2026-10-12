# Concurrency and Fencing

## Independent fences

authority_epoch identifies the active authority universe; lease_generation identifies one job owner; credential_generation identifies a component session/key; evaluation sequence and subject generation identify gate currentness; repository_tenant_epoch identifies tenant ownership; projection generation orders desired provider state. Every applicable fence is required. No single fence substitutes for the others.

## PostgreSQL transaction and lock order

Protected mutations lock tenant authority, repository ownership/currentness, policy/trust/revocation pointers, evaluation, job/attempt, parent operation, then individual effect in that order where applicable. Sort multiple same-kind identities by canonical bytes. READ COMMITTED with explicit FOR UPDATE locking plus expected-version CAS is the normative default; a stronger isolation level does not remove the need to recheck after locks. No network request is made while these business locks are held. A committed state transition includes audit and outbox; conflict returns no authority.

PostgreSQL rows locked FOR UPDATE exclude concurrent writers until transaction end. A statement may wait before seeing the current version, which is why expiry sampling follows acquisition, rather than transaction/statement start. [PostgreSQL explicit locking](https://www.postgresql.org/docs/18/explicit-locking.html).

For heartbeat, completion, artifact authority promotion, verification submission, model-work authorization and all other lease-dependent mutations, require atomically: exact job/attempt and tenant; owner; generation; authority epoch; allowed state; and lease_expires_at > a freshly sampled clock_timestamp() after acquiring locks. Heartbeat writes its new expiry only if the old expiry passed this guard. A reaper is cleanup, never the source of expiry. A fresh lease always increments generation. Authority clock-health failure stops these mutations until fenced recovery.

Gate commitment locks and compares exact current subject/tree, policy/trust/revocation generations, evaluation sequence, manifest and qualifying receipt roots. Revocation uses the same locks, producing an unambiguous ordering. Signature validation may be prepared before locking, but critical currentness and expiry are rechecked inside the transaction. A signed stale snapshot cannot win a newer pointer race.

## Effect dispatch

Each actual provider call is represented by an immutable EffectIntentV2 and append-only EffectStateV1. PREPARED journal durability precedes the STARTED transaction. That transaction checks current approval/capability, exact effect membership/request, predecessors, custody, scope, generations, deadlines and parent cancellation; consumes the one-use call authorization and appends STARTED. A separately witnessed SEND_ARMED claim binds that committed start, one dispatcher session and the same exact request before network send. No confirmed journal acknowledgment means DO NOT SEND.

Authorization is checked anew before every individual effect enters STARTED. Revocation ordered after one effect STARTED may leave that one bounded call in flight; it cannot enable its not-yet-started siblings. Dispatch starts within at most 5 seconds of STARTED and client activity ends within the effect request deadline (default 60 seconds after STARTED, or stricter grant). A missed dispatch deadline denies send; a timed-out sent request remains UNKNOWN because provider execution can outlive client cancellation. Retries and reconciliation follow document 50, never a blanket retry-on-error policy.

## Idempotency, stale work and split brain

Inbox/outbox are at least once. Unique tenant/logical-effect/attempt keys and journal send claims prevent duplicate dispatch; unsupported providers do not gain exactly-once semantics by an internal UUID. A new attempt after ambiguity is prohibited unless terminal non-execution or provider-guaranteed retry safety is proven and current authority revalidated. An eventual-consistency read returning not found proves neither.

Fenced work may upload only to bounded untrusted quarantine under an unexpired upload grant; it cannot promote authoritative artifacts, receipts, verdicts, approval or publication. Append-only actual usage still settles under its original billing identity. Duplicate settlement is unique by provider/account/request; overspend becomes debt and freezes new reservations.

During failover/restore, one active epoch is selected only after old credential-bearing egress is fenced. DNS, a lease reaper or restored database epoch alone cannot stop a partitioned old gateway. The journal sequencer and witness require a durably recorded epoch fence, and old dispatcher sessions are denied new claims. Previously STARTED calls are reconciled. If old-region fencing or witness coverage is unproven, effects remain disabled. Document 35 describes recovery, including intents lost from the restored PostgreSQL snapshot.
