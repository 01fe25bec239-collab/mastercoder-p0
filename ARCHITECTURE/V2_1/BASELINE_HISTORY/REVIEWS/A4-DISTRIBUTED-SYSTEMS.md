# A4 Distributed Systems Review

Verdict: PASS_WITH_RELEASE_BLOCKERS. Independence: NOT_AN_EXTERNAL_ATTESTATION.

## Strengths

PostgreSQL supplies one transactional authority for jobs, budget, intent, audit, and generations. At-least-once delivery is paired with compare-and-set and deterministic effect identity. STARTED plus UNKNOWN_EFFECT avoids unsafe blind retry. Late spend settles without reviving authority. Single-region writes keep fencing understandable.

## Required evidence

P0-08 must demonstrate reservation/intent/settlement conservation under partition, cancel, lease loss, duplicate/late usage, and crash. P0-10 must demonstrate inbox/outbox replay, PostgreSQL failover, authority_epoch split-brain fencing, migration skew, regional recovery, and external-effect reconciliation.

## Residuals

Provider APIs cannot be made transactional with PostgreSQL. Post-STARTED effects may complete after revocation. One write region accepts bounded outage. These are explicit, reconciled tradeoffs rather than hidden exactly-once claims.

Conclusion: state-machine design is coherent; implementation remains blocked.

