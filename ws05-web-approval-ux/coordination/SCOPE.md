# SCOPE — WORKSTREAM-05-WEB-APPROVAL-UX (Web & Approval UX)

**Mission.** Build the truthful human surfaces and the independent TypeScript codec: the ordinary Next.js UI (untrusted preview only), the AA-origin trusted renderer on a separate registrable domain with strict CSP and no framing, the trusted resolution UI, WebAuthn browser ceremony harness and authenticator matrix, accessible state language (no color-only security states, no 'complete' for partial work), and accessibility validation evidence.

## Owns (primary, exclusive)
- apps/web (ordinary UI prototype)
- apps/aa-origin (trusted display frontend, P0-09A)
- apps/resolution-origin (trusted resolution UI)
- packages/wire-ts (independent TS codec, I-10)
- packages/state-language
- packages/contracts-ts (generated)
- packages/trusted-origin (shared protections for both trusted origins)
- packages/webauthn-harness + authenticator matrix
- mocks (OpenAPI/MSW)
- a11y evidence
- Synthetic vulnerable demo/test repositories (W05-T009, moved from W02)
- Escape regression corpus probes (W05-T036, moved from W03; built and self-tested only in a throwaway container — the W03 lane runs them on the lab hosts)
- Path, pack and registry/archive fixtures for W03 (W05-T010/T011/T012, moved from W03)
- Moved from W01 (light, non-authority work): external audit packet (W05-T005), I-07/I-08 mock servers (W05-T013/T014), test IdP (W05-T007), WebAuthn cross-check fixtures (W05-T008), G-047 resolution-origin triage (W05-T006)

## Does NOT own (read-only or other workstream)
- AA backend verification/signing, credential registry (W01)
- Resolution Authority backend (W01)
- Any Rust codec (W01) — you must not read its source
- Parity harness (W06)

## Primary P0 claims (1)
P0-09A — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
