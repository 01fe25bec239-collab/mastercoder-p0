# Git Integration

## Provider identities

GitHub is integrated through at least two separate Apps:

- Review App: repository/PR metadata read, checks read/write, and only permissions proven necessary by P0-03.
- Publication App: provider permissions necessary for Git objects/refs and fix PRs, with no check authority. Its token resides only in the independent Git Publication Capability Gateway. The credential-free planner is create-only constrained by 53; the underlying provider token has broader residual authority and is not described as inherently create-only.

Checkout acquisition may use the Review App or a narrower dedicated App, but its token remains inside the acquisition broker. Installation tokens are narrowed and short lived.

## Inbound truth

Webhooks are untrusted notifications. The Review Broker verifies delivery authentication, records the raw delivery digest, then reads current provider state. Stable provider repository and actor IDs are authoritative identifiers; mutable names are display metadata. A repository transfer creates a new repository_tenant_epoch after explicit reconciliation.

## MergeSubjectV2

The subject includes provider repository ID, PR number, head OID, base ref, current base OID, merge method, candidate kind, candidate commit OID, candidate tree OID, provider event/generation, and repository_tenant_epoch. Merge queue work binds the merge_group candidate. Normal/squash/rebase candidates are enabled only where P0-03 proves exact or provider-equivalent identity.

## Required check

The canonical check name is Mastercoder Review. The broker derives the conclusion from a signed gate decision and current provider subject; the caller cannot supply it. Missing, partial, unsupported, stale, canceled, ambiguous, neutral, or skipped required work maps to a non-permitting failure. Before publishing each check effect, the broker re-reads current subject and rejects stale EvaluationIdentityV2. CheckProjectionV2 in 54 defines confirmation, invalidation, withdrawal, duplicate and delayed writes, shared-SHA associations, normal staged policy activation and emergency freeze. Internal revocation is immediate; provider revocation is not. Enforcement setup requires proven check App pinning and freeze posture; an unconfirmed withdrawal/freeze is visibly EXTERNAL_GATE_UNSAFE.

## Checkout

A logical CheckoutGrantV2 authorizes a finite sequence of typed object/pack requests within OID, byte, object-count, call-count, and deadline bounds. The parser receives bytes, never a token. Ambiguous grant use terminates it; a new explicit grant with a fresh nonce is required. LFS and submodules are separate capabilities.

## Publication

Publication uses Git data/object operations against a bound base_tree and changed-object manifest. The provider-returned tree must equal approved candidate_tree before commit/ref creation. The independently controlled gateway allows only the exact deterministic fix ref `refs/heads/mastercoder/fix/<publication_id>` and approved PR effects. No merge/update/delete or arbitrary endpoint is allowed. Every provider call has its own EffectIntentV2 and durable journal receipt; future effects recheck approval/revocation. Provider ambiguity enters reconciliation without blind replay. Patch bytes stay pinned under PatchCustodyV1 (14/24). See 53 for body/endpoint validation and stolen-token residual authority.


For checkout/dependency egress, each actual HTTP call uses AcquisitionRequestV1 with exact broker-derived origin/method/target/body and selected OID/package, bounded response size and grant_digest. EffectIntentV2.canonical_request_digest hashes this per-call request, not the aggregate grant. Authorization proof still references the finite grant. No credential value is included in persisted request bytes.
