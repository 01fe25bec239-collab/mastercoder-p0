# Assumptions

| ID | Assumption | Type | If false |
|---|---|---|---|
| A-001 | PostgreSQL can meet declared workload with partitioning/indexing before extra infrastructure | Load | Reduce limits/scale database; architecture remains |
| A-002 | Customer can operate an outbound Rust runner and supported sandbox | Product | Tenant cannot use runner-dependent capabilities |
| A-003 | GitHub exposes enough read/check/object/ref APIs with least permissions | Provider | Disable affected integration/effect cell |
| A-004 | WebAuthn UV is available to target users and browser origins | Product/provider | Publication disabled; patch export only |
| A-005 | At least one model auth mode can satisfy ModelTurnV2 and terms | Prototype/provider | Model-assisted review disabled |
| A-006 | At least one VM-grade hostile sandbox is feasible per supported OS | Prototype | OS/tier disabled or S3 required |
| A-007 | Exact base/candidate objects remain retrievable within bounded windows | Provider/product | Evaluation/publication expires and must restart |
| A-008 | Stable provider repository and actor IDs persist through rename | Provider | Integration disabled until replacement identity proof |
| A-009 | Provider usage can eventually be reconciled by request/account/time identity | Provider | Paid model execution disabled or prepaid hard request caps |
| A-010 | Separate workload identities, egress, and KMS policies are deployable | Security/operations | Privileged feature disabled; no shared-authority fallback |
| A-011 | S3-compatible artifact storage provides immutable-object/lifecycle primitives | Provider | Store locally/alternate vendor; never treat as authority |
| A-012 | Deterministic CBOR and SHA-256 are supported across Rust/TypeScript | Mechanical | Replace before protocol freeze; P0-06 |

Assumptions A-003 through A-010 are architecture or capability blocking. They are not asserted true by this document; their designated gate or provider validation must establish them.

Safe assumptions include duplicate delivery, eventual runner reconnect, rebuildable projections, explicit unsupported capability, and the availability of standard Rust/PostgreSQL/Next.js tooling. Safe does not mean untested at production load.

