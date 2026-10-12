# Privacy and Data Classification

| Class | Examples | Default location/retention |
|---|---|---|
| P0 Public metadata | Public repo coordinates, public tool versions | Control plane, policy retained |
| P1 Tenant metadata | Stable IDs, policy, state, digests, usage | PostgreSQL, tenant scoped |
| P2 Source-derived bounded | Paths, line numbers, fingerprints, minimal excerpts, tool summaries | Excerpts opt-in/explicit; encrypted and time bounded |
| P3 Raw source/patch/prompt/output | Repository files, complete diff, model turns, logs | Execution ephemeral by default; approved publication bundles require pinned durable tenant custody; managed custody opt-in |
| P4 Secret/authentication | Tokens, keys, WebAuthn private material, credentials | Dedicated broker/KMS/authenticator only; never artifacts/audit |

## Canonical audit

AuditFact contains actor/component IDs, operation, subject and result digests, state transition, provider/object identities, times, policy/generation/version, and error class. It excludes raw source, excerpts, patches, prompts, model output, secrets, untrusted stdout/stderr, and arbitrary exception text.

Hash-chain and compliance-mode immutable audit facts are never redacted or rewritten. A correction is a new linked fact. Remote WORM/export is a durability control, not permission to promise later deletion of immutable facts.

## Artifacts

Artifacts are separate encrypted objects referenced by digest. Tenant policy controls class, region, size, retention, legal hold, and reviewer access. Per-tenant DEK/grant boundaries make denial and crypto-erasure meaningful. Deletion records a tombstone fact; it does not rewrite historical audit.

## Source residency

The default is customer-runner execution with no raw source retention in the managed control plane. However, any configured excerpts, finding text, prompts, tool output, patch data, or crash/telemetry content is source-derived and disclosed as such. Marketing and UI report the actual policy and observed transfer.

## Logging

Typed application events are the safe path. Boundary middleware, SDKs, subprocess streams, stderr, and crash dumps are separately configured with allowlists, size limits, secret/source detection, and no-body defaults. V2 does not claim the type system alone covers them.

## Data subject and tenant lifecycle

Tenant deletion closes access, applies legal-hold rules, removes eligible artifacts/keys, and retains only legally required metadata facts. Repository transfer does not transfer historical artifacts; the old tenant epoch remains isolated.


## Publication custody exception

PatchCustodyV1 (14) requires exact immutable patch/changed-object bytes and rendering material through approval, publication and final reconciliation. Customer-local durable encrypted custody is the default publication profile; managed durable custody is a disclosed opt-in. AA/gateway transient reads still transfer source-derived content and must match tenant residency policy. A tenant that disallows the necessary access retains review but cannot publish. Pins prevent normal retention/deletion/crypto-erasure while effects remain uncertain; show DELETION_PENDING and operator ownership. Legally compelled erasure records evidence loss and leaves unresolved effects manual, never fabricated completion. Authenticator private keys never enter custody; credential public keys/ceremony evidence are restricted authentication metadata with independent retention. See 52 for registry access and ADR-025.
