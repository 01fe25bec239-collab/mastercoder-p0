# P0 Claims — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION

Primary owner of 10 atomic claims (all **NOT_RUN**). Source: `ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1/TRACEABILITY/atomic-prototype-claims.json` (authoritative) and `57_ATOMIC_PROTOTYPE_CLAIMS.md`. Program mapping columns are added by this program. RUN tasks are in `task.md` as `W06-RUN-<claim>`.

| Claim | Requirement | Build task(s) | Integration milestone | BLOCKS_FREEZE | External provider | Evidence lenses → reviewer playbook (fresh A3 sessions) |
|---|---|---|---|---|---|---|
| P0-06A | Canonical positive encoding parity | W06-T016 | IM-1 | YES | NO | DS→W04 playbook, CC→W02 playbook |
| P0-06B | Canonical rejection parity | W06-T017 | IM-1 | YES | NO | DS→W04 playbook, CC→W02 playbook |
| P0-07D | Administrative independence capability truth | W06-T026 | IM-8 | CELL (C3) | YES — two independently administered cloud accounts | SEC→W03 playbook, CC→W02 playbook |
| P0-10B | Transfer invalidates all old authority | W06-T031 | IM-7 | YES | YES — two GitHub staging orgs (transfer) | DS→W04 playbook, CC→W02 playbook |
| P0-10C | Artifact deletion respects holds and revocation | W06-T024 | — | YES | NO (local MinIO) | DS→W04 playbook, CC→W02 playbook |
| P0-10D | Sensitive logs and immutable audit | W06-T025 | IM-7 | YES | NO | DS→W04 playbook, CC→W02 playbook |
| P0-10E | Load preserves fencing and mandatory coverage | W06-T027 | IM-7 | YES | NO | DS→W04 playbook, CC→W02 playbook |
| P0-10F | Rolling schema/protocol upgrade | W06-T028 | IM-7 | YES | NO | DS→W04 playbook, CC→W02 playbook |
| P0-10G | PostgreSQL failover fencing | W06-T029 | IM-7 | YES | NO (local HA) | DS→W04 playbook, CC→W02 playbook |
| P0-10H | Regional restore recovers intents lost from database | W06-T032 | IM-7 | YES | YES — two independent storage domains (cloud) | DS→W04 playbook, CC→W02 playbook |

## P0-06A — Canonical positive encoding parity

- **Status:** NOT_RUN · **Family:** P0-06 · **Architecture:** ENCODING.md, REGISTRY.json, ADR-024, INV-18
- **Invariant:** Canonical positive encoding parity; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 mc-wire CLI, W05 wire-ts CLI
- **Environment:** profile `P0-06A-CELL`. Topology: Independent Rust and TypeScript harnesses built only after later authorization; all SCHEMAS signed/hashed contracts and TEST_VECTORS fixtures.
  Fixture: P0-06A fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-06A/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Independent Rust and TypeScript harnesses built only after later authorization; all SCHEMAS signed/hashed contracts and TEST_VECTORS fixtures.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-06A.
  - Encode/decode every positive fixture independently and compare complete bytes, digest and signature input.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Vary host locale, timezone and map insertion order without changing semantic values.
- **Expected observation:**
  - Canonical bytes identical across implementations and environments
  - signature domain and ordered fields match normative vectors.
- **Quantitative threshold:** 100% positive fixtures byte/digest/signature-input identical; zero unexplained differences.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** rust-vector-results.json, typescript-vector-results.json, byte-comparison.json, P0-06A/run-manifest.json, P0-06A/fixture-digests.json, P0-06A/evidence-manifest.sha256, P0-06A/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-06B — Canonical rejection parity

- **Status:** NOT_RUN · **Family:** P0-06 · **Architecture:** ENCODING.md, ADR-024, INV-18
- **Invariant:** Canonical rejection parity; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 mc-wire CLI, W05 wire-ts CLI
- **Environment:** profile `P0-06B-CELL`. Topology: Same harnesses and all negative fixtures, including unknown fields, null, integer bounds, malformed bytes and array semantics.
  Fixture: P0-06B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-06B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Same harnesses and all negative fixtures, including unknown fields, null, integer bounds, malformed bytes and array semantics.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-06B.
  - Decode each negative fixture and record exact rejection category in both languages.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Duplicate keys
  - noncanonical integer
  - wrong domain/type version
  - unknown critical/noncritical field inside closed signed record.
- **Expected observation:**
  - Both reject every forbidden encoding
  - no normalizing malformed signed input into acceptable equivalent.
- **Quantitative threshold:** 100% normative negative fixtures rejected by both implementations.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** rejection-matrix.json, malformed-fixtures.bin, decoder-traces.json, P0-06B/run-manifest.json, P0-06B/fixture-digests.json, P0-06B/evidence-manifest.sha256, P0-06B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-07D — Administrative independence capability truth

- **Status:** NOT_RUN · **Family:** P0-07 · **Architecture:** 13, 31, ADR-006, INV-08
- **Invariant:** Administrative independence capability truth; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W03 runner pools, W02 verification, W01 trust authority
- **Environment:** profile `P0-07D-CELL`. Topology: Profile A has two runners under same root admin; profile B has separately administered runners, signing roots, enrollment and recovery.
  Fixture: P0-07D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-07D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Profile A has two runners under same root admin; profile B has separately administered runners, signing roots, enrollment and recovery.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-07D.
  - Evaluate claimed C3 for both; exercise compromised A admin against B enrollment/keys/control plane.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Shared cloud root or CI credential
  - copied runner key
  - claimed native subagent graph.
- **Expected observation:**
  - A cannot qualify C3
  - B qualifies only when all separation checks pass
  - critical C3-required work on A is non-permitting.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** administration-access-matrix.json, enrollment-probes.json, C-class-decisions.json, P0-07D/run-manifest.json, P0-07D/fixture-digests.json, P0-07D/evidence-manifest.sha256, P0-07D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-SECURITY → SEC (fresh A3 session with the W03 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10B — Transfer invalidates all old authority

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 15, 18, 28, ADR-013, INV-15
- **Invariant:** Transfer invalidates all old authority; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease/AA/custody, W02 eligibility/projection, W04 model intents
- **Environment:** profile `P0-10B-CELL`. Topology: Transfer repository A→B while leases, approval, patch grants, model intents and projection are live.
  Fixture: P0-10B fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10B/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Transfer repository A→B while leases, approval, patch grants, model intents and projection are live.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10B.
  - Replay each old authority type after observed current ownership/epoch transition.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Delayed webhook
  - stale cache
  - restore old epoch snapshot.
- **Expected observation:**
  - Old authority cannot grant evidence/approval/effect in B
  - audit remains tenant-authorized and no private artifacts cross ownership.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** transfer-replay-matrix.json, epoch-history.json, artifact-access-log.json, P0-10B/run-manifest.json, P0-10B/fixture-digests.json, P0-10B/evidence-manifest.sha256, P0-10B/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10C — Artifact deletion respects holds and revocation

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 24, 14, ADR-025, INV-16, INV-19
- **Invariant:** Artifact deletion respects holds and revocation; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 custody
- **Environment:** profile `P0-10C-CELL`. Topology: Tenant artifacts include ordinary expired object, legal hold and active PatchCustodyV1 pin with separate grants.
  Fixture: P0-10C fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10C/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Tenant artifacts include ordinary expired object, legal hold and active PatchCustodyV1 pin with separate grants.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10C.
  - Request deletion, revoke grants, complete custody reconciliation, release hold with authorized ceremony.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Premature delete under active pin
  - replay revoked download grant
  - unauthorized hold release.
- **Expected observation:**
  - Ordinary deletion becomes verifiable unavailable
  - active pin/hold prevents premature removal
  - revoked reads fail.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** deletion-receipts.json, retention-hold-history.json, download-probes.json, P0-10C/run-manifest.json, P0-10C/fixture-digests.json, P0-10C/evidence-manifest.sha256, P0-10C/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10D — Sensitive logs and immutable audit

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 24, 34, INV-16
- **Invariant:** Sensitive logs and immutable audit; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 audit, W04 broker, all component logging
- **Environment:** profile `P0-10D-CELL`. Topology: Inject distinct canary secret/source strings in prompts, tool output, URL query and exception; audit protected role records effect hashes.
  Fixture: P0-10D fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10D/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Inject distinct canary secret/source strings in prompts, tool output, URL query and exception; audit protected role records effect hashes.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10D.
  - Trigger normal/error/timeout paths and export every declared telemetry sink; verify audit seal chain.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Oversize error
  - privileged dump
  - secret in malformed request
  - audit-row update/delete attempt.
- **Expected observation:**
  - No canary raw content in policy-forbidden sinks
  - authoritative audit immutable and sealed within bound.
- **Quantitative threshold:** Forbidden canary matches 0; audit mutation success 0; seal lag <=declared 60 s test bound.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** sink-inventory.json, canary-search-results.json, audit-seal-verification.json, P0-10D/run-manifest.json, P0-10D/fixture-digests.json, P0-10D/evidence-manifest.sha256, P0-10D/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10E — Load preserves fencing and mandatory coverage

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 33, 17, INV-03, INV-13
- **Invariant:** Load preserves fencing and mandatory coverage; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease/outbox, W02 planner/reducer, W04 graph
- **Environment:** profile `P0-10E-CELL`. Topology: Declared load envelope: 10 tenants, 100 active jobs, 1000 queued jobs, 100 events/s for 10 min; pool max 40 DB connections.
  Fixture: P0-10E fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10E/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Declared load envelope: 10 tenants, 100 active jobs, 1000 queued jobs, 100 events/s for 10 min; pool max 40 DB connections.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10E.
  - Run at envelope then 2x admission while racing lease handoffs, outbox replay and graph expansion.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Slow provider
  - DB lock contention
  - duplicate receipts
  - expiration while waiting for row lock.
- **Expected observation:**
  - Within envelope correctness holds
  - above envelope shedding is explicit
  - mandatory units never omitted into PASS or stale completion accepted.
- **Quantitative threshold:** Unauthorized/stale accepts 0; DB connections <=40; 100 events/s sustained for 10 min; report p95/p99 rather than invent SLA.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** load-config.json, latency-histogram.json, conservation-and-fence-checks.json, P0-10E/run-manifest.json, P0-10E/fixture-digests.json, P0-10E/evidence-manifest.sha256, P0-10E/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10F — Rolling schema/protocol upgrade

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 47, 07 migrations, ADR-024, INV-18
- **Invariant:** Rolling schema/protocol upgrade; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 negotiation, W03 runner
- **Environment:** profile `P0-10F-CELL`. Topology: Two current and two previous-minor server/runner instances, expand-contract migration plan and a live old-version attempt.
  Fixture: P0-10F fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10F/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Two current and two previous-minor server/runner instances, expand-contract migration plan and a live old-version attempt.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10F.
  - Expand readers/DB, deploy writers, finish old attempt, then contract after compatibility window.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Old reader sees new critical field
  - writer downgrade
  - crash midway
  - resume old session with new version.
- **Expected observation:**
  - No mixed-semantic authority accepted
  - downgrade rejects
  - old supported attempts preserve pinned semantics
  - rollback preserves records.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** upgrade-step-log.json, version-negotiation.json, before-after-record-digests.json, P0-10F/run-manifest.json, P0-10F/fixture-digests.json, P0-10F/evidence-manifest.sha256, P0-10F/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10G — PostgreSQL failover fencing

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 29, 35, 50, INV-01, INV-13
- **Invariant:** PostgreSQL failover fencing; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 lease/journal/effects
- **Environment:** profile `P0-10G-CELL`. Topology: Primary and synchronous local standby, two workers, journal witness and controllable network partition.
  Fixture: P0-10G fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10G/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Primary and synchronous local standby, two workers, journal witness and controllable network partition.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10G.
  - Commit lease/effect state, partition primary, promote standby through fenced process, resume both workers.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Old primary remains reachable to worker
  - ambiguous transaction commit
  - delayed heartbeat.
- **Expected observation:**
  - Only active epoch can mutate/send
  - duplicate effect/ledger operations remain idempotent
  - unknown commits reconciled.
- **Quantitative threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** failover-timeline.json, fencing-log.json, DB-and-journal-diff.json, P0-10G/run-manifest.json, P0-10G/fixture-digests.json, P0-10G/evidence-manifest.sha256, P0-10G/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.

## P0-10H — Regional restore recovers intents lost from database

- **Status:** NOT_RUN · **Family:** P0-10 · **Architecture:** 35, 50, ADR-023, INV-01, INV-17
- **Invariant:** Regional restore recovers intents lost from database; failure cannot be hidden by an umbrella gate result.
- **Supporting read-only components:** W01 journal/effects, W02 reconciliation reads
- **Environment:** profile `P0-10H-CELL`. Topology: Recovery backup timestamp T0; create and externally confirm effect at T1>T0 with durable journal, then restore only DB to T0.
  Fixture: P0-10H fixture as described in the procedure; fixtures committed and content-digested before execution.
  Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **Version manifest:** NOT_SELECTED_NOT_RUN: P0-10H/run-manifest.json
- **Pinning rule:** Before NOT_RUN → RUNNING, record exact Rust/rustc/Cargo.lock, TypeScript/Node/lockfile, PostgreSQL server build/extensions, OS/kernel/hypervisor, runner/broker/analyzer image SHA-256 and schema/vector revision for every participating component. For provider-managed components record API version, tenant/account/App identity, request date/response version headers and the exact tested auth/permission/feature configuration; unobservable provider deployment version is explicitly UNDISCLOSED_PROVIDER_MANAGED, never invented. Unused components are explicitly NOT_APPLICABLE. Reject execution if any participating locally controlled version/digest is missing.
- **Preconditions:**
  - Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair.
  - Recovery backup timestamp T0; create and externally confirm effect at T1>T0 with durable journal, then restore only DB to T0.
  - Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **Procedure:**
  - Provision exactly the sealed cell and empty evidence directory named P0-10H.
  - Enumerate journal witness uncertain window including effect absent from restored DB; reconcile provider and reconstruct audit linkage before unfreeze.
  - Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces.
  - Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **Failure injection:**
  - Delete intent from restored snapshot by RPO
  - delay provider query
  - witness unavailable
  - old region still sending.
- **Expected observation:**
  - Recovered effect is discovered without surviving-row scan
  - no new send while journal uncertain
  - regional external authority remains fenced.
- **Quantitative threshold:** All journaled T0..freeze effects accounted for (100%); zero blind duplicate sends; unavailable journal leaves gate closed.
- **PASS condition:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS.
- **FAIL condition:** Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
- **Evidence artifacts:** backup-manifest.json, witness-range-export.json, recovered-effect-inventory.json, P0-10H/run-manifest.json, P0-10H/fixture-digests.json, P0-10H/evidence-manifest.sha256, P0-10H/independent-review.json
- **Evidence reviewers (claim JSON → program lens):** A4-V2_1-DISTRIBUTED-SYSTEMS → DS (fresh A3 session with the W04 lane playbook), A4-V2_1-CONTRACT-CONSISTENCY → CC (fresh A3 session with the W02 lane playbook)
- **Evidence expiry:** 90 days; invalidate on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
