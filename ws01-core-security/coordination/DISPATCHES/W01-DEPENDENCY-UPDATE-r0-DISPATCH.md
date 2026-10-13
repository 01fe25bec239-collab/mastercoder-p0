FROM: STARTUP-A1 (acting W01-A1)
TO: W01-A2[W01-DEPENDENCY-UPDATE-r0]
MESSAGE_TYPE: DISPATCH
TASK_ID: W01-DEPENDENCY-UPDATE
ATTEMPT: r0
DISPOSITION: PREPARED_FOR_FRESH_A2; NOT_LAUNCHED
DATE_UTC: 2026-10-13T13:01:06.559058+00:00
WORKSTREAM: W01
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0
  BRANCH: ws01/w01-dependency-update-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: c87dea0a5fa5439f127830a88d19bbb1d3bd499d
IMPLEMENTATION_SHA / REVIEWED_SHA: NONE
TREE: NONE
START_TREE: 564493a44f73de569a25381e8dd0a12ae4504b1f
SOLE_PARENT: NONE
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

READ FIRST: print actual WORK LOCATION; verify clean task HEAD/branch/tree above. Read authoritative /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md (§13.2 and all applicable procedures), task.md standing task specification, memory.md, NOTICES/W01-T004-PREFLIGHT-DEPENDENCY-REQUEST.md, this dispatch twice, and architecture listed in the task. Your checkout coordination copies may be stale; live absolute coordinator paths above govern. A1 remains in this chat; you are fresh A2, not A1 or A3.

AUTHORIZED PRODUCT WRITES: only the three literal paths in EXACT WRITE SCOPE. The sole coordination write exception is /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-DEPENDENCY-UPDATE-r0-A2_RETURN.md. Never edit task.md/memory.md/dispatches/reviews, returned attempts or other lanes. No reset/stash/delete/amend/force/push/PR/merge/provider mutation or self-approval.

RESOURCE/ENTRY CONDITIONS: This standing dependency task runs alone (AGENTS dependency rule). A1 reserves one slot, but has not launched an implementation agent. W01-T004, W03-T000 and W06-T003 are not dispatched/executing. Before starting recheck the live global in-flight boards and disk floor20GiB. Owner caps remain3 global A2,1/lane,1 executing build, compiler jobs2/cache5G; model API1M tokens/$5 total, cloud$0/keys$0. G2NOT_PASSED/G3none/allclaimsNOT_RUN. No new paid work authorized. Do not load credential-shaped env for this dependency-only task. Use CARGO_BUILD_JOBS=2, per-worktree target (no shared CARGO_TARGET_DIR), existing nonsensitive sccache configuration only. Build-time database runtime is deferred under phased G0 for this pure manifest closure; no DB-dependent behavior/test is added. If a required check really needs DB, report the actual blocker, never PASS a missing resource.

RUN EVERY EXECUTABLE VALIDATION via:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0/ws01-core-security -- COMMAND ARGUMENTS
For root check_spec set --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0. This applies to scans and scratch checks too. Capture actual output/exitcodes without secrets; keep scratch/evidence outside tracked product scope. Never change wrapper/hooks/rust pin. All task TESTS REQUIRED commands are mandatory locally; remote CI remains NOT_RUN pending separate authorization. Hook gitleaks scans must remain active. New/staged range must be clean; full-history scan is knowingly nonzero for individually classified inherited contract-list matches. Report/classify each actual baseline occurrence, never suppress or call history clean.

VERBATIM STANDING TASK SPECIFICATION (same text added to live task.md; no T004 field edited):
### W01-DEPENDENCY-UPDATE
- **TASK_ID:** W01-DEPENDENCY-UPDATE
- **OBJECTIVE:** In plain words: Give mc-contracts access to the already pinned Unicode normalization library, so the next contract-generation task can reject non-NFC text exactly as ENCODING requires. Precisely: add unicode-normalization.workspace = true to mc-contracts normal dependencies, synchronize its Cargo.lock package dependency entry, and document this dependency ownership; implement no contract code.
- **WHY IT EXISTS:** A1 read-only preflight found that T004 must validate Unicode refs/NFC but cannot edit manifests. Resolve this concrete dependency request through the pre-registered standing route before T004.
- **ARCHITECTURE REQUIREMENTS:** SCHEMAS/ENCODING.md text/NFC rule; SCHEMAS/REGISTRY.json exact primitives and ref grammar; frozen Rust stack and pinned lockfile rules.
- **P0 CLAIMS:** None executed; enables future T004 BUILD only.
- **INPUT CONTRACTS:** Exact REGISTRY.json hash b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787; existing workspace unicode-normalization =0.1.25.
- **OUTPUT CONTRACTS:** mc-contracts direct normal dependency unicode-normalization at existing exact workspace pin =0.1.25, coherent lockfile, MODULES dependency note. No new API.
- **EXACT WRITE SCOPE:** ws01-core-security/crates/mc-contracts/Cargo.toml; ws01-core-security/Cargo.lock; ws01-core-security/docs/MODULES.md only. This narrows the existing standing scope; do not edit workspace Cargo.toml because its exact pin already exists.
- **READ-ONLY DEPENDENCIES:** Root rust-toolchain.toml; ws01-core-security/Cargo.toml; all other manifests/source; ARCHITECTURE/V2_1/SCHEMAS/**; authoritative live coordination AGENTS/SKILLS/memory/task and this dispatch.
- **PRECONDITIONS:** Signed phased G0/G1 BUILD authorization; W01-T000 INTEGRATED and its merge 9a7922bf29c0a6689903b96896dab8851d1eeeeb is ancestor of BASE_SHA; no other in-flight A2. Standing dependency task runs alone.
- **IMPLEMENTATION STEPS:** 1. Verify base, clean status, scopes, exact pin and dependency request. 2. Add only unicode-normalization.workspace = true under mc-contracts [dependencies]. 3. Synchronize lockfile with the smallest Cargo resolution; no upgrades, downgrades or unrelated feature changes. 4. Add a concise MODULES note binding this dependency to T004 non-NFC rejection; preserve module ownership. 5. Verify locked dependency closure and all required commands below. 6. Inspect whole diff and active hooks; make signed scoped local commit(s), verify signature and clean status. 7. Write original RETURN with exact SHA/tree/parents/check exit codes/evidence hashes and MEMORY_UPDATES. Stop.
- **SECURITY INVARIANTS:** Reject non-NFC rather than normalizing authenticated input; no ASCII-only substitute. No credentials/logged secrets; no bypass of hooks/checks, exact version pins and protected scopes.
- **FAILURE CASES:** T004 stuck on an inaccessible transitive crate; Unicode behavior silently deferred; broad lockfile regeneration upgrades unrelated packages; any source-code or architecture edits.
- **TESTS REQUIRED:** Through the single-build wrapper: cargo metadata --locked --format-version 1; cargo tree -p mc-contracts --locked; cargo fmt --all --check; cargo check --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked; root python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (p0_executed must remain false). Validate dependency metadata before/after: exact same workspace members, package versions/sources/checksums and every unrelated dependency; only mc-contracts gains unicode-normalization. Preserve command output as hashed local evidence. This is dependency closure validation, not T004 implementation or proof of its not-yet-written behavior.
- **NEGATIVE TESTS:** Assert that any out-of-scope path, changed pre-existing package version/source/checksum, added unexpected package, removed dependency, unrelated dependency/feature drift, or unpinned unicode-normalization version fails the scope/closure verification. Use isolated scratch controls if needed; commit no permanent tests outside this manifest-only standing scope.
- **ACCEPTANCE CRITERIA:** Exact direct dependency =0.1.25 resolves under --locked; no unrelated drift; all applicable required commands succeed; only three authorized product paths changed; signed local candidate and clean task worktree; fresh independent A3 plus actual authorized remote CI/integration required before T004 starts.
- **EVIDENCE TO RETURN:** Before/after metadata comparison, dependency tree, exact lockfile delta, all required exit codes and hashed evidence, scoped diff, signature output, SHA/tree/parents. Use COMMON/A2_RETURN_TEMPLATE.md; preliminary local readiness is not final acceptance.
- **OUT-OF-SCOPE:** All Rust source and new production behavior; generating contract types; CBOR; semantics; other manifests or lanes; coordination edits except this RETURN; paid API/cloud/security keys; outward actions.
- **BLOCKERS:** Preserve and return NOT_READY if dependency synchronization needs scope expansion/unrelated changes, required tool/check fails, source ambiguity, foreign dirt, cap violation, or secret/provider/claim action. Remote push/PR/CI/merge authorization is not granted for this new task.
- **A3 REVIEW FOCUS:** Exact minimal direct dependency and lock closure; no source/API changes or drift; pinned normalization library accessible to mc-contracts; scope/signature/base/clean status; real local validation with no invented NFC/product/remote results.

RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-DEPENDENCY-UPDATE-r0-A2_RETURN.md
TEMPLATE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A2_RETURN_TEMPLATE.md
STOP AND OUTPUT: preserve work on any blocker or repeated failure. State exact progress/remaining work; no false PASS/ACCEPTED. Write RETURN first; put your entire final chat response in exactly one fenced text code block containing result/checks/blockers/absolute RETURN, WORK LOCATION and A1 resume prompt. Nothing outside it. No extra fences. Root A1 processes your RETURN; no further task work.
