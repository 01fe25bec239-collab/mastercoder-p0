# A4 Contract Consistency Review

Verdict: PASS_WITH_RELEASE_BLOCKERS. Independence: NOT_AN_EXTERNAL_ATTESTATION.

## Checks

- authority_epoch is uniformly UUID; generations are counters.
- WorkflowSpecCoreV2 and AttemptBindingV2 are non-self-referential.
- AttemptEventV2 and ProducerEventV2 remove the non-attempt binding contradiction.
- MergeSubjectV2/EvaluationIdentityV2 are shared by Git, runner, verifier, gate, and checks.
- PatchCandidate, VerifiedPublicationPlan, ApprovalProof, and effect identity carry the same source/candidate/target/effect fields.
- Verdict vocabulary and non-permitting partial/unknown semantics are consistent.
- Protocol negotiation is pinned and critical-field aware.

## Open proof

P0-06 must prove independent Rust/TypeScript byte equality and N/N-1 rejection behavior. P0-03/P0-09 must prove provider mappings do not reinterpret canonical identity.

No unresolved document contradiction was found among the V2 canonical files. If a subordinate file conflicts, 00E and the numbered owning contract win.

