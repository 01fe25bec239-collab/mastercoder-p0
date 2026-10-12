# ADR-016: Git publication capability enforcement

Status: ACCEPTED_V2_1_CONTRACT; capability qualification pending atomic prototypes.

Finding 1: V2 App separation remains sound, but Contents write also permits broader reference mutation. Amend ADR-003 with an independently deployed credential-bearing gateway and credential-free planner. Gateway constructs only exact approved single-effect requests; no merge/update/delete/arbitrary endpoint. Provider no-bypass rulesets, selected repositories and short token lifetime limit residual authority, but gateway/token compromise remains broader than planner compromise. Normative contracts: 53, 14, GitPublicationCapabilityV2 and EffectIntentV2. Validate denied operations, credential theft and each-step revocation in P0-03/P0-09.

Rust/Axum/Tokio/SQLx/Serde, PostgreSQL, Next.js/React/TypeScript and Rust runner remain unchanged. No product implementation or prototype execution is authorized by this ADR.
