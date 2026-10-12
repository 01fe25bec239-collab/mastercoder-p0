# ADR-003: Distinct Git Authorities

Status: ACCEPTED_DIRECTION.

## Context

SOL-OPUS-001 showed that a shared App signer collapses gate and publication authority. Process-level conventions cannot repair a permission/key collapse.

## Decision

Use distinct Review and Publication GitHub Apps, keys, workload identities, egress, and closed operation vocabularies. Review reads subject and writes only the canonical check. Publisher creates approved Git objects, create-only fix refs, and fix PRs; it cannot publish checks or merge/update source/default/protected refs.

## Consequences

More provider installation and operational ceremony is required. Compromise blast radius is materially smaller and cross-authority actions are impossible without compromising both planes.

Gate: P0-03/P0-09 and provider permission validation.


## V2.1 amendment

The preceding text is retained as historical V2 rationale. Its effective V2.1 semantics are amended by [ADR-016-GIT-PUBLICATION-CAPABILITY.md](ADR-016-GIT-PUBLICATION-CAPABILITY.md). In particular, create-only is gateway-enforced rather than intrinsic token authority; trusted AA display and scoped credential lifecycle are required; patch bytes remain in pinned custody and every external effect rechecks authority. See 14, 52, 53 and 54.
