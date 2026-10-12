# ADR-021: Agent-authored test oracle trust

Status: ACCEPTED_FOR_V2_1_ARCHITECTURE; capability evidence NOT_RUN.

## Decision and rationale

Finding 7: V2 clean-room base-pass/candidate-fail proves a causal difference, not validity of an agent's expectation. Amend ADR-006: blocking requires a separately qualified OracleTrustClassV1 origin and source/expectation/applicability digest with protected approval authority. UNTRUSTED_AGENT_EXPECTATION remains advisory/human review at any repetition count or C class. Existing baseline tests qualify only under explicit protected provenance policy. See 09, 13, 51 and SCHEMAS. Prototype: P0-07C/D. Customer-controlled single-domain installations cannot manufacture C3; this is a clarified product limitation, not a redesign.

## Rejected alternative

Do not replace the existing stack or rebuild the full architecture; retain the sound V2 structure and close the missing authority/acceptance contract at its existing boundary.
