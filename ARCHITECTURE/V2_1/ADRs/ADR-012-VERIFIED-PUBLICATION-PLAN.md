# ADR-012: Verified Publication Plan

Status: ACCEPTED_DIRECTION.

## Context

The safer V1 publisher lacked an authorized source/materialization path; the more capable publisher could retarget or overreach.

## Decision

The clean-room verifier emits VerifiedPublicationPlanV2 with exact base/candidate tree and changed-object manifest. The Publisher uses base_tree plus only changed objects, requires the returned tree to equal the approved candidate tree, and performs only deterministic create-only commit/ref/fix-PR effects.

## Consequences

No second credentialed checkout is needed. Complex Git feature cells must be proved by checkout/provider gates. Tree mismatch is a hard security failure.

Gate: P0-05/P0-09.


## V2.1 amendment

The preceding text is retained as historical V2 rationale. Its effective V2.1 semantics are amended by [ADR-025-PATCH-BYTE-CUSTODY.md](ADR-025-PATCH-BYTE-CUSTODY.md). In particular, create-only is gateway-enforced rather than intrinsic token authority; trusted AA display and scoped credential lifecycle are required; patch bytes remain in pinned custody and every external effect rechecks authority. See 14, 52, 53 and 54.
