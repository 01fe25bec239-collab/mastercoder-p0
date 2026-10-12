# DEPENDENCIES — WORKSTREAM-05-WEB-APPROVAL-UX

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-10
- UX fixtures/screenshots for the dossier
- escape corpus for W03 (W05-T036)

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-07 AA API (W01)
- I-08 Resolution API (W01)
- W02 finding/manifest fixtures
- W04 graph fixtures
- I-13 ledger format (W06)

## Integration milestones your claim RUNs need
IM-4 — definitions in the integration contract.

## Supporting components per claim
- P0-09A: W01 AA backend + custody

## External / human dependencies
- Install Node LTS (pinned in .nvmrc), pnpm, Playwright browsers (`pnpm exec playwright install --with-deps`).
- Install mkcert; add `/etc/hosts` entries for `web.mc-local.test`, `aa.mc-aa-local.test`, `resolve.mc-aa-local.test` → 127.0.0.1 (two different registrable domains on purpose).
- Authenticators: built-in ones are enough to start: Windows Hello (fingerprint/face/PIN) on Windows laptops and Touch ID on Macs. Physical FIDO2 security keys (USB/NFC, e.g. YubiKey) are OPTIONAL; buy 1–2 only if you want to test roaming keys too. Record what you used.
- Screen readers: VoiceOver (macOS) and NVDA (Windows VM) for manual a11y runs.
- Independence rule: do not open `ws01-core-security/crates/mc-wire/**` while working on wire-ts.
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
