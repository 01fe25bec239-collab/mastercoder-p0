# WRITE SET — WORKSTREAM-05-WEB-APPROVAL-UX

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS05 — `ws05-web-approval-ux/`

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `corpus/escape/**` (hostile regression corpus probes; moved from W03; self-test only in a throwaway container) | W05 | W03, W06 | TEST | — |
| `package.json`, `pnpm-workspace.yaml`, `pnpm-lock.yaml`, `.env.example`, every app/package `package.json`, app root layouts, CI | W05 (scaffold task W05-T000 only) | — | IMPL(config) | — |
| `packages/contracts-ts/` (generated from SCHEMAS) | W05 | — | GEN | all 85 schemas |
| `packages/trusted-origin/` (headers/session/CSRF/ceremony helper for both trusted origins) | W05 | W01 | IMPL, TEST | 52 |
| `apps/web/` (ordinary Next.js UI, untrusted preview only) | W05 | W06 | IMPL, TEST | JSON projections (read-only) |
| `apps/aa-origin/` (AA trusted renderer frontend) | W05 | W01 | IMPL, TEST | I-07 AA API; TrustedDisplayReceiptV1 fields; renderer_bundle_digest |
| `apps/resolution-origin/` (trusted resolution UI) | W05 | W01 | IMPL, TEST | I-08 Resolution API |
| `packages/wire-ts/` (independent TS codec) | W05 | W06 (parity harness) — **W01 must not read its source** | IMPL, TEST | ENCODING.md, REGISTRY.json, vectors |
| `packages/state-language/`, `packages/webauthn-harness/` | W05 | W01 | IMPL, TEST | — |
| `mocks/**` (OpenAPI of ordinary API + MSW; `mocks/aa-api/` and `mocks/resolution-api/` built from W01's I-07/I-08 OpenAPI) | W05 | W01 | IMPL, TEST | I-07, I-08 |
| `packages/webauthn-fixtures/` (independent cross-check fixtures for W01's verifier) | W05 | W01 | TEST | WebAuthn rules (52) |
| `test-idp/` (test OIDC IdP, G-017) | W05 | W01 | IMPL(config), TEST | — |
| `docs/external-audit/**` (external audit packet, G-002) | W05 | ALL | DOC | 58 external audit focus |
| `fixtures/repos/**` (synthetic vulnerable demo/test repositories, G-067) | W05 | ALL | TEST | — |
| `fixtures/{paths,packs,registries}/**` (W03 test fixtures, moved from W03) | W05 | W03 | TEST | — |
| `a11y/**` | W05 | W06 | TEST, DOC | — |
| `claims/P0-09A/` | W05 | W06, evidence A3s | IMPL, TEST, DOC | claim JSON |
| `coordination/**` | W05 | master | COORD | — |



## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws05-web-approval-ux/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws05-web-approval-ux/`
- `.github/workflows/ws05-*.yml` only
