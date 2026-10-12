# Prototype Gate Status

| Gate | Question | Current status | Pass artifact | Failure consequence |
|---|---|---|---|---|
| P0-01 | Can Codex and Claude API/subscription modes execute real multi-turn tool loops through ModelTurnV2 without a reusable guest credential and with truthful cancel/usage/parity? | NOT_RUN | Provider/version matrix, adversarial transcript, containment and billing receipts | Disable each failing provider/mode cell |
| P0-02 | Do Linux, macOS, and Windows backends contain hostile repositories at the declared S2/S3 tier? | NOT_RUN | Escape drills, credential exfiltration tests, kernel/VM identity, reset proof | Disable affected OS/tier; shared-kernel cannot stand in for S2 |
| P0-03 | Can GitHub exact candidate commit/tree, merge queue, stale evaluation, and required-check semantics be observed and enforced? | PARTIALLY_DOCUMENTED | Staging normal/squash/rebase/queue matrix and check-conclusion tests | Block unsupported merge mode/repository posture |
| P0-04 | Can current author/membership eligibility be proven using stable actor IDs and bounded freshness without Administration authority? | NOT_RUN | Staging identity and membership evidence with revocation cases | Disable automatic eligibility mode |
| P0-05 | Can bounded multi-call checkout acquire declared monorepo, shallow/filter, LFS, and submodule cells without exposing credentials to parsers? | NOT_RUN | Byte/object/time profiles and ambiguity/reissue drill | Disable unproved repository feature cells |
| P0-06 | Do independent Rust and TypeScript canonicalizers produce identical WorkflowSpecCoreV2, AttemptBindingV2, proofs, and rejection behavior? | NOT_RUN | Golden fixtures, differential/fuzz results, version-skew matrix | No signed or gate-affecting protocol release |
| P0-07 | Can runner compromise, cloned keys, and agent-authored tests be prevented from manufacturing qualified evidence? | NOT_RUN | Key-theft and causal-test red-team results | Disable affected evidence grade/critical PASS |
| P0-08 | Do reservations, cancellation, partitions, lease loss, ambiguity, retry, and late billing preserve hard envelopes and exact ledger settlement? | NOT_RUN | Fault-injection ledger reconciliation with provider invoices | Disable paid model execution or enforce lower synchronous limit |
| P0-09 | Can WebAuthn approval and create-only publication preserve displayed=approved=verified=published under revocation and ambiguous provider responses? | NOT_RUN | End-to-end adversarial ceremony and object/tree/ref reconciliation | Disable publication; export patch only |
| P0-10 | Do RLS, repository transfer, per-tenant grants, audit immutability, protocol skew, load, replay, and DR preserve isolation and recovery? | NOT_RUN | Cross-tenant, transfer, deletion, replay, load, and regional restore evidence | No production authorization |

## Gate policy

All gates are release blocking for the capability they cover. PARTIALLY_DOCUMENTED means official documentation narrows the hypothesis but is not staging proof. Tests are rerun for provider, protocol, sandbox, kernel, model, or permission-version changes. Evidence expires according to the compatibility matrix.

