# Invariant proof obligations

The repaired normative invariants are 00D_V2_1_INVARIANTS.md. Each finding's affected invariant boundary, contracts, states and atomic prototype IDs are traced in ASTRA_FINDING_RESOLUTION_MATRIX.md. NON_REGRESSION_REVIEW.md checks the preserved sound decisions.

All runtime proof remains PENDING/NOT_RUN: provider semantics (P0-03/09), hostile credential confinement (01/02/05), canonical independent encoders (06), protected evidence/oracles/coverage (07), actual expiry/budget/settlement (08), tenant and recovery/manual resolution (10). Local fixture validation is evidence that the specification is internally inspectable, not evidence that a future implementation enforces it. An accepted architecture review never substitutes for required P0 evidence.

Release admission must compute the dependency closure of the selected capability profile and require independently ACCEPTED,current evidence with measured outcome PASS for every applicable claim. A missing/expired/inconclusive claim disables that cell. A claim's evidence hash binds exact registry, fixture and execution manifest; a changed semantic contract reopens its proof obligation.
