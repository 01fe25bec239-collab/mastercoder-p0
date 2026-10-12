# ADR-028: Atomic prototype acceptance contracts

Status: ACCEPTED_FOR_V2_1_ARCHITECTURE; capability evidence NOT_RUN.

## Decision and rationale

Finding 18: V2 P0 families combined unrelated assertions and allowed unreproducible umbrella PASS. Replace them with AtomicPrototypeClaimV1 records in TRACEABILITY/atomic-prototype-claims.json and readable 57. Each claim names invariant, exact environment/version-lock requirements, preconditions, procedure, injected failures, observable thresholds, PASS/FAIL, evidence, independent reviewers and expiry. All claims are NOT_RUN and no prototype execution occurs in this repair. Capability enablement derives only from applicable independently reviewed current atomic PASS. Findings 17/20 additionally remove MODEL → verifier dependencies and direct runner/sandbox → PostgreSQL arrows in 05/25/26/39. See 40/41/42/57 and SCHEMAS.

## Rejected alternative

Do not replace the existing stack or rebuild the full architecture; retain the sound V2 structure and close the missing authority/acceptance contract at its existing boundary.
