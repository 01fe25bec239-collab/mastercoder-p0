# ADR-017: Check projection and revocation semantics

Status: ACCEPTED_V2_1_CONTRACT; capability qualification pending atomic prototypes.

Finding 2: pre-write currentness cannot withdraw an already successful or delayed check. Add CheckProjectionV2 with generation, provider App/check identity, confirmation and withdrawal, per-call journaling and duplicate/shared-SHA reconciliation. Normal policy activation is freeze-staged; emergency internal invalidation is immediate with explicit external stale-success interval until provider freeze/withdrawal confirms. Provider ambiguity holds freeze. No instantaneous revocation claim. Normative contract: 54. P0-03 and restore P0-10 prove actual provider semantics before enforcement enablement.

Rust/Axum/Tokio/SQLx/Serde, PostgreSQL, Next.js/React/TypeScript and Rust runner remain unchanged. No product implementation or prototype execution is authorized by this ADR.
