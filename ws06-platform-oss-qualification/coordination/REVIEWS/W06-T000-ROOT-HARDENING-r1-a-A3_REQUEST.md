FROM: STARTUP-A1 (acting W06-A1)
TO: W06-A3[W06-T000-ROOT-HARDENING-r1-a] (fresh independent session)
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W06-T000-ROOT-HARDENING
ATTEMPT: r1
DISPOSITION: PRELIMINARY_LOCAL_REVIEW_REQUESTED; mandatory remote CI pending
DATE_UTC: 2026-10-12T23:10:55Z
WORKSTREAM: W06
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r1-review-a
  BRANCH: NONE (detached exact candidate)
  WORKSTREAM_DIR: ws06-platform-oss-qualification/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
IMPLEMENTATION_SHA / REVIEWED_SHA: b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
TREE: af193b0df3b88e64f0616b8d04d61799ba0d5045
SOLE_PARENT: 5a48f18aa52532d2b4e8d055911e6c067da2054d
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: A1 NONE; verification only, no product repairs
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

REVIEW_PHASE: PRELIMINARY_LOCAL
IMPLEMENTATION_BRANCH: ws06/w06-t000-root-hardening-r1
COMMITS: 5a48f18aa52532d2b4e8d055911e6c067da2054d, b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03; two-commit linear delivery on import base, not a single implementation commit.
WORKTREE: A1 created the detached review worktree, verified exact SHA/tree/clean and lane env symlink. CREATE: not needed. Check again at entry/end; no reset/stash/amend. Only write your review at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/REVIEWS/W06-T000-ROOT-HARDENING-r1-a-A3_REVIEW.md.

READ FIRST:
1. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/AGENTS.md; /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/SKILLS.md (§13.3/14/15–17); live memory.md, task.md T000 and KNOWN_ISSUES.md. These live coordination records supersede old task-worktree seed copies.
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/DISPATCHES/W06-T000-ROOT-HARDENING-r1-DISPATCH.md; /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/RETURNS/W06-T000-ROOT-HARDENING-r1-A2_RETURN.md including all reproduction appendices.
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/BUILD_REVIEW_CI_SEQUENCE_DECISION.md and /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md.
4. Task architecture in your exact candidate: 00F, applicable00D invariants,36_SUPPLY_CHAIN.md and49 glossary/checker requirements; root SKILL.md/SUBAGENT_POLICY.md. Frozen schemas/architecture remain read-only.
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W06-T000-ROOT-HARDENING-r1-processing/a1-verification.json and a1-guard-tests.log; A1 spot checks are context, not your independent proof. Preserved r0 at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W06-T000-ROOT-HARDENING-r0-preservation is historical and must not be modified.

AUTHORIZED IMPLEMENTATION SCOPE (exact nine paths, protected import-base scope):
- .editorconfig
- .gitattributes
- .github/workflows/common-arch-check.yml
- .github/workflows/common-write-set-guard.yml
- rust-toolchain.toml
- scripts/check_action_pins.py
- scripts/check_write_set.sh
- scripts/verify_architecture.py
- ws06-platform-oss-qualification/docs/branch-protection.md

INDEPENDENCE: You and every helper must be fresh and must not have authored, repaired or dispatched r0/r1 or assisted A1/A2 on them. Do not reuse prior W06 scratch-diagnosis, security-draft or runbook helpers. A1 is not A3. Declare independence and immutable HEAD/tree at entry/end. If invalid, return INVALID_REVIEW_TARGET. No product edits or commits, no memory/task writes; use disposable copies for mutation/negative controls.

REVIEW FOCUS (task): Tamper detection; least privilege.
ADDITIONAL FOCUS:
- Trace all protected-base code/scopes versus candidate data. Never execute candidate workflow/scripts with credentials. Inspect fetch/checkout trust, exact SHA binding, contents:read, event restrictions, no secret persistence, path/size/mode/symlink/FIFO handling, archive/export-ignore/export-subst behavior and safe diagnostics.
- Verify original cwd bug is fixed at call time, isolation preflight proves fixture ownership and complete self-test preserves source HEAD/index/status/bytes. Mutate the cwd default only in isolated synthetic copies and require failure. Retain safe bounded Git stderr tests.
- Audit conservative action-YAML parser (hidden literal/folded target events, permitted common guards, full40hex plus version comment, Git object metadata/symlink/size handling). Reproduce meaningful positive/negative/mutant controls; test defaults/predicates removed, not just mirrored assertions.
- Reproduce exact workflow replay and trusted spec checker leading-JSON/trailing-summary handling; assert p0_executed:false. Offline replay is not remote GitHub evidence.
- Critical rollout concern: real PR guards execute protected base, currently import 58b6ab38b0a514d94d8851679ad66ef9bf696c9e; new pin lint/verifier/workflow logic is not deployed there. Distinguish existing-bootstrap PR checks from evidence exercising the returned implementation. Identify exactly which controls need a separately authorized safe remote rollout/test arrangement, and any circular acceptance constraint. Do not resolve by running PR content with tokens, trusting a synthetic base as live main, waiving checks or marking remote cases passed.
- Verify branch-protection runbook matches actual recorded policy: merge commits only, eight Actions-app-pinned checks, no force/delete, admin enforcement, code ownership documentation; zero code-owner approvals/zero required owner approvals under recorded single-operator setup. Runbook is not authorization to mutate settings. Root Rust1.97.1 pin retained; all-lane NOTICE remains A1 pre-merge responsibility.
- Full-history gitleaks is nonzero baseline, A2reported26repeated findings at2unchangedW01contract-list locations. Independently classify actual current scan with redaction, scan the implementation range, and never suppress additional findings or report history clean. Shared-ref counts may change.

REPRODUCE (every executable test/install/scan through shared wrapper):
  python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r1-review-a -- COMMAND ARGUMENTS
- Verify git HEAD/tree=b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03/af193b0df3b88e64f0616b8d04d61799ba0d5045, both commit signatures, import ancestry and exact nine-path scope. Preserve original r1 and r0; no candidate writes in immutable worktrees.
- bash scripts/check_write_set.sh --isolation-test; bash scripts/check_write_set.sh --self-test.
- python3 -B scripts/verify_architecture.py ARCHITECTURE/V2_1; same --self-test; python3 -B scripts/check_action_pins.py --self-test; same --revision b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03; canonical check_spec.py.
- All RETURN AppendixA/B/C exact workflow replay/mutants/actual-scope/diagnostic tests, inspecting before use and redirecting only scratch roots to your isolated fixtures. All task local negative cases must specifically fail for the intended invariant.
- gitleaks detect --no-banner --redact; gitleaks detect --no-banner --redact --log-opts 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03; diff --check and hardcoding/prototype/syntax checks. Record actual status, not invented clean results.
- Cargo/database commands currently inapplicable: root/lane Cargo.toml and lane compose.yaml absent before T001/T002. Verify absence; no new manifest/crate/database or borrowed workspace. No claim RUN/provider/modelAPI calls.

OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/REVIEWS/W06-T000-ROOT-HARDENING-r1-a-A3_REVIEW.md using full A3 template plus REVIEW_PHASE:PRELIMINARY_LOCAL and PRELIMINARY_LOCAL_STATUS. For satisfactory local work, final verdict remains BLOCKED:FINAL_PR_CI_PENDING with separate local SATISFACTORY; real local defects require REPAIR_REQUIRED. Never final PASS/PASS_WITH_NONBLOCKING_FINDINGS/ACCEPTED with mandatory remote evidence absent. Account every criterion and negative with local/remote distinctions; include exact command exits/log locations and durable MEMORY_UPDATES.

STOP: independence or SHA/tree mismatch, foreign dirt, true secret, scope/contract ambiguity, repeated same failure twice, missing material/context, disk below20GiB. Caps3A2global/1lane/1executingbuild,compiler2/cache5G. G0/G1buildonly;G2NOT_PASSED/G3none/allclaimsNOT_RUN/productionunauthorized.

NEXT: A1 processes your report; only after satisfactory preliminary work will A1 request concrete separate push/PR authorization and genuine green/red CI evidence. Then a fresh final exact-SHA/tree A3 is required before acceptance/authorizedmerge. No remote action authorized here.

VERBATIM TASK SPECIFICATION (unchanged):

### W06-T000-ROOT-HARDENING
- **TASK_ID:** W06-T000-ROOT-HARDENING
- **OBJECTIVE:** **In plain words:** Lock down the repository's top level: one pinned Rust version for everyone, a CI check that fails if anyone changes the architecture files, a check that every GitHub Action is pinned, and the written branch-protection rules. **Precisely:** Harden the repository root created at bootstrap: pin `rust-toolchain.toml` (stable, exact version), add `.editorconfig` and `.gitattributes`, keep `common-arch-check.yml` running the architecture verifier (all package files, no extra files, pinned hashes) and `check_spec.py` on every PR (G-057) **from the protected base revision against the candidate's files as data** (audit F-2), keep `common-write-set-guard.yml` + `scripts/check_write_set.sh` enforcing **both** broad ownership and the exact per-task scope from `wsNN-*/coordination/WRITE_SCOPES.json` read from `main` (audit F-1), add a lint step that fails on GitHub Actions not pinned by full SHA, and write the branch-protection runbook.
- **WHY IT EXISTS:** Everyone depends on an architecture-integrity gate and one toolchain; root files are W06-owned.
- **ARCHITECTURE REQUIREMENTS:** 00F; 36 pinned builds; 49 (run the checker before export).
- **P0 CLAIMS:** enables all.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** root config files; `.github/workflows/common-arch-check.yml`, `common-write-set-guard.yml`; `scripts/verify_architecture.py`; `docs/branch-protection.md`.
- **EXACT WRITE SCOPE:** `/rust-toolchain.toml`, `/.editorconfig`, `/.gitattributes`, `/.github/workflows/common-arch-check.yml`, `/.github/workflows/common-write-set-guard.yml`, `/scripts/verify_architecture.py`, `/scripts/check_write_set.sh`, `/scripts/check_action_pins.py`, `ws06-platform-oss-qualification/docs/branch-protection.md`
- **READ-ONLY DEPENDENCIES:** `ARCHITECTURE/V2_1/**`, `COMMON/verify_architecture.py` (logic to port).
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `/rust-toolchain.toml`; `/.editorconfig`; `/.gitattributes`; `/.github/workflows/common-arch-check.yml`; `/.github/workflows/common-write-set-guard.yml`; `/scripts/verify_architecture.py`; `/scripts/check_write_set.sh`; `/scripts/check_action_pins.py`; `docs/branch-protection.md`. **Steps:** 1. Toolchain pin; A1 sends a NOTICE to all A1s before merging (every workspace rebuilds with it). 2. Verifier port: every file of the architecture package checked against the pinned manifest; fail on any mismatch or on `p0_executed:true`. 3. Action-pin lint: every `uses:` must be a 40-hex SHA with a `# vX.Y` comment. 4. Branch-protection runbook: merge commits only, CODEOWNERS required, required status checks, no force push, branches need not be up to date (parallel merges, AGENTS.md §3a).
- **SECURITY INVARIANTS:** Workflows use `permissions: contents: read` and no secrets. `pull_request_target` is used **only** by the two `common-*` guard workflows, which check out the base SHA, read the PR head purely as git data (`git diff --name-only`, `git cat-file -s`, `git archive` of `ARCHITECTURE/`) and never execute PR content. Required checks are pinned to the GitHub Actions app (`create_github_repo.sh`). Every other workflow: no `pull_request_target`.
- **FAILURE CASES:** Architecture modified without detection; floating action tags; a PR that edits the guard or verifier makes its own forbidden change pass; a task branch writes a neighbouring task's file inside its own workstream.
- **TESTS REQUIRED:** A scratch PR changing one byte under `ARCHITECTURE/` fails arch-check; a scratch PR adding an extra file under `ARCHITECTURE/` fails arch-check; an unpinned `uses:` fails the pin lint; guard self-test script `scripts/check_write_set.sh` table of cases run locally against a temporary repo. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scratch PR changing one byte under ARCHITECTURE/ → arch-check red`; `scratch PR adding an extra file under ARCHITECTURE/ → arch-check red`; `main → arch-check green`; `scratch unpinned `uses:` → pin lint red`; `python3 scripts/verify_architecture.py ARCHITECTURE/V2_1` → mismatches: []; `scratch task branch touching a neighbouring task's file → write-set-guard red`; `scratch task branch touching coordination/memory.md → write-set-guard red`; `scratch harness branch touching the excluded harness/Cargo.toml → write-set-guard red`; `scratch branch for a task absent from WRITE_SCOPES.json → write-set-guard red`; `scratch PR whose title names another task → write-set-guard red`; `scratch PR that edits check_write_set.sh, verify_architecture.py and the workflow to always pass AND changes ARCHITECTURE/ + a foreign path → both checks still red`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Those scratch cases (shown in the return, then deleted), plus: (a) a task branch changing a file of another task in the same workstream → write-set-guard red; (b) a task branch changing `coordination/memory.md` → red; (c) a harness branch changing the excluded `claims/<C>/harness/Cargo.toml` → red; (d) a branch naming a task that is not in WRITE_SCOPES.json → red; (e) a PR title naming a different task than its branch → red; (f) **one PR that both weakens `scripts/check_write_set.sh` / `scripts/verify_architecture.py` / the workflow file and makes a forbidden change still gets both checks red** (they run from base).
- **ACCEPTANCE CRITERIA:** arch-check green on main, red on tamper; pin lint active; runbook written.
- **EVIDENCE TO RETURN:** CI links for the green and red runs.
- **OUT-OF-SCOPE:** Workstream CI files; supply-chain CI (T007).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Tamper detection; least privilege.
