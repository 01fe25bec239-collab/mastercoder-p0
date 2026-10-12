# Mandatory Invariants

These invariants override every other document.

| ID | Invariant |
|---|---|
| INV-01 | PostgreSQL is the sole transactional authority for control state, work ownership, budget, policy generations, and effect intent. |
| INV-02 | Every gate-affecting result binds tenant, stable repository ID and tenant epoch, exact source/head/base/candidate commit and tree, workflow digest, policy digest, capability digest, authority epoch, and protocol version. |
| INV-03 | Required work that is missing, stale, ambiguous, unsupported, unknown, partial, neutral, skipped, canceled, or unverifiable never creates a merge-permitting conclusion. |
| INV-04 | Provider check authority and content/publication authority use distinct identities, credentials, egress, processes, and allowed operations. |
| INV-05 | Reusable credentials never enter hostile repository execution. Credential-bearing code never parses untrusted repository or model content. |
| INV-06 | A runner signature proves origin and session identity only. Correctness requires policy-qualified independent evidence. |
| INV-07 | AI output is a claim. Agent-authored tests become gate evidence only after clean-room, candidate-specific causal reproduction. |
| INV-08 | Sandbox tier is a bound capability. No downgrade is permitted; unsupported execution is an explicit non-pass result. |
| INV-09 | All resource-consuming descendants share one broker-enforced root envelope. No child can mint budget, authority, credentials, or stronger capabilities. |
| INV-10 | Human approval is minted only by the independent Approval Authority after direct WebAuthn user verification and binds the exact publication tuple. |
| INV-11 | DISPLAYED candidate equals APPROVED candidate equals VERIFIED candidate equals PUBLISHED candidate by digest and tree identity. |
| INV-12 | The Publication Effect Broker performs only typed create-only object/ref/fix-PR effects; it cannot merge, update default/protected/source refs, or choose a target supplied outside the proof. |
| INV-13 | All external effects are idempotent or reconciled after ambiguity. A fenced or expired attempt can never regain authority through a retry or late result. |
| INV-14 | Actual external spend is always settled to the ledger, including after lease loss. Settlement may create debt/freeze but cannot revive a stale result. |
| INV-15 | Tenant identity is structural: composite tenant keys, transaction-local RLS context, stable provider IDs, repository_tenant_epoch, and per-tenant cryptographic grants. |
| INV-16 | Canonical audit facts contain no raw source, secret, prompt, model output, or patch. Immutable audit facts are never post-hoc redacted. |
| INV-17 | Revocation is generation based. Once an effect crosses STARTED, it may finish and must be reconciled; no document claims atomic revocation of an in-flight provider call. |
| INV-18 | Unknown critical protocol fields, major-version mismatch, unproved provider semantics, or unqualified capability cells fail closed. |
| INV-19 | Local/customer runner custody is the default for private source. Any transmitted excerpt or derived artifact is disclosed, classified, bounded, and policy controlled. |
| INV-20 | Implementation is not authorized until every P0 gate required by the intended production capability matrix passes independently. |

## Proof obligation

Each invariant has an enforcement location and validation method in TRACEABILITY/INVARIANT_PROOFS.md. An invariant without executable or staging evidence remains an open release blocker, not an assumed property.

