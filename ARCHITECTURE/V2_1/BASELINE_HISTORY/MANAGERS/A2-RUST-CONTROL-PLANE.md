# A2 Rust Control Plane

## Charter

Own the Rust/Axum/Tokio/SQLx/Serde modular monolith, request transactions, policy/currentness orchestration, pure gate invocation, and role configuration.

## Inputs and outputs

Consumes canonical contracts 18-21, identity/tenant context, PostgreSQL workflow, and broker interfaces. Emits authoritative transactional state plus typed outbox intent. It owns no provider/model credential, ApprovalProof signer, or Git effect.

## Required properties

Tenant context before SQL; idempotency for mutations; network-free transactions; typed unsupported/conflict/ambiguity errors; no caller-selected check conclusion; no local canonical variants.

## Dependencies and gates

ADR-001/002/005/013/014. P0-06, P0-08, P0-10. Implementation status: NOT_AUTHORIZED.

## Review handoff

Provide module authority map, SQL role matrix, transaction boundaries, pure decision vectors, API threat cases, and N/N-1 rollout evidence to A4 Contract Consistency, Security, and Operability.

