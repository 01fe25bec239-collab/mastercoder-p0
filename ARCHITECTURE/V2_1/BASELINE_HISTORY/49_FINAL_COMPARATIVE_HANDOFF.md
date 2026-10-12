# Final Comparative Handoff

## Outcome

The new architecture is MASTERCODER-CONVERGENCE-SOL-V2.

Final status: SYNTHESIS_V2_BLOCKED_PENDING_PROTOTYPES.

Production implementation authorized: NO.

## What V2 retains

From OPUS, V2 retains the Rust/customer-runner orientation, exact lineage, closed evidence vocabulary, host-side model brokerage, local-first source posture, and PostgreSQL durable workflow. From SOL, it retains the modular-monolith discipline, PostgreSQL as sole authority, typed trust boundaries, Review/Publisher separation, independent receipts, immutable patch approval intent, RLS, and explicit capability matrices.

## What V2 changes

- The backend is frozen to Rust/Axum/Tokio/SQLx/Serde.
- Git review/check and publication use distinct Apps and positive protocols.
- Merge truth is an exact MergeSubjectV2 including candidate commit/tree and queue subject.
- Attempt identity is deterministic CBOR without circular digests or epoch type drift.
- Model integration uses structured multi-turn ModelTurnV2; credentials never enter guests.
- Hostile code defaults to VM-grade S2, not shared-kernel isolation.
- Evidence grades include explicit C0-C4 independence and causal proposed-test proof.
- Budgets use a broker-enforced root envelope and always accept late actual settlement.
- Eligibility binds stable provider actor/membership proof.
- Human approval is a separate direct WebAuthn authority.
- Publication uses a verifier-sourced plan and provider-returned tree equality, then only create-only fix effects.
- Tenancy includes repository_tenant_epoch, composite keys, RLS, and per-tenant artifact grants.
- Audit facts are immutable metadata; raw artifacts are separately erasable.
- Revocation, STARTED effects, ambiguity, and protocol N/N-1 behavior are explicit.

## Finding closure

All 44 numbered findings have exactly one adjudication in 00C:

- 26 VALID
- 3 VALID_BUT_WRONG_SEVERITY
- 5 REQUIRES_PROVIDER_CONFIRMATION
- 2 DUPLICATE_ROOT_CAUSE
- 3 REQUIRES_PROTOTYPE_TO_RESOLVE
- 2 VALID_BUT_SCOPE_DEFERRED
- 2 ALREADY_MITIGATED
- 1 PARTIALLY_VALID
- 0 INVALID

## Next authorized decision

The only architectural next step is to authorize disposable Phase-0 prototypes P0-01 through P0-10. Their evidence may enable individual compatibility cells or require an ADR revision. It may not weaken mandatory invariants.

## Handoff checklist

- Read 00A, 00C, 00E, 00G, 42, and 44 before planning.
- Verify source hashes in TRACEABILITY/INPUT_MANIFEST.md.
- Assign each P0 gate to its A2 manager and independent A4 reviewers.
- Use only the architecture identity and canonical V2 schemas.
- Do not begin production implementation, provider installation, or credential provisioning under this package.

