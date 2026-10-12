# A2 Identity and Secrets

## Charter

Own OIDC/user/tenant membership, provider actor links, EligibilityProofV2, component enrollment/session/generation, workload identity, KMS/HSM policy, per-tenant artifact grants, rotation/revocation, and break glass.

## Required properties

Stable IDs over names/emails/authors; current eligibility proof; distinct component authorities; supervisors cannot impersonate children; authority_epoch UUID and generations typed; KMS context not tenant authorization; no reusable secret in guest/audit/artifact.

## Dependencies and gates

ADRs 003, 005, 010, 011, 013, 014. P0-04, P0-06, P0-09, P0-10. Implementation status: NOT_AUTHORIZED.

## Review handoff

Provide identity threat matrix, enrollment/custody/rotation/revocation drills, provider transfer/actor change, KMS policy denial, artifact crypto-erasure, break-glass tests, and generation/epoch vectors.

