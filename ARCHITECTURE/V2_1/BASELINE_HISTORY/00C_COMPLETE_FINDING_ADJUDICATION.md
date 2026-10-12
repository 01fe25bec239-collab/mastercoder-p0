# Complete Finding Adjudication

Allowed classifications used: VALID, VALID_BUT_WRONG_SEVERITY, REQUIRES_PROVIDER_CONFIRMATION, DUPLICATE_ROOT_CAUSE, REQUIRES_PROTOTYPE_TO_RESOLVE, VALID_BUT_SCOPE_DEFERRED, ALREADY_MITIGATED, PARTIALLY_VALID, INVALID.

No numbered finding is classified INVALID. Related findings remain separate unless the audit claim is substantively the same root assertion.

## SOL2 audit of SOL1

| ID | Classification | V2 disposition |
|---|---|---|
| SOL2-SOL1-001 | VALID_BUT_WRONG_SEVERITY | Provenance mismatch invalidates the claimed independence, not the technical observations. V2 records immutable input hashes and architect identity. |
| SOL2-SOL1-002 | VALID | Normal API cannot mint approval. A separate Approval Authority requires direct WebAuthn user verification and signs the exact V2 tuple. |
| SOL2-SOL1-003 | REQUIRES_PROVIDER_CONFIRMATION | MergeSubjectV2 is defined; P0-03 must prove provider candidate and queue behavior. |
| SOL2-SOL1-004 | REQUIRES_PROVIDER_CONFIRMATION | EligibilityProofV2 uses current stable provider identities; P0-04 proves membership and author semantics. |
| SOL2-SOL1-005 | DUPLICATE_ROOT_CAUSE | Same unsafe/unproven model-harness seam as OPUS-SOL-002 and SOL-OPUS-002/011; resolved by ModelTurnV2 and P0-01. |
| SOL2-SOL1-006 | VALID | authority_epoch is exactly a 16-byte UUID everywhere; lease_generation and credential_generation are unsigned counters. |

## SOL audit of OPUS

| ID | Classification | V2 disposition |
|---|---|---|
| SOL-OPUS-001 | VALID | Separate Review and Publication Apps, credentials, egress, and broker identities. |
| SOL-OPUS-002 | VALID | Reusable provider/model credentials never enter an untrusted guest. |
| SOL-OPUS-003 | REQUIRES_PROTOTYPE_TO_RESOLVE | S2 VM-grade containment is the hostile default; P0-02 selects proven OS backends. |
| SOL-OPUS-004 | REQUIRES_PROVIDER_CONFIRMATION | Exact candidate commit/tree and merge-group truth are P0-03 evidence requirements. |
| SOL-OPUS-005 | VALID | Incomplete, unavailable, skipped, neutral, or unknown required work produces a non-permitting check conclusion. |
| SOL-OPUS-006 | VALID | Runner signatures establish authenticated origin, not correctness or faithful execution. |
| SOL-OPUS-007 | VALID | Agent-proposed tests affect gating only after clean-room candidate-specific causal reproduction. |
| SOL-OPUS-008 | VALID | ApprovalProofV2 binds destination, ref, effect vocabulary, repo identity, candidate tree, and patch digest. |
| SOL-OPUS-009 | VALID | Provider checks are keyed by EvaluationIdentityV2; stale identities cannot overwrite current state. |
| SOL-OPUS-010 | VALID | Model broker meters actual spend; late settlement always records debt and never revives stale authority. |
| SOL-OPUS-011 | REQUIRES_PROTOTYPE_TO_RESOLVE | Native harness parity and confinement are capability cells proved by P0-01; no compatibility fallback. |
| SOL-OPUS-012 | VALID | Execution-control classification is positive and capability based; semantic uncertainty raises required tier or blocks. |
| SOL-OPUS-013 | VALID | KMS context is not tenant authorization; RLS plus per-tenant grants/DEKs define the deletion unit. |
| SOL-OPUS-014 | VALID | Historical data binds provider repository ID plus immutable repository_tenant_epoch. |
| SOL-OPUS-015 | VALID_BUT_WRONG_SEVERITY | Provider calls cannot be atomically revoked; V2 defines the STARTED linearization boundary and recovery. |
| SOL-OPUS-016 | VALID | Canonical audit facts are immutable and never redacted; erasable artifacts are outside the hash-chain payload. |
| SOL-OPUS-017 | VALID | Evidence class and run-count rules have one definition in 13_REVIEW_VERIFICATION.md. |
| SOL-OPUS-018 | VALID_BUT_SCOPE_DEFERRED | V1/V2 machine analysis requires a runner; pre-runner analysis is explicitly out of scope. |
| SOL-OPUS-019 | VALID | Source-residency claims disclose excerpts and source-derived artifacts; raw retention is opt-in. |
| SOL-OPUS-020 | VALID | Typed application events do not claim complete logging coverage; boundary DLP and raw-stream controls cover other paths. |
| SOL-OPUS-021 | ALREADY_MITIGATED | V2 has one canonical UNKNOWN rule: security, secret, and supply-chain uncertainty blocks; other unknowns cannot create PASS. |

## OPUS audit of SOL

| ID | Classification | V2 disposition |
|---|---|---|
| OPUS-SOL-001 | VALID | Clean-room verifier emits an immutable publication plan containing the source/candidate tree proof and changed-object manifest. |
| OPUS-SOL-002 | REQUIRES_PROTOTYPE_TO_RESOLVE | ModelTurnV2 expresses provider-native multi-turn tools; P0-01 proves actual Codex/Claude behavior and confinement. |
| OPUS-SOL-003 | VALID | WorkflowSpecCoreV2 excludes its digest; AttemptBindingV2 contains the workflow digest but never itself. |
| OPUS-SOL-004 | VALID | Every injector, gate, and dialer has enrolled identity, session, generation, rotation, and revocation contracts. |
| OPUS-SOL-005 | REQUIRES_PROVIDER_CONFIRMATION | Checkout is a bounded multi-call object protocol; P0-05 proves monorepo, pack, shallow, LFS, and submodule cells. |
| OPUS-SOL-006 | DUPLICATE_ROOT_CAUSE | Same exact-current-candidate root as SOL2-SOL1-003; P0-03 is the common resolution. |
| OPUS-SOL-007 | VALID | Stale attempt spend remains billable and auditable while its result remains fenced. |
| OPUS-SOL-008 | VALID | Mastercoder-authored fix PRs are marked and excluded from automatic fix generation by default; maximum one explicit cycle. |
| OPUS-SOL-009 | VALID | MY_PRS_ONLY means provider PR opener stable actor ID matched to the linked user, never commit author metadata. |
| OPUS-SOL-010 | VALID | Ambiguous checkout use terminates the grant and calls an explicit create-grant operation with a new nonce. |
| OPUS-SOL-011 | PARTIALLY_VALID | Broker-created graph edges are authoritative; hidden native subagents are declared unenforceable and governed only by outer envelopes. |
| OPUS-SOL-012 | VALID_BUT_WRONG_SEVERITY | Audit-export lag cannot block review tokens; local append durability is required, remote sealing is an alert/SLO. |
| OPUS-SOL-013 | VALID_BUT_SCOPE_DEFERRED | C3 independence is unavailable in one-runner plans; such plans cannot produce the affected critical PASS. |
| OPUS-SOL-014 | REQUIRES_PROVIDER_CONFIRMATION | LFS and submodule support are independent checkout capabilities, disabled until P0-05 proves them. |
| OPUS-SOL-015 | VALID | Non-attempt events use ProducerBindingV2; attempt_binding is required only for attempt-scoped events. |
| OPUS-SOL-016 | ALREADY_MITIGATED | All V2 artifacts use the new architecture identifier and input-manifest digest. |
| OPUS-SOL-017 | VALID | Publication authorization has pre-start expiry and post-start reconciliation semantics; no mid-effect retry assumption. |

## Totals

| Classification | Count |
|---|---:|
| VALID | 26 |
| VALID_BUT_WRONG_SEVERITY | 3 |
| REQUIRES_PROVIDER_CONFIRMATION | 5 |
| DUPLICATE_ROOT_CAUSE | 2 |
| REQUIRES_PROTOTYPE_TO_RESOLVE | 3 |
| VALID_BUT_SCOPE_DEFERRED | 2 |
| ALREADY_MITIGATED | 2 |
| PARTIALLY_VALID | 1 |
| INVALID | 0 |
| Total | 44 |

