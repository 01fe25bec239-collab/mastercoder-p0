# P0 Claims — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

Primary owner of 12 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W02-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-03A | Exact Git candidate per merge mode | W02-T022 | — | YES (≥1 merge-mode cell) | YES — GitHub staging | DS→W06 playbook, CC→W05 playbook |
| P0-03B | Required check identity and conclusion semantics | W02-T020 | — | YES | YES — GitHub staging | DS→W06 playbook, CC→W05 playbook |
| P0-03C | Projection withdrawal after currentness change | W02-T031 | IM-3, IM-6 | YES | YES — GitHub staging | DS→W06 playbook, CC→W05 playbook |
| P0-03D | Emergency security revocation freeze | W02-T035 | IM-3 | CELL (enforcement mode; advisory remains) | YES — GitHub staging + freeze operator | DS→W06 playbook, CC→W05 playbook |
| P0-03E | Concurrent projection writer ordering | W02-T032 | IM-3 | YES | YES — GitHub staging (or relay) | DS→W06 playbook, CC→W05 playbook |
| P0-04A | Eligibility uses stable actor and explicit binding | W02-T023 | — | YES | YES — GitHub staging | SEC→W01 playbook, CC→W05 playbook |
| P0-04B | Eligibility freshness and role revocation | W02-T024 | — | YES | YES — GitHub staging | SEC→W01 playbook, CC→W05 playbook |
| P0-04C | Repository transfer invalidates eligibility | W02-T025 | — | YES | YES — two GitHub staging orgs | SEC→W01 playbook, CC→W05 playbook |
| P0-07A | Expected analysis completeness including zero findings | W02-T026 | IM-6 | YES | NO | SEC→W01 playbook, CC→W05 playbook |
| P0-07C | Causality cannot establish oracle validity | W02-T027 | IM-6 | CELL (trusted-oracle promotion) | NO | SEC→W01 playbook, CC→W05 playbook |
| P0-07E | Environment and artifact reproduction identity | W02-T028 | IM-6 | YES | NO | SEC→W01 playbook, CC→W05 playbook |
| P0-09E | Gateway/token residual authority characterized | W02-T019 | — | CELL (publication) | YES — GitHub staging Publication App | SEC→W01 playbook, CC→W05 playbook |

## P0-03A — Exact Git candidate per merge mode

- **Status:** NOT_RUN · **Family:** P0-03 · **Architecture:** 08, 30, ADR-004, INV-02
- **Invariant:** Exact Git candidate per merge mode; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-03A-CELL`. Topology: Disposable GitHub repository with two divergent commits and one PR; test exactly one merge/squash/rebase/queue/ruleset cell.
  Fixture: P0-03A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-03A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Disposable GitHub repository with two divergent commits and one PR; test exactly one merge/squash/rebase/queue/ruleset cell.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-03A.
  - Record head/base/candidate commit/tree before evaluation and compare with actual permitted integration result.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Move base after evaluation
  - regenerate merge queue
  - amend head without changing displayed title.
- **Expected observation:**
  - Mismatch stales evaluation and prevents internal eligibility
  - enabled mode has proven exact candidate tree mapping.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** git-objects.bundle, provider-state.json, candidate-comparison.json, P0-03A/run-manifest.json, P0-03A/fixture-digests.json, P0-03A/evidence-manifest.sha256, P0-03A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W06 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-03B — Required check identity and conclusion semantics

- **Status:** NOT_RUN · **Family:** P0-03 · **Architecture:** 30, 54, INV-03
- **Invariant:** Required check identity and conclusion semantics; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-03B-CELL`. Topology: Staging repository with fixed expected App/check configuration and no bypass actor.
  Fixture: P0-03B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-03B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Staging repository with fixed expected App/check configuration and no bypass actor.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-03B.
  - Test SUCCESS, failure, pending, neutral, skipped, canceled and same-name foreign-App checks against actual merge eligibility.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Create same-name success from other App
  - retain success on prior head
  - substitute check-run ID.
- **Expected observation:**
  - Only configured current authority can permit
  - observed neutral/skipped behavior documented and unsupported posture disabled.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** ruleset-export.json, check-runs.json, merge-eligibility-observations.json, P0-03B/run-manifest.json, P0-03B/fixture-digests.json, P0-03B/evidence-manifest.sha256, P0-03B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W06 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-03C — Projection withdrawal after currentness change

- **Status:** NOT_RUN · **Family:** P0-03 · **Architecture:** 54, ADR-017, INV-21
- **Invariant:** Projection withdrawal after currentness change; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 journal + gate authority
- **Environment:** profile `P0-03C-CELL`. Topology: Staging repository initially has confirmed successful CheckProjectionV2 and merge blocked unless that exact gate permits.
  Fixture: P0-03C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-03C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Staging repository initially has confirmed successful CheckProjectionV2 and merge blocked unless that exact gate permits.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-03C.
  - Individually change policy, analyzer trust, epoch, base, head and queue; reconcile/read back old and replacement runs.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Duplicate webhook
  - delayed success response
  - provider write/read timeout.
- **Expected observation:**
  - Every internal invalidation schedules withdrawal
  - no stale generation is internally accepted
  - provider interval is measured and surfaced.
- **Quantitative threshold:** Zero internally reaccepted stale generations; normal transition remains pending until confirmed withdrawal; report measured provider revocation latency without claiming atomicity.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** projection-timeline.json, provider-readbacks.json, stale-evaluations.json, P0-03C/run-manifest.json, P0-03C/fixture-digests.json, P0-03C/evidence-manifest.sha256, P0-03C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W06 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-03D — Emergency security revocation freeze

- **Status:** NOT_RUN · **Family:** P0-03 · **Architecture:** 54, 48, INV-21
- **Invariant:** Emergency security revocation freeze; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 revocation authority
- **Environment:** profile `P0-03D-CELL`. Topology: Staging repository with successful check, separately authorized emergency freeze operator and pinned ruleset posture.
  Fixture: P0-03D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-03D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Staging repository with successful check, separately authorized emergency freeze operator and pinned ruleset posture.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-03D.
  - Revoke trusted analyzer; activate prescribed merge freeze; try merge with revoked success until withdrawal confirmed.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Delay provider check withdrawal for 60 s
  - fail freeze API
  - make operator unreachable.
- **Expected observation:**
  - Emergency incident remains open
  - unavailable freeze never reported safe
  - supported posture prevents merge only after verified freeze acknowledgment.
- **Quantitative threshold:** Zero merges after verified freeze; all pre-freeze residual interval and failed freeze attempts explicitly recorded; no instantaneous-revocation claim.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** freeze-ruleset-before-after.json, timed-merge-probes.json, incident-record.json, P0-03D/run-manifest.json, P0-03D/fixture-digests.json, P0-03D/evidence-manifest.sha256, P0-03D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W06 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-03E — Concurrent projection writer ordering

- **Status:** NOT_RUN · **Family:** P0-03 · **Architecture:** 54, 50, INV-21
- **Invariant:** Concurrent projection writer ordering; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 journal
- **Environment:** profile `P0-03E-CELL`. Topology: Two Review Broker replicas and one projection with sequential generations 5 and 6; controllable network relay.
  Fixture: P0-03E fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-03E/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two Review Broker replicas and one projection with sequential generations 5 and 6; controllable network relay.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-03E.
  - Pause generation 5 write, commit invalidation generation 6, release delayed generation 5 and reconcile.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Reverse provider response order
  - crash writer after send
  - replay same outbox event.
- **Expected observation:**
  - Database keeps generation 6 authoritative
  - regression at provider detected, non-permitting correction/freeze remains active until readback.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** writer-fence-trace.json, provider-history.json, reconciliation-log.json, P0-03E/run-manifest.json, P0-03E/fixture-digests.json, P0-03E/evidence-manifest.sha256, P0-03E/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W06 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-04A — Eligibility uses stable actor and explicit binding

- **Status:** NOT_RUN · **Family:** P0-04 · **Architecture:** 15, ADR-010, INV-15
- **Invariant:** Eligibility uses stable actor and explicit binding; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 AA credential bindings (double)
- **Environment:** profile `P0-04A-CELL`. Topology: Two provider users with similar display names; PR opener linked to only one tenant approver record.
  Fixture: P0-04A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-04A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two provider users with similar display names; PR opener linked to only one tenant approver record.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-04A.
  - Resolve eligibility by immutable provider actor ID and repository-bound user mapping.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Rename login
  - spoof display name/email
  - use automated service principal for a human-only scope.
- **Expected observation:**
  - Only exact stable linked identity qualifies
  - no display-name or email matching grants authority.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** identity-fixtures.json, EligibilityProofV2.json, rejection-trace.json, P0-04A/run-manifest.json, P0-04A/fixture-digests.json, P0-04A/evidence-manifest.sha256, P0-04A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-04B — Eligibility freshness and role revocation

- **Status:** NOT_RUN · **Family:** P0-04 · **Architecture:** 15, 07, 53 eligibility boundary, INV-15
- **Invariant:** Eligibility freshness and role revocation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 gateway/AA consumer (double)
- **Environment:** profile `P0-04B-CELL`. Topology: Eligible user with read-fresh membership proof and declared proof TTL.
  Fixture: P0-04B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-04B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Eligible user with read-fresh membership proof and declared proof TTL.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-04B.
  - Remove membership/suspend user while proof is live; request new approval/publication authorization.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Return stale cached membership
  - fail provider refresh
  - expire proof before consumption.
- **Expected observation:**
  - Stale/coarse/unavailable eligibility cannot mint or consume new authority
  - revocation generation enforced.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** membership-timeline.json, proof-lifetimes.json, authority-denials.json, P0-04B/run-manifest.json, P0-04B/fixture-digests.json, P0-04B/evidence-manifest.sha256, P0-04B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-04C — Repository transfer invalidates eligibility

- **Status:** NOT_RUN · **Family:** P0-04 · **Architecture:** 15, 28 transfer, INV-15
- **Invariant:** Repository transfer invalidates eligibility; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 AA credential bindings
- **Environment:** profile `P0-04C-CELL`. Topology: Repository transfers from tenant A installation to tenant B with distinct policies and identity mappings.
  Fixture: P0-04C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-04C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Repository transfers from tenant A installation to tenant B with distinct policies and identity mappings.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-04C.
  - Create eligibility under A, transfer, then replay A proof into B and vice versa.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Delay transfer webhook
  - reuse installation ID cache
  - replay old repository_tenant_epoch.
- **Expected observation:**
  - Current provider ownership/epoch refresh rejects old proof
  - no cross-tenant approver or credential binding survives transfer.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** transfer-provider-events.json, epoch-transitions.json, denied-requests.json, P0-04C/run-manifest.json, P0-04C/fixture-digests.json, P0-04C/evidence-manifest.sha256, P0-04C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-07A — Expected analysis completeness including zero findings

- **Status:** NOT_RUN · **Family:** P0-07 · **Architecture:** 51, 13, ADR-020, INV-03
- **Invariant:** Expected analysis completeness including zero findings; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 gate authority/ingestion, W03 executor
- **Environment:** profile `P0-07A-CELL`. Topology: Manifest declares scan-A(files a,b), scan-B(file c), build-1 and test-1; all legitimate receipts report zero findings.
  Fixture: P0-07A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-07A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Manifest declares scan-A(files a,b), scan-B(file c), build-1 and test-1; all legitimate receipts report zero findings.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-07A.
  - Complete baseline and obtain PASS only with four qualified receipts; remove or mutate one receipt in each run.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Omit scan-B
  - set required scope empty
  - replace analyzer digest
  - local ignore-all config
  - mark exit-zero skipped tests complete.
- **Expected observation:**
  - Every incomplete/substituted run is non-permitting
  - legitimate all-zero complete run may PASS
  - planned required set unchanged by repo config.
- **Quantitative threshold:** Exactly four qualified matching mandatory units required; any PASS with 0–3 or wrong-scope receipts = FAIL.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** ExpectedAnalysisManifestV1.json, completion-receipts.json, gate-decisions.json, P0-07A/run-manifest.json, P0-07A/fixture-digests.json, P0-07A/evidence-manifest.sha256, P0-07A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-07C — Causality cannot establish oracle validity

- **Status:** NOT_RUN · **Family:** P0-07 · **Architecture:** 13, 56, ADR-021, INV-07
- **Invariant:** Causality cannot establish oracle validity; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 resolution (oracle approval), W03 clean sessions
- **Environment:** profile `P0-07C-CELL`. Topology: Base returns 1; candidate intentionally returns 2 under a trusted specification; agent writes test expecting 1.
  Fixture: P0-07C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-07C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Base returns 1; candidate intentionally returns 2 under a trusted specification; agent writes test expecting 1.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-07C.
  - Reproduce base-pass/candidate-fail in three clean independent sessions; then supply separate trusted specification of expected 2.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Label arbitrary agent expectation SECURITY_INVARIANT without source
  - replay maintainer approval for changed assertion.
- **Expected observation:**
  - Arbitrary expectation remains nonblocking/human review despite E3 repeats
  - false provenance/approval rejects
  - actual trusted violation control can block.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** oracle-sources.json, base-candidate-runs.json, trust-decisions.json, P0-07C/run-manifest.json, P0-07C/fixture-digests.json, P0-07C/evidence-manifest.sha256, P0-07C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-07E — Environment and artifact reproduction identity

- **Status:** NOT_RUN · **Family:** P0-07 · **Architecture:** 13, 40, INV-06, INV-07
- **Invariant:** Environment and artifact reproduction identity; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 clean-workspace executor, W01 ingestion
- **Environment:** profile `P0-07E-CELL`. Topology: Immutable test/tool/environment/artifact set with candidate/base bindings and required C class.
  Fixture: P0-07E fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-07E/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Immutable test/tool/environment/artifact set with candidate/base bindings and required C class.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-07E.
  - Verify artifact hashes and reproduce in clean workspace; compare environment manifests.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Swap artifact bytes
  - change toolchain/config
  - reuse previous candidate output
  - replay stale verification receipt.
- **Expected observation:**
  - Mismatch never qualifies E/C evidence or current gate
  - exact legitimate control remains qualified.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** artifact-manifests.json, environment-diffs.json, VerificationReceiptV2-results.json, P0-07E/run-manifest.json, P0-07E/fixture-digests.json, P0-07E/evidence-manifest.sha256, P0-07E/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09E — Gateway/token residual authority characterized

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 53, 44 R-012, INV-12
- **Invariant:** Gateway/token residual authority characterized; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-09E-CELL`. Topology: Disposable repository and App credential with exact production-intended permissions; isolated red-team holder of actual token.
  Fixture: P0-09E fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09E/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Disposable repository and App credential with exact production-intended permissions; isolated red-team holder of actual token.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09E.
  - Inventory provider-authorized mutation endpoints directly, separately from gateway policy; verify ruleset App bypass list.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Use stolen token for existing ref/source mutation and unauthorized merge attempt on disposable refs.
- **Expected observation:**
  - Record actual residual authority honestly
  - protected source rules constrain what they really constrain
  - never claim provider-native create-only token if absent.
- **Quantitative threshold:** 100% tested operation outcomes recorded; any undocumented broader token authority or configured bypass actor = FAIL.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** permission-matrix.json, ruleset-export.json, direct-token-probes.json, P0-09E/run-manifest.json, P0-09E/fixture-digests.json, P0-09E/evidence-manifest.sha256, P0-09E/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
