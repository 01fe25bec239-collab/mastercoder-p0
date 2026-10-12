# ADR-026: Bounded authenticated dependency acquisition

Status: ACCEPTED_FOR_V2_1_ARCHITECTURE; capability evidence NOT_RUN.

## Decision and rationale

Finding 15: V2 prohibited guest credentials/egress and pinned tools but did not supply a complete realistic dependency path. Use exact lockfile-bound DependencyAcquisitionGrantV1, a credential-free constrained resolver, a minimal credential-bearing allowlisted dialer, and tenant-scoped integrity-checked offline artifacts. Cover public/private registries, Git, toolchains and lifecycle scripts; unsupported dynamic resolution blocks rather than opening egress. Actual expiry, root resource ceilings, endpoint/redirect/DNS checks and cache provenance apply. See 10, 36, 55 and SCHEMAS. Prototype: P0-05E/F/G/H. This uses the existing Rust broker/runner and PostgreSQL authority stack.

## Rejected alternative

Do not replace the existing stack or rebuild the full architecture; retain the sound V2 structure and close the missing authority/acceptance contract at its existing boundary.
