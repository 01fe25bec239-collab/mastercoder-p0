# Prototype gates

V2.1 replaces broad umbrella experiments with separately reviewable AtomicPrototypeClaimV1 records in [57_ATOMIC_PROTOTYPE_CLAIMS.md](57_ATOMIC_PROTOTYPE_CLAIMS.md) and TRACEABILITY/atomic-prototype-claims.json. The exact cell, sealed execution manifest, injected faults, observables, thresholds, evidence reviewers and expiry are mandatory per claim. All claims are NOT_RUN. No prototypes are authorized or executed in this architecture pass.

| Family | Atomic claims | Qualification scope |
|---|---|---|
| P0-01 Model harness and credentials | P0-01A, P0-01B, P0-01C, P0-01D | Each exact applicable cell independently |
| P0-02 Hostile containment per platform | P0-02A, P0-02B, P0-02C, P0-02D | Each exact applicable cell independently |
| P0-03 Git candidate and provider projection | P0-03A, P0-03B, P0-03C, P0-03D, P0-03E | Each exact applicable cell independently |
| P0-04 Eligibility authority | P0-04A, P0-04B, P0-04C | Each exact applicable cell independently |
| P0-05 Checkout and dependency acquisition | P0-05A, P0-05B, P0-05C, P0-05D, P0-05E, P0-05F, P0-05G, P0-05H | Each exact applicable cell independently |
| P0-06 Canonical protocol | P0-06A, P0-06B, P0-06C, P0-06D | Each exact applicable cell independently |
| P0-07 Coverage, oracle and evidence trust | P0-07A, P0-07B, P0-07C, P0-07D, P0-07E | Each exact applicable cell independently |
| P0-08 Expiry, graph budget and settlement | P0-08A, P0-08B, P0-08C, P0-08D | Each exact applicable cell independently |
| P0-09 Approval and per-effect publication | P0-09A, P0-09B, P0-09C, P0-09D, P0-09E, P0-09F, P0-09G, P0-09H | Each exact applicable cell independently |
| P0-10 Isolation, operability and recovery | P0-10A, P0-10B, P0-10C, P0-10D, P0-10E, P0-10F, P0-10G, P0-10H, P0-10I, P0-10J | Each exact applicable cell independently |

Family PASS is a computed aggregate of all applicable, independently reviewed, unexpired atomic PASS results. NOT_RUN, FAIL, INCONCLUSIVE, EXPIRED or unsupported required claims prevent enablement. Explicitly inapplicable claims require a scope record proving the capability is disabled or absent; implementers cannot self-exempt a relevant security boundary. Deterministic FAST does not require P0-01 model qualification. It does require applicable core, runner, Git, dependency and evidence claims. C3 is unavailable to same-domain deployments regardless of repeated test count. Production freeze is separate from architecture readiness for narrow external audit.
