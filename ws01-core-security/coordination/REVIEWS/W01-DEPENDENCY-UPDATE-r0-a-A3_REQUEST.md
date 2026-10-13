FROM: STARTUP-A1 (acting W01-A1)
TO: W01-A3[W01-DEPENDENCY-UPDATE-r0-a]
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W01-DEPENDENCY-UPDATE
ATTEMPT: r0
DISPOSITION: PRELIMINARY_LOCAL_REVIEW_PREPARED; NOT_LAUNCHED
DATE_UTC: 2026-10-13T15:49:08.828008+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  REVIEW_WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0-review-a
  BRANCH: NONE (detached)
  IMPLEMENTATION_BRANCH: ws01/w01-dependency-update-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: c87dea0a5fa5439f127830a88d19bbb1d3bd499d
IMPLEMENTATION_SHA / REVIEWED_SHA: 72410409afd609bd98eb9d8742592287d8b1cb6a
TREE: 1c88f7f7d5d58d48c6d685e2c94ee3ea67d3a307
SOLE_PARENT: c87dea0a5fa5439f127830a88d19bbb1d3bd499d
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: A2 three files only; A1/A3 no product edits
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

PHASE: Independent preliminary local review, before separately authorized remote CI/integration. Local SATISFACTORY is not final acceptance. All required remote evidence remains NOT_RUN. Record PRELIMINARY_LOCAL_STATUS and precise BLOCKED:FINAL_PR_CI_PENDING when local criteria are met; no final PASS/ACCEPTED/INTEGRATED or downstream unlock with required evidence missing. Root A1 handles later CI disposition per actual owner authorization; this request creates no extra post-CI A3 or outward permission.

INDEPENDENCE: Fresh A3 session. You and helpers must not have authored, repaired or dispatched this candidate. Do not reuse its A2, root A1 or prior read-only preflight helpers next_t004_preflight/next_parallel_preflight. Declare independence explicitly. A1 created your own detached review worktree; it exists clean at exact SHA/tree. CREATE not needed. Verify identities/status before and after all checks; unexpected dirt or moving SHA/tree invalidates review. No reset/stash/delete/repair/amend/product edits.

READ FIRST:
1. Print your actual WORK LOCATION. Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md and SKILLS.md (§13.3 plus all applicable packs/checklists), memory.md, task.md standing W01-DEPENDENCY-UPDATE specification.
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-DEPENDENCY-UPDATE-r0-DISPATCH.md, /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/W01-T004-PREFLIGHT-DEPENDENCY-REQUEST.md and /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-DEPENDENCY-UPDATE-r0-A2_RETURN.md in full.
3. Exact candidate architecture SCHEMAS/ENCODING.md, REGISTRY.json, frozen pin/lock requirements; root toolchain and workspace manifests. No need to inspect unrelated TypeScript codec or generate contract types.
4. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-DEPENDENCY-UPDATE-r0-processing/verification.json and original retained A2 evidence /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-DEPENDENCY-UPDATE-r0-A2/SHA256SUMS, full before/final metadata, before.lock, final-diff.patch, command outputs, verifier source and all baseline classifications. Verify retained hashes first; inspect retained executable scratch code before any use. These artifacts are context, not independent proof. Use independent checks for scope and metadata/lock drift.
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md; governing /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/BUILD_REVIEW_CI_SEQUENCE_DECISION.md. This new task has no push/PR/merge grant.

SHARP REVIEW FOCUS:
- Exactly three intended paths; workspace unicode-normalization =0.1.25 already pinned, direct normal dependency inherited by mc-contracts (not only transitive/dev).
- Parse entire lock before/after: only mc-contracts gains one dependency; no new/upgraded/removed package or altered source/checksum.
- Independently compare complete locked metadata/feature graph/member identities: 41 members and364 package/resolve nodes unchanged except intended normal declaration/edge. Do not trust equality summaries without examining artifacts/logic.
- MODULES preserves ownership and accurately describes future T004 access; no actual NFC validation, contract generation or security behavior implemented. NFC requirements cannot be claimed proven by empty suites.
- Inspect all12 isolated closure negative controls and positive control; they must reject for intended reasons, not generic failures. Source/guard mutation not applicable to this manifest-only change; do not manufacture runtime tests.
- Verify signature/base/parent/tree/cleanliness and active hooks; new/candidate scans clean, full-history nonzero classified baseline retained occurrence-by-occurrence. A2 reports58matches/29commits and68empty test suites; do not claim68behavior tests or clean history. Your fresh scan may include newer refs; classify actual occurrences honestly.
- All task required commands below remain mandatory applicable local checks. No DB/service/credentials/paid API/provider/P0 action is needed or authorized; return blocker if a required check actually depends on them.

EXECUTION: all executable validation, dependency checks, tests, scans and scratch controls through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0-review-a/ws01-core-security -- COMMAND ARGUMENTS
For root specification check use --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-DEPENDENCY-UPDATE-r0-review-a. Use jobs2, existing nonsensitive sccache prefs/cache5G, own target, no CARGO_TARGET_DIR override or credential-shaped env sourcing. One active build/floor20GiB; do not bypass wrapper, modify pins/hooks or buy capacity. Fresh isolated scratch files/evidence outside product repo. Preserve r0/old evidence and worktrees. A3 not counted as A2, no other task starts while standing dependency awaits integration.

VERBATIM TASK SPECIFICATION:
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

OUTPUT ONLY COORDINATION WRITE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-DEPENDENCY-UPDATE-r0-a-A3_REVIEW.md
Write original report using COMMON/A3_REVIEW_TEMPLATE.md with SHA/tree verified at start/end, explicit independence, each criterion/result, required command actual exitcodes, local verdict/findings, immutable hashed evidence and MEMORY_UPDATES. Do not edit task/memory/RETURN/request or accept yourself. Root A1 processes report. Entire final chat response exactly one fenced text code box (result/checks/blockers/report path/WORK LOCATION/A1 resume prompt). Stop after writing it. No outward action; T004 stays held.
