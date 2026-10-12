# A4 Security Review

Verdict: PASS_WITH_RELEASE_BLOCKERS. Independence: NOT_AN_EXTERNAL_ATTESTATION.

## Findings

- No known architectural path lets the normal API alone mint WebAuthn approval, publish a check, or mutate Git contents.
- Positive protocols now exist for checkout, model tools, approval, and publication; this closes the V1 pattern of defining only prohibitions.
- Exact tree equality, distinct Git authorities, S2 hostile default, origin-only runner signatures, tenant epochs/RLS, and immutable metadata audit materially improve compromise containment.
- Residual catastrophic risks remain for PostgreSQL, privileged broker/App, Approval/Publisher keys, hypervisor compromise, and colluding C3 domains.

## Blockers

P0-01, P0-02, P0-03, P0-06, P0-07, P0-09, and the isolation/privacy portions of P0-10 require raw adversarial evidence. Any test that exposes guest credentials, approval forgery, permitting incomplete checks, tree/target substitution, stale/fenced acceptance, cross-tenant access, or downgrade fails the release.

Conclusion: the architecture is security coherent; production is not authorized.

