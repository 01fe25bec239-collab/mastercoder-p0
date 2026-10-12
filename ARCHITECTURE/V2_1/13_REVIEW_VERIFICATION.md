# Review and Verification

## Roles

Producers create FindingCandidateV2 and execution receipts. Verifiers create VerificationReceiptV2 against immutable subjects. The pure Gate Decision role reduces those receipts under a pinned policy. Provider publication is separate.

## Independence classes

| Class | Separation | Permitted use |
|---|---|---|
| C0 | Same attempt/process or unverifiable producer claim | Advisory only |
| C1 | Clean workspace, same runner/admin domain | Low-risk corroboration |
| C2 | Separate sandbox/session on same runner | Routine deterministic claims if policy permits |
| C3 | Separate runner/pool and administrative failure domain | Critical gate evidence |
| C4 | External independent trust root or provider truth | Highest-assurance claims |

The policy names the minimum class per finding category/severity. If the deployment cannot produce it, the result is HUMAN_REVIEW_REQUIRED, PARTIALLY_AVAILABLE, or BLOCKED; it is never waived implicitly.

## Evidence kinds

- E0: model or human-readable rationale; advisory.
- E1: deterministic static/tool result with environment and input identity.
- E2: reproduced result in an independent clean workspace.
- E3: candidate-specific causal test that passes base and fails candidate under the same verified environment.
- E4: policy-qualified multi-run or external proof at the required C class. E4 requires three successful independent executions unless a named policy profile explicitly defines a stronger rule.

Runner signatures authenticate origin, not E grade. A cloned current key cannot independently manufacture C3/E4.

## Gate precedence

1. STALE or subject mismatch: no current verdict; recompute.
2. BLOCKED: mandatory security/identity/capability condition failed.
3. HUMAN_REVIEW_REQUIRED: evidence cannot safely decide.
4. PARTIALLY_AVAILABLE: required noncritical work unavailable; provider check remains non-permitting.
5. FAIL: verified policy violation.
6. PASS: every mandatory claim complete, current, supported, and qualified.

## Unknown novelty

Security, secret exposure, and supply-chain UNKNOWN block. Other UNKNOWN states never create PRE_EXISTING or PASS and are resolved by policy as human review or partial.

## EvaluationIdentityV2

Gate output binds MergeSubjectV2, policy digest, verification-set digest, capability digest, authority_epoch, evaluation sequence, architecture ID, and protocol version. The Review Broker publishes only the latest current identity.

## V2.1 oracle trust is independent of causality

OracleTrustClassV1 distinguishes TRUSTED_REPOSITORY_TEST, TRUSTED_SPECIFICATION, SECURITY_INVARIANT, STANDARDS_REQUIREMENT, MAINTAINER_APPROVED, DETERMINISTIC_EXPLOIT and UNTRUSTED_AGENT_EXPECTATION. An origin label alone grants nothing: a qualified oracle record pins the exact source content digest, trusted issuer or maintainer authorization, policy applicability, base/candidate coordinates, expectation/test digest, and current authority epoch. Existing repository tests qualify only when policy trusts their protected baseline provenance; candidate-added/modified tests do not inherit that trust. Specifications and protocol requirements must identify an authoritative revision and the asserted requirement. A deterministic exploit must demonstrate a named security invariant violation, not merely arbitrary different output.

The protected Evidence Authority validates oracle provenance and applicability; the ordinary API, agent and producer cannot self-certify the source. Maintainer approval uses the trusted Approval Authority display/authenticator boundary with a distinct oracle-approval purpose; it does not grant Git publication authority. A changed oracle/expectation requires a new approval and verification. Revoked source/approver/trust bundle invalidates dependent evaluations through check projection reconciliation.

UNTRUSTED_AGENT_EXPECTATION is always NON_BLOCKING or HUMAN_REVIEW_REQUIRED and remains a CANDIDATE_FINDING regardless of E3/E4 repetition or C3 execution. E3 proves causality, not validity. All blocking categories require policy-qualified trusted oracle, exact subject, qualified evidence and required independence. A manifest-complete result is still non-pass if a required oracle cannot be established.

## V2.1 completeness and deployment profiles

Gate precedence item 6 means every mandatory unit from ExpectedAnalysisManifestV1 has a qualified AnalysisCompletionReceiptV1, including zero-findings units, and all required verification is complete. See 51 for the exact set reducer and protected producer rules; a query that finds no FAIL rows is insufficient.

INDIVIDUAL_SINGLE_DOMAIN supports C0-C2 only. TEAM_SINGLE_DOMAIN may use distinct machines/pools but still supports C0-C2 when their administrator/control plane is shared. INDEPENDENT_DOMAINS can claim C3 only with separately administered runner pools, signing roots, enrollment/revocation and infrastructure access: compromise of one admin domain must not enroll/control the other. Shared owner, cloud root, CI credentials or recovery authority defeats that assertion. EXTERNAL_TRUST_ROOT may qualify C4 for the specific external proof. These are capability declarations requiring P0-07D evidence, not product tiers that automatically grant trust.

A personal deployment can run useful deterministic FAST reviews and advisory findings. If pinned policy requires C3 for a critical decision and only C2 exists, the result is HUMAN_REVIEW_REQUIRED/BLOCKED, never a lower-class PASS. A human cannot relabel the same-domain runs C3; any policy change creates a new evaluation and honest capability profile.

## Exact maintainer-oracle approval protocol

The oracle approval purpose uses ManualResolutionActionV1 with kind UNTRUSTED_ORACLE and decision PROVIDE_ORACLE, scoped to the exact candidate and requirement evidence. The trusted resolution authority displays the action, uses ResolutionChallengeV1 and fresh UV, and signs ManualResolutionV1. OracleTrustClassV1.approval_record_digest references that resolution; its approval_authority_id is the qualified resolution authority. This is distinct from publication-only HumanApprovalAssertionV2. An ordinary review comment or a publication assertion cannot approve an oracle.
