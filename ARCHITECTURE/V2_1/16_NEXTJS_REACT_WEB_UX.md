# Next.js and React Web UX

The web application is a truthful projection of PostgreSQL-backed state, not an authority. Server components may read through the Rust API; mutations use CSRF-protected, idempotent API operations. Approval is completed in a top-level browser navigation to the separately controlled Approval Authority origin, with its own assets/session/RP and renderer (52). The ordinary Next.js app provides navigation and untrusted preview only.

## Primary views

- Repository setup: provider installation, stable repository identity, ownership epoch, policy, runner/capability readiness.
- Review: exact head/base/candidate identities, currentness, coverage, findings, evidence class, unsupported/partial reasons, and provider-check projection.
- Attempt: lease/generation, sandbox tier, tool/model cells, budget envelope, receipts, and stale/canceled state.
- Patch: immutable diff/manifest, candidate tree, verification set, destination/effects, approval ceremony, publication/reconciliation.
- Administration: identities, runner/broker generations, revocation, retention, audit export, and P0 capability status.

## Approval ceremony

The ordinary app sends only a plan ID. AA retrieves and validates immutable plan/custody itself, renders complete repository/source/base/candidate, file/mode/binary effects, destination/ref and exact provider effects, and issues its own plan/display-bound WebAuthn challenge. No ordinary-app rendering or API-supplied diff counts as the trusted approval display. Any plan, custody, scope, generation or expiry change invalidates the ceremony. A compromised ordinary app cannot substitute its rendering at AA; browser/OS or AA compromise remains an endpoint threat. Recovery and credential management also occur on AA, never ordinary login reset.

## State language

PASS, FAIL, BLOCKED, HUMAN_REVIEW_REQUIRED, PARTIALLY_AVAILABLE, STALE, UNSUPPORTED, and UNKNOWN have distinct visual and accessible labels. “Complete” is never shown for partial work. Provider-check status includes exact EvaluationIdentityV2, projection generation, last confirmed provider state, withdrawal/freeze progress and whether it is current. EXTERNAL_GATE_UNSAFE clearly identifies an internal revocation not yet enforced externally; no green “safe” state is inferred from a stale provider success. Custody availability, transfer/residency, pending deletion and uncertainty holds are explicit. Manual resolution uses 56 and never relabels insufficient evidence as deterministic PASS.

## Accessibility and safety

Keyboard operation, focus order, labels, contrast, reduced motion, and screen-reader status announcements are required. Security decisions never rely only on color. Large diffs are virtualized for performance, but the approval digest covers the complete immutable patch; truncation is explicit and cannot be approved without an alternate complete review path.

## Local-first disclosure

Each artifact shows where it was produced, whether source-derived content left the runner, retention policy, and deletion/legal-hold state. Source-residency marketing is derived from actual policy/capability, not a blanket promise.

