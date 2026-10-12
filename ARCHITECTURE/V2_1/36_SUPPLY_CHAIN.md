# Supply Chain

## Build and release

Rust, Next.js, runner, sandbox images, tools, and broker artifacts are built from pinned lockfiles in isolated CI. Releases produce SBOMs, provenance attestations, checksums, vulnerability results, and signatures. Deployment verifies signatures and digests; branch names/tags are not sufficient.

## Dependency policy

Minimize dependencies, especially in privileged brokers and canonicalization. Provider SDKs do not bypass typed positive interfaces. Critical parsers, CBOR canonicalization, Git object validation, WebAuthn verification, SQL migrations, and cryptographic libraries receive focused review and version pinning.

## Tool registry

Analyzer/tool images are immutable by digest with declared OS/architecture, permissions, network needs, output schema, and supported repository languages. Repository-controlled configuration cannot select an unregistered binary or weaken sandbox tier.

## Runner distribution

Runner releases are signed separately from control-plane releases. Enrollment records exact version and binary digest. Rollback is allowed only to a compatibility-listed nonrevoked version. Auto-update remains out of scope until a signed updater and interruption/rollback design passes review.

## Compromise response

Revoked release/tool/model-adapter bundles stop new scheduling through capability generation. Already accepted verdicts and published patches are not silently rewritten; an impact-analysis job marks affected immutable identities, publishes corrective provider state where possible, and notifies tenants. New evidence supersedes, never erases.

## Third-party services

Provider permission and terms changes are compatibility events. Model subscription integration, Git merge/check behavior, KMS/Object Lock semantics, and sandbox platform changes require renewed provider/P0 evidence.

## V2.1 dependency and analysis trust

Build dependencies follow 55_DEPENDENCY_ACQUISITION.md: public/private registries, lockfiles, exact Git dependencies, toolchains and package-manager adapters each need bounded authenticated grants and offline execution. Repository metadata cannot select credential authority, bypass destination policy or execute inside a privileged broker. Artifact caches remain tenant scoped and rehash before use. Analyzer bundle/configuration/scope are pinned in ExpectedAnalysisManifestV1, and revoked bundles invalidate both future receipts and dependent historical current evaluations through CheckProjectionV2 reconciliation. Ordinary API rows cannot substitute a trusted bundle or oracle.
