# SCOPE — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION (Runner, Sandbox & Acquisition)

**Mission.** Prove hostile execution containment and bounded acquisition: the customer-controlled Rust runner, S2 VM-grade sandbox backends per OS (deciding U-001 by evidence), the escape corpus, native filesystem capability matching, the credential-bearing checkout dialer with a separate credential-free pack parser, LFS/submodule cells, and offline package lifecycle scripts inside the sandbox — on all three OS cells (Linux, macOS, Windows), each mandatory. Dependency resolution, the registry dialer and dependency grants moved to W04; the escape corpus to W05; the host-lab inventory and local services to W06.

## Owns (primary, exclusive)
- mc-runner (enrollment, capability advertisement, Runner Control client)
- mc-sandbox-core (I-17 unit execution) + mc-sandbox-{linux,macos,windows}
- Guest images and escape corpus
- mc-checkout-broker (credential-bearing) + mc-pack-parser (credential-free)
- LFS + submodule acquisition
- Offline install/extraction inside the sandbox (P0-05H)
- mc-egress-proxy (guest egress allowlist shared by all OS backends)
- Running the escape corpus and the lab capture scripts on the three lab hosts

## Does NOT own (read-only or other workstream)
- Runner Control server and lease authority (W01)
- CheckoutGrantV2 issuance (W02)
- Analyzer adapters/manifest (W02)
- Model broker (W04)
- Dependency resolver, acquisition dialer and DependencyAcquisitionGrantV1 authority (W04)
- Escape corpus content (W05-T036)
- Host-lab inventory and local Git/registry services (W06-T033/T034)
- Admin-domain profiles (W06)

## Primary P0 claims (9)
P0-02A, P0-02B, P0-02C, P0-02D, P0-05A, P0-05B, P0-05C, P0-05D, P0-05H — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
