# Competing Architecture Matrix

| Concern | OPUS V1 contribution | SOL V1 contribution | V2 decision |
|---|---|---|---|
| Control plane | Strong Rust/customer-execution orientation in the original corpus; some package drift | Clear modular-monolith and PostgreSQL-only authority | Rust modular monolith with independently deployable roles from one codebase |
| Durable workflow | PostgreSQL jobs, outbox, leases, fencing | PostgreSQL sole authority, transactional budget and outbox | Retain both; no required Redis, Kafka, or workflow engine |
| Git authority | Exact SHA lineage, but authority was sometimes shared | Review and publisher separation, but publisher lacked a positive source path | Distinct Review Broker and Publication Effect Broker; verified tree plan supplies the missing path |
| Merge truth | Head and merge-base emphasis | Typed subject identity, but ordinary-PR candidate ambiguity | MergeSubjectV2 binds exact candidate commit and tree; unsupported equivalence is non-permitting |
| Model access | Host brokerage and pinned harness intent | Strong credential/parser separation | Structured multi-turn ModelTurnV2 through an external secret dialer; no guest reusable credential |
| Sandbox | Tiered containment, with disputed Linux shared-kernel T2 | Capability-based cross-platform runner | S0-S3 tiers; hostile untrusted execution requires a VM-grade boundary by default |
| Evidence | Closed grades and claims-not-facts | Independent receipts and immutable verification sets | C0-C4 independence plus causal evidence rules; signatures prove origin only |
| Attempt identity | Digest-bound workflow intent | Full binding, but circular and inconsistent | Deterministic CBOR WorkflowSpecCoreV2 and non-self-referential AttemptBindingV2 |
| Human approval | Digest-specific UI intent | Immutable exact patch approval, but API could forge authority | Direct WebAuthn Approval Authority signs an exact publication tuple |
| Publication | Isolated publisher intent, but target/revocation gaps | Create-only fix publication, but no materialization path | Verifier-sourced PublicationPlanV2; base tree plus changed objects; returned tree must equal approval |
| Budget | Reservations and metering | Transactional ledger, but stale spend rejected | Root resource envelope plus late-settlement ledger and debt/freeze semantics |
| Tenancy | Structural isolation intent | RLS and local-first custody | Composite tenant keys, RLS, repository tenant epochs, per-tenant encryption grants |
| Privacy/audit | Bounded evidence and audit chain | Opt-in raw artifacts and immutable log | Metadata/digest audit only; immutable facts never post-hoc redacted; artifacts separately erasable |
| Cross-platform | Customer Rust runner | Capability matrix and explicit unsupported states | OS support is separate from native-project verification support; no silent downgrade |

## Rejected V1 patterns

V2 rejects shared check/publication signers, credentials inside hostile guests, caller-selected required-check conclusions, head-only merge truth, shared-kernel containment as the hostile default, producer-asserted independence, path denylists as complete policy, fixed opaque model envelopes, one-HTTP-call checkout contracts, mutable approval targets, current-binding-only billing, and claims of post-hash audit redaction.

## Central synthesis

The winning unit is not one source architecture. It is a narrow Rust/PostgreSQL control core surrounded by small authority-specific brokers, with all security-relevant decisions bound to one canonical subject and all external effects independently rechecked at the last responsible moment.

