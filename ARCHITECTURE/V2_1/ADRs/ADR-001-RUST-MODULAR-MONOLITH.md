# ADR-001: Rust Modular Monolith

Status: ACCEPTED_DIRECTION.

## Context

The source packages drift between multiple deployables/languages and a modular monolith. The user froze Rust, Axum, Tokio, SQLx, and Serde. Most control transitions share PostgreSQL transactions; speculative services add distributed failure without authority benefit.

## Decision

Use one Rust modular-monolith codebase for API, scheduling, workflow, analysis orchestration, ledger, audit, and pure gate logic. Deploy roles separately where workload scaling helps. Create separate security workloads only for Review, Checkout acquisition, Model secret dialing, Approval, Gate signing, Publication, and hostile execution.

## Consequences

Business contracts remain local and transactional. Security boundaries still require distinct workload identity, key/permission, egress, and protocol. A process split without those controls is not accepted as isolation.

Gate: P0-06/P0-10 for contract and operational proof.

