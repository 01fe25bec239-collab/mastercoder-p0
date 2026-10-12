# SCOPE — WORKSTREAM-01-CORE-SECURITY-AUTHORITY (Core Security Authority (HARD AUTHORITY))

**Mission.** Own every freeze-closure component that mints, verifies, or revokes authority: canonical Rust codec and contract types, PostgreSQL authority roles/RLS and writer matrix, lease/fencing and Runner Control ingress, gate-input authority and evidence ingestion, effect intent/state and the two-copy witnessed journal, Approval Authority backend and credential lifecycle, patch custody, the credential-free Publication Planner plus the independent Git Publication Capability Gateway, Manual Resolution Authority, post-restore fresh security bootstrap, and coordination of the narrow external audit.

## Owns (primary, exclusive)
- mc-contracts (generated contract types, I-01)
- mc-wire Rust canonical codec (I-02)
- PostgreSQL authority role framework, RLS, writer matrix (doc 07)
- mc-lease + mc-runner-control (I-03, I-09)
- mc-gate-authority: evidence ingestion writers, gate commit, GateProofV2 signing (I-05)
- mc-effects + mc-journal: EffectIntentV2/EffectStateV1/DbStartCertificateV1/two-copy witnessed journal (I-04)
- mc-approval-authority backend + credential lifecycle (I-07)
- mc-custody PatchCustodyV1 (I-16)
- mc-publication-planner (credential-free) + mc-git-gateway (credential-bearing) + clean-room plan verifier
- mc-resolution Manual Resolution Authority (I-08)
- mc-bootstrap fresh security bootstrap
- External audit coordination: engage the auditor, convert findings to CCRs, decide G2 (the packet is written by W05-T005)
- G1/G3 authorization records (as master, program owner)

## Does NOT own (read-only or other workstream)
- GitHub review/check semantics and CheckProjectionV2 (W02)
- Pure gate reducer, analysis planner, verification authority (W02)
- Runner, sandbox, checkout/dependency acquisition (W03)
- Model broker, secret dialer, ledger, graph envelope (W04)
- AA-origin frontend renderer, CSP, browser ceremonies, TS codec (W05)
- Parity harness, cross-component drills, evidence kit, CI root (W06)

## Primary P0 claims (13)
P0-06C, P0-06D, P0-07B, P0-08A, P0-09B, P0-09C, P0-09D, P0-09F, P0-09G, P0-09H, P0-10A, P0-10I, P0-10J — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
