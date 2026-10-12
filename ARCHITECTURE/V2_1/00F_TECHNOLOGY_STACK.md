# Frozen Technology Stack

| Layer | Required choice | Boundary |
|---|---|---|
| Backend | Rust, Axum, Tokio, SQLx, Serde | One modular-monolith codebase; API, scheduler, outbox, gate, and authority brokers may deploy as separate roles |
| Database | PostgreSQL | Sole authoritative state, jobs, inbox/outbox, leases, fencing, budget ledger, RLS, audit index |
| Web | Next.js, React, TypeScript | Human review/approval UX; no server-side ability to mint approval without Approval Authority |
| Runner | Rust | Customer-controlled worker, cross-platform protocol, capability-advertised execution |
| Serialization | Deterministic CBOR for signed/bound records; JSON for human-facing APIs | Independent Rust and TypeScript implementations must match golden vectors |
| Identity | OIDC for user/session identity; WebAuthn for approval; mTLS/workload identity for components | Provider identities remain stable external IDs; no commit-author substitution |
| Git | Provider APIs through isolated Git brokers | Review/check and publication Apps are distinct; least permissions and typed operations |
| Artifacts | S3-compatible immutable object store is optional for retained artifacts | Not an authority; object metadata and digest are indexed in PostgreSQL |
| Observability | OpenTelemetry-compatible traces/metrics plus typed audit events | Raw boundary streams require separate secret/source controls |

## Explicit exclusions

No required Redis, Kafka, NATS, RabbitMQ, general workflow engine, service mesh, authoritative search index, cross-tenant result cache, or second primary business database. ADR-023 requires a two-copy synchronous external-effect journal solely for recovery/send knowledge; it is not a second business authority or general queue. Such components may be added only after measured need and must remain rebuildable projections.

## Dependency posture

Use standard library and existing stack capabilities first. PostgreSQL constraints, RLS, advisory locks where appropriate, SKIP LOCKED jobs, and transactional outbox replace speculative infrastructure. Security boundaries that need separate credentials or process identity remain separate even when that adds a deployment role.

