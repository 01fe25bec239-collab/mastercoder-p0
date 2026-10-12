# A2 Platform Reliability

## Charter

Own deployment topology, network/SQL roles, capacity/admission, observability, SLOs, backups, DR/regional fencing, effect reconciliation operations, audit export, supply-chain operations, and runbooks.

## Required properties

One write region; database loss stops authority; projections never substitute; privileged egress least; seal-export lag does not halt fleet review; split-brain rotation/reconciliation; no stale/partial PASS for availability.

## Dependencies and gates

All ADRs, especially 001/002/003/009/013/014. P0-08/P0-10. Implementation status: NOT_AUTHORIZED.

## Review handoff

Provide capacity/load results, failure attribution, SLO/runbooks, backup/restore and regional drills, egress/role tests, telemetry leakage, audit lag/backfill, supply-chain provenance, and incident rotation evidence.

