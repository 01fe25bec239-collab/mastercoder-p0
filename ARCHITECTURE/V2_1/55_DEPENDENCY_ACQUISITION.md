# Dependency acquisition

Normative V2.1 repair for finding 15. Dependencies are acquired through a bounded authenticated broker or a pre-resolved immutable dependency bundle. No package-manager or registry secret enters repository execution.

## Bound request and resolution

DependencyAcquisitionGrantV1 binds tenant, repository, AttemptBindingV2, authority epoch, lease generation, lockfile digest, ecosystem and package-manager/toolchain versions, exact package identity/version/hash or Git commit, allowed source/registry, credential-authority reference, network destination policy, tenant cache scope, provenance requirements, package/byte/request/time ceilings, expiry and a single-use nonce. SCHEMAS defines the wire record. Grants are minted by a protected dependency authority after policy validation, not by the runner or agent. The broker resolves identities before creating a grant; unpinned dependencies need an explicit resolution operation with an immutable result and re-planning before execution. A repository lockfile is untrusted input, not network authorization.

Each grant contains a closed dependency set including transitive packages. Unknown transitive packages require a new manifest/grant under remaining root budget; the guest cannot add URLs. Digest-less private packages can qualify only by a separately approved immutable manifest with exact fetched content digests and provenance. An unknown checksum, mutable version, floating Git branch, or unverifiable resolution is UNSUPPORTED/BLOCKED for mandatory builds.

## Network and credential boundary

A credential-free bounded resolver parses ecosystem metadata in a constrained process. The privileged acquisition dialer receives only validated typed fetch requests; it selects the pinned credential authority and permitted registry endpoint itself. It never invokes package managers, lifecycle scripts, compilers, Git hooks or archive extraction. Endpoint policy validates scheme (HTTPS), host/port/path family, resolved IP ranges and every redirect; loopback, link-local, cloud metadata, private-network destinations outside explicit private-registry policy, cross-origin credential forwarding and DNS rebinding are denied. Private registries require an explicit tenant-owned endpoint/network policy and dedicated read-only credential; the public default cannot reach them. Credentials are injected only after destination validation and stripped on redirects; cross-origin redirects require a separately allowed unauthenticated/CDN fetch and exact content hash.

Downloads are streamed with bounded compressed bytes, response time, requests and retries. Artifact identity is verified on the credential-free side before cache promotion. Extraction occurs only in the target sandbox, with decompressed-byte, file-count, symlink/path-traversal and process limits. Integrity failure quarantines bytes and fails the unit; retry cannot silently choose another source. Registry metadata, archive names and package scripts are hostile content throughout.

## Supported acquisition classes

| Class | Required path |
|---|---|
| Public registry packages | Lockfile-bound identity, exact hash and allowlisted registry/CDN; anonymous where possible |
| Private registry packages | Tenant-scoped read credential in dialer; repository/tenant grant and endpoint policy; no guest npmrc/pip.conf/Cargo credential |
| Git dependencies | Exact repository/commit/tree through Checkout Broker; no arbitrary Git transport, SSH agent, hooks or branch fallback |
| Toolchain downloads | Signed/pinned toolchain manifest, immutable artifact hash and declared architecture; no runtime curl installer |
| Cargo, npm/pnpm, Python and other managers | Separate versioned adapter cell; dependency closure staged offline; native manager runs only inside bound sandbox without network/credentials |
| Lifecycle/build/install scripts | Disabled during trusted resolution; if required, run as repository code at S2/S3 with default-deny egress and root budgets |
| Pre-resolved artifacts | Same grant checks, immutable package-set manifest, full tenant/repository provenance and exact lockfile match |

Unsupported package-manager behavior, unbounded network discovery, dynamic install-time downloads or absent authenticated source remain explicit capability failures. Approved additional fetches re-enter the same broker with new exact grants; arbitrary guest egress is never the convenience fallback.

## Storage, transactions and recovery

Cache keys bind tenant, repository policy where required, ecosystem, registry authority, package ID/version/hash and provenance; no cross-tenant dependency artifact sharing in V2.1. Private artifacts use per-tenant encryption, authorized read grants, retention/deletion rules and revoke on tenant transfer. A cache hit is rehashed and provenance checked; cache poisoning is not an authority shortcut.

In one PostgreSQL transaction the Dependency Authority locks attempt and root envelope, validates owner/generation/epoch/state and lease_expires_at > database_current_time, reserves acquisition ceilings and inserts a unique (tenant_id, grant_nonce) record. The broker consumes the grant exactly once via conditional transition RESERVED → ACQUIRING; database-time grant expiry and current authority are checked at consumption. Individual authenticated external requests use the per-effect intent/journal rules in 50 when they have externally accountable effects. Failed or uncertain reads consume actual request/byte budget; retries are bounded and do not reset grant counters. Completion records exact delivered artifact digests and settles actual usage; late results after expiry may settle cost but cannot acquire artifact authority. Authority loss stops future fetches and quarantines late bytes.

P0-05E through P0-05H qualify public/private integrity, credentials/SSRF, Git/toolchain and lifecycle/resource behaviors separately. Every registry/ecosystem/adapter version remains disabled until its claim evidence passes. ADR-026 preserves local-first execution and the frozen Rust/PostgreSQL stack.

For checkout/dependency egress, each actual HTTP call uses AcquisitionRequestV1 with exact broker-derived origin/method/target/body and selected OID/package, bounded response size and grant_digest. EffectIntentV2.canonical_request_digest hashes this per-call request, not the aggregate grant. Authorization proof still references the finite grant. No credential value is included in persisted request bytes.
