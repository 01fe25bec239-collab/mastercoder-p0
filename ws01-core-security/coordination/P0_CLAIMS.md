# P0 Claims — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

Primary owner of 13 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W01-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-06C | Attempt and tenant domain separation | W01-T024 | — | YES | NO | DS→W03 playbook, CC→W06 playbook |
| P0-06D | Version negotiation cannot weaken signed semantics | W01-T038 | IM-2 | YES | NO | DS→W03 playbook, CC→W06 playbook |
| P0-07B | Gate evidence origin and revocation authority | W01-T040 | IM-6 | YES | NO | SEC→W02 playbook, CC→W06 playbook |
| P0-08A | Actual lease expiry independent of reaper | W01-T039 | IM-2 | YES | NO | DS→W03 playbook, CC→W06 playbook |
| P0-09B | Authenticator lifecycle does not inherit account recovery | W01-T045 | IM-4 | CELL (publication) | YES — authenticators + test IdP | SEC→W02 playbook, CC→W06 playbook |
| P0-09C | Approval WebAuthn purpose and exact binding | W01-T041 | IM-4 | CELL (publication) | YES — authenticators | SEC→W02 playbook, CC→W06 playbook |
| P0-09D | Compromised planner confined by Git gateway | W01-T048 | IM-5 | CELL (publication) | YES — GitHub staging Publication App | SEC→W02 playbook, CC→W06 playbook |
| P0-09F | Patch custody exact byte lifetime | W01-T034 | IM-5 | CELL (publication) | NO (local MinIO) | SEC→W02 playbook, CC→W06 playbook |
| P0-09G | Revocation checked per individual effect | W01-T049 | IM-5 | CELL (publication) | YES — GitHub staging (or relay) | SEC→W02 playbook, CC→W06 playbook |
| P0-09H | Ambiguous effects never inferred absent from one read | W01-T051 | IM-5 | CELL (publication) | YES — GitHub staging (or relay) | SEC→W02 playbook, CC→W06 playbook |
| P0-10A | PostgreSQL tenant row and composite-key isolation | W01-T013 | — | YES | NO | DS→W03 playbook, CC→W06 playbook |
| P0-10I | No authority resurrection after restore | W01-T050 | IM-7 | YES | NO | DS→W03 playbook, CC→W06 playbook |
| P0-10J | Manual resolution is authorized and auditable | W01-T044 | — | YES | NO | DS→W03 playbook, CC→W06 playbook |

## P0-06C — Attempt and tenant domain separation

- **Status:** NOT_RUN · **Family:** P0-06 · **Architecture:** ENCODING signatures, CONTRACT_AUTHORITY, INV-02, INV-15
- **Invariant:** Attempt and tenant domain separation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-06C-CELL`. Topology: Fixtures differ only in tenant, repository, epoch, generation or canonical record domain.
  Fixture: P0-06C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-06C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Fixtures differ only in tenant, repository, epoch, generation or canonical record domain.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-06C.
  - Generate signatures for baseline and attempt replay across each single-field mutation.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Reuse attempt signature as approval or effect
  - swap equal-width integer/UUID positions.
- **Expected observation:**
  - Every changed binding/domain invalidates verification
  - no cross-record or cross-tenant reuse.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** binding-mutation-vectors.json, signature-verification-results.json, P0-06C/run-manifest.json, P0-06C/fixture-digests.json, P0-06C/evidence-manifest.sha256, P0-06C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-06D — Version negotiation cannot weaken signed semantics

- **Status:** NOT_RUN · **Family:** P0-06 · **Architecture:** 20, 47, ENCODING version rule, INV-18
- **Invariant:** Version negotiation cannot weaken signed semantics; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 runner negotiation client
- **Environment:** profile `P0-06D-CELL`. Topology: Current and prior permitted minor readers plus unsupported major; explicit policy minimum.
  Fixture: P0-06D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-06D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Current and prior permitted minor readers plus unsupported major; explicit policy minimum.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-06D.
  - Negotiate supported pairs and pin version into attempt; complete current fixture.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Downgrade advertisement
  - unknown field
  - change protocol mid-attempt
  - newer writer to incapable reader.
- **Expected observation:**
  - Only authenticated mutually supported semantics proceed
  - unsupported combinations fail before authority issuance.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** negotiation-transcript.json, version-matrix.json, rejected-authority.json, P0-06D/run-manifest.json, P0-06D/fixture-digests.json, P0-06D/evidence-manifest.sha256, P0-06D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-07B — Gate evidence origin and revocation authority

- **Status:** NOT_RUN · **Family:** P0-07 · **Architecture:** 07 matrix, ADR-019, INV-06
- **Invariant:** Gate evidence origin and revocation authority; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 reducer/planner/verification tables, W03 runner key
- **Environment:** profile `P0-07B-CELL`. Topology: Protected planner/evidence/policy/trust roles plus compromised ordinary API role and stolen current runner key.
  Fixture: P0-07B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-07B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Protected planner/evidence/policy/trust roles plus compromised ordinary API role and stolen current runner key.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-07B.
  - Try inserting/modifying each gate-critical record via API DB role; submit correctly signed but invalid producer/attempt records through ingress.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Forge completion-set rows
  - replay old epoch
  - revoke analyzer after completion
  - substitute trust-set digest.
- **Expected observation:**
  - API SQL writes denied
  - protected consumers reject unauthorized/stale provenance
  - dependent success withdraws after trust revocation.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** db-role-denials.json, ingress-results.json, gate-input-provenance.json, P0-07B/run-manifest.json, P0-07B/fixture-digests.json, P0-07B/evidence-manifest.sha256, P0-07B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-08A — Actual lease expiry independent of reaper

- **Status:** NOT_RUN · **Family:** P0-08 · **Architecture:** 07, 29, ADR-022, INV-13
- **Invariant:** Actual lease expiry independent of reaper; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 runner, W04 model broker + dependency endpoint
- **Environment:** profile `P0-08A-CELL`. Topology: PostgreSQL authority; lease expires at database T; reaper disabled; heartbeat/completion/artifact/verification/model/dependency endpoints instrumented.
  Fixture: P0-08A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-08A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - PostgreSQL authority; lease expires at database T; reaper disabled; heartbeat/completion/artifact/verification/model/dependency endpoints instrumented.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-08A.
  - Invoke each lease-dependent transition at T-1ms then T+1ms with correct owner/generation/epoch.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Hold expired lease row in active state
  - advance worker clock backward
  - delay request across T while waiting for DB lock.
- **Expected observation:**
  - After database expiry every new authoritative transition rejects even though state still active
  - audit and cost settlement remain possible.
- **Quantitative threshold:** Zero authoritative acceptances where lease_expires_at <= DB time at locked authorization statement.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** db-clock-trace.json, endpoint-results.json, lease-row-history.json, P0-08A/run-manifest.json, P0-08A/fixture-digests.json, P0-08A/evidence-manifest.sha256, P0-08A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09B — Authenticator lifecycle does not inherit account recovery

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 52, 15, ADR-018, INV-10
- **Invariant:** Authenticator lifecycle does not inherit account recovery; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W05 webauthn-harness / aa-origin
- **Environment:** profile `P0-09B-CELL`. Topology: User with existing approval credential and separate ordinary account recovery; authorized credential registrar identity.
  Fixture: P0-09B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - User with existing approval credential and separate ordinary account recovery; authorized credential registrar identity.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09B.
  - Recover ordinary account, try enrollment/replacement, then complete valid separately authorized approval recovery.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Forged registrar
  - add credential using ordinary session only
  - replay recovery
  - revoke credential while challenge live.
- **Expected observation:**
  - Ordinary recovery grants no approval power
  - fresh bound registration and scope/credential generation enforced.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** credential-lineage.json, ceremony-transcripts.json, denied-assertions.json, P0-09B/run-manifest.json, P0-09B/fixture-digests.json, P0-09B/evidence-manifest.sha256, P0-09B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09C — Approval WebAuthn purpose and exact binding

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 52, ENCODING WebAuthn, INV-10, INV-11
- **Invariant:** Approval WebAuthn purpose and exact binding; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W05 webauthn-harness
- **Environment:** profile `P0-09C-CELL`. Topology: Approved canonical plan with one-use nonce and valid credential binding.
  Fixture: P0-09C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Approved canonical plan with one-use nonce and valid credential binding.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09C.
  - Perform valid trusted-origin fresh UV then try each one-field mutation.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Wrong RP/origin, absent UV, expired challenge, reused nonce, changed tree/ref/effects/epoch, oracle-purpose assertion reused for publication.
- **Expected observation:**
  - Only exact current plan-purpose assertion qualifies once
  - all altered/replayed ceremonies reject.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** webauthn-fixtures.json, HumanApprovalAssertionV2-results.json, nonce-history.json, P0-09C/run-manifest.json, P0-09C/fixture-digests.json, P0-09C/evidence-manifest.sha256, P0-09C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09D — Compromised planner confined by Git gateway

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 53, ADR-016, INV-12
- **Invariant:** Compromised planner confined by Git gateway; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 staging GitHub + fake GitHub
- **Environment:** profile `P0-09D-CELL`. Topology: Planner has no credential; gateway has staged App token and approved fix namespace refs/heads/mastercoder/fix/<publication_id>.
  Fixture: P0-09D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Planner has no credential; gateway has staged App token and approved fix namespace refs/heads/mastercoder/fix/<publication_id>.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09D.
  - Send each allowed object/ref/PR request with exact plan; attempt forbidden provider operations through gateway.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Merge
  - PATCH existing ref
  - force update
  - delete
  - default/source branch mutate
  - Contents API
  - arbitrary URL/body/repository
  - bypass flag.
- **Expected observation:**
  - Gateway denies every forbidden operation before dial
  - allowed new exact plan creates only fixed namespace/effects.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** gateway-denials.json, provider-request-log.json, resulting-refs.json, P0-09D/run-manifest.json, P0-09D/fixture-digests.json, P0-09D/evidence-manifest.sha256, P0-09D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09F — Patch custody exact byte lifetime

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 14, 24, ADR-025, INV-11
- **Invariant:** Patch custody exact byte lifetime; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W06 artifact-store
- **Environment:** profile `P0-09F-CELL`. Topology: Verified patch/object manifest in tenant-encrypted immutable store with custody pin lasting through reconciliation.
  Fixture: P0-09F fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09F/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Verified patch/object manifest in tenant-encrypted immutable store with custody pin lasting through reconciliation.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09F.
  - Verify, approve, publish and reconcile exact bytes; attempt deletion at each intermediate state.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Substitute object
  - lose local runner workspace
  - storage digest corruption
  - premature retention expiry
  - revoke grant.
- **Expected observation:**
  - Substitution/missing bytes blocks authority
  - pinned exact bytes survive required phases
  - deletion releases only after allowed terminal reconciliation.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** custody-pin-history.json, artifact-digests.json, publication-tree-comparison.json, P0-09F/run-manifest.json, P0-09F/fixture-digests.json, P0-09F/evidence-manifest.sha256, P0-09F/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09G — Revocation checked per individual effect

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 50, 53, ADR-023, INV-17
- **Invariant:** Revocation checked per individual effect; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 staging/fake GitHub
- **Environment:** profile `P0-09G-CELL`. Topology: Publication sequence blob→tree→commit→ref→PR with separate EffectIntentV2 rows and controllable send barrier.
  Fixture: P0-09G fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09G/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Publication sequence blob→tree→commit→ref→PR with separate EffectIntentV2 rows and controllable send barrier.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09G.
  - Revoke immediately after commit confirmation and before ref STARTED; resume workers.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Parent operation still STARTED
  - old worker tries later ref/PR using cached authorization.
- **Expected observation:**
  - Orphaned immutable objects may remain
  - no not-yet-started ref/PR send after revocation
  - existing started deadline bounded.
- **Quantitative threshold:** Post-revocation ref/PR sends not already STARTED =0; each started effect obeys its declared deadline.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** per-effect-state-history.json, gateway-send-log.json, provider-ref-inventory.json, P0-09G/run-manifest.json, P0-09G/fixture-digests.json, P0-09G/evidence-manifest.sha256, P0-09G/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-09H — Ambiguous effects never inferred absent from one read

- **Status:** NOT_RUN · **Family:** P0-09 · **Architecture:** 50, 56, ADR-023, ADR-027, INV-13
- **Invariant:** Ambiguous effects never inferred absent from one read; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 staging/fake GitHub, W05 resolution UI
- **Environment:** profile `P0-09H-CELL`. Topology: Staging/provider relay can delay CREATE_REF/CREATE_PR completion beyond client timeout and temporary not-found read.
  Fixture: P0-09H fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-09H/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Staging/provider relay can delay CREATE_REF/CREATE_PR completion beyond client timeout and temporary not-found read.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-09H.
  - Send once, lose response, observe not-found, then allow provider completion; attempt automatic retry and manual resolution.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Delayed completion
  - duplicate request
  - operator supplies only screenshot of not-found.
- **Expected observation:**
  - Original remains UNKNOWN/MANUAL_RECONCILIATION absent terminal non-execution proof
  - no unsafe retry
  - late result binds original intent.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** delayed-provider-timeline.json, effect-state-history.json, ManualResolutionV1-record.json, P0-09H/run-manifest.json, P0-09H/fixture-digests.json, P0-09H/evidence-manifest.sha256, P0-09H/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W02 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10A — PostgreSQL tenant row and composite-key isolation

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 07 grants, 18, ADR-013, ADR-019, INV-15
- **Invariant:** PostgreSQL tenant row and composite-key isolation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W06 testdb
- **Environment:** profile `P0-10A-CELL`. Topology: Two tenants with same local IDs and populated evaluations, artifacts, jobs and evidence; exact DB roles under test.
  Fixture: P0-10A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two tenants with same local IDs and populated evaluations, artifacts, jobs and evidence; exact DB roles under test.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10A.
  - Run select/insert/update/delete and join via API/worker/readonly roles; try cross-tenant references.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Unset/falsify session tenant
  - reused pooled connection
  - SECURITY DEFINER path
  - cross-tenant FK.
- **Expected observation:**
  - No cross-tenant row or reference succeeds
  - authorized same-tenant controls succeed.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** RLS-query-matrix.json, role-grants.sql.txt, denial-results.json, P0-10A/run-manifest.json, P0-10A/fixture-digests.json, P0-10A/evidence-manifest.sha256, P0-10A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10I — No authority resurrection after restore

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 35 step 6, 48, INV-10, INV-13
- **Invariant:** No authority resurrection after restore; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W06 restore drill
- **Environment:** profile `P0-10I-CELL`. Topology: Snapshot contains live lease, success, unused approval nonce, old credential binding and trust bundle subsequently revoked.
  Fixture: P0-10I fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10I/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Snapshot contains live lease, success, unused approval nonce, old credential binding and trust bundle subsequently revoked.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Create trusted analyzer key K, approver credential A, policy P and membership M; take business-DB backup B.
  - Revoke K/A and remove M after B, then restore B while deliberately omitting every post-backup revocation record. Fence old sender egress and verify complete effect journal.
  - Attempt fresh-epoch bulk re-sign/reuse of restored K/A/P/M; each must reject before authoritative evaluation or send.
  - Perform independently reviewed bootstrap with offline recovery roots and fresh current provider ownership, new keys/authenticators, explicit current policy/analyzer approval; only this new inventory may admit work.
  - Remove the offline recovery-root proof or ownership proof in separate reset runs; verify privileged admission stays frozen.
- **Failure injection:**
  - Lost post-backup revocations with restored valid old signatures.
  - Bulk re-sign of restored grants into new epoch.
  - Replay old authenticator or workload key under new epoch.
  - Missing offline recovery root or current owner proof.
- **Expected observation:**
  - Zero restored entitlements accepted or re-signed automatically.
  - Every old key/authenticator remains ineligible even if revocation history is absent.
  - Only complete independently approved fresh bootstrap enables new authority; missing root/ownership remains frozen.
  - Historical effect tombstones and consumed send claims survive bootstrap.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** fresh-bootstrap-inventory.json, replay-denial-matrix.json, unfreeze-decision.json, P0-10I/run-manifest.json, P0-10I/fixture-digests.json, P0-10I/evidence-manifest.sha256, P0-10I/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10J — Manual resolution is authorized and auditable

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 56, ADR-027, INV-22
- **Invariant:** Manual resolution is authorized and auditable; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W05 resolution UI, W02 oracle case
- **Environment:** profile `P0-10J-CELL`. Topology: One UNKNOWN provider effect and one HUMAN_REVIEW_REQUIRED untrusted oracle case; scoped independent resolution operators.
  Fixture: P0-10J fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10J/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One UNKNOWN provider effect and one HUMAN_REVIEW_REQUIRED untrusted oracle case; scoped independent resolution operators.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10J.
  - Attempt allowed resolution with proof, then verify resulting state plus any fresh approval/re-evaluation required.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Ordinary API closes case
  - same actor bypasses required dual control
  - conflicting provider proof
  - request terminal state resurrection.
- **Expected observation:**
  - Only scoped proof-backed transitions occur
  - ambiguity not laundered into retry or PASS
  - fresh state keeps lineage and invalidates old authority.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** resolution-cases.json, authorization-denials.json, transition-audit.json, P0-10J/run-manifest.json, P0-10J/fixture-digests.json, P0-10J/evidence-manifest.sha256, P0-10J/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W06 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
