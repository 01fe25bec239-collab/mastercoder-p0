# Implementation Decomposition

This is a future decomposition, not authorization to implement.

## Phase 0: evidence

Execute P0-01 through P0-10 with disposable prototypes and staging accounts. Finalize only capability rows that pass. No production repository or customer secret is used.

## Phase 1: canonical core

Freeze schemas, deterministic CBOR vectors, state machines, tenant/RLS model, jobs/inbox/outbox, ledger, audit facts, and compatibility negotiation. Exit requires P0-06 and the database portions of P0-08/P0-10.

## Phase 2: read-only review

Build provider normalization, exact subject, checkout acquisition/parser, runner enrollment/leases, S0/S2 execution cells, deterministic analysis, verification, gate, and Review Broker. No patch publication. Exit requires P0-02/03/04/05/07.

## Phase 3: model-assisted review

Add ModelTurnV2, broker/dialer, root graph envelope, metering, cancellation, and declared provider cells. Exit requires P0-01/08.

## Phase 4: human-approved fix publication

Add patch plan, clean-room candidate-tree verifier, Approval Authority/WebAuthn, Publication Effect Broker, create-only ref/PR effects, revocation and ambiguity reconciliation. Exit requires P0-09.

## Phase 5: production qualification

Complete load, observability, supply-chain, privacy, transfer, deletion, restore, regional recovery, and operational drills. Exit requires P0-10 and independent A4 reviews with no unresolved release blocker.

## Stop rule

A failed boundary prototype returns to the owning ADR. It does not authorize an implementation workaround that weakens invariants. Work packages remain blocked until their dependency gate passes.

## V2.1 ordering corrections

All phases above are planning only; this architecture pass does not execute Phase 0. P0 family labels are umbrellas whose exact atomic requirements and evidence scope are listed in 57. Phase 2 includes ExpectedAnalysisManifestV1, completion-set reduction, trusted-oracle validation, dependency acquisition, protected evidence DB roles and CheckProjectionV2. It has no Phase 3 dependency and must demonstrate FAST with model components disabled. Phase 4 includes the credential-free Publication Planner, independent Git capability gateway, trusted display/credential lifecycle, PatchCustodyV1 and per-effect durable journal. Passing one family member never unlocks an entire family or all product capabilities.
