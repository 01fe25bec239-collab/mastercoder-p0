# P0 Claims — WORKSTREAM-04-AGENT-MODEL-GRAPH

Primary owner of 10 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W04-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-01A | Structured customer-owned model tool loop | W04-T019 | IM-2, IM-3 | CELL | YES — customer-owned model API/harness cell | DS→W01 playbook, CC→W05 playbook |
| P0-01B | Model secret and endpoint confinement | W04-T020 | IM-2 | CELL | YES — model provider endpoint | DS→W01 playbook, CC→W05 playbook |
| P0-01C | Model cancellation and late usage settlement | W04-T021 | IM-2, IM-3 | CELL | YES — provider (or controllable relay for injections) | DS→W01 playbook, CC→W05 playbook |
| P0-01D | Truthful entitlement and native harness capabilities | W04-T025 | — | CELL | YES — named API/harness cell + terms | DS→W01 playbook, CC→W05 playbook |
| P0-05E | Public/private package exactness and cache scope | W04-T031 | IM-2 | CELL | NO (local registries) | SEC→W03 playbook, CC→W05 playbook |
| P0-05F | Dependency credentials and SSRF confinement | W04-T032 | IM-2 | CELL | NO (local registries) | SEC→W03 playbook, CC→W05 playbook |
| P0-05G | Git and toolchain dependency pinning | W04-T033 | — | CELL | NO (local) | SEC→W03 playbook, CC→W05 playbook |
| P0-08B | Root graph budget under contention | W04-T022 | — | CELL (model graph) | NO | DS→W01 playbook, CC→W05 playbook |
| P0-08C | Cancellation and settlement conservation | W04-T018 | IM-3 | CELL (paid model) | NO (relay) | DS→W01 playbook, CC→W05 playbook |
| P0-08D | Broker restart and late model effect | W04-T023 | IM-3 | CELL (paid model) | NO (relay) | DS→W01 playbook, CC→W05 playbook |

## P0-01A — Structured customer-owned model tool loop

- **Status:** NOT_RUN · **Family:** P0-01 · **Architecture:** 11, 12, 47, ADR-008, INV-05, INV-09
- **Invariant:** Structured customer-owned model tool loop; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 sandbox (guest side), W01 lease/journal
- **Environment:** profile `P0-01A-CELL`. Topology: One customer-owned API/harness/provider/auth/version cell; fixture asks two dependent tools, bounded streaming output and one tool error.
  Fixture: P0-01A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-01A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One customer-owned API/harness/provider/auth/version cell; fixture asks two dependent tools, bounded streaming output and one tool error.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-01A.
  - Run three turns: tool request, correlated result, dependent tool; compare normalized sequence to declared ModelTurnV2 schema.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Duplicate tool correlation
  - wrong result ID
  - malformed streamed part
  - unsupported tool schema.
- **Expected observation:**
  - Valid turns complete with exact correlation
  - malformed requests reject before provider send and retain bounded diagnostic.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** normalized-turns.json, broker-validation.json, provider-request-count.json, P0-01A/run-manifest.json, P0-01A/fixture-digests.json, P0-01A/evidence-manifest.sha256, P0-01A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-01B — Model secret and endpoint confinement

- **Status:** NOT_RUN · **Family:** P0-01 · **Architecture:** 12, 22, ADR-008, INV-05
- **Invariant:** Model secret and endpoint confinement; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 S2 sandbox guest, W06 canary-scanner
- **Environment:** profile `P0-01B-CELL`. Topology: Same cell; canary provider credential present only in Secret Dialer; instrument guest filesystem, environment and broker network.
  Fixture: P0-01B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-01B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Same cell; canary provider credential present only in Secret Dialer; instrument guest filesystem, environment and broker network.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-01B.
  - Exercise valid request; enumerate guest credential paths and submit caller-controlled host/header/account fields.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Inject metadata URL, redirect to attacker host, Authorization header and endpoint in prompt/tool JSON.
- **Expected observation:**
  - Only configured provider receives valid request
  - no canary reaches guest, logs or attacker endpoint.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** egress.pcap, canary-scan.json, dialer-denials.json, P0-01B/run-manifest.json, P0-01B/fixture-digests.json, P0-01B/evidence-manifest.sha256, P0-01B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-01C — Model cancellation and late usage settlement

- **Status:** NOT_RUN · **Family:** P0-01 · **Architecture:** 12, 07 ledger, ADR-014, INV-14
- **Invariant:** Model cancellation and late usage settlement; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease + journal, W03 sandbox
- **Environment:** profile `P0-01C-CELL`. Topology: Cell with sandbox plus controllable provider-response relay; one admitted intent and root spend limit 100 units.
  Fixture: P0-01C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-01C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Cell with sandbox plus controllable provider-response relay; one admitted intent and root spend limit 100 units.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-01C.
  - Cancel after provider accepts but before response; deliver final usage 7 units twice and retry finalization.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Drop cancellation acknowledgment
  - return response after lease expiry
  - replay usage event.
- **Expected observation:**
  - No new turn starts after revocation
  - 7 units settle once
  - late output has no current evidence authority.
- **Quantitative threshold:** Final debit exactly 7 units; duplicate debit 0; post-cancel new requests 0.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** provider-usage.json, ledger-before-after.json, lease-trace.json, P0-01C/run-manifest.json, P0-01C/fixture-digests.json, P0-01C/evidence-manifest.sha256, P0-01C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-01D — Truthful entitlement and native harness capabilities

- **Status:** NOT_RUN · **Family:** P0-01 · **Architecture:** 11 parity, 12, 47, INV-05
- **Invariant:** Truthful entitlement and native harness capabilities; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-01D-CELL`. Topology: One explicitly named customer API or LOCAL_USER_HARNESS cell; signed entitlement record and checklist for plugins, skills, hooks, context and opaque subagents.
  Fixture: P0-01D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-01D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One explicitly named customer API or LOCAL_USER_HARNESS cell; signed entitlement record and checklist for plugins, skills, hooks, context and opaque subagents.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-01D.
  - Exercise each advertised capability with a distinct sentinel result; record observable and unobservable graph edges.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Remove entitlement
  - disable a claimed hook
  - make native subagent topology opaque.
- **Expected observation:**
  - Unavailable feature becomes unsupported
  - no automatic credential/auth/provider fallback
  - opaque agents cannot grant independence.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** capability-checklist.json, terms-entitlement-record.pdf, sentinel-results.json, P0-01D/run-manifest.json, P0-01D/fixture-digests.json, P0-01D/evidence-manifest.sha256, P0-01D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 30 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05E — Public/private package exactness and cache scope

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 55, ADR-026, INV-19
- **Invariant:** Public/private package exactness and cache scope; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease, W04 root envelope ceilings (same workstream), W06 local registries
- **Environment:** profile `P0-05E-CELL`. Topology: Public registry fixture and two private registries for distinct tenants; lockfile pins package q@1 and content digest.
  Fixture: P0-05E fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05E/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Public registry fixture and two private registries for distinct tenants; lockfile pins package q@1 and content digest.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05E.
  - Resolve transitive closure, fetch, verify provenance, then rerun from tenant cache.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Same name/version different bytes
  - dependency confusion
  - poisoned cache entry
  - tenant B requests tenant A package.
- **Expected observation:**
  - Hash/source mismatch quarantines
  - no cross-tenant cache read
  - exact permitted immutable package succeeds offline.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** dependency-manifests.json, cache-access-log.json, digest-comparisons.json, P0-05E/run-manifest.json, P0-05E/fixture-digests.json, P0-05E/evidence-manifest.sha256, P0-05E/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05F — Dependency credentials and SSRF confinement

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 55, ADR-026, INV-05
- **Invariant:** Dependency credentials and SSRF confinement; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W06 canary-scanner, W06 local registries
- **Environment:** profile `P0-05F-CELL`. Topology: Private-registry canary token only in acquisition dialer; public default endpoint policy plus one explicit tenant private endpoint.
  Fixture: P0-05F fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05F/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Private-registry canary token only in acquisition dialer; public default endpoint policy plus one explicit tenant private endpoint.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05F.
  - Fetch permitted private package and inspect guest/package-manager configuration for credential residue.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Redirect to attacker origin
  - metadata/loopback URL
  - DNS rebinding
  - malicious proxy/header configuration.
- **Expected observation:**
  - No canary leaves allowed credential authority or enters guest
  - unauthorized destinations denied at each connection/redirect.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** dns-redirect-fixtures.json, egress.pcap, canary-scan.json, P0-05F/run-manifest.json, P0-05F/fixture-digests.json, P0-05F/evidence-manifest.sha256, P0-05F/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05G — Git and toolchain dependency pinning

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 55, 36
- **Invariant:** Git and toolchain dependency pinning; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 checkout broker (Git commit dependencies)
- **Environment:** profile `P0-05G-CELL`. Topology: One exact Git commit dependency and one signed hashed compiler archive for a declared OS/architecture.
  Fixture: P0-05G fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05G/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One exact Git commit dependency and one signed hashed compiler archive for a declared OS/architecture.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05G.
  - Acquire through Checkout Broker and toolchain manifest respectively; verify before offline install.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Swap floating branch/tag
  - replace archive bytes
  - invalid signature
  - wrong architecture.
- **Expected observation:**
  - Only exact approved commit/tree/toolchain qualifies
  - unsupported or mismatched dependencies block mandatory build.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** git-toolchain-manifest.json, provenance-verification.json, grant-trace.json, P0-05G/run-manifest.json, P0-05G/fixture-digests.json, P0-05G/evidence-manifest.sha256, P0-05G/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-08B — Root graph budget under contention

- **Status:** NOT_RUN · **Family:** P0-08 · **Architecture:** 11, 33, ADR-009, INV-09
- **Invariant:** Root graph budget under contention; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease
- **Environment:** profile `P0-08B-CELL`. Topology: Root allowance 10 calls, 100 tokens, 100 spend units, 8 tasks, depth 2, concurrency 3; 16 concurrent workers.
  Fixture: P0-08B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-08B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Root allowance 10 calls, 100 tokens, 100 spend units, 8 tasks, depth 2, concurrency 3; 16 concurrent workers.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-08B.
  - Race child creation and model-intent consumption; independently sum accepted reservations.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Duplicate idempotency key
  - child mints own budget
  - hidden descendants claim new allowance.
- **Expected observation:**
  - Aggregate accepted authority never exceeds root limits
  - duplicate intent consumes once
  - hidden topology grants no authority.
- **Quantitative threshold:** Calls <=10, tokens <=100, spend reservations <=100, tasks <=8, depth <=2, concurrent active <=3.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** root-counter-history.json, child-edges.json, admission-results.json, P0-08B/run-manifest.json, P0-08B/fixture-digests.json, P0-08B/evidence-manifest.sha256, P0-08B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-08C — Cancellation and settlement conservation

- **Status:** NOT_RUN · **Family:** P0-08 · **Architecture:** 12, 07 ledger, ADR-014, INV-14
- **Invariant:** Cancellation and settlement conservation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 effects
- **Environment:** profile `P0-08C-CELL`. Topology: Three requests: confirmed charge 3, canceled-before-send 0, sent-ambiguous later charge 5; opening ledger zero.
  Fixture: P0-08C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-08C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Three requests: confirmed charge 3, canceled-before-send 0, sent-ambiguous later charge 5; opening ledger zero.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-08C.
  - Cancel all, deliver usage twice/out of order, restore broker and settle again.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Duplicate invoice
  - response after attempt fence
  - crash between receipt and ledger transaction.
- **Expected observation:**
  - Final actual debit 8 once, canceled unsent zero
  - ambiguous reservation not released as free budget before accounting resolves.
- **Quantitative threshold:** Final debit exactly 8; no duplicate settlement; no stale result authority; conservation residual 0.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** provider-invoices.json, ledger-journal.json, reconciliation-results.json, P0-08C/run-manifest.json, P0-08C/fixture-digests.json, P0-08C/evidence-manifest.sha256, P0-08C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-08D — Broker restart and late model effect

- **Status:** NOT_RUN · **Family:** P0-08 · **Architecture:** 50, ADR-023, INV-13, INV-17
- **Invariant:** Broker restart and late model effect; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 journal
- **Environment:** profile `P0-08D-CELL`. Topology: Two broker replicas share PostgreSQL and durable effect journal; one one-use model request.
  Fixture: P0-08D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-08D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two broker replicas share PostgreSQL and durable effect journal; one one-use model request.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-08D.
  - Crash after durable STARTED before response and restart second replica.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Provider responds late
  - old replica resumes
  - journal temporarily unavailable.
- **Expected observation:**
  - No blind re-dial under ambiguity
  - late usage reconciles
  - journal unavailable prohibits new send.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** effect-journal.json, provider-call-log.json, restart-trace.json, P0-08D/run-manifest.json, P0-08D/fixture-digests.json, P0-08D/evidence-manifest.sha256, P0-08D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W05 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
