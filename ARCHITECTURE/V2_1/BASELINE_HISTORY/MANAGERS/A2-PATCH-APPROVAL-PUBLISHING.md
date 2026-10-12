# A2 Patch, Approval, and Publishing

## Charter

Own PatchCandidateV2, clean-room candidate-tree verification, VerifiedPublicationPlanV2, WebAuthn Approval Authority, ApprovalProofV2, Publication Effect Broker, create-only effects, revocation and reconciliation.

## Required properties

Displayed=approved=verified=published. Ordinary API cannot mint approval. Proof binds every effective field. Publisher uses approved base/changed objects and requires returned-tree equality. No merge/source/default/protected update. Revocation uses the STARTED boundary; ambiguity never blind retries.

## Dependencies and gates

ADRs 003, 005, 011, 012, 014. P0-05, P0-06, P0-09. Implementation status: NOT_AUTHORIZED.

## Review handoff

Deliver WebAuthn origin/RP/UV/replay cases, API-compromise drill, tuple/tree mismatch, permission/egress denial, ref conflict/idempotency, revocation race, ambiguous response, and self-fix recursion tests.

