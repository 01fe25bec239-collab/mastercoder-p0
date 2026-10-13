FROM: STARTUP-A1 (acting W06-A1)
TO: W06-A2[W06-T000-ROOT-HARDENING-r0]
MESSAGE_TYPE: DISPATCH
TASK_ID: W06-T000-ROOT-HARDENING
ATTEMPT: r0
DISPOSITION: DISPATCHED — prepared for a fresh A2 session; agent not launched
DATE_UTC: 2026-10-12T18:13:11Z
WORKSTREAM: W06
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r0
  BRANCH: ws06/w06-t000-root-hardening-r0
  WORKSTREAM_DIR: ws06-platform-oss-qualification/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
IMPLEMENTATION_SHA / REVIEWED_SHA: NONE
TREE: NONE
START_TREE: aac9179014e2b2981a4efeb95e44ab5450bdd547
SOLE_PARENT: NONE
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

OBJECTIVE: **In plain words:** Lock down the repository's top level: one pinned Rust version for everyone, a CI check that fails if anyone changes the architecture files, a check that every GitHub Action is pinned, and the written branch-protection rules. **Precisely:** Harden the repository root created at bootstrap: pin `rust-toolchain.toml` (stable, exact version), add `.editorconfig` and `.gitattributes`, keep `common-arch-check.yml` running the architecture verifier (all package files, no extra files, pinned hashes) and `check_spec.py` on every PR (G-057) **from the protected base revision against the candidate's files as data** (audit F-2), keep `common-write-set-guard.yml` + `scripts/check_write_set.sh` enforcing **both** broad ownership and the exact per-task scope from `wsNN-*/coordination/WRITE_SCOPES.json` read from `main` (audit F-1), add a lint step that fails on GitHub Actions not pinned by full SHA, and write the branch-protection runbook.

ENTRY: Print your actual WORK LOCATION first. Also read /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/NOTICES/INSTRUCTION_AUDIT_2026-10-13.md.

READ FIRST (authoritative live coordination records, absolute paths):
1. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/AGENTS.md
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/SKILLS.md
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/task.md
4. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/memory.md
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/NOTICES/STARTUP_ROUTING_2026-10-12.md
6. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/NOTICES/G0_PHASED_BUILD_AUTHORIZATION.json
Then read the task's architecture requirements in /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r0/ARCHITECTURE/V2_1 and root SKILL.md/SUBAGENT_POLICY.md. Follow SKILLS sections 1–12, 13.2, 14 and 15. The local task-worktree coordination snapshot was inherited from main before startup records were committed; do not mistake its old G0/G1 values for the authoritative coordinator records above. Do not copy or commit coordination changes onto this task branch.

AUTHORIZED WRITE SET — repository-relative expanded globs from the protected-base task scope:
- .editorconfig
- .gitattributes
- .github/workflows/common-arch-check.yml
- .github/workflows/common-write-set-guard.yml
- rust-toolchain.toml
- scripts/check_action_pins.py
- scripts/check_write_set.sh
- scripts/verify_architecture.py
- ws06-platform-oss-qualification/docs/branch-protection.md

Scope remains constrained by the verbatim task qualifiers below (empty modules/skeleton only where stated). The only coordination write exception is this attempt's RETURN at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/RETURNS/W06-T000-ROOT-HARDENING-r0-A2_RETURN.md; do not edit memory.md, task.md, other returns or notices.

PREREQUISITES VERIFIED BY A1:
- Fetched origin/main: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e; fresh task branch exists, clean, hooks active and .env.local symlink verified.
- G0 PASS_PHASED_BUILD_STARTUP and G1 signature/digests recorded in authoritative coordination memory. No upstream INTEGRATED task is required by this task's PRECONDITIONS.
- Three disjoint tasks prepared globally, one per lane. No other task implementation worktree/in-flight task existed at dispatch.
- Owner caps: three active A2 tasks globally, one per lane; one active build; compiler jobs two, cache five G; stop new build/dispatch below twenty GiB free. Model budget one million API tokens/five USD total; cloud and security keys zero.
- Deferred resources retain their owning task preconditions. No claims have run; G2 not passed, no G3, production unauthorized.
- Prior helper attempts hit the account usage limit; verify availability and never silently substitute models. Independent work may continue without unavailable helpers.

SINGLE BUILD SLOT (mandatory for all dependency resolution/install, Rust builds/tests, frontend builds/tests and browser installs):
Use the local A1-managed wrapper, outside the product repository:
    python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd "/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r0/ws06-platform-oss-qualification" -- COMMAND ARGUMENTS
Replace COMMAND ARGUMENTS with the actual command, e.g. cargo build --workspace --locked or pnpm -r typecheck. The wrapper waits for the shared OS lock, checks the twenty-GiB floor, and releases the slot on exit. For grouped commands use sh -c with proper quoting. Load lane env explicitly for commands that need it without printing it. Do not run build commands around the lock or set a shared CARGO_TARGET_DIR. Do not change the wrapper or any other task's files.

VERBATIM TASK SPECIFICATION — every original field retained unchanged:

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


A1 APPLICATION NOTES (do not replace the task's exact acceptance criteria):
- All task TESTS REQUIRED, NEGATIVE TESTS and acceptance criteria above remain mandatory. Keep a criterion-to-evidence checklist.
- The owner-approved phase defers database startup until the owning scaffold is integrated. Do not run compose up/down or borrow existing unrelated databases. Report database-dependent checks as pending when no genuine database exists; never fabricate a pass.
- /deny.toml is W06-T007's future deliverable, not W06-T000's. Do not fabricate it or edit outside this task scope. If a required command actually needs an absent dependency, record the exact blocker in RETURN.
- A task requiring an unpublished shared interface uses only its explicitly allowed fallback. W01-T000 may use the PostgreSQL digest recorded in authoritative W01 memory until I-14 lands.
- W06-T000 is a root-file task, before W06-T001 creates a Cargo workspace. Record any inapplicable/unavailable generic workspace command accurately; do not invent a manifest or run against another task's workspace. No unavailable command may be represented as passed.
- The canonical verifier source named COMMON/verify_architecture.py by the task is available read-only at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/verify_architecture.py; the product repository has no COMMON directory.
- All new files carry the prototype marking in syntax appropriate for that file format. Do not add invalid comments to JSON or lockfiles merely to satisfy a generic marking rule.
- Verify exact current dependency versions from official registries/documentation when implementing; do not guess versions. Build tasks do not call paid model APIs.
- Remote scratch PRs and PR CI remain required where specified. This dispatch authorizes local implementation/tests/commits only. Do not push, open PRs or mutate provider settings. If only remote PR CI remains pending after all local requirements pass, return NOT_READY: LOCAL_COMPLETE_PENDING_PR_CI. Under the owner-approved BUILD_REVIEW_CI_SEQUENCE_DECISION.md, A1 arranges preliminary independent review, obtains separate push/PR authorization, then collects real CI evidence for final fresh A3 review. No final PASS or ACCEPTED before all mandatory evidence exists. Never invent a green CI URL or treat local tests as remote CI.
- Baseline full-history gitleaks has two recorded contract-name false positives in W01 task.md at the INPUT CONTRACTS lists. Run the required full scan and record its actual result; also scan this attempt's changes. Do not suppress new findings or call a nonzero full scan clean.

REQUIRED COMMANDS:
1. Verify git branch, initial HEAD=58b6ab38b0a514d94d8851679ad66ef9bf696c9e and clean status before edits; stop on a mismatch without reset/stash/delete.
2. Run every task-specific exact command/test listed in the verbatim specification above, recording cwd and exit status; use the single-build wrapper for build commands. Do not invent CLI interfaces to scripts that do not yet exist; implement only allowed task scripts and document their exact invocation.
3. Run applicable lane AGENTS.md section 11 commands with the lane env loaded explicitly, --locked/--frozen-lockfile after initial scaffold lock generation. Report absent prerequisites exactly as described above.
4. From repository root: python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (must report p0_executed:false).
5. From repository root: gitleaks detect --no-banner; report actual baseline findings distinctly from any new finding, with no secrets in outputs.
6. From repository root: git diff --name-only 58b6ab38b0a514d94d8851679ad66ef9bf696c9e and git diff --check. Confirm every product change matches the authorized globs/qualifiers.

STOP CONDITIONS:
Needed change outside scope; missing or contradictory contract/interface; true secret exposure; unauthorized provider mutation/claim RUN; ambiguous architecture; foreign dirty changes; base not ancestor; disk below the floor; repeated same failure twice or three failed hypotheses; context too low to finish safely. Stop and return NOT_READY with the exact blocker and completed work. Never weaken a test, guard or frozen contract.

COMPLETION CONDITIONS:
arch-check green on main, red on tamper; pin lint active; runbook written.
All required applicable commands/test evidence and negative cases must be accounted for. READY_FOR_A3_REVIEW requires all mandatory acceptance evidence, clean signed committed product changes, exact IMPLEMENTATION_SHA/TREE, scope compliance, and no true new secret. Pending mandatory remote CI/resource evidence means NOT_READY with explicit blockers, not an invented PASS. A return blocked only on PR CI may enter preliminary independent review under the owner-approved sequence; final A3 PASS and acceptance still require actual green PR CI. A1 alone records acceptance after fresh independent A3 review.

RETURN:
Write /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/RETURNS/W06-T000-ROOT-HARDENING-r0-A2_RETURN.md using the A2 RETURN TEMPLATE below. List durable facts under MEMORY_UPDATES; never edit memory.md. Do not commit the RETURN in the task worktree. End with a one-line absolute coordinator worktree path and an A1 resume prompt pointing to that RETURN, then stop. Do not self-approve, push, open PRs, merge, amend a commit after RETURN, or continue to another task.

A2 RETURN TEMPLATE (fill actual values; NONE for absent results):
# A2 RETURN TEMPLATE

File: `coordination/RETURNS/<TASK_ID>-<ATTEMPT>-A2_RETURN.md`. Fill every field. Write `NONE` rather than leaving one out. Do not paste secrets, raw provider tokens, or source excerpts beyond what a test name needs.

```
FROM: W0N-A2[<TASK_ID>-<ATTEMPT>]
TO: W0N-A1#<n>
MESSAGE_TYPE: RETURN
TASK_ID: <TASK_ID>
ATTEMPT: <r0|r1|...>
DISPOSITION: READY_FOR_A3_REVIEW | NOT_READY: <reason>
DATE_UTC: <ISO-8601>
WORKSTREAM: WORKSTREAM-0N-<NAME>
LOCATION:
  REPO: mastercoder-p0 (<remote>)
  WORKTREE: <abs path>
  BRANCH: <branch>
  WORKSTREAM_DIR: ws0N-<name>/
  COORDINATION_DIR: ws0N-<name>/coordination/
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: <sha>

TASK_ID: <TASK_ID>
BASE_SHA: <40-hex>
IMPLEMENTATION_SHA: <40-hex>
TREE: <40-hex from `git rev-parse <IMPLEMENTATION_SHA>^{tree}`>
COMMITS: <list of 40-hex from `git rev-list <BASE_SHA>..<IMPLEMENTATION_SHA>`>
CHANGED_FILES:
  <output of `git diff --stat <BASE_SHA>..<IMPLEMENTATION_SHA>`>
WRITE_SET_CHECK: <PASS: every changed path matches the authorized globs | FAIL: <paths>>
TESTS_ADDED:
  - <test path::name> — <what it proves> — POSITIVE|NEGATIVE
TESTS_RUN:
  - <exact command> (cwd: <dir>)
TEST_RESULTS:
  - <command> → exit <code>; <passed>/<failed>/<ignored>
  - python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py → exit <code>; p0_executed:false
ARCHITECTURE_CONTRACTS_USED:
  - <doc number/section, ADR, invariant IDs>
SCHEMAS_USED:
  - <registered type names + vector files/ids>
DOUBLES_USED:
  - <I-xx@vN path> (DRY_RUN only, never evidence)
ASSUMPTIONS:
  - <each assumption + why the architecture permits it; none may weaken an invariant>
KNOWN_LIMITATIONS:
  - <prototype shortcuts, labeled PROTOTYPE_ONLY, with the ceiling>
BLOCKERS: <NONE | exact blocker + owner>
CCR_DRAFTED: <NONE | CCR-W0N-NNN path>
SECRETS_CHECK: <gitleaks/canary-scanner command + result>
MEMORY_UPDATES:   # A2 never edits memory.md (audit F-13); the A1 merges accepted items when it processes this RETURN
  - <section: Decisions | Known pitfalls | Failed approaches | Interface pins | Environment> — <one durable fact, no secrets> | NONE
PUSH / PR / MERGE: NONE (A2 never pushes unless the human authorized it in this session)
READY_FOR_A3_REVIEW: YES | NO
```

Rules:
- `READY_FOR_A3_REVIEW: YES` requires: all completion conditions met, worktree clean, WRITE_SET_CHECK PASS, SECRETS_CHECK clean.
- Never amend or force-push a commit after writing the return. A change after the return is a new attempt.
- If you found that the architecture is wrong or ambiguous, say so in BLOCKERS and draft a CCR. Do not "fix" it in code.
