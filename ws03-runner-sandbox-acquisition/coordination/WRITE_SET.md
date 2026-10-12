# WRITE SET — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS03 — `ws03-runner-sandbox-acquisition/`

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `Cargo.toml`, `Cargo.lock`, `.env.example`, every crate's `Cargo.toml` and `src/lib.rs` | W03 (scaffold task W03-T000 only) | — | IMPL(config) | — |
| `crates/mc-egress-proxy/` (guest egress allowlist) | W03 | W06 | IMPL, TEST | — |
| `crates/mc-runner-test-support/` (dev-only) | W03 | — | TEST | — |
| `compose.yaml` (PostgreSQL for runner test support only) | W03 | — | IMPL(config) | — |
| `spikes/**` (backend decision spikes) | W03 | — | DOC, TEST | — |
| `crates/mc-runner/` (runner + Runner Control client) | W03 | W06 | IMPL, TEST | AttemptBindingV2, AttemptEventV2, CapabilityProfileV1, ResourceEnvelopeV2 (read) via I-03/I-09 |
| `crates/mc-sandbox-core/`, `crates/mc-sandbox-{linux,macos,windows}/` | W03 | W04, W06 | IMPL, TEST | CapabilityProfileV1 (sandbox_tier) |
| `images/**` (guest image definitions, digest-pinned) | W03 | W02, W04 | IMPL | 36 tool registry |
| `crates/mc-checkout-broker/` (credential-bearing, opaque transfer) | W03 | W06 | IMPL, TEST | CheckoutGrantV2, AcquisitionRequestV1, EffectIntentV2 via I-04 |
| `crates/mc-pack-parser/` (credential-free) | W03 | W06 | IMPL, TEST | GitOidV1 |
| `doubles/**` (fake-runner, fake-sandbox) | W03 | ALL | IMPL, TEST | — |
| `claims/{P0-02A..D,P0-05A..D,P0-05H}/` (P0-02A..D on all three OS cells: linux, macos, windows) | W03 | W06, evidence A3s | IMPL, TEST, DOC | claim JSON |
| `coordination/**` | W03 | master | COORD | — |



## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws03-runner-sandbox-acquisition/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws03-runner-sandbox-acquisition/`
- `.github/workflows/ws03-*.yml` only
