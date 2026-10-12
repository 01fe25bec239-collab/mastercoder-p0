# Rust Control Plane

## Preserved shape

The backend remains Rust with Axum HTTP, Tokio concurrency, SQLx PostgreSQL, and Serde JSON. Identity/tenant context, repository normalization, policy/eligibility, workflow/attempts, budgets/ledger, evidence/verification, gate decisions, patch/publication planning, inbox/outbox, and audit remain business modules. API, scheduler, and outbox roles may share a build; authority-bearing workloads have separate identities, database pools, egress, credentials, and deployment policy. Separation is enforced by grants and workload identity, not Rust module visibility.

## Request transaction

Authenticate actor and tenant before opening a transaction; resolve a stable repository/tenant epoch and mutation idempotency key. Lock authority/currentness rows in the order in document 29, validate state, current epoch/generation and actual database time after locks, perform one permitted transition, append audit, and insert outbox rows atomically. A duplicate with identical canonical request returns the original result; the same key with different bytes is CONFLICT. No provider call occurs in the business transaction.

Ordinary API requests express intent. They cannot write gate-critical records, sign receipts, set conclusions, register approval credentials, consume approval nonces, or enter an effect into STARTED. The authenticated authority service validates the request and owns the resulting transition. Runners, agents, sandboxes and ordinary browsers have no database credentials.

## Gate derivation and commitment

The pure Rust reducer consumes immutable canonical inputs and a supplied database-time sample. It has no provider client, clock lookup, mutable configuration fallback, or API-supplied verdict. The Gate Authority first validates every input under the matrix in document 07, checks expected mandatory analysis completion, oracle class and verification requirements, then locks/rechecks current subject, policy, trust and revocation generations. Its committed decision records the complete input digest, immutable evaluation identity and reason codes. Signing occurs over that committed canonical record; the signer independently checks input provenance, commitment and currentness rather than accepting an arbitrary API digest. An outbox projection request references the resulting GateProofV2, never a caller-selected conclusion. Document 54 governs later projection and withdrawal.

## Typed failures and boundaries

Errors distinguish VALIDATION, CONFLICT_CURRENTNESS, UNSUPPORTED_CAPABILITY, UNAUTHORIZED, QUOTA, TRANSIENT_INTERNAL and EXTERNAL_UNCERTAINTY. Only internal pre-dispatch failures may be automatically retried; provider writes follow document 50. HUMAN_REVIEW_REQUIRED is resolved under document 56 and cannot become PASS through a status edit.

No API returns component credentials or signer material. Authoritative PostgreSQL roles are non-owner, non-superuser, NOINHERIT and NOBYPASSRLS, without membership in migration or other authority roles. Privileged pools never share ordinary API connection credentials. SQLx transactions use tenant-scoped keys and verified session/tenant context; critical wire decoders reject unknown fields and wrong versions according to SCHEMAS. No SQL security assumption relies on a Rust type alone.

## Scaling

API replicas remain stateless; PostgreSQL provides durable admission, jobs, leases and backpressure. Connections, provider calls, runner capacity and tenant spend have explicit resource budgets. The separately witnessed effect journal is a durability prerequisite for privileged sends, not a work queue or alternative gate authority. Deterministic FAST review does not depend on model availability.
