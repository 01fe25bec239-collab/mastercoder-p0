# ADR-010: Eligibility Proof

Status: ACCEPTED_DIRECTION.

## Context

MY_PRS_ONLY and organization membership were stale or based on forgeable/mutable author metadata.

## Decision

Use stable provider PR-opener actor ID linked to the authenticated user for MY_PRS_ONLY. Use a current, TTL-bound provider membership/role proof for ORG_MEMBERS. Bind EligibilityProofV2 to repository tenant epoch, policy, and reservation.

## Consequences

Eligibility may become unavailable when provider proof is unavailable or would require unacceptable permissions. Automation disables rather than guessing.

Gate: P0-04.

