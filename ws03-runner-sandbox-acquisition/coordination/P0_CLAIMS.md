# P0 Claims — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION

Primary owner of 9 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W03-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-02A | S2 host secret and escape confinement | W03-T029, W03-T036, W03-T039 | — | YES (≥1 hostile-execution cell) | NO (hardware lab: hypervisor host) | SEC→W01 playbook, CC→W04 playbook |
| P0-02B | Sandbox process and resource ceilings | W03-T030, W03-T037, W03-T040 | — | YES (≥1 hostile-execution cell) | NO (hardware lab) | SEC→W01 playbook, CC→W04 playbook |
| P0-02C | Default-deny network and reset | W03-T031, W03-T038, W03-T041 | — | YES (≥1 hostile-execution cell) | NO (hardware lab) | SEC→W01 playbook, CC→W04 playbook |
| P0-02D | Native filesystem and capability matching | W03-T021 | — | CELL | NO (macOS/Windows/Linux native hosts) | SEC→W01 playbook, CC→W04 playbook |
| P0-05A | Bounded exact checkout and pack validation | W03-T023 | — | YES | Optional — local git server; GitHub cell for production mode | SEC→W01 playbook, CC→W04 playbook |
| P0-05B | Checkout credential-bearing component never parses content | W03-T024 | — | YES | Optional — local git server | SEC→W01 playbook, CC→W04 playbook |
| P0-05C | LFS acquisition is separately authorized | W03-T034 | — | CELL | Optional — LFS endpoint (local or GitHub) | SEC→W01 playbook, CC→W04 playbook |
| P0-05D | Submodule recursive bounds and replacement grants | W03-T035 | — | CELL | Optional — local git server | SEC→W01 playbook, CC→W04 playbook |
| P0-05H | Package lifecycle and extraction boundaries | W03-T043 | — | CELL | NO (local) | SEC→W01 playbook, CC→W04 playbook |

## P0-02A — S2 host secret and escape confinement

- **Status:** NOT_RUN · **Family:** P0-02 · **Architecture:** 10, 22, 23, ADR-007, INV-05, INV-08
- **Invariant:** S2 host secret and escape confinement; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W06 canary-scanner
- **Environment:** profile `P0-02A-CELL`. Topology: One OS/architecture/hypervisor cell; guest hostile repository; host-only canary credential and inaccessible database endpoint.
  Fixture: P0-02A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-02A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One OS/architecture/hypervisor cell; guest hostile repository; host-only canary credential and inaccessible database endpoint.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-02A.
  - Try guest mount/device/socket/process inspection and declared escape regression corpus in disposable host.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Offer host socket, shared writable mount, nested-virtualization request and cloud metadata route.
- **Expected observation:**
  - No host canary or database credential accessible
  - forbidden device/mount routes denied
  - shared kernel cannot advertise S2.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** host-guest-config.json, access-probes.json, network-trace.pcap, P0-02A/run-manifest.json, P0-02A/fixture-digests.json, P0-02A/evidence-manifest.sha256, P0-02A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-02B — Sandbox process and resource ceilings

- **Status:** NOT_RUN · **Family:** P0-02 · **Architecture:** 10, ADR-007, INV-08
- **Invariant:** Sandbox process and resource ceilings; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-02B-CELL`. Topology: Exact cell with limits CPU 1 core, memory 256 MiB, processes 32, disk 64 MiB, output 1 MiB, wall time 10 s.
  Fixture: P0-02B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-02B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Exact cell with limits CPU 1 core, memory 256 MiB, processes 32, disk 64 MiB, output 1 MiB, wall time 10 s.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-02B.
  - Run allocation, fork/churn, file fill and output producers separately; cancel a tree containing grandchildren.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Grandchild ignores graceful termination
  - parent exits early
  - child forks during cancellation.
- **Expected observation:**
  - Boundaries stop work
  - no surviving attempt process or writable state after reset.
- **Quantitative threshold:** Limits never exceed configured enforcement semantics; all attempt processes gone within 5 s of cancellation; output retained <=1 MiB.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** process-tree-snapshots.json, cgroup-or-hypervisor-metrics.json, termination-times.json, P0-02B/run-manifest.json, P0-02B/fixture-digests.json, P0-02B/evidence-manifest.sha256, P0-02B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-02C — Default-deny network and reset

- **Status:** NOT_RUN · **Family:** P0-02 · **Architecture:** 10, 22, ADR-007, INV-05
- **Invariant:** Default-deny network and reset; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W04 broker audience (allowlist), W06 canary-scanner
- **Environment:** profile `P0-02C-CELL`. Topology: Exact S2 cell with broker endpoints allowlisted and two successive unrelated tenant attempts.
  Fixture: P0-02C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-02C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Exact S2 cell with broker endpoints allowlisted and two successive unrelated tenant attempts.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-02C.
  - Attempt DNS/IPv4/IPv6/TCP/UDP egress then write sentinel to all writable guest locations; recycle sandbox and search next attempt.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - DNS rebinding
  - alternate IP family
  - attempted host persistence and stale broker session reuse.
- **Expected observation:**
  - Only typed broker audience routes succeed
  - next tenant sees no prior sentinel and old session is rejected.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** network-matrix.json, reset-disk-inventory.json, session-denials.json, P0-02C/run-manifest.json, P0-02C/fixture-digests.json, P0-02C/evidence-manifest.sha256, P0-02C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-02D — Native filesystem and capability matching

- **Status:** NOT_RUN · **Family:** P0-02 · **Architecture:** 31, ENCODING git paths, ADR-015, INV-08, INV-18
- **Invariant:** Native filesystem and capability matching; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-02D-CELL`. Topology: Each native Linux/macOS/Windows cell; fixtures case collision, symlink/junction, long path and non-UTF8 Git path.
  Fixture: P0-02D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-02D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Each native Linux/macOS/Windows cell; fixtures case collision, symlink/junction, long path and non-UTF8 Git path.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-02D.
  - Checkout and execute declared native build fixture; compare exact tree/path handling; schedule unsupported cell.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Case-fold collision
  - path traversal
  - claim S2 on process container
  - withdraw required toolchain.
- **Expected observation:**
  - Unsupported/mismatched cells never start mandatory execution or yield PASS
  - declared supported paths have exact byte identity.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** checkout-path-results.json, scheduler-decisions.json, capability-manifest.json, P0-02D/run-manifest.json, P0-02D/fixture-digests.json, P0-02D/evidence-manifest.sha256, P0-02D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05A — Bounded exact checkout and pack validation

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 08, 10, 22, INV-05
- **Invariant:** Bounded exact checkout and pack validation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 checkout-grant issuer, W01 effects
- **Environment:** profile `P0-05A-CELL`. Topology: Fixtures with 1 MiB and 64 MiB repositories; grant ceilings 80 MiB bytes, 10000 objects, 100 requests, 60 s.
  Fixture: P0-05A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Fixtures with 1 MiB and 64 MiB repositories; grant ceilings 80 MiB bytes, 10000 objects, 100 requests, 60 s.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05A.
  - Acquire exact allowed OIDs and verify candidate tree with credential-free parser.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Corrupt pack object
  - inflated object count
  - wrong OID
  - timeout
  - send byte limit+1.
- **Expected observation:**
  - Only exact hash-verified objects enter workspace
  - each excessive or corrupt response aborts boundedly.
- **Quantitative threshold:** Bytes <=80 MiB accepted, objects <=10000, requests <=100, deadline <=60 s; any mismatched object accepted = FAIL.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** pack-fixtures.sha256, parser-results.json, transfer-counters.json, P0-05A/run-manifest.json, P0-05A/fixture-digests.json, P0-05A/evidence-manifest.sha256, P0-05A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05B — Checkout credential-bearing component never parses content

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 08, 22, INV-05
- **Invariant:** Checkout credential-bearing component never parses content; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 checkout-grant issuer, W06 canary-scanner
- **Environment:** profile `P0-05B-CELL`. Topology: Checkout dialer with canary provider token and credential-free hostile pack parser.
  Fixture: P0-05B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Checkout dialer with canary provider token and credential-free hostile pack parser.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05B.
  - Trace process boundaries during valid/corrupt pack fetch and feed parser crash fixture.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Malformed delta chain
  - hostile filename
  - parser process compromise simulation.
- **Expected observation:**
  - Privileged dialer performs bounded opaque transfer only
  - parser has neither provider token nor arbitrary authenticated endpoint capability.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** process-capability-map.json, canary-scan.json, dialer-request-log.json, P0-05B/run-manifest.json, P0-05B/fixture-digests.json, P0-05B/evidence-manifest.sha256, P0-05B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05C — LFS acquisition is separately authorized

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 08, 47, INV-05
- **Invariant:** LFS acquisition is separately authorized; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 checkout-grant issuer
- **Environment:** profile `P0-05C-CELL`. Topology: Repository with one LFS pointer, explicit object hash/size and tenant endpoint policy.
  Fixture: P0-05C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Repository with one LFS pointer, explicit object hash/size and tenant endpoint policy.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05C.
  - Request permitted object and validate hash/size before materialization.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Pointer references metadata URL, foreign tenant object or unknown endpoint
  - oversized object.
- **Expected observation:**
  - Only allowed tenant-scoped hashed object is delivered
  - unsupported LFS produces explicit non-pass when mandatory.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** lfs-fixtures.json, fetch-audit.json, hash-results.json, P0-05C/run-manifest.json, P0-05C/fixture-digests.json, P0-05C/evidence-manifest.sha256, P0-05C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05D — Submodule recursive bounds and replacement grants

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 08, 47, INV-05
- **Invariant:** Submodule recursive bounds and replacement grants; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W02 checkout-grant issuer
- **Environment:** profile `P0-05D-CELL`. Topology: Repository with two-level pinned submodules; grant limits depth 2 and submodules 4.
  Fixture: P0-05D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Repository with two-level pinned submodules; grant limits depth 2 and submodules 4.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05D.
  - Resolve every pinned commit under separate allowed repository grant.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Cycle, depth 3, floating branch, cross-tenant URL
  - replay replaced grant after timeout.
- **Expected observation:**
  - No recursive authority expansion
  - all unauthorized/mutable/expired acquisitions fail without guest credentials.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** submodule-graph.json, grants.json, broker-denials.json, P0-05D/run-manifest.json, P0-05D/fixture-digests.json, P0-05D/evidence-manifest.sha256, P0-05D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-05H — Package lifecycle and extraction boundaries

- **Status:** NOT_RUN · **Family:** P0-05 · **Architecture:** 55, 10, INV-05
- **Invariant:** Package lifecycle and extraction boundaries; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** —
- **Environment:** profile `P0-05H-CELL`. Topology: One package manager/version cell with install script and archive fixtures; grant 10 packages, 32 MiB compressed, 128 MiB extracted, 50 requests, 30 s.
  Fixture: P0-05H fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-05H/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - One package manager/version cell with install script and archive fixtures; grant 10 packages, 32 MiB compressed, 128 MiB extracted, 50 requests, 30 s.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-05H.
  - Resolve without running scripts; stage artifacts; run required script offline in S2; measure all ceilings.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Archive bomb/path traversal
  - dynamic script curl
  - 11th package
  - 51st request
  - credential-seeking script.
- **Expected observation:**
  - Broker runs no package code
  - extraction stays in sandbox
  - egress denied
  - limit excess makes completion non-permitting.
- **Quantitative threshold:** No script executed in credential-bearing process; accepted packages <=10, compressed <=32 MiB, extracted <=128 MiB, requests <=50, fetch duration <=30 s.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** script-process-trace.json, archive-results.json, resource-counters.json, P0-05H/run-manifest.json, P0-05H/fixture-digests.json, P0-05H/evidence-manifest.sha256, P0-05H/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W01 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W04 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
