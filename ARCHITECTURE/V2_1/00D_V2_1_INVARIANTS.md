# V2.1 mandatory invariants

These preserve V2 INV-01–20 except the explicitly amended semantics below. They are design obligations; production proof is still required.

| ID | Normative invariant |
|---|---|
| INV-01 | PostgreSQL remains transactional business authority for work, policy, ownership and budgets. A synchronously witnessed journal independently preserves all possible external sends across business-DB restore; it cannot mint business authorization. |
| INV-02 | Gate-affecting data binds tenant, stable repository/ownership epoch, exact subject commit/tree, workflow/policy/capability/trust digests, authority epoch and supported protocol. |
| INV-03 | Missing, stale, unknown, unsupported, partial, skipped, neutral or unverifiable required work never produces permitting PASS. Every expected mandatory unit has a qualified completion receipt, including zero findings. |
| INV-04 | Review/check and publication use separate App identities, credentials, egress, signing and DB roles. |
| INV-05 | No reusable trusted credential enters hostile execution. Credential-bearing processes accept bounded typed data only; hostile semantic parsers remain isolated. |
| INV-06 | A runner signature proves origin only; protected authority verifies accepted evidence, current producer trust and independently registered failure domains. Ordinary API cannot forge gate inputs via database writes. |
| INV-07 | Agent output is a claim. Blocking test evidence requires an independently trusted oracle AND applicable causal reproduction; repetition alone never validates expectation. |
| INV-08 | Sandbox tier/capability is bound, with no silent downgrade. Individual/single-domain deployments cannot claim C3. |
| INV-09 | All graph descendants share a broker-enforced root resource envelope; no child mints budget or authority. |
| INV-10 | Only independent Approval Authority mints approval after its own trusted display and fresh WebAuthn UV using an active scoped credential; ordinary account recovery does not restore approval authority. |
| INV-11 | Exact pinned bytes and candidate tree link VERIFIED = DISPLAYED = APPROVED = PUBLISHED; loss/expiry invalidates remaining publication authority. Human comprehension and compromised browser/OS are not cryptographically proved. |
| INV-12 | Credential-free planner is confined by independently administered Git gateway to exact approved create-object/ref/fix-PR operations. Gateway/token compromise retains honest provider-permission residual risk. |
| INV-13 | Lease authority requires owner/generation/epoch/state AND actual DB-clock expiry after lock. Late result or retry never resurrects authority. External ambiguity remains UNKNOWN/manual unless exact outcome or terminal non-execution is proven. |
| INV-14 | Actual provider spend is settled even after cancellation/fencing; debt or admission freeze cannot revive stale results. |
| INV-15 | Tenant isolation is structural: composite keys/RLS, authenticated tenant grants, stable provider IDs and repository ownership epochs. |
| INV-16 | Canonical audit contains metadata only; source, credentials, prompts and patch bytes remain separately governed custody. |
| INV-17 | STARTED authorizes one external call, never future publication steps. Witness durability precedes send. Revocation stops unstarted effects; started provider calls can finish later. |
| INV-18 | Unsupported schema revision, unknown field/enum, missing reference or unproved provider/capability cell fails closed. |
| INV-19 | Customer/local source custody remains default. Dependency credentials stay in broker; network and retention are explicitly bounded/disclosed. |
| INV-20 | No production implementation authorization or capability enablement follows from this architecture package. Independent P0 evidence and subsequent explicit authorization remain necessary. |
| INV-21 | Loss of internal currentness requires provider-check withdrawal/reconciliation. During the unavoidable external stale interval, enforcement is unsafe until provider withdrawal/freeze is confirmed; no instantaneous revocation claim. |
| INV-22 | Manual resolution never converts insufficient evidence directly into deterministic PASS; changed policy/oracle/evidence creates a fresh evaluation. |

State, schema, API, event, threat and prototype propagation is traced for each finding; see TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md.
