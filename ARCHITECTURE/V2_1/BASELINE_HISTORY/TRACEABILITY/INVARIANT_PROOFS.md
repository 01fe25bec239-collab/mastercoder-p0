# Invariant Proof Obligations

| Invariant | Enforcement location | Minimum proof | Status |
|---|---|---|---|
| INV-01 | PostgreSQL roles/schema/transactions | Failover/replay shows no cache/effect authority | PENDING P0-10 |
| INV-02 | MergeSubject/AttemptBinding/Gate schemas | Golden vectors plus stale-subject provider test | PENDING P0-03/P0-06 |
| INV-03 | Gate table and Review Broker mapping | Exhaustive outcomes with Git required checks | PENDING P0-03/P0-07 |
| INV-04 | Apps/keys/egress/broker protocols | Cross-authority denial and permission staging | PENDING P0-03/P0-09 |
| INV-05 | Checkout/model/runner paths | Guest exfiltration and credential inventory | PENDING P0-01/P0-02/P0-05 |
| INV-06 | Evidence policy | Stolen/current runner-key drill | PENDING P0-07 |
| INV-07 | Causal verifier | Malicious agent-test base/candidate drill | PENDING P0-07 |
| INV-08 | Scheduler capability match | Cross-OS downgrade/unsupported tests | PENDING P0-02 |
| INV-09 | Model/task brokers and ledger | Child contention/hidden agent/limit fault test | PENDING P0-08 |
| INV-10 | Approval Authority/WebAuthn | API compromise, origin/RP/UV/replay test | PENDING P0-09 |
| INV-11 | Plan/proof/publisher tree equality | Full ceremony with mutation/mismatch cases | PENDING P0-09 |
| INV-12 | Publisher App and closed effects | Permission/egress/ref mutation denial | PENDING P0-09 |
| INV-13 | Jobs/outbox/effects | Duplicate/partition/ambiguity/retry fault test | PENDING P0-08/P0-09 |
| INV-14 | Ledger settlement | Lease loss/cancel/late invoice conservation | PENDING P0-08 |
| INV-15 | Schema/RLS/epochs/grants | Cross-tenant, transfer, backup, deletion tests | PENDING P0-10 |
| INV-16 | Audit schema/log/artifact separation | Secret/source corpus and WORM correction test | PENDING P0-10 |
| INV-17 | Generations/STARTED | Revocation race and effect reconciliation | PENDING P0-09/P0-10 |
| INV-18 | Protocol/capability decoders | N/N-1, unknown-critical, downgrade fixtures | PENDING P0-06 |
| INV-19 | Artifact/model/telemetry policy | Data-flow capture and UI disclosure review | PENDING P0-01/P0-10 |
| INV-20 | Release process | Gate registry blocks enablement/production | PENDING ALL P0 |

No PENDING row is treated as proved. The architecture remains blocked until the production capability set has PASS evidence for every applicable row.

