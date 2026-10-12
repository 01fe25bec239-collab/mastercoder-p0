# ADR-004: Exact Merge Subject

Status: ACCEPTED_DIRECTION.

## Context

Head-only review, delegated base freshness, stale check identities, and merge queues can authorize a different tree than the one verified.

## Decision

MergeSubjectV2 binds stable repository/tenant epoch, PR, head, base ref/current base, merge method, candidate kind, exact candidate commit/tree, and provider generation. PASS applies only to the exact candidate or a provider equivalence demonstrated for that cell. Incomplete required work maps non-permitting.

## Consequences

Some merge modes/repository postures may be unavailable. Availability is sacrificed instead of claiming unobserved equivalence.

Gate: P0-03.

