# Patch Approval and Publication

## Immutable verified plan

PatchCandidateV2 contains exact base tree, canonical patch bytes/manifest digest, generated-by identity and AttemptBindingV2. The clean-room verifier applies it without Git publication authority and emits VerifiedPublicationPlanV2, binding tenant/stable repository/ownership epoch, exact base commit/tree and candidate tree, patch and changed-object manifest digests, verification/policy/capability/protocol digests, exact destination ref and typed effects, expiry and component generations. Commit identity/message/parent/timestamps and PR title/body/base/draft/provenance are fixed plan data, not planner choices after approval. Exact normative field order and signed envelopes are in SCHEMAS/REGISTRY.json.

## PatchCustodyV1

A patch remains reconstructible through verification, trusted display, publication and reconciliation. Before plan issuance, the custody authority stores a content-addressed immutable bundle containing canonical patch bytes, complete changed-object bytes and manifest, and required base material for rendering/reproduction. The manifest explicitly represents path bytes, object algorithm/OID, mode, addition/deletion and symlink/binary content; unsupported path/object features reject instead of lossy text conversion. Verifier validates every object's digest/size and applies the complete manifest to the exact base; custody is then pinned by publication ID. Same Git tree equality does not license substituting different displayed patch bytes.

PatchCustodyV1 binds tenant/repository epoch, bundle/artifact digests and byte counts, patch/manifest digests, storage location and residency, tenant encryption key reference, access policy, retention deadline, legal hold, state and pinned publication IDs. Store write-once objects under tenant-scoped content identity; no mutable URL alias is authority. Authenticated reads carry tenant, audience, artifact identity, operation and expiry; AA and gateway hash retrieved bytes before use. A signed retrieval URL is transport only, never evidence that bytes are trusted. The ordinary API may request a read but cannot replace bytes, change pins or edit custody authority records.

Default customer-local mode keeps P3 bundle in customer-controlled encrypted durable custody accessible through an authenticated read broker to the independently controlled AA display and gateway; it is not a temporary sandbox directory. AA/gateway may transiently process exact approved source-derived bytes, disclosed in residency policy. Managed durable custody requires explicit tenant opt-in to region, retention and source transfer. If policy forbids required transfer, or no durable readable custody is configured, review still works but publication is unavailable. No hidden cloud retention is assumed.

Minimum retention is through latest plan/approval expiry plus seven days, extended while any publication effect is STARTED, UNKNOWN or MANUAL_RECONCILIATION. A pin is released only after all effects are terminal with durable reconciliation and audit receipts. Legal hold can extend retention; it cannot authorize publication. Tenant quota checks occur before pinning; inability to reserve custody makes patch unavailable, never silently evictable. Indefinite uncertainty can require indefinite hold and is shown to tenant operators for resolution.

Deletion request before publication revokes plan/challenges/assertion and prevents new effects; once all effects settle it releases pins and deletes eligible objects/keys with metadata-only tombstone. During uncertain effects deletion is DELETION_PENDING and the UI states the hold. If legally mandatory deletion overrides a hold, security authority stops publication and records LOSS_OF_RECONCILIATION_EVIDENCE; remaining effects stay UNKNOWN/MANUAL_RECONCILIATION, never assumed absent. LOST/corrupt bytes fail closed at display, verification or send; reacquisition requires exact digest confirmation and fresh verification/approval if any authority expired. No patch regeneration under an old approval. See 24 and ADR-025.

## Human authorization

AA, on its separate controlled origin, retrieves and renders canonical custody bytes itself, then performs fresh scoped WebAuthn UV under 52. HumanApprovalAssertionV2 binds exact plan/custody/display/ceremony, active authenticator generation, stable approver, tenant/repository, authority epoch, policy, effects and expiry. Ordinary API or account recovery cannot mint approval. One assertion nonce is consumed into one publication ID; repeated admission with that same immutable identity is idempotent, never a second publication.

DISPLAYED = APPROVED = VERIFIED = PUBLISHED means exact trusted rendering inputs and signed candidate bytes/tree/effects are bound. It does not assert the authenticator rendered the diff or prove human comprehension. Binary and large changes require a complete review path.

## Per-effect publication and revocation

A credential-free Publication Planner proposes work to the independently deployed Git Publication Capability Gateway. The gateway validates current plan, AA assertion and binding, custody, policy, scope/epochs, effect kind/request digest, lease expiry and live revocation state before EACH EffectIntentV2 STARTED. Parent PublicationOperation is coordination only and grants no future call. Every call is journaled durably before send under 50; unconfirmed durability means DO NOT SEND. GitPublicationCapabilityV2 narrows that single effect as defined in 53.

The gateway creates approved changed blobs/tree, confirms returned candidate-tree equality, creates the exact deterministic commit, creates the exact fix ref, and creates the fixed provenance-marked PR. It never updates a ref or merges. Each step awaits prerequisite confirmation. Revocation after commit but before ref may leave orphan objects; the ref/PR do not start. Already-started individual requests may complete and must reconcile, but do not authorize the remaining sequence. Publication token/gateway compromise retains provider-level residual authority described in 53.

Timeout or ambiguous response becomes UNKNOWN then MANUAL_RECONCILIATION if unresolved. Read-only queries compare exact object/ref/PR identities and request provenance. A momentary absence does not prove non-execution or allow retry; terminal non-execution requires provider-specific evidence under 50. Custody stays pinned throughout.

## Self-review loop

Fix PRs carry immutable Mastercoder provenance. Automatic fix generation on those PRs is disabled by default. Policy may permit at most one additional cycle with fresh plan, verification and human approval.
