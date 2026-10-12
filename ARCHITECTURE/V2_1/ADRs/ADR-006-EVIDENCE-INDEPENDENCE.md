# ADR-006: Evidence and Independence

Status: ACCEPTED_DIRECTION.

## Context

Runner signatures, agent assertions, repeated tests, and claimed agent graphs were granted more truth than they establish. One-runner deployments cannot produce every independence level.

## Decision

Separate provenance from correctness. Use C0-C4 administrative/execution independence and E0-E4 evidence kinds. Critical policy names a minimum class. Agent-proposed tests require clean-room base-pass/candidate-fail causality. Unsupported independence produces non-pass.

## Consequences

Small deployments have less gate availability. The system is honest about that limitation rather than manufacturing independence.

Gate: P0-07.

## V2.1 amendment

Amended by ADR-020/021: completeness is an expected-unit set; E3 causality alone cannot qualify an untrusted oracle. C3 requires an independently administered domain; individual/same-domain deployments remain C0-C2.
