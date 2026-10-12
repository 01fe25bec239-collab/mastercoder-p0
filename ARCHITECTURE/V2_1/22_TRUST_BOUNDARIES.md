# Trust Boundaries

| Boundary | Untrusted input | Privileged output | Required control |
|---|---|---|---|
| Browser → API | User JSON/session/CSRF | Control intent | Auth, tenant context, validation, idempotency |
| Provider → Review Broker | Webhook/API state | Normalized subject/check | Delivery auth plus authoritative read-back |
| Checkout Broker → Parser | Git bytes | Materialized tree | Token isolation, OID/size/call bounds, object verification |
| Repository → Sandbox | Arbitrary files/code | Execution receipt/artifact | S2/S3, no secrets, egress/resource limits |
| Sandbox → Model Broker | Prompt/tool content | Provider request/usage | ModelTurn schema, policy, root envelope |
| Model Broker → Secret Dialer | Canonical request | Credentialed egress | Fixed provider/account, no arbitrary URL/header |
| Runner → Verifier | Signed claims | Qualified evidence | Clean-room reproduction and C-class policy |
| Ordinary UI → Approval Authority origin | Plan identifier/navigation | HumanApprovalAssertionV2 | Independent plan fetch, custody rendering, isolated origin/assets/session, registered scoped UV credential; see 52 |
| Approval administrator → credential registry | Enrollment/recovery/scope request | ApprovalCredentialBindingV1 | Independent bootstrap authority, UV, recovery lineage, generation and audit |
| Gate/Outbox → Review Broker | Signed gate proof and projection intent | Required check | Independently authenticated evidence authority, generation/currentness, per-effect journal, read-back and freeze protocol (54) |
| Planner → Git Publication Capability Gateway | Publication/effect ID | Git objects/ref/PR | Separate credential/config/egress authority; exact body/endpoint, current approval/custody/epoch, per-effect journal (53) |
| Custody broker → AA/Gateway | Immutable source/patch bundle | Rendered diff or Git bytes | Tenant scoped authenticated reads, full hash/size checks, retention pin (14) |
| App → PostgreSQL | SQL/tenant session | Authoritative state | SQLx parameters, RLS, composite keys, constrained role |
| App → Artifact store | Source-derived bytes | Retained object | Per-tenant encryption grant, digest, policy, size, lifecycle |
| Component N ↔ N-1 | Versioned messages | State/effect | Negotiation, critical fields, pinned version, no downgrade |

## Non-boundaries

Process names, containers, TLS alone, KMS encryption context, signatures alone, and “internal network” labels are not sufficient trust boundaries. Each boundary is defined by identity, permission, positive protocol, egress, data path, generation, and validation.

## Compromise containment

- Ordinary API compromise cannot mint AA approval, enroll/recover approval credentials, write gate-critical authority records or make provider effects; independent role/write matrix and signatures are required, not trusted-looking database rows.
- Review Broker compromise cannot write contents or publish patches.
- Compromised Publication Planner is confined by independent gateway to exact approved effects. Compromised credential-bearing gateway or stolen token retains actual provider permissions; rulesets, no bypass, selected repositories, TTL and token revocation bound that residual (53).
- Runner/sandbox compromise cannot obtain reusable provider/model secrets or make a gate decision.
- Model dialer compromise can misuse its provider account within outer egress/account limits, but cannot obtain Git credentials or database authority.
- Database compromise remains catastrophic for control truth; approval WebAuthn and separate provider/KMS authorities limit silent external effect completion but do not eliminate recovery.

