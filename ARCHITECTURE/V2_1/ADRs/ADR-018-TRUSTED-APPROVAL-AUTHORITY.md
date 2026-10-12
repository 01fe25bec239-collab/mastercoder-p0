# ADR-018: Trusted approval display and credential lifecycle

Status: ACCEPTED_V2_1_CONTRACT; capability qualification pending atomic prototypes.

Findings 3 and 4: WebAuthn verifies the ceremony, not ordinary UI rendering or entitlement to enroll credentials. Amend ADR-011: independent AA origin owns complete canonical rendering, challenge, RP and registry. Scoped ApprovalCredentialBindingV1 controls registration/add/replace/revoke/recovery; ordinary login recovery cannot grant approval. HumanApprovalAssertionV2 binds trusted display/custody and active credential generation. Recovery is independently authorized with fresh enrollment and revocation lineage. Browser/OS/AA compromise remains residual. Normative contract: 52; P0-09 covers substitution, origin isolation, lifecycle and replay.

Rust/Axum/Tokio/SQLx/Serde, PostgreSQL, Next.js/React/TypeScript and Rust runner remain unchanged. No product implementation or prototype execution is authorized by this ADR.
