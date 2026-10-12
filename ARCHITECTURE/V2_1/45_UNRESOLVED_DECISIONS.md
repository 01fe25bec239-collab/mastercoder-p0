# Unresolved Decisions

These choices do not alter invariants and may be made after evidence:

| ID | Decision | Constraint / owner |
|---|---|---|
| U-001 | Concrete Linux/macOS/Windows S2/S3 backend | P0-02; Runner/Sandbox manager |
| U-002 | Enabled Codex/Claude API/subscription cells | P0-01 plus terms; Model manager |
| U-003 | Enabled GitHub merge modes/ruleset postures | P0-03; Git manager |
| U-004 | Checkout repository/LFS/submodule envelopes | P0-05; Git/Runner managers |
| U-005 | Cloud, primary/recovery region, artifact vendor | P0-10; Reliability manager |
| U-006 | OIDC provider and enterprise federation set | Identity manager; stable-subject requirements |
| U-007 | Artifact/transcript retention defaults | Privacy/product/legal; raw default remains off |
| U-008 | Initial tenant plan limits and SLOs | P0-08/P0-10; Reliability/product |
| U-009 | Signing/KMS/HSM products and key ceremonies | Security validation; separate policies required |
| U-010 | Runner updater design | Supply-chain review; manual install is baseline |
| U-011 | Future managed private-source execution | New architecture review; out of scope |
| U-012 | Future GitLab/Bitbucket providers | Provider-specific exact-subject and effect proof |

The following are resolved and not open choices: Rust stack, PostgreSQL sole authority, no required cache/broker, distinct Git authorities, no guest credentials, exact binding, S2 hostile default, WebAuthn approval, publication tree equality, RLS/tenant epochs, fail-closed required checks, and no implementation before gates.

