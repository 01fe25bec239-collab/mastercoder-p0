# A4 Operability Review

Verdict: PASS_WITH_RELEASE_BLOCKERS. Independence: NOT_AN_EXTERNAL_ATTESTATION.

## Strengths

The topology is small: one Rust codebase, PostgreSQL authority, stateless roles, customer runners, and only authority-driven broker splits. Failure modes are explicit. Backpressure is durable. External ambiguity, generation rotation, provider outages, audit seal lag, repository transfer, and regional recovery have named operational states.

## Blockers

P0-08/P0-10 must set tested capacity, provider-rate behavior, SLOs, partition/replay outcomes, backup/restore, regional fencing, RLS/transfer lifecycle, telemetry leakage, and audit export behavior. P0-02 is required for runner reset/termination operations. P0-09 is required for publication reconciliation runbooks.

## On-call hazards avoided

No global review outage on remote audit-seal lag; no silent cache authority; no automatic retry after ambiguous mutation; no false success from unsupported capability; no multi-region active-active external effects.

Conclusion: operable direction with conservative failure semantics; production readiness remains unproved.

