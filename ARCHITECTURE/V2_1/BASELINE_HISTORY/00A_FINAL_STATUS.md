# Final Status

## Decision

SYNTHESIS_V2_BLOCKED_PENDING_PROTOTYPES

Production implementation authorized: NO.

The architecture is internally converged: all known V1 contradictions and all 44 numbered audit findings have a disposition, a V2 decision, or an explicit release gate. It remains blocked because core properties depend on provider behavior and hostile-environment experiments that documents cannot prove.

## What is frozen

- The technology stack and PostgreSQL-only authority model.
- The component and trust-boundary split.
- Exact Git subject identity and fail-closed check publication.
- Deterministic canonical binding and protocol-version rules.
- Evidence, independence, sandbox, budget, approval, publication, tenancy, privacy, revocation, and recovery invariants.
- The P0-01 through P0-10 evidence program and its stop conditions.

## What is not frozen

- Concrete microVM products and OS-specific containment implementations.
- Whether subscription-backed Codex or Claude modes can meet the safe harness contract.
- Provider-specific equivalence for normal, squash, rebase, and merge-queue candidates.
- Exact capacity numbers, regional vendor, IdP, retention periods, and customer plan limits.

Those choices are capability selections constrained by the frozen contracts. They cannot weaken an invariant.

## Unblock rule

Every P0 gate must have a reproducible artifact, independent review, named owner, environment identity, protocol version, and explicit pass or fail. A partial pass unlocks only the demonstrated capability cell. No aggregate score, waiver by silence, or documentation-only assertion authorizes production.

