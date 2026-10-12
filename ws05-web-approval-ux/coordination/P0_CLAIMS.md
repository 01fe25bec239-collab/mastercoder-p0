# P0 Claims — WORKSTREAM-05-WEB-APPROVAL-UX

Primary owner of 1 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W05-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-09A | Trusted display binds approval bytes | W05-T033 | IM-4 | CELL (publication) | YES — real browsers + authenticators | SEC→W01 playbook, CC→W06 playbook |

## P0-09A — Trusted display binds approval bytes

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 52, 16, ADR-018, INV-10, INV-11
- **Invariant:** Trusted display binds approval bytes; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 AA backend + custody
- **Environment:** profile `P0-09A-CELL`. Topology: Two plans A/B with distinct diff, repository/ref and candidate tree; ordinary UI/API compromised, Authority origin trusted.
  Fixture: P0-09A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two plans A/B with distinct diff, repository/ref and candidate tree; ordinary UI/API compromised, Authority origin trusted.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09A.
  - Start review A; Authority independently fetches canonical plan/bytes and renders exact digest/effects before fresh UV.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - UI supplies diff A with plan B digest
  - iframe overlay
  - cross-origin script
  - replaced object bytes.
- **Expected observation:**
  - Authority displays canonical B or rejects, never asserts approved A for B
  - framing/origin/content restrictions verified.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** authority-render-capture.pdf, canonical-diff.sha256, browser-security-results.json, P0-09A/run-manifest.json, P0-09A/fixture-digests.json, P0-09A/evidence-manifest.sha256, P0-09A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
