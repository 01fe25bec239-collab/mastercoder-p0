# Failure and Recovery

| Failure | Detection | Safe action | Resolution |
|---|---|---|---|
| PostgreSQL primary/authority clock loss | Connection, replication and clock-health checks | Stop new authority and sends | Fence old senders, choose one primary/epoch, replay independent journal |
| Runner disconnect/lease expires without reaper | Actual database-time guard after locks | Reject heartbeat/completion/new authority; retain quarantined facts | New lease generation/attempt; settle late actual usage |
| Duplicate/lost provider webhook | Inbox identity and authoritative provider cursor | Do not infer currentness/absence from delivery | Reconcile provider subject and CAS forward current pointer |
| Provider check timeout or delayed write | Journal/CheckProjectionV2 mismatch | UNKNOWN effect, no blind retry; expose unconfirmed projection | Exact check identity read; document 54 withdrawal protocol and emergency freeze |
| Model timeout/cancel ambiguity | Per-call effect/usage identity | Stop new turns; incomplete mandatory work is non-pass | Provider usage reconciliation and append-only late settlement |
| Publication revocation between effects | Current authority transaction before each STARTED | Stop unstarted children; retain already-created immutable objects | Reconcile started call; no ref/PR send under revoked approval |
| Publication send/result ambiguity | SEND_ARMED without definitive terminal observation | UNKNOWN/MANUAL_RECONCILIATION | Exact object/ref/PR evidence; never equate not found with terminal nonexecution |
| Journal/witness unavailable or acknowledgment lost | Missing signed durable receipt/high-water agreement | DO NOT SEND; do not reuse a potentially armed claim | Restore verified witness coverage; classify complete journal facts |
| DB restore loses an intent | Journal sequence beyond restored checkpoint | Keep egress fenced, import missing historical intent/tombstone | Enumerate all witness entries and unresolved older records, then reconcile |
| Artifact/custody corruption or expiry | Digest/read/pin failure | Mandatory evidence unavailable; approval/publication invalid | New exact-input verification/custody and fresh approval; no substituted bytes |
| Analyzer/key/component compromise | Security revocation | Append revocation, quarantine, deny egress and future effects | Fresh credential/trust identity and evaluation; withdraw old success |
| Repository transfer | Provider identity/ownership evidence | Close tenant epoch; stop old authority | Explicit new association/epoch; historical records stay old tenant |
| Protocol skew | Strict handshake/decoder | Reject unsupported critical schema/version | Readers before writers; no silent downgrade |
| Human-review uncertainty | Case state/expiry | No manufactured PASS | Scoped ManualResolutionV1; evidence and fresh evaluation or visible non-pass |
| Audit exporter lag | Seal SLO | Continue required local append; alert | Ordered export backfill; effect journal remains separately synchronous |

## Recovery ordering

Contain credential-bearing egress and prove old region/session fencing; establish a single PostgreSQL write authority and healthy clock; authenticate complete external journal coverage; rotate authority_epoch to a fresh UUID and fence old generations; import all missing intents and settled/unknown facts; reconcile provider state read-only and settle known spend; rebuild candidate and check currentness; verify custody and required evidence; resume read-only deterministic work, then each privileged capability only after its own proof and reconciliation prerequisites.

Epoch UUIDs compare by equality, never numeric order; journal sequence and fencing records establish which epoch is current. Old signed evidence remains history. No repair rewrites audit, turns late completion into current evidence, resends UNKNOWN, reconstructs missing approved bytes, or changes a terminal evaluation into PASS. Document 35 defines restore coverage; document 50 defines terminal proof; document 56 defines authorized manual outcomes.
