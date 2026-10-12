# ADR-011: WebAuthn Approval Authority

Status: ACCEPTED_DIRECTION.

## Context

An ordinary authenticated API could manufacture an approval row, and V1 approval did not bind every effective publication field.

## Decision

A distinct Approval Authority controls the RP ceremony and signing key. The browser performs WebAuthn user verification directly for a server-issued plan-bound challenge. HumanApprovalAssertionV2 binds the exact tenant/repository/source/candidate/patch/verification/policy/destination/effects/approver/expiry/nonce/generation/version tuple.

## Consequences

Publication requires compatible authenticators and an additional recoverable service/key ceremony. API compromise alone cannot mint approval.

Gate: P0-09.


## V2.1 amendment

The preceding text is retained as historical V2 rationale. Its effective V2.1 semantics are amended by [ADR-018-TRUSTED-APPROVAL-AUTHORITY.md](ADR-018-TRUSTED-APPROVAL-AUTHORITY.md). In particular, create-only is gateway-enforced rather than intrinsic token authority; trusted AA display and scoped credential lifecycle are required; patch bytes remain in pinned custody and every external effect rechecks authority. See 14, 52, 53 and 54.
