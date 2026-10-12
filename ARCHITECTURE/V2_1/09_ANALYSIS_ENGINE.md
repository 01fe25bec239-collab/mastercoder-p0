# Analysis Engine

The analysis engine creates candidates, never verdicts. It combines deterministic repository metadata, configured scanners, optional model claims, and bounded runner results under one AttemptBindingV2.

## Pipeline

1. Validate exact tree and capability manifest.
2. Derive changed paths, languages, build surfaces, ownership, and policy-relevant metadata without executing repository code where possible.
3. Select analyzers from an allowlisted, digest-pinned tool registry.
4. Execute tools at the bound sandbox tier.
5. Normalize outputs into FindingCandidateV2 with source, location, category, confidence, tool/model identity, and raw-artifact digest.
6. Deduplicate by deterministic fingerprint while retaining all provenance.
7. Request independent verification for gate-eligible claims.

## Novelty

Baseline comparison is exact-subject aware. PRE_EXISTING requires evidence against the bound base. NEW requires evidence against candidate and base. UNKNOWN never becomes PRE_EXISTING by default. For security, secret exposure, and supply chain, UNKNOWN blocks. For other categories, UNKNOWN may require human review or remain partial, but cannot contribute to PASS.

## Machine value before execution

V2 does not promise pre-runner machine analysis beyond provider metadata and safe object parsing. Repository-dependent static analysis is runner work. This removes the false source-residency and availability promise identified in the audits.

## Cache policy

Analysis caches are nonauthoritative, tenant scoped, tool/version/policy/tree keyed, and verified by digest before use. No cross-tenant value result is shared in V2, even for public repositories. Cache miss or corruption triggers recomputation or explicit partial state, never acceptance.

## Agent-proposed tests

An agent-proposed test starts advisory. Base-pass/candidate-fail establishes causality only. Blocking additionally requires an independently trusted, applicable oracle under 13_REVIEW_VERIFICATION.md; repeated runs or additional models cannot promote an untrusted expectation.

## V2.1 mandatory analysis units

Before step 4, the protected planner freezes ExpectedAnalysisManifestV1 under 51_ANALYSIS_COVERAGE_CONTRACT.md. Every required analyzer/build/test unit supplies AnalysisCompletionReceiptV1 even with zero findings. The gate compares the entire expected set and actual coverage; missing execution never produces PASS. Repository ignore files and agent plans cannot disable policy-required work. Deterministic FAST selects no required model unit and remains usable when every model integration is disabled.
