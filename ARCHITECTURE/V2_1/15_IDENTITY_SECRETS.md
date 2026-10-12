# Identity and Secrets

## Human identity

OIDC authenticates users and maps them to a tenant membership plus stable linked provider actor IDs. Display names, emails, commit authors, and provider handles are mutable metadata. Privileged actions require recent authentication; patch publication additionally requires WebAuthn user verification on the separately controlled Approval Authority origin using an active ApprovalCredentialBindingV1. Ordinary OIDC/session/account recovery never creates approval authority. Enrollment, additional credentials, replacement, scope changes, suspension/revocation and recovery use the independent authority and lineage rules in 52; AA alone can write its credential registry.

## Author eligibility

MY_PRS_ONLY means the current PR opener’s stable provider actor ID equals the authenticated user’s linked stable provider ID. ORG_MEMBERS requires a current membership/role proof fetched by the Review Broker within a policy TTL. Automatic modes require an explicit service-policy principal and current eligibility proof. Failure to prove eligibility disables the action.

EligibilityProofV2 binds repository, repository_tenant_epoch, actor, proof source/time/expiry, policy digest, and provider generation. It is included in reservation and publication inputs.

## Component identity

Every runner, broker, gate, dialer, injector, verifier, and dispatcher has:

- enrolled workload identity and permitted role
- short session lifetime and audience-bound authentication
- credential_generation and configuration/capability digest
- rotation, quarantine, and revocation state
- audited supervisor and key-custody boundary

A supervisor identity cannot impersonate a child role merely because it launches the process.

## Secrets

Provider App private keys and approval/gate signing keys reside in separately authorized KMS/HSM policies. Installation/access tokens are short lived and stay inside their credential-bearing broker/gateway. Publication Planner has no provider key/token, mint permission, egress or authority to change gateway configuration; Git Publication Capability Gateway alone holds the Publication App token. Review and Publication Apps remain separate. A stolen token retains its actual provider permissions despite planner restrictions. Model credentials stay in the Secret Dialer. Runner enrollment keys do not enter sandboxes.

KMS encryption context is defense in depth, not tenant authorization. Tenant artifacts use a per-tenant data-encryption key or grant boundary so access denial and crypto-erasure have a real unit. RLS and application authorization still apply.

## Authority epochs and generations

authority_epoch is a random 16-byte UUID, equality-only, rotated for fleet/tenant authority reset. lease_generation and credential_generation are monotonic unsigned counters scoped to their object/component. They are never coerced into each other or encoded as both integer and UUID.

## Break glass

Break-glass operations use a separate role, two-person approval, bounded tenant/operation scope, short expiry, and immutable audit. Break glass cannot mint a WebAuthn approval on behalf of a user or bypass exact publication equality.


## Eligibility at the protected boundary

ResourceEnvelopeV2 and GitPublicationCapabilityV2 bind eligibility_proof_digest to EligibilityProofV2. Before reservation and every publication effect STARTED, the protected authority resolves that signed proof, verifies its issuer, tenant/repository/epoch, exact PR and stable opener/linked user, membership generation, policy digest and DB-time expiry, and obtains current provider/membership confirmation when required by the policy freshness bound. MY_PRS_ONLY/ORG_MEMBERS restrictions cannot be replaced by approver scope. Mismatch, unavailable required read or changed eligibility blocks the effect; a renewed proof must bind the same exact permitted subject and current policy.
