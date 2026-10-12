# ADR-013: Tenant Epochs, RLS, and Artifact Grants

Status: ACCEPTED_DIRECTION.

## Context

KMS context is not tenant authorization, provider repository ownership changes, and cross-tenant cache/value paths threaten isolation.

## Decision

Use composite tenant keys/FKs, deny-by-default RLS with transaction-local tenant context, immutable repository_tenant_epoch, tenant-scoped caches only, and per-tenant artifact encryption grants/DEKs.

## Consequences

Queries and operations carry tenant structurally. Repository transfer starts a new epoch; historical data never follows mutable ownership. Operational administration requires explicit tenant batches.

Gate: P0-10.

## V2.1 amendment — finding 5

RLS and tenant context remain necessary but do not prove record provenance. Dedicated non-owner/NOBYPASSRLS writer roles, immutable signed receipts, independently checked producer scope/key roles and protected gate transitions are mandatory under ADR-019 and document 07. The ordinary API has no write/EXECUTE path to gate-critical inputs; a caller-set tenant GUC is not the authentication boundary. Table ownership and migration privileges remain offline and separated.
