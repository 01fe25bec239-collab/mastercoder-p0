# task.md — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION (Platform, OSS & Qualification)

Operator: master (one human, all six lanes). Coordinator: this lane's current A1 (see latest `HANDOFFS/`). Every task below is execution-ready with all 20 fields. A1 runs up to `MC_MAX_PARALLEL_A2` A2 tasks at once (AGENTS.md §3a), each in its own worktree with a fresh A2.

**Mission.** Own the shared platform and cross-component qualification: repo root and CI, architecture tamper check, pinned test DB, supply-chain and secret scanning, evidence kit and generated claim ledger, canary scanner and observability, the Rust↔TS parity harness, admin-domain profiles, artifact store lifecycle, transfer/failover/restore/load/upgrade/logs drills, OSS governance drafts, and the freeze dossier.

## In flight (A1 keeps this current; copy it into every handoff)

| TASK_ID | ATTEMPT | WORKTREE | BASE_SHA | WRITE SCOPE | STARTED (UTC) | WAITING FOR |
|---|---|---|---|---|---|---|
| W06-DOCS-REPOSITORY-OVERVIEW | r0 | /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-DOCS-REPOSITORY-OVERVIEW-r0 | 9a7922bf29c0a6689903b96896dab8851d1eeeeb | four exact Markdown draft paths | 2026-10-13T05:51:57.586435+00:00 | fresh A2 drafts/RETURN; publication pending |

## Parallel waves (BUILD tasks, from PRECONDITIONS)

A wave can start when every task in the earlier waves that it depends on is INTEGRATED (merged, and an ancestor of the new BASE_SHA). Within a wave, tasks run in parallel if their write scopes don't overlap (AGENTS.md §3a). 35 BUILD tasks, 4 waves, widest wave 13 tasks. Cross-workstream preconditions are listed; for BUILD, use the published double if the real interface is not merged yet.

| WAVE | TASKS | TASK IDS |
|---|---|---|
| 1 | 9 | W06-T000, W06-T001 (also needs W01-T000, W02-T000, W03-T000, W04-T000), W06-T002, W06-T003, W06-T004, W06-T005, W06-T006, W06-T033, W06-T034 |
| 2 | 13 | W06-T007, W06-T008, W06-T009, W06-T011, W06-T012, W06-T013, W06-T014, W06-T015, W06-T019, W06-T020 (also needs W01-T015, W01-T018, W02-T003, W04-T006), W06-T021 (also needs W01-T023, W03-T022), W06-T022 (also needs W01-T015, W01-T018), W06-T023 (also needs W01-T015, W01-T022, W02-T007, W02-T018, W04-T006) |
| 3 | 12 | W06-T010, W06-T016, W06-T017, W06-T018, W06-T024 (also needs W01-T022), W06-T025, W06-T026, W06-T027, W06-T028, W06-T029, W06-T030 (also needs W01-T017), W06-T031 |
| 4 | 1 | W06-T032 |

## Task state ledger (A1 updates; never delete rows)

| TASK_ID | KIND | STATE | ATTEMPT | IMPLEMENTATION_SHA | ACCEPTED_SHA | MERGED_SHA | NOTES |
|---|---|---|---|---|---|---|---|
| W06-DOCS-REPOSITORY-OVERVIEW | BUILD | DISPATCHED | r0 | NONE | NONE | NONE | Owner README questions; local draft-only A2; no staging/commit/outward authorization; proposed new doc scope not in protected main |
| W06-T000-ROOT-HARDENING | BUILD | INTEGRATED | r1 | b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 | b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 | 5f92752cac8d9ba39ef20a6caf4c1c95227d7036 | Fresh final r1-d A3 PASS_WITH_NONBLOCKING_FINDINGS; actual commissioning merge/main activation verified; N-1/N-2 retained |
| W06-T001-WS06-WORKSPACE-SCAFFOLD | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T002-TESTDB-I14 | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T003-EVIDENCE-SCHEMAS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T004-OSS-GOVERNANCE-DRAFTS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T005-STAGING-ENVIRONMENT-DOC | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T006-SUPPLY-CHAIN-POLICY | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T007-SUPPLY-CHAIN-CI | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T008-INDEPENDENCE-AND-REPRO-CHECKS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T009-EVIDENCE-CLI | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T010-EVIDENCE-STORE | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T011-CLAIM-LEDGER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T012-CANARY-SCANNER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T013-OBSERVABILITY-COLLECTOR | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T014-ARTIFACT-STORE | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T015-PARITY-RUNNER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T016-P0-06A-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T017-P0-06B-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T018-FREEZE-DOSSIER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T019-ADMIN-DOMAIN-IAC | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T020-LOAD-GENERATOR-AND-CHECKERS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T021-UPGRADE-ORCHESTRATOR | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T022-FAILOVER-RIG | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T023-TRANSFER-ORCHESTRATOR | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T024-P0-10C-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T025-P0-10D-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T026-P0-07D-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T027-P0-10E-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T028-P0-10F-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T029-P0-10G-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T030-RESTORE-RIG | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T031-P0-10B-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T032-P0-10H-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T033-HOST-LAB-INVENTORY | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-T034-LOCAL-SERVICES | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W06-RUN-P0-06A | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-06B | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-07D | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10B | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10C | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10D | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10E | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10F | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10G | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |
| W06-RUN-P0-10H | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |

States: NOT_STARTED · DISPATCHED · RETURNED · IN_REVIEW · REPAIR · ACCEPTED · INTEGRATED · BLOCKED. RUN tasks additionally carry, per cell and per immutable run, the measured outcome (PASS/FAIL/INCONCLUSIVE, `claims/<id>/cells/<cell>/runs/<RUN_ID>/outcome.json`, written once) and the evidence lifecycle (NOT_RUN/RUNNING/SUBMITTED/ACCEPTED/REJECTED, `lifecycle.jsonl`); `cells/<cell>/current.json` names the current run.

## Ordering rules

- BUILD tasks may start after G0 + G1 when their PRECONDITIONS hold; use published doubles for anything another workstream has not INTEGRATED yet. Never let a lane idle waiting for another lane — use the double and record the pin.
- RUN tasks (`W06-RUN-*`) start only after G2 + G3 + their INTEGRATED build task(s) + integration milestone. Until then the harness is exercised as DRY_RUN inside BUILD tasks.
- Interfaces you provide (I-13, I-14, canary-scanner, observability config, local-services (Git server + registries) for W03/W04 (W06-T034), host-lab inventory for W03 (W06-T033)) should be published early (doubles first) — consumers depend on them.
- Post-freeze production work is NOT in this file; see `POST_FREEZE/POST_FREEZE_BACKLOG.md` (not dispatched).

---

## BUILD tasks

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

### W06-T001-WS06-WORKSPACE-SCAFFOLD
- **TASK_ID:** W06-T001-WS06-WORKSPACE-SCAFFOLD
- **OBJECTIVE:** **In plain words:** Set up the empty Rust project for the W06 lane's own tools (evidence CLI, ledger, canary scanner, parity runner, drill kit, freeze dossier) and the test-program folders for his 10 claims, with no logic yet. **Precisely:** Create the `ws06-platform-oss-qualification/` skeleton so every later W06 task only edits its own module files, scripts, tests and drill folders: Cargo workspace with every W06 crate and claim harness as a compiling empty skeleton with its final dependency list and declared modules, drill folder layout, compose base and `ws06-ci.yml`.
- **WHY IT EXISTS:** Parallel A2s conflict only on shared files; the scaffold owns them once (AGENTS.md §3a).
- **ARCHITECTURE REQUIREMENTS:** 00F; ADR-001; 36.
- **P0 CLAIMS:** none directly; enables all 10 W06 claims.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** Workspace; `docs/MODULES.md`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/{Cargo.toml,Cargo.lock,compose.yaml,.env.example,README.md,.gitignore}`, `ws06-platform-oss-qualification/docs/{MODULES.md,README.md}`, `ws06-platform-oss-qualification/crates/*/{Cargo.toml,src/lib.rs,src/main.rs}`, `ws06-platform-oss-qualification/crates/*/src/*.rs` (empty declared modules only), `ws06-platform-oss-qualification/claims/*/harness/{Cargo.toml,src/main.rs}`, `ws06-platform-oss-qualification/claims/README.md`, `ws06-platform-oss-qualification/drills/README.md`, `ws06-platform-oss-qualification/drills/{admin-domains,deletion,logs-audit,load,upgrade,failover,restore,transfer}/README.md`, `.github/workflows/ws06-ci.yml`
- **READ-ONLY DEPENDENCIES:** `ws01-core-security/crates/{mc-contracts,mc-wire,mc-lease,mc-effects,mc-journal,mc-negotiation}` and W01 doubles `lease`, `effects-journal`, `custody`; W02 `doubles/fake-github/v1` and `crates/mc-eligibility`; W03 `doubles/fake-runner/v1`; W04 `doubles/model-broker/v1` (path deps; audit F-6 — the drills and harnesses of W06-T020…T030 name them).
- **PRECONDITIONS:** G0, G1; W01-T000 INTEGRATED; W02-T000 INTEGRATED; W03-T000 INTEGRATED; W04-T000 INTEGRATED (every crate/double this workspace path-depends on must exist on main).
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `Cargo.toml`; `Cargo.lock`; `compose.yaml`; `.env.example`; `README.md`; `.gitignore`; `docs/{MODULES.md,README.md}`; `crates/{mc-evidence,mc-ledger-gen,canary-scanner,parity-harness,mc-drill-kit,freeze-dossier}/{Cargo.toml,src/lib.rs or main.rs,src/<declared module>.rs}`; `claims/<10 claims>/harness/{Cargo.toml,src/main.rs}`; `claims/README.md`; `drills/README.md`; `drills/{admin-domains,deletion,logs-audit,load,upgrade,failover,restore,transfer}/README.md`; `../.github/workflows/ws06-ci.yml`. **Steps:** 1. Virtual workspace, `members = ["crates/*", "claims/*/harness"]`, pinned `[workspace.dependencies]` (tokio, serde, serde_json, jsonschema, sha2, hex, clap, sqlx, reqwest (rustls, no default features), flate2, regex, base64, thiserror, tempfile, proptest, aws-sdk-s3 or an S3-compatible minimal client — record the choice). 2. Crates and declared modules: mc-evidence (cli: seal, verify, bundle, status, store), mc-ledger-gen (ledger, expiry), canary-scanner (generate, scan, formats), parity-harness (runner, compare, matrix), mc-drill-kit (orchestrate, partition, clock, report) shared by all drills, freeze-dossier (gen). 3. Harness binaries for P0-06A, P0-06B, P0-07D, P0-10B, P0-10C, P0-10D, P0-10E, P0-10F, P0-10G, P0-10H. 4. `drills/{admin-domains,deletion,logs-audit,load,upgrade,failover,restore,transfer}/README.md` placeholders. 5. CI `ws06-ci.yml` (job `ws06-ci`; change gate over the lane directory + itself); read-only; fmt, clippy, test. 6. `docs/MODULES.md` with owner table, the **crate → imports table** built from every W06 task's "Exact public API" and READ-ONLY DEPENDENCIES (each W01/W02/W03/W04 crate or double a drill or harness names is a path dependency of exactly the crate that uses it; audit F-6) and the dependency rule; CI runs cargo with `--locked`. **Shared versions:** for every crate that `ws01-core-security/Cargo.toml` also pins in `[workspace.dependencies]`, copy W01's exact version (path dependencies on W01 crates fail to resolve if two workspaces pin different exact versions of the same crate); only crates W01 does not use are chosen fresh. **Required-check rule (one operator, parallel auto-merge):** the workflow is named and has one job named `ws06-ci` (a required check on main), triggers on **every** pull request to main and every push to main (no workflow-level `paths:` filter, so it always reports), and its first step decides with `git diff --name-only <merge-base> HEAD` whether this lane's paths or the upstream paths listed here changed; if not, the remaining steps are skipped by `if:` and the job still reports success. Without this, auto-merge could merge a PR whose lane tests never ran.
- **SECURITY INVARIANTS:** No secrets; CI read-only.
- **FAILURE CASES:** A crate missing a dependency a later task needs.
- **TESTS REQUIRED:** `cargo build --workspace`, `cargo test --workspace`; CI green. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `cargo build --workspace` exits 0; `cargo test --workspace` exits 0; `cargo metadata --no-deps` lists exactly 16 members (6 crates + 10 harnesses); `CI green`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** none beyond build.
- **ACCEPTANCE CRITERIA:** 6 crates and 10 harnesses compile; modules declared.
- **EVIDENCE TO RETURN:** CI URL; pinned versions.
- **OUT-OF-SCOPE:** Any logic.
- **BLOCKERS:** G0/G1.
- **A3 REVIEW FOCUS:** Dependency completeness.

### W06-T002-TESTDB-I14
- **TASK_ID:** W06-T002-TESTDB-I14
- **OBJECTIVE:** **In plain words:** Provide the one shared test database everyone uses: a PostgreSQL image pinned to an exact version, a compose snippet and a script that hands out a fresh database per test run. **Precisely:** Publish I-14, the shared test database: a PostgreSQL image pinned by digest that implements the PostgreSQL semantics the architecture cites (G-081), a compose fragment every workspace can include, and `scripts/testdb up|down|url` that starts it and hands out a fresh database per test run (applying W01's role framework first when present). Other workstreams switch to it when it lands.
- **WHY IT EXISTS:** Everyone needs one deterministic database version; behaviour like `clock_timestamp()` and RLS must match the docs.
- **ARCHITECTURE REQUIREMENTS:** 07 PostgreSQL semantics; 00F; 36.
- **P0 CLAIMS:** enables all DB-backed claims.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `testdb/{compose.fragment.yaml,README.md,IMAGE_DIGEST}`, `/scripts/testdb`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/testdb/**`, `/scripts/testdb`
- **READ-ONLY DEPENDENCIES:** `ws01-core-security/db/authority/roles/*.sql` (optional).
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `testdb/{compose.fragment.yaml,README.md,IMAGE_DIGEST}`; `/scripts/testdb`; `testdb/tests/test_testdb.sh`. **Steps:** 1. Choose the image by digest; record version and settings (`max_connections` etc.). 2. Script creates `mc_<ws>_<random>` databases and prints the URL. 3. README tells each workstream how to switch from its own pin.
- **SECURITY INVARIANTS:** Password from env only.
- **FAILURE CASES:** Floating image tag.
- **TESTS REQUIRED:** `scripts/testdb up && scripts/testdb url` works twice concurrently; `SELECT clock_timestamp()` advances inside one transaction. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_testdb.sh::two_concurrent_urls_differ`; `test_testdb.sh::clock_timestamp_advances_in_one_tx`; `test_testdb.sh::tag_only_image_rejected`; `test_testdb.sh::w01_roles_applied_when_present`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Tag-only image reference rejected by a check in the script.
- **ACCEPTANCE CRITERIA:** Usable by all workspaces; digest recorded.
- **EVIDENCE TO RETURN:** Image digest. A1 sends INTERFACE_NOTICE I-14@v1 to all.
- **OUT-OF-SCOPE:** HA setups (T022).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Digest pin; isolation.

### W06-T003-EVIDENCE-SCHEMAS
- **TASK_ID:** W06-T003-EVIDENCE-SCHEMAS
- **OBJECTIVE:** **In plain words:** Define the evidence file formats every claim run must produce (which versions were used, which files, who reviewed it, the result), as JSON schemas with examples. Every claim test program writes its draft in this format. **Precisely:** Publish the I-13@v1 evidence formats as JSON Schemas: `run-manifest.json` (every pinning-rule field: Rust/rustc/Cargo.lock digests, Node/TS/lockfile, PostgreSQL build/extensions, OS/kernel/hypervisor, image SHA-256s, REGISTRY_SHA256, provider API version/account/App identity/request dates, UNDISCLOSED_PROVIDER_MANAGED, NOT_APPLICABLE), `fixture-digests.json`, `evidence-manifest.sha256` (format), `independent-review.json` (lens, reviewer, independence declaration, ACCEPTED/REJECTED, outcome PASS/FAIL/INCONCLUSIVE), `outcome.json` (measured outcome written once after execution), `lifecycle.jsonl` (append-only NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED→EXPIRED|INVALIDATED entries with time and actor) and `current.json` (per cell: current RUN_ID, superseded RUN_ID, reason) and `vector-results.json` (the one output format both codec CLIs — W01 `mc-wire-vectors` and W05 `wire-ts-vectors` — emit for the parity harness: per vector id, type name, outcome ACCEPT/REJECT, error class, output bytes hex, digest hex, signature-check result), with example files, for the layout `claims/<claim>/cells/<cell>/runs/<RUN_ID>/` (audit F-16). Every harness's `run-manifest.draft.json` follows these.
- **WHY IT EXISTS:** 58 execution manifests; 19/57 lifecycle vs outcome; all harness tasks need the format early.
- **ARCHITECTURE REQUIREMENTS:** 57 claim fields (pinning_rule, evidence_artifacts, evidence_expiry); 19 Prototype Evidence; AtomicPrototypeClaimV1 schema.
- **P0 CLAIMS:** enables all RUN tasks.
- **INPUT CONTRACTS:** `TRACEABILITY/atomic-prototype-claims.json`, `SCHEMAS/AtomicPrototypeClaimV1.schema.json`.
- **OUTPUT CONTRACTS:** `evidence-kit/schemas/*.schema.json`, `evidence-kit/examples/**`, `evidence-kit/FORMATS.md`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/evidence-kit/**`
- **READ-ONLY DEPENDENCIES:** claims JSON.
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `evidence-kit/schemas/{run-manifest,fixture-digests,evidence-manifest,independent-review,outcome,lifecycle,current,vector-results}.schema.json`; `evidence-kit/examples/{valid,invalid}/**`; `evidence-kit/FORMATS.md`; `evidence-kit/tests/test_schemas.py`. **Steps:** 1. One schema per file type, `additionalProperties: false`. 2. Lifecycle and outcome are separate: an outcome may exist at any lifecycle after execution (SUBMITTED + PASS is valid and normal) and is preserved through REJECTED/EXPIRED/INVALIDATED; ACCEPTED needs two reviews of the claim's lenses; capability enablement (ACCEPTED + PASS + current) is computed by the ledger, never stored (19; audit F-15). 3. Every run lives in its own immutable `runs/<RUN_ID>/` directory; `current.json` per cell selects the current run with an explicit supersession record. 4. Valid and invalid examples, including a Linux PASS run and a Windows FAIL run of one claim coexisting. 5. FORMATS.md explains each field in plain words.
- **SECURITY INVARIANTS:** No field may hold a secret (canary references only).
- **FAILURE CASES:** Outcome mixed into lifecycle.
- **TESTS REQUIRED:** Examples validate (valid pass, invalid fail) with `python3 -m jsonschema` or the CLI later. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_schemas.py::valid_examples_pass`; `test_schemas.py::manifest_missing_cargo_lock_invalid`; `test_schemas.py::accepted_with_one_review_invalid`; `test_schemas.py::submitted_with_pass_is_valid`; `test_schemas.py::outcome_preserved_after_rejection_expiry_invalidation`; `test_schemas.py::linux_pass_and_windows_fail_runs_coexist`; `test_schemas.py::current_pointing_to_other_cell_invalid`; `test_schemas.py::lifecycle_going_backwards_invalid`; `test_schemas.py::additional_properties_rejected`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Manifest missing Cargo.lock digest invalid; lifecycle ACCEPTED with one review invalid; an outcome value outside PASS/FAIL/INCONCLUSIVE invalid; `current.json` naming a RUN_ID of another cell invalid; a lifecycle file whose entries go backwards invalid. (Positive: SUBMITTED + PASS valid.)
- **ACCEPTANCE CRITERIA:** All seven formats; examples correct; per-cell run layout documented.
- **EVIDENCE TO RETURN:** Schema SHA-256s. A1 sends INTERFACE_NOTICE I-13@v1 to all.
- **OUT-OF-SCOPE:** CLI (T009), store (T010), ledger (T011).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Lifecycle/outcome separation.

### W06-T004-OSS-GOVERNANCE-DRAFTS
- **TASK_ID:** W06-T004-OSS-GOVERNANCE-DRAFTS
- **OBJECTIVE:** **In plain words:** Draft the files a public repository would need (contribution rules, security policy, code of conduct, license options and so on) without choosing a license and without publishing anything. **Precisely:** Draft the public-repository governance set under `oss/` per COMMON/OPEN_SOURCE_STRATEGY.md, license-neutral because the licensing decision is still open (proprietary vs open core AGPL + commercial vs Apache-2.0): LICENSE options memo and `LICENSE_DECISION.md` (records master's eventual decision), license-compatibility policy, README skeleton, CONTRIBUTING (DCO and CLA variants), CODE_OF_CONDUCT, SECURITY.md (private vulnerability reporting, disclosure timeline), SUPPORT, GOVERNANCE, CODEOWNERS template, issue/PR templates, ARCHITECTURE_CONTRIBUTING (changes via RFC/CCR), RELEASING (SemVer, signed releases), DEPENDENCY_POLICY, ROADMAP_POLICY, and a public/private content boundary checklist (G-010, G-078). Nothing is published.
- **WHY IT EXISTS:** Prepare the option to open-source without publishing anything now.
- **ARCHITECTURE REQUIREMENTS:** 36; 24 privacy; OPEN_SOURCE_STRATEGY.md.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `oss/**`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/oss/**`
- **READ-ONLY DEPENDENCIES:** COMMON/OPEN_SOURCE_STRATEGY.md.
- **PRECONDITIONS:** G0.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `oss/{LICENSE_OPTIONS.md,LICENSE_DECISION.md,LICENSE_COMPATIBILITY.md,README.md,CONTRIBUTING.dco.md,CONTRIBUTING.cla.md,CODE_OF_CONDUCT.md,SECURITY.md,SUPPORT.md,GOVERNANCE.md,CODEOWNERS.template,ARCHITECTURE_CONTRIBUTING.md,RELEASING.md,DEPENDENCY_POLICY.md,ROADMAP_POLICY.md,PUBLIC_PRIVATE_BOUNDARY.md}`; `oss/.github/{ISSUE_TEMPLATE/**,PULL_REQUEST_TEMPLATE.md}`; `oss/boundary_scan.py`. **Steps:** 1. Each file license-neutral; both license variants prepared side by side. 2. No SPDX headers until master decides. 3. Boundary checklist with a scan script.
- **SECURITY INVARIANTS:** No secrets, evidence or staging identifiers.
- **FAILURE CASES:** Drafts referencing private infrastructure.
- **TESTS REQUIRED:** Link check; gitleaks. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `link check passes`; `gitleaks detect` clean; `boundary_scan.py flags a planted staging hostname`; `no SPDX header present anywhere`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** The boundary scan flags a planted staging hostname.
- **ACCEPTANCE CRITERIA:** All files present; checklist complete.
- **EVIDENCE TO RETURN:** File list.
- **OUT-OF-SCOPE:** Publishing; choosing the license.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Nothing private; neutrality.

### W06-T005-STAGING-ENVIRONMENT-DOC
- **TASK_ID:** W06-T005-STAGING-ENVIRONMENT-DOC
- **OBJECTIVE:** **In plain words:** Write down which cloud accounts and regions the drills will use, who administers each, the spend caps and how to tear them down. Prototype only. **Precisely:** Write `docs/staging-environment.md`: the staging cloud/region/accounts chosen for the drills (P0-07D two independently administered accounts, P0-10G local HA, P0-10H two independent storage domains for the journal, G-051), who administers each, spend caps, and teardown steps — prototype choices only, no production decision (G-016 prototype part).
- **WHY IT EXISTS:** Drills need named, disposable infrastructure decided once.
- **ARCHITECTURE REQUIREMENTS:** 31 administrative independence; 35; 50 two storage domains.
- **P0 CLAIMS:** enables P0-07D, P0-10G, P0-10H.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `docs/staging-environment.md`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/docs/staging-environment.md`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `docs/staging-environment.md`. **Steps:** 1. Planned baseline (master, 2026-10-11; compare and confirm, do not invent another): **AWS** account A (master) as the staging cloud — primary region `ap-south-1`, recovery region `ap-south-2` — for P0-10G HA (RDS PostgreSQL Multi-AZ or two EC2 PostgreSQL nodes with fencing), P0-10F restore and the drills; **Cloudflare** for staging DNS/TLS of the two registrable domains; the two journal storage domains for P0-10H in two independent failure/admin domains (for example S3 in account A plus a second provider such as Cloudflare R2 or a second AWS account), each with its own credentials. 2. Options compared on cost (include your GitHub Student Developer Pack cloud credits), independence and teardown. 3. Domain B for P0-07D is a separate account administered by a second, independent administrator (G-052); without one, record P0-07D as not runnable. 4. Account hygiene: MFA on every root user, no root access keys, IAM Identity Center or scoped roles, AWS Budgets alarm at 50%/80% of the G1 cloud cap, everything tagged `mastercoder-staging`. 5. Spend caps and teardown steps per drill. 6. **Free tier first (master has free AWS and Cloudflare accounts):** run every drill that does not need separate cloud administrators locally on the Mac with Docker (two PostgreSQL containers with fencing for P0-10G HA, local restore for P0-10F where the claim allows); use the cloud only where a claim needs it: Cloudflare R2 free tier (10 GB) as the second journal storage domain for P0-10H next to S3 in AWS account A — a separate provider and failure domain, both administered by master (record that in the run-manifest; if the claim requires separate administrators, the cell is recorded accordingly); AWS only small instances (t4g/t3 micro or small), never RDS Multi-AZ or NAT gateways in staging, everything stopped or deleted after each drill; AWS Budgets alarms at USD 5 and 10 (or the G1 cap if lower); the Azure for Students credit pays for the nested-virtualization VMs of the W03 lab. 7. Two registrable domains are needed only for real-device WebAuthn on staging (normal web + separate approval authority); locally the `.test` hosts and mkcert suffice, so buy domains (Cloudflare Registrar, at cost) only when a staging RUN needs them.
- **SECURITY INVARIANTS:** No account IDs or keys in the doc (aliases only).
- **FAILURE CASES:** Both "independent" domains under one root account.
- **TESTS REQUIRED:** A3 checks independence statements. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `A3 confirms domain A and B have no shared root, CI credential or recovery authority`; `gitleaks detect` clean; `every drill names its infrastructure and owner`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** gitleaks clean.
- **ACCEPTANCE CRITERIA:** Every drill has named infrastructure and an owner.
- **EVIDENCE TO RETURN:** Doc SHA-256.
- **OUT-OF-SCOPE:** Creating accounts (human).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Real independence.

### W06-T006-SUPPLY-CHAIN-POLICY
- **TASK_ID:** W06-T006-SUPPLY-CHAIN-POLICY
- **OBJECTIVE:** **In plain words:** Write the dependency and secret rules for every lane: how to pick and review libraries (extra care for crypto and parsers), allowed licenses, how to request a new dependency and how secrets are handled. **Precisely:** Write `supply-chain/POLICY.md`: dependency selection and review rules (extra review for critical areas — CBOR, WebAuthn, Git parsing, crypto — G-055), allowed licenses, how a team requests a new dependency (DEPENDENCY_REQUEST, AGENTS.md §3a), pinning rules, secret-handling policy (G-082), and what the CI checks enforce.
- **WHY IT EXISTS:** 36 dependency policy; one written rule set for all six workstreams.
- **ARCHITECTURE REQUIREMENTS:** 36; 41.
- **P0 CLAIMS:** supports P0-06A/B validity.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `supply-chain/POLICY.md`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/supply-chain/POLICY.md`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `supply-chain/POLICY.md`. **Steps:** 1. Rules per dependency class. 2. Critical-area review checklist. 3. Secret policy (where secrets may live, rotation on exposure, incident steps).
- **SECURITY INVARIANTS:** Policy never allows secrets in CI for PRs.
- **FAILURE CASES:** Policy contradicting AGENTS.md.
- **TESTS REQUIRED:** A3 cross-check with AGENTS.md §3a and SECURITY rules. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `A3 cross-check: policy matches AGENTS.md §3a dependency rule`; `A3 cross-check: secret rules match every PRESTART_CHECKLIST`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** none.
- **ACCEPTANCE CRITERIA:** All sections present.
- **EVIDENCE TO RETURN:** Doc SHA-256.
- **OUT-OF-SCOPE:** CI implementation (T007, T008).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Consistency with program rules.

### W06-T007-SUPPLY-CHAIN-CI
- **TASK_ID:** W06-T007-SUPPLY-CHAIN-CI
- **OBJECTIVE:** **In plain words:** Add the supply-chain checks to CI for every workspace: leaked-secret scan, vulnerable or badly licensed dependencies, and a software bill of materials, plus automatic dependency update PRs. **Precisely:** Add `common-supply-chain.yml` and `deny.toml`: gitleaks on every push and PR, cargo-deny (licenses, advisories, bans) and cargo-audit per Cargo workspace, `pnpm audit` for ws05, CycloneDX SBOM per workspace, and Dependabot config — with workspaces discovered by glob so new ones are covered automatically.
- **WHY IT EXISTS:** 36/41 supply-chain validation on every change.
- **ARCHITECTURE REQUIREMENTS:** 36; 41.
- **P0 CLAIMS:** supports all (hygiene).
- **INPUT CONTRACTS:** policy (T006).
- **OUTPUT CONTRACTS:** `.github/workflows/common-supply-chain.yml`, `/deny.toml`, `/.github/dependabot.yml`.
- **EXACT WRITE SCOPE:** `/.github/workflows/common-supply-chain.yml`, `/deny.toml`, `/.github/dependabot.yml`, `ws06-platform-oss-qualification/supply-chain/ci/**`
- **READ-ONLY DEPENDENCIES:** all workspaces (scanned, not modified).
- **PRECONDITIONS:** W06-T000 INTEGRATED; W06-T006 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `/.github/workflows/common-supply-chain.yml`; `/deny.toml`; `/.github/dependabot.yml`; `supply-chain/ci/{discover_workspaces.py,README.md}`. **Steps:** 1. Matrix over exactly the expected targets — `ws01-core-security/Cargo.toml`, `ws02-git-analysis-review/Cargo.toml`, `ws03-runner-sandbox-acquisition/Cargo.toml`, `ws04-agent-model-graph/Cargo.toml`, `ws06-platform-oss-qualification/Cargo.toml` and `ws05-web-approval-ux/package.json` — discovered by `ws0*-*/Cargo.toml` + `ws05-web-approval-ux/package.json`; the job fails if any expected target is missing or the discovered set is empty (audit F-19). 2. License allow-list from the policy. 3. SBOMs uploaded as artifacts.
- **SECURITY INVARIANTS:** No `pull_request_target`; no secrets in PR CI.
- **FAILURE CASES:** Scanner bypass via file extension or new workspace not scanned.
- **TESTS REQUIRED:** A scratch commit with a fake key is caught; a deliberately vulnerable pinned crate is flagged. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scratch commit with a fake key → gitleaks red`; `scratch vulnerable pinned crate → cargo-audit red`; `scratch disallowed license → cargo-deny red`; `new workspace directory discovered automatically`; `discovery finds exactly the 5 Cargo workspaces + ws05-web-approval-ux/package.json; a missing expected target or an empty match turns the job red`; `SBOM artifact produced per workspace`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Those two scratch cases; a disallowed license flagged.
- **ACCEPTANCE CRITERIA:** All checks run on every PR.
- **EVIDENCE TO RETURN:** CI links; SBOM sample.
- **OUT-OF-SCOPE:** Release signing (POST_FREEZE).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No bypass paths.

### W06-T008-INDEPENDENCE-AND-REPRO-CHECKS
- **TASK_ID:** W06-T008-INDEPENDENCE-AND-REPRO-CHECKS
- **OBJECTIVE:** **In plain words:** Add CI checks that prove the Rust and TypeScript codecs stay independent (neither copies the other) and that the approval website builds to the same fingerprint every time. **Precisely:** Add `common-independence.yml`: an import-ban check (G-056) — `ws05-web-approval-ux/packages/wire-ts` must not reference `ws01-core-security` codec sources, `ws01-core-security/crates/mc-wire` must not reference `wire-ts`, and no implementation may import `check_spec.py` — plus a reproducible-build check for `ws05-web-approval-ux/apps/aa-origin` (two clean builds must give the same bundle digest, G-048).
- **WHY IT EXISTS:** P0-06A/B are only meaningful if the two codecs are independent; the AA bundle digest must be stable.
- **ARCHITECTURE REQUIREMENTS:** TEST_VECTORS README independence; 36; TrustedDisplayReceiptV1.renderer_bundle_digest.
- **P0 CLAIMS:** supports P0-06A, P0-06B, P0-09A.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `.github/workflows/common-independence.yml`; `supply-chain/independence/{check.py,allowlist.txt}`.
- **EXACT WRITE SCOPE:** `/.github/workflows/common-independence.yml`, `ws06-platform-oss-qualification/supply-chain/independence/**`
- **READ-ONLY DEPENDENCIES:** ws01, ws05 sources (scanned).
- **PRECONDITIONS:** W06-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `/.github/workflows/common-independence.yml`; `supply-chain/independence/{check.py,allowlist.txt,README.md}`; `supply-chain/independence/tests/test_check.py`. **Steps:** 1. Path/AST-based search with an allowlist file, over the exact directories `ws05-web-approval-ux/packages/wire-ts/` and `ws01-core-security/crates/mc-wire/`; the check fails if either directory is missing or yields zero scanned files (audit F-19). 2. Reproducible build job builds aa-origin twice in clean containers and compares digests.
- **SECURITY INVARIANTS:** The check cannot be skipped by renaming files (matches content, not only names).
- **FAILURE CASES:** Copied code under a new name not detected.
- **TESTS REQUIRED:** Scratch commit importing the forbidden module fails; scratch import of check_spec fails. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_check.py::wire_ts_referencing_ws01_codec_flagged`; `test_check.py::mc_wire_referencing_wire_ts_flagged`; `test_check.py::check_spec_import_flagged`; `test_check.py::renamed_copy_detected_by_content`; `test_check.py::missing_or_empty_target_directory_fails`; `test_check.py::planted_forbidden_import_in_real_ws05_wire_ts_path_flagged`; `repro job: two clean aa-origin builds give the same digest; a nondeterministic change turns it red`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Those scratch cases; a non-reproducible change to aa-origin fails the repro job.
- **ACCEPTANCE CRITERIA:** Checks active on every PR.
- **EVIDENCE TO RETURN:** CI links.
- **OUT-OF-SCOPE:** Parity harness (T015).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Evasion resistance.

### W06-T009-EVIDENCE-CLI
- **TASK_ID:** W06-T009-EVIDENCE-CLI
- **OBJECTIVE:** **In plain words:** Build the evidence command-line tool every claim run uses: record exact versions (refusing if any is missing), bundle the evidence, verify a bundle, and move the status forward only in the allowed order. **Precisely:** Build `mc-evidence`, the CLI every RUN task uses: `seal` (fills and validates a run-manifest from the actual environment; refuses if any locally controlled version or digest is missing), `verify` (checks a bundle against its evidence-manifest), `bundle` (builds the bundle and `evidence-manifest.sha256`), `new-run` (allocates a fresh RUN_ID directory under `claims/<claim>/cells/<cell>/runs/`; refuses to reuse one), `outcome` (writes `outcome.json` once; refuses a second write), `lifecycle` (appends validated transitions to `lifecycle.jsonl`), `set-current` (updates the cell's `current.json` with a supersession record).
- **WHY IT EXISTS:** RUN tasks must seal exact versions and produce verifiable bundles the same way.
- **ARCHITECTURE REQUIREMENTS:** 57 pinning rule; 19; 40.
- **P0 CLAIMS:** enables all RUN tasks.
- **INPUT CONTRACTS:** I-13 schemas (T003).
- **OUTPUT CONTRACTS:** binary `mc-evidence`; `crates/mc-evidence/README.md` (CLI usage). **Exact public API (names and signatures are binding; do not rename):** binary `mc-evidence new-run --claim <id> --cell <cell>` (prints RUN_ID), `seal --claim <id> --cell <cell> --run <RUN_ID>`, `outcome --run-dir <dir> <PASS|FAIL|INCONCLUSIVE>`, `lifecycle --run-dir <dir> <state>`, `set-current --claim <id> --cell <cell> --run <RUN_ID> --reason <text>`, `bundle --dir <evidence dir>`, `verify --bundle <dir>`; exit code non-zero on any refusal.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/mc-evidence/src/{seal.rs,verify.rs,bundle.rs,status.rs,runs.rs,main.rs}`, `ws06-platform-oss-qualification/crates/mc-evidence/tests/{seal.rs,verify.rs,bundle.rs,status.rs,runs.rs}`, `ws06-platform-oss-qualification/crates/mc-evidence/README.md`
- **READ-ONLY DEPENDENCIES:** schemas.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W06-T003 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/mc-evidence/src/{seal.rs,verify.rs,bundle.rs,status.rs,runs.rs,main.rs}`; `crates/mc-evidence/tests/{seal.rs,verify.rs,bundle.rs,status.rs,runs.rs}`; `crates/mc-evidence/README.md (CLI usage)`. **Steps:** 1. `seal` collects versions by running the documented commands (rustc -V, sha256 of lockfiles, uname, image digests) and validates. 2. Lifecycle transitions only NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED (then EXPIRED|INVALIDATED); the outcome is independent of lifecycle and immutable once written (audit F-15). 3. Bundle digest stable. 4. No command ever modifies a file inside an earlier run directory (audit F-16).
- **SECURITY INVARIANTS:** Never reads or stores secrets; refuses files matching secret patterns.
- **FAILURE CASES:** Sealing with a missing version.
- **TESTS REQUIRED:** Seal/verify/bundle on example data. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `seal::fills_all_pinning_fields`; `seal::missing_cargo_lock_digest_refused`; `seal::secret_looking_file_refused`; `bundle::manifest_digest_stable`; `verify::tampered_bundle_fails`; `status::illegal_transition_refused`; `status::outcome_independent_of_lifecycle (SUBMITTED + PASS accepted)`; `status::second_outcome_write_refused`; `runs::existing_run_id_refused`; `runs::earlier_run_dir_never_modified`; `runs::linux_and_windows_runs_coexist`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Missing Cargo.lock digest → refuse; illegal lifecycle transition → refuse; second outcome write → refuse; reusing an existing RUN_ID → refuse; tampered bundle → verify fails; secret-looking file → refuse.
- **ACCEPTANCE CRITERIA:** All commands documented and tested.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Store upload (T010).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Refusal on missing versions.

### W06-T010-EVIDENCE-STORE
- **TASK_ID:** W06-T010-EVIDENCE-STORE
- **OBJECTIVE:** **In plain words:** Set up the private, write-once evidence store (it cannot be overwritten or deleted early) and add upload/download to the evidence tool. **Precisely:** Set up the private immutable evidence store (G-009, G-010): an S3-compatible bucket with object lock / WORM, separate from the code repository, uploads keyed by content digest, access limited to owners and reviewers; add `mc-evidence store put|get` and `docs/evidence-store.md`.
- **WHY IT EXISTS:** Raw evidence must be immutable and private.
- **ARCHITECTURE REQUIREMENTS:** 19 Prototype Evidence; 24 privacy; 40.
- **P0 CLAIMS:** enables all RUN tasks.
- **INPUT CONTRACTS:** bundles from T009.
- **OUTPUT CONTRACTS:** `store` module; `docs/evidence-store.md`; `evidence-store/policy.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/mc-evidence/src/store.rs`, `ws06-platform-oss-qualification/crates/mc-evidence/src/main.rs` (only to add the `store` subcommand; W06-T009 is finished first), `ws06-platform-oss-qualification/crates/mc-evidence/tests/store.rs`, `ws06-platform-oss-qualification/docs/evidence-store.md`, `ws06-platform-oss-qualification/evidence-store/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W06-T009 INTEGRATED; PRESTART: storage account created (human).
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/mc-evidence/src/store.rs`; `crates/mc-evidence/src/main.rs (store subcommand only)`; `crates/mc-evidence/tests/store.rs`; `docs/evidence-store.md`; `evidence-store/{policy.json,README.md}`. **Steps:** 1. Bucket policy with object lock; credentials only in `.env.local`. 2. put by digest; get verifies digest. 3. Local MinIO mode for tests.
- **SECURITY INVARIANTS:** Store never public; no overwrite or delete within retention.
- **FAILURE CASES:** Overwrite allowed.
- **TESTS REQUIRED:** put/get on local MinIO with object lock. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `store::put_get_by_digest_on_local_minio`; `store::overwrite_denied`; `store::delete_within_retention_denied`; `store::wrong_digest_on_get_fails`; `store::bucket_not_public`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Overwrite attempt denied; delete within retention denied; wrong digest on get fails.
- **ACCEPTANCE CRITERIA:** Immutable private store documented and tested.
- **EVIDENCE TO RETURN:** Test report; policy SHA-256.
- **OUT-OF-SCOPE:** Production storage.
- **BLOCKERS:** Storage account (human).
- **A3 REVIEW FOCUS:** Immutability.

### W06-T011-CLAIM-LEDGER
- **TASK_ID:** W06-T011-CLAIM-LEDGER
- **OBJECTIVE:** **In plain words:** Build the generated claim ledger: one page listing every claim's progress and result, kept separate, with expired evidence flagged. It is regenerated by a tool, never hand-edited. **Precisely:** Build `mc-ledger-gen`, the generated program ledger `/evidence-ledger/LEDGER.md` and `LEDGER.json`: it reads every `ws0*/claims/*/cells/*/current.json` and the immutable run each one names (plus run history for the audit trail), shows per claim and per cell the lifecycle and the measured outcome separately, computes expiry from `max_age_days` and `invalidate_on` triggers (registry change, image digest change, provider version change) and flags EXPIRED (G-061); hand edits are overwritten and CI fails on diff.
- **WHY IT EXISTS:** One computed view of all 55 claims (+ cells).
- **ARCHITECTURE REQUIREMENTS:** 42; 57 evidence_expiry; 19.
- **P0 CLAIMS:** aggregates all.
- **INPUT CONTRACTS:** status files (I-13).
- **OUTPUT CONTRACTS:** `/evidence-ledger/**`; binary `mc-ledger-gen`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/mc-ledger-gen/src/**`, `ws06-platform-oss-qualification/crates/mc-ledger-gen/tests/**`, `/evidence-ledger/**`
- **READ-ONLY DEPENDENCIES:** all status files; claims JSON.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W06-T003 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/mc-ledger-gen/src/{lib.rs,ledger.rs,expiry.rs,main.rs}`; `crates/mc-ledger-gen/tests/{ledger.rs,expiry.rs}`; `/evidence-ledger/{LEDGER.md,LEDGER.json}`. **Steps:** 1. Reader + validator. 2. Expiry engine. 3. Markdown + JSON output; CI check.
- **SECURITY INVARIANTS:** A cell is shown ENABLED only when its current run is ACCEPTED (two lens reviews) + PASS + not expired/invalidated; a measured PASS on a SUBMITTED run is displayed as an outcome, never as enablement.
- **FAILURE CASES:** A SUBMITTED + PASS run shown as enabled; one cell's run overwriting or hiding another cell's.
- **TESTS REQUIRED:** Ledger computation tests on synthetic status sets. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `ledger::all_claims_and_cells_not_run_today`; `ledger::submitted_with_pass_never_enabled`; `ledger::accepted_with_one_lens_rejected`; `ledger::lifecycle_and_outcome_separate`; `expiry::max_age_exceeded_flagged`; `expiry::registry_change_invalidates`; `ledger::hand_edit_detected_by_ci_diff`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** SUBMITTED+PASS never enables; one-lens ACCEPTED rejected; expired evidence flagged; Linux PASS and Windows FAIL of one claim both shown unchanged; a superseded run is not used for enablement.
- **ACCEPTANCE CRITERIA:** All 55 claims (with cells) appear NOT_RUN today.
- **EVIDENCE TO RETURN:** Ledger output.
- **OUT-OF-SCOPE:** Freeze dossier (T018).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Enablement logic.

### W06-T012-CANARY-SCANNER
- **TASK_ID:** W06-T012-CANARY-SCANNER
- **OBJECTIVE:** **In plain words:** Build the canary scanner: create unique fake secrets per experiment and search every file, log, network capture and bundle for them, including encoded and compressed copies. **Precisely:** Build `canary-scanner` (G-069): generate distinct high-entropy canary secrets/source strings per experiment (recorded in the run-manifest) and scan files, environment dumps, pcaps (via tshark export), logs, telemetry exports, compressed data and evidence bundles for them — including base64, URL and hex encodings — emitting `canary-scan.json`.
- **WHY IT EXISTS:** P0-01B, P0-05B, P0-05F, P0-10D and others require canary scans.
- **ARCHITECTURE REQUIREMENTS:** 24 logging; 34 privacy; INV-16.
- **P0 CLAIMS:** supports P0-01B, P0-05B, P0-05F, P0-10D, P0-02A/C.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** binary `canary-scanner generate|scan`; `canary-scan.json` format.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/canary-scanner/src/**`, `ws06-platform-oss-qualification/crates/canary-scanner/tests/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W06-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/canary-scanner/src/{generate.rs,scan.rs,formats.rs,main.rs}`; `crates/canary-scanner/tests/{detect.rs,clean.rs}`; `crates/canary-scanner/tests/corpus/{planted,clean}/**`. **Steps:** 1. Canary format with a unique per-run prefix. 2. Scanners per format. 3. Report with location of each hit.
- **SECURITY INVARIANTS:** Canaries are never real credentials.
- **FAILURE CASES:** Missing canaries in compressed data.
- **TESTS REQUIRED:** Detection on a planted corpus in every format; false-positive run on a clean corpus. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `detect::plain_text`; `detect::binary`; `detect::pcap_via_tshark`; `detect::gzip`; `detect::base64`; `detect::url_encoded`; `detect::hex`; `clean::zero_false_positives`; `generate::unique_per_run_prefix`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** base64, URL-encoded, hex and gzip-wrapped canaries detected.
- **ACCEPTANCE CRITERIA:** 100% detection on the planted corpus; zero false positives on the clean corpus.
- **EVIDENCE TO RETURN:** Test report. A1 sends INTERFACE_NOTICE to W01, W03, W04.
- **OUT-OF-SCOPE:** Telemetry config (T013).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Encoding coverage.

### W06-T013-OBSERVABILITY-COLLECTOR
- **TASK_ID:** W06-T013-OBSERVABILITY-COLLECTOR
- **OBJECTIVE:** **In plain words:** Configure the local telemetry collector so logs and traces drop message bodies and unknown fields by default. Every drill sends its telemetry here. **Precisely:** Provide `observability/`: a local OpenTelemetry collector configuration with typed-log attribute allowlists and body-dropping defaults, a compose service, and a sink inventory template, used by P0-10D and as the common sink for drills.
- **WHY IT EXISTS:** 34: telemetry must not carry raw content by default.
- **ARCHITECTURE REQUIREMENTS:** 34 signals and privacy; 24; INV-16.
- **P0 CLAIMS:** supports P0-10D.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `observability/{collector.yaml,compose.yaml,README.md,SINKS_TEMPLATE.md}`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/observability/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W06-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `observability/{collector.yaml,compose.yaml,README.md,SINKS_TEMPLATE.md}`; `observability/tests/test_collector.sh`. **Steps:** 1. Collector pinned by digest. 2. Processors drop bodies and non-allowlisted attributes. 3. File exporter for scanning.
- **SECURITY INVARIANTS:** Default drops bodies.
- **FAILURE CASES:** Body passed through by default.
- **TESTS REQUIRED:** Send a span/log with a planted canary in body and in a non-allowlisted attribute; exported file contains neither. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_collector.sh::body_canary_dropped`; `test_collector.sh::non_allowlisted_attribute_dropped`; `test_collector.sh::allowlisted_attribute_kept`; `collector image pinned by digest`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** That test.
- **ACCEPTANCE CRITERIA:** Allowlist enforced.
- **EVIDENCE TO RETURN:** Test output.
- **OUT-OF-SCOPE:** Production telemetry.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Default-deny attributes.

### W06-T014-ARTIFACT-STORE
- **TASK_ID:** W06-T014-ARTIFACT-STORE
- **OBJECTIVE:** **In plain words:** Run the S3-style artifact store (MinIO) with write-once locking, a separate encryption key per tenant and expiring download links. the W01 lane's patch storage uses it. **Precisely:** Stand up the S3-compatible artifact store for prototypes (MinIO with object lock and versioning, A-011, G-034) with per-tenant encryption keys (local KES or a staging KMS) enabling crypto-erasure, scoped download grants, and a compose file other workstreams can use (W01 custody backend).
- **WHY IT EXISTS:** 24 artifacts and lifecycle; W01 custody and P0-10C need a real store.
- **ARCHITECTURE REQUIREMENTS:** 24; 14; ADR-013/025.
- **P0 CLAIMS:** supports P0-10C, P0-09F.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `artifact-store/{compose.yaml,setup.sh,README.md,policies/**}`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/artifact-store/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W06-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `artifact-store/{compose.yaml,setup.sh,README.md}`; `artifact-store/policies/**`; `artifact-store/tests/test_store.sh`. **Steps:** 1. MinIO + KES pinned by digest. 2. Per-tenant bucket/key setup script. 3. Grant (presigned) issuance with expiry.
- **SECURITY INVARIANTS:** INV-16; keys never in git.
- **FAILURE CASES:** Lifecycle rule deleting a locked object.
- **TESTS REQUIRED:** Setup from clean; put/get per tenant. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_store.sh::setup_from_clean`; `test_store.sh::per_tenant_put_get`; `test_store.sh::cross_tenant_read_denied`; `test_store.sh::locked_object_delete_denied`; `test_store.sh::expired_grant_denied`; `test_store.sh::crypto_erasure_by_key_removal`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Cross-tenant read denied; delete of a locked object denied; expired grant denied.
- **ACCEPTANCE CRITERIA:** Usable store with tenant isolation.
- **EVIDENCE TO RETURN:** Test output. A1 sends INTERFACE_NOTICE to W01.
- **OUT-OF-SCOPE:** P0-10C harness (T024).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Lock vs lifecycle.

### W06-T015-PARITY-RUNNER
- **TASK_ID:** W06-T015-PARITY-RUNNER
- **OBJECTIVE:** **In plain words:** Build the codec comparison runner: run the W01 lane's Rust codec and the W05 lane's TypeScript codec on every test vector, under different locales and timezones, and compare both against the official expected bytes, never against each other. **Precisely:** Build `parity-harness`: it runs `mc-wire-vectors` (W01) and `wire-ts-vectors` (W05) as black boxes on every fixture under a matrix of host locales, timezones and map-insertion orders, compares complete bytes, digests and signature inputs against the **committed vectors** (the vectors are the oracle, never either implementation), records rejection categories, and reports an absent implementation as INCONCLUSIVE.
- **WHY IT EXISTS:** P0-06A/B require independent implementations compared by a third party (A-012).
- **ARCHITECTURE REQUIREMENTS:** ENCODING.md; TEST_VECTORS README; 47; ADR-024; INV-18.
- **P0 CLAIMS:** supports P0-06A, P0-06B.
- **INPUT CONTRACTS:** I-02 and I-10 CLI outputs; vectors.
- **OUTPUT CONTRACTS:** crate `parity-harness` (runner, compare, matrix). **Exact public API (names and signatures are binding; do not rename):** `pub fn run_matrix(impls: &[ImplCli], vectors: &Path, envs: &[EnvVariant]) -> MatrixReport`; `pub enum ImplStatus { Present, Absent }`; `pub enum Verdict { Match, Mismatch, Inconclusive }` (never derived by comparing implementations to each other).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/parity-harness/src/**`, `ws06-platform-oss-qualification/crates/parity-harness/tests/**`
- **READ-ONLY DEPENDENCIES:** CLI binaries (not sources).
- **PRECONDITIONS:** W06-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/parity-harness/src/{runner.rs,compare.rs,matrix.rs,lib.rs}`; `crates/parity-harness/tests/{selftest.rs,absent.rs}`; `crates/parity-harness/tests/wrong_mock/**`. **Steps:** 1. Env matrix runner (LANG, TZ variants). 2. Comparator vs vectors. 3. Missing binary → INCONCLUSIVE.
- **SECURITY INVARIANTS:** Neither implementation's output is used as expected bytes.
- **FAILURE CASES:** Comparing implementations only to each other.
- **TESTS REQUIRED:** Self-test with a deliberately wrong mock implementation (must fail). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `selftest::wrong_mock_implementation_fails`; `selftest::correct_mock_passes`; `absent::missing_implementation_inconclusive_never_pass`; `selftest::locale_and_timezone_matrix_applied`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Wrong mock fails; missing implementation → INCONCLUSIVE, never PASS.
- **ACCEPTANCE CRITERIA:** Runner works with whichever implementation exists.
- **EVIDENCE TO RETURN:** Self-test output.
- **OUT-OF-SCOPE:** Claim harness specifics (T016, T017).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Vector-as-oracle.

### W06-T016-P0-06A-HARNESS
- **TASK_ID:** W06-T016-P0-06A-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-06A: "Canonical positive encoding parity". It runs one normal case and then 3 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-06A harness on the parity runner: canonical positive encoding parity — every ACCEPT vector through both implementations under varied locale, timezone and map insertion order, emitting Rust and TypeScript result files and a byte comparison.
- **WHY IT EXISTS:** Canonical positive encoding parity.
- **ARCHITECTURE REQUIREMENTS:** ENCODING.md; ADR-024; INV-18.
- **P0 CLAIMS:** P0-06A (primary)
- **INPUT CONTRACTS:** parity runner.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `rust-vector-results.json`, `typescript-vector-results.json`, `byte-comparison.json`, `P0-06A/run-manifest.json`, `P0-06A/fixture-digests.json`, `P0-06A/evidence-manifest.sha256`, `P0-06A/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-06A/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** parity runner, CLIs.
- **PRECONDITIONS:** W06-T015 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-06A/harness/src/main.rs`; `claims/P0-06A/harness/src/{scenarios.rs,report.rs}`; `claims/P0-06A/fixtures/** (synthetic only)`; `claims/P0-06A/fixture-digests.json`; `claims/P0-06A/run-manifest.draft.json`; `claims/P0-06A/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Positive suite. 2. Env variations. 3. Emit results. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-06A):** (1) Provision exactly the sealed cell and empty evidence directory named P0-06A. (2) Encode/decode every positive fixture independently and compare complete bytes, digest and signature input. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** Vectors are the oracle.
- **FAILURE CASES:** One implementation used as truth.
- **TESTS REQUIRED:** DRY_RUN (INCONCLUSIVE where an implementation is absent). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_vary_host_locale`; `scenarios::injection_b_timezone`; `scenarios::injection_c_map_insertion_order_without_changing_semantic_values`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-06A injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Vary host locale; (b) timezone; (c) map insertion order without changing semantic values.
- **ACCEPTANCE CRITERIA:** Byte-identical results to vectors in every environment for present implementations. **Claim expected observations:** Canonical bytes identical across implementations and environments; signature domain and ordered fields match normative vectors. **Threshold:** 100% positive fixtures byte/digest/signature-input identical; zero unexplained differences. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-1).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Environment variation coverage.

### W06-T017-P0-06B-HARNESS
- **TASK_ID:** W06-T017-P0-06B-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-06B: "Canonical rejection parity". It runs one normal case and then 6 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-06B harness: canonical rejection parity — all REJECT vectors plus extra malformed fixtures (duplicate keys, noncanonical integer, wrong domain/type version, unknown critical/non-critical field inside a closed signed record) through both implementations, recording the rejection category per language and decoder traces.
- **WHY IT EXISTS:** Canonical rejection parity.
- **ARCHITECTURE REQUIREMENTS:** ENCODING.md; ADR-024; INV-18.
- **P0 CLAIMS:** P0-06B (primary)
- **INPUT CONTRACTS:** parity runner.
- **OUTPUT CONTRACTS:** harness; `malformed-fixtures.bin`; `run-manifest.draft.json`. **Evidence files the harness writes:** `rejection-matrix.json`, `malformed-fixtures.bin`, `decoder-traces.json`, `P0-06B/run-manifest.json`, `P0-06B/fixture-digests.json`, `P0-06B/evidence-manifest.sha256`, `P0-06B/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-06B/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** parity runner, CLIs.
- **PRECONDITIONS:** W06-T015 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-06B/harness/src/main.rs`; `claims/P0-06B/harness/src/{scenarios.rs,report.rs}`; `claims/P0-06B/fixtures/** (synthetic only)`; `claims/P0-06B/fixture-digests.json`; `claims/P0-06B/run-manifest.draft.json`; `claims/P0-06B/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Generate extra malformed fixtures deterministically. 2. Run both. 3. Rejection matrix. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-06B):** (1) Provision exactly the sealed cell and empty evidence directory named P0-06B. (2) Decode each negative fixture and record exact rejection category in both languages. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** No acceptance of malformed signed input in either language.
- **FAILURE CASES:** Normalization counted as rejection.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_duplicate_keys`; `scenarios::injection_b_noncanonical_integer`; `scenarios::injection_c_wrong_domain_version`; `scenarios::injection_d_wrong_type_version`; `scenarios::injection_e_unknown_critical_field_inside_closed_signed_record`; `scenarios::injection_f_unknown_noncritical_field_inside_closed_signed_record`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-06B injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Duplicate keys; (b) noncanonical integer; (c) wrong domain version; (d) wrong type version; (e) unknown critical field inside closed signed record; (f) unknown noncritical field inside closed signed record.
- **ACCEPTANCE CRITERIA:** Every malformed input rejected by every present implementation. **Claim expected observations:** Both reject every forbidden encoding; no normalizing malformed signed input into acceptable equivalent. **Threshold:** 100% normative negative fixtures rejected by both implementations. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-1).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Fixture determinism.

### W06-T018-FREEZE-DOSSIER
- **TASK_ID:** W06-T018-FREEZE-DOSSIER
- **OBJECTIVE:** **In plain words:** Build the freeze report generator: it collects every claim's status, evidence, disabled features, open change requests and the audit result, and computes whether the architecture could be frozen. It never makes the decision itself. **Precisely:** Build the freeze dossier generator: claim ledger snapshot, per-claim evidence pointers, capability-cell table (QUALIFIED_FOR_FREEZE / DISABLED + reason), scope records for inapplicable claims (G-007), CCR log, external audit result, residual risk list (44 deltas), BLOCKS_FREEZE check, and a regenerated CURRENT_IMPLEMENTATION_INVENTORY. It never decides G4; it feeds master's decision and the two independent dossier reviews.
- **WHY IT EXISTS:** 42/00E: freeze needs a computed, reviewable basis.
- **ARCHITECTURE REQUIREMENTS:** 42 family computation and inapplicability records; 00E; 44; DEFINITION_OF_DONE §5–6.
- **P0 CLAIMS:** none (aggregates all 55).
- **INPUT CONTRACTS:** ledger JSON; CCR files; audit tracker.
- **OUTPUT CONTRACTS:** `freeze-dossier/` (generated) + `SCOPE_RECORD_TEMPLATE.md`; binary `freeze-dossier`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/crates/freeze-dossier/src/**`, `ws06-platform-oss-qualification/crates/freeze-dossier/tests/**`, `ws06-platform-oss-qualification/freeze-dossier/**`
- **READ-ONLY DEPENDENCIES:** ledger, `ws05-web-approval-ux/docs/external-audit/**`, all `coordination/CONTRACT_CHANGE_REQUESTS/**`.
- **PRECONDITIONS:** W06-T011 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `crates/freeze-dossier/src/{gen.rs,main.rs}`; `crates/freeze-dossier/tests/gen.rs`; `freeze-dossier/{DOSSIER.md,SCOPE_RECORD_TEMPLATE.md}`. **Steps:** 1. Generator. 2. Scope record template. 3. BLOCKS_FREEZE check against the claim matrix (≥1-cell rules for P0-02/P0-03A families).
- **SECURITY INVARIANTS:** No self-exemption of a security boundary (42).
- **FAILURE CASES:** Counting SUBMITTED as PASS.
- **TESTS REQUIRED:** Generator tests with synthetic status sets. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `gen::current_state_freeze_not_supported`; `gen::blocks_freeze_inconclusive_not_supported`; `gen::one_cell_family_with_zero_passing_cells_not_supported`; `gen::submitted_never_counted_as_pass`; `gen::regenerates_current_implementation_inventory`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** One BLOCKS_FREEZE claim INCONCLUSIVE → FREEZE_NOT_SUPPORTED; ≥1-cell family with zero passing cells → FREEZE_NOT_SUPPORTED.
- **ACCEPTANCE CRITERIA:** Current dossier shows 55 NOT_RUN and FREEZE_NOT_SUPPORTED.
- **EVIDENCE TO RETURN:** Dossier output.
- **OUT-OF-SCOPE:** The G4 decision.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Enablement computation.

### W06-T019-ADMIN-DOMAIN-IAC
- **TASK_ID:** W06-T019-ADMIN-DOMAIN-IAC
- **OBJECTIVE:** **In plain words:** Write the cloud setup files for two admin domains (one shared admin vs two truly separate admins in separate accounts) and a tool that lists who can administer what, to prove the separation. **Precisely:** Write infrastructure-as-code for the two P0-07D profiles: profile A (two runners under one root admin) and profile B (separately administered runners, signing roots, enrollment and recovery in a separate cloud account administered by a second, independent administrator, G-052), with no shared root, CI credential or recovery authority; plus the administration-access matrix capture script (A-010, G-033).
- **WHY IT EXISTS:** ADR-006: C3 requires actually separate administrative domains.
- **ARCHITECTURE REQUIREMENTS:** 13 deployment profiles; 31 administrative independence; 32; INV-08.
- **P0 CLAIMS:** supports P0-07D.
- **INPUT CONTRACTS:** CapabilityProfileV1, TrustMemberV1 (administrative_domain_id).
- **OUTPUT CONTRACTS:** `drills/admin-domains/{iac/**,access_matrix.py,README.md}`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/admin-domains/**`
- **READ-ONLY DEPENDENCIES:** staging environment doc.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W06-T005 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/admin-domains/iac/{profile-a,profile-b}/**`; `drills/admin-domains/access_matrix.py`; `drills/admin-domains/tests/test_matrix.py`; `drills/admin-domains/README.md`. **Steps:** 1. IaC per profile with variables for account aliases. 2. Access-matrix script lists every identity with rights in each domain. 3. Shared-credential search (CI secrets, root users, recovery contacts).
- **SECURITY INVARIANTS:** INV-08; no credential in IaC.
- **FAILURE CASES:** Hidden shared CI secret.
- **TESTS REQUIRED:** `plan`/validate of the IaC; matrix script on fake inventories. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `IaC validate/plan passes for both profiles`; `test_matrix.py::planted_shared_ci_secret_flagged`; `test_matrix.py::planted_shared_root_flagged`; `test_matrix.py::separate_domains_clean`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Matrix script flags a planted shared credential.
- **ACCEPTANCE CRITERIA:** Both profiles defined; matrix tool proven.
- **EVIDENCE TO RETURN:** IaC SHA-256.
- **OUT-OF-SCOPE:** Harness (T026).
- **BLOCKERS:** Second administrator + account (human).
- **A3 REVIEW FOCUS:** Shared-credential search completeness.

### W06-T020-LOAD-GENERATOR-AND-CHECKERS
- **TASK_ID:** W06-T020-LOAD-GENERATOR-AND-CHECKERS
- **OBJECTIVE:** **In plain words:** Build the load tester and independent checkers: generate the declared load (tenants, jobs, events per second) and check from raw records that budgets and locks were never violated. **Precisely:** Build the P0-10E load tooling: a load generator for the declared envelope (10 tenants, 100 active jobs, 1,000 queued, 100 events/s for 10 min, DB pool max 40, and 2× admission), and independent conservation and fence checkers that recompute invariants from raw records without using the system under test's own code.
- **WHY IT EXISTS:** 33: limits must stay within demonstrated envelopes; checkers must be independent.
- **ARCHITECTURE REQUIREMENTS:** 33 in full; 17 objectives; INV-03, INV-13.
- **P0 CLAIMS:** supports P0-10E.
- **INPUT CONTRACTS:** I-03, I-04 doubles; W02 reducer double; W04 broker double.
- **OUTPUT CONTRACTS:** `drills/load/{generator/**,checkers/**,README.md}` and `mc-drill-kit` orchestration helpers.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/load/**`, `ws06-platform-oss-qualification/crates/mc-drill-kit/src/{orchestrate.rs,report.rs}`, `ws06-platform-oss-qualification/crates/mc-drill-kit/tests/{orchestrate.rs,report.rs}`
- **READ-ONLY DEPENDENCIES:** W01 lease/effects doubles, W02 reducer double, W04 broker double, testdb.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W06-T002 INTEGRATED; W01-T015 INTEGRATED; W01-T018 INTEGRATED; W02-T003 INTEGRATED; W04-T006 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/load/generator/**`; `drills/load/checkers/**`; `drills/load/config/{envelope.toml,double.toml}`; `drills/load/README.md`; `crates/mc-drill-kit/src/{orchestrate.rs,report.rs}`; `crates/mc-drill-kit/tests/{orchestrate.rs,report.rs}`. **Steps:** 1. Generator with config file (no hard-coded rates). 2. Checkers read DB/journal exports only. 3. Reduced-scale mode for CI.
- **SECURITY INVARIANTS:** Checkers independent of the system under test.
- **FAILURE CASES:** Checker reusing system code.
- **TESTS REQUIRED:** Reduced-scale run; checkers detect a planted violation. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `reduced_scale_run_completes`; `checkers::planted_double_settlement_detected`; `checkers::planted_lost_fence_detected`; `checkers::independent_of_system_code (static: no ws01–ws04 crate imported by checkers)`; `rates read from config only`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Planted double-settlement and lost-fence records detected.
- **ACCEPTANCE CRITERIA:** Tooling runs at reduced scale; checkers proven.
- **EVIDENCE TO RETURN:** Reduced-scale report.
- **OUT-OF-SCOPE:** Harness (T027).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Checker independence.

### W06-T021-UPGRADE-ORCHESTRATOR
- **TASK_ID:** W06-T021-UPGRADE-ORCHESTRATOR
- **OBJECTIVE:** **In plain words:** Build the upgrade drill tooling: build an old and a new version from exact commits and run the upgrade steps in order while an old job is still running. **Precisely:** Build the P0-10F tooling: build two versions of the server and runner prototypes from tagged SHAs (current and previous minor), an orchestrator for the expand → writers → contract migration plan with a live old-version attempt, and `docs/compatibility-tracking.md` covering N↔N-1 for all seven boundaries in doc 47 (G-058).
- **WHY IT EXISTS:** 47 upgrade order; INV-18.
- **ARCHITECTURE REQUIREMENTS:** 47; 07 migrations; ADR-014/024.
- **P0 CLAIMS:** supports P0-10F.
- **INPUT CONTRACTS:** W01 negotiation; W03 fake runner.
- **OUTPUT CONTRACTS:** `drills/upgrade/{build_versions.sh,orchestrator/**,README.md}`; `docs/compatibility-tracking.md`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/upgrade/**`, `ws06-platform-oss-qualification/docs/compatibility-tracking.md`
- **READ-ONLY DEPENDENCIES:** W01 `mc-negotiation`, W03 `doubles/fake-runner/v1`.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W01-T023 INTEGRATED; W03-T022 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/upgrade/{build_versions.sh,README.md}`; `drills/upgrade/orchestrator/**`; `docs/compatibility-tracking.md`. **Steps:** 1. Build script by SHA into separate dirs. 2. Orchestrator steps with checkpoints. 3. Compatibility table per boundary.
- **SECURITY INVARIANTS:** INV-18.
- **FAILURE CASES:** Building from branch names instead of SHAs.
- **TESTS REQUIRED:** Orchestrator dry run with two builds of the same SHA (no-op upgrade). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `no-op upgrade (same SHA twice) completes`; `build from branch name refused`; `compatibility table covers all seven boundaries of doc 47`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Branch-name build refused.
- **ACCEPTANCE CRITERIA:** Tooling ready; table complete.
- **EVIDENCE TO RETURN:** Dry-run log.
- **OUT-OF-SCOPE:** Harness (T028).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** SHA pinning.

### W06-T022-FAILOVER-RIG
- **TASK_ID:** W06-T022-FAILOVER-RIG
- **OBJECTIVE:** **In plain words:** Build the database failover rig: main database plus standby, a way to cut the network, and a promotion script that always fences off the old main database before promoting the standby. **Precisely:** Build the P0-10G rig: PostgreSQL primary + synchronous local standby (testdb image), two workers, the journal witness, a partition tool (network namespaces/iptables) and a fenced manual promotion script with explicit fencing steps (no extra HA infrastructure).
- **WHY IT EXISTS:** 29 split brain; 35 recovery steps 1–3.
- **ARCHITECTURE REQUIREMENTS:** 29; 35; 50 journal epoch fence.
- **P0 CLAIMS:** supports P0-10G, P0-10H.
- **INPUT CONTRACTS:** I-03, I-04, JournalEpochFenceV1.
- **OUTPUT CONTRACTS:** `drills/failover/{compose.yaml,promote.sh,partition.sh,README.md}`; `mc-drill-kit` partition and clock helpers.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/failover/**`, `ws06-platform-oss-qualification/crates/mc-drill-kit/src/{partition.rs,clock.rs}`, `ws06-platform-oss-qualification/crates/mc-drill-kit/tests/{partition.rs,clock.rs}`
- **READ-ONLY DEPENDENCIES:** testdb, W01 lease/journal doubles.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W06-T002 INTEGRATED; W01-T015 INTEGRATED; W01-T018 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/failover/{compose.yaml,promote.sh,partition.sh,README.md}`; `crates/mc-drill-kit/src/{partition.rs,clock.rs}`; `crates/mc-drill-kit/tests/{partition.rs,clock.rs}`. **Steps:** 1. Primary/standby compose. 2. Promotion script: fence old primary first, then promote. 3. Partition helpers.
- **SECURITY INVARIANTS:** Only the active epoch can mutate/send.
- **FAILURE CASES:** Promotion before fencing.
- **TESTS REQUIRED:** Rig up; partition; promote; old primary refuses writes. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `rig up from clean`; `partition::cuts_and_restores`; `promote.sh fences old primary before promotion (log order)`; `write to old primary after promotion fails`; `clock::skew_injection`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Write to the old primary after promotion fails.
- **ACCEPTANCE CRITERIA:** Rig reproducible from a clean start.
- **EVIDENCE TO RETURN:** Rig log.
- **OUT-OF-SCOPE:** Harness (T029).
- **BLOCKERS:** Linux host with namespaces.
- **A3 REVIEW FOCUS:** Fence-before-promote.

### W06-T023-TRANSFER-ORCHESTRATOR
- **TASK_ID:** W06-T023-TRANSFER-ORCHESTRATOR
- **OBJECTIVE:** **In plain words:** Build the repository-transfer scenario on the fakes: move a repository to another owner while jobs, approvals and AI calls are live, then try to reuse each old permission. **Precisely:** Build the P0-10B scenario orchestrator against doubles: transfer repository A→B while a lease, approval, patch grant, model intent and check projection are live, then replay each old authority type after the observed ownership/epoch transition.
- **WHY IT EXISTS:** ADR-013 / INV-15.
- **ARCHITECTURE REQUIREMENTS:** 15; 18; 28 transfer row; 24; INV-15.
- **P0 CLAIMS:** supports P0-10B.
- **INPUT CONTRACTS:** ScopeV1, EligibilityProofV2, HumanApprovalAssertionV2, ModelIntentV2, CheckProjectionV2, AttemptBindingV2.
- **OUTPUT CONTRACTS:** `drills/transfer/{orchestrator/**,README.md}`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/transfer/**`
- **READ-ONLY DEPENDENCIES:** W01 lease/custody doubles, W02 fake GitHub + eligibility, W04 broker double.
- **PRECONDITIONS:** W06-T001 INTEGRATED; W01-T015 INTEGRATED; W01-T022 INTEGRATED; W02-T007 INTEGRATED; W02-T018 INTEGRATED; W04-T006 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/transfer/orchestrator/**`; `drills/transfer/README.md`. **Steps:** 1. Scenario script. 2. One replay per authority type. 3. Report per type.
- **SECURITY INVARIANTS:** INV-15.
- **FAILURE CASES:** One authority type left out.
- **TESTS REQUIRED:** Orchestrator run on doubles. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `orchestrator run on doubles completes`; `completeness check flags a missing authority type`; `each of six authority types replayed after transfer`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Missing authority type detected by a completeness check.
- **ACCEPTANCE CRITERIA:** All six authority types covered.
- **EVIDENCE TO RETURN:** Run log.
- **OUT-OF-SCOPE:** Harness (T031).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Completeness.

### W06-T024-P0-10C-HARNESS
- **TASK_ID:** W06-T024-P0-10C-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10C: "Artifact deletion respects holds and revocation". It runs one normal case and then 3 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10C harness on the artifact store: an ordinary expired object, a legal hold and an active PatchCustodyV1 pin (W01 custody double); deletion requests, grant revocation, custody reconciliation completion and authorized hold release; injections premature delete under an active pin, replay of a revoked download grant, unauthorized hold release.
- **WHY IT EXISTS:** Artifact deletion respects holds and revocation.
- **ARCHITECTURE REQUIREMENTS:** 24; 14; ADR-013/025; INV-16, INV-19.
- **P0 CLAIMS:** P0-10C (primary)
- **INPUT CONTRACTS:** ArtifactCustodyEntryV1, PatchCustodyV1 (I-16).
- **OUTPUT CONTRACTS:** harness; `drills/deletion/`; `run-manifest.draft.json`. **Evidence files the harness writes:** `deletion-receipts.json`, `retention-hold-history.json`, `download-probes.json`, `P0-10C/run-manifest.json`, `P0-10C/fixture-digests.json`, `P0-10C/evidence-manifest.sha256`, `P0-10C/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10C/**` (except `harness/Cargo.toml`), `ws06-platform-oss-qualification/drills/deletion/**`
- **READ-ONLY DEPENDENCIES:** artifact store, W01 custody double.
- **PRECONDITIONS:** W06-T014 INTEGRATED; W01-T022 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10C/harness/src/main.rs`; `claims/P0-10C/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10C/fixtures/** (synthetic only)`; `claims/P0-10C/fixture-digests.json`; `claims/P0-10C/run-manifest.draft.json`; `claims/P0-10C/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Scenario setup. 2. Injections from reset baselines. 3. Emit receipts and probes. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10C):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10C. (2) Request deletion, revoke grants, complete custody reconciliation, release hold with authorized ceremony. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-16; tombstones metadata only.
- **FAILURE CASES:** Lifecycle rule deleting a pinned object.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_premature_delete_under_active_pin`; `scenarios::injection_b_replay_revoked_download_grant`; `scenarios::injection_c_unauthorized_hold_release`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10C injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Premature delete under active pin; (b) replay revoked download grant; (c) unauthorized hold release.
- **ACCEPTANCE CRITERIA:** Ordinary deletion verifiably unavailable; pin/hold prevent removal; revoked reads fail. **Claim expected observations:** Ordinary deletion becomes verifiable unavailable; active pin/hold prevents premature removal; revoked reads fail. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Lifecycle vs pins.

### W06-T025-P0-10D-HARNESS
- **TASK_ID:** W06-T025-P0-10D-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10D: "Sensitive logs and immutable audit". It runs one normal case and then 5 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10D harness: inject distinct canaries into prompts, tool output, URL query and exception text; trigger normal, error and timeout paths; export every declared telemetry sink and scan it with the canary scanner; verify the AuditEnvelopeV1 chain (per-stream sequence + previous digest) and seal; attempt audit row update/delete; injections oversize error, privileged dump, secret in malformed request, audit-row update/delete.
- **WHY IT EXISTS:** Sensitive logs and immutable audit.
- **ARCHITECTURE REQUIREMENTS:** 24; 34; AuditEnvelopeV1; INV-16.
- **P0 CLAIMS:** P0-10D (primary)
- **INPUT CONTRACTS:** canary scanner, collector, AuditEnvelopeV1.
- **OUTPUT CONTRACTS:** harness; `drills/logs-audit/`; `run-manifest.draft.json`. **Evidence files the harness writes:** `sink-inventory.json`, `canary-search-results.json`, `audit-seal-verification.json`, `P0-10D/run-manifest.json`, `P0-10D/fixture-digests.json`, `P0-10D/evidence-manifest.sha256`, `P0-10D/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10D/**` (except `harness/Cargo.toml`), `ws06-platform-oss-qualification/drills/logs-audit/**`
- **READ-ONLY DEPENDENCIES:** canary scanner, collector, component doubles.
- **PRECONDITIONS:** W06-T012 INTEGRATED; W06-T013 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10D/harness/src/main.rs`; `claims/P0-10D/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10D/fixtures/** (synthetic only)`; `claims/P0-10D/fixture-digests.json`; `claims/P0-10D/run-manifest.draft.json`; `claims/P0-10D/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Sink inventory. 2. Injection runner. 3. Chain verifier. 4. Injections from reset baselines. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10D):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10D. (2) Trigger normal/error/timeout paths and export every declared telemetry sink; verify audit seal chain. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-16.
- **FAILURE CASES:** Crash dumps enabled by default.
- **TESTS REQUIRED:** DRY_RUN on doubles. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_oversize_error`; `scenarios::injection_b_privileged_dump`; `scenarios::injection_c_secret_in_malformed_request`; `scenarios::injection_d_audit_row_update_attempt`; `scenarios::injection_e_audit_row_delete_attempt`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10D injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Oversize error; (b) privileged dump; (c) secret in malformed request; (d) audit-row update attempt; (e) audit-row delete attempt.
- **ACCEPTANCE CRITERIA:** No canary in forbidden sinks; audit immutable and sealed. **Claim expected observations:** No canary raw content in policy-forbidden sinks; authoritative audit immutable and sealed within bound. **Threshold:** Forbidden canary matches 0; audit mutation success 0; seal lag <=declared 60 s test bound. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Sink inventory completeness.

### W06-T026-P0-07D-HARNESS
- **TASK_ID:** W06-T026-P0-07D-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-07D: "Administrative independence capability truth". It runs one normal case and then 3 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-07D harness: a compromised profile-A admin attempts profile-B enrollment, keys and control plane; capture the administration-access matrix, enrollment probes and C-class decisions; injections shared cloud root or CI credential, copied runner key, claimed native subagent graph.
- **WHY IT EXISTS:** Administrative independence capability truth.
- **ARCHITECTURE REQUIREMENTS:** 13; 31; ADR-006; INV-08.
- **P0 CLAIMS:** P0-07D (primary)
- **INPUT CONTRACTS:** IaC (T019); VerificationReceiptV2.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `administration-access-matrix.json`, `enrollment-probes.json`, `C-class-decisions.json`, `P0-07D/run-manifest.json`, `P0-07D/fixture-digests.json`, `P0-07D/evidence-manifest.sha256`, `P0-07D/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-07D/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** admin-domain IaC, W03 fake runner.
- **PRECONDITIONS:** W06-T019 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-07D/harness/src/main.rs`; `claims/P0-07D/harness/src/{scenarios.rs,report.rs}`; `claims/P0-07D/fixtures/** (synthetic only)`; `claims/P0-07D/fixture-digests.json`; `claims/P0-07D/run-manifest.draft.json`; `claims/P0-07D/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Probes. 2. Injections from reset baselines. 3. C-class decision log. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-07D):** (1) Provision exactly the sealed cell and empty evidence directory named P0-07D. (2) Evaluate claimed C3 for both; exercise compromised A admin against B enrollment/keys/control plane. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-08; same domain is never C3.
- **FAILURE CASES:** Hidden shared credential.
- **TESTS REQUIRED:** DRY_RUN with fake runners. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_shared_cloud_root_or_ci_credential`; `scenarios::injection_b_copied_runner_key`; `scenarios::injection_c_claimed_native_subagent_graph`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-07D injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Shared cloud root or CI credential; (b) copied runner key; (c) claimed native subagent graph.
- **ACCEPTANCE CRITERIA:** A cannot qualify C3; B qualifies only when all separation checks pass. **Claim expected observations:** A cannot qualify C3; B qualifies only when all separation checks pass; critical C3-required work on A is non-permitting. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-8).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Separation checks.

### W06-T027-P0-10E-HARNESS
- **TASK_ID:** W06-T027-P0-10E-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10E: "Load preserves fencing and mandatory coverage". It runs one normal case and then 4 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10E harness on the load tooling: run the declared envelope, then 2× admission while racing lease handoffs, outbox replay and graph expansion; injections slow provider, DB lock contention, duplicate receipts, expiry while waiting for a row lock; write `docs/measured-envelope.md` (U-008, G-019, A-001).
- **WHY IT EXISTS:** Load preserves fencing and mandatory coverage.
- **ARCHITECTURE REQUIREMENTS:** 33; 17; INV-03, INV-13.
- **P0 CLAIMS:** P0-10E (primary)
- **INPUT CONTRACTS:** load tooling.
- **OUTPUT CONTRACTS:** harness; `docs/measured-envelope.md`; `run-manifest.draft.json`. **Evidence files the harness writes:** `load-config.json`, `latency-histogram.json`, `conservation-and-fence-checks.json`, `P0-10E/run-manifest.json`, `P0-10E/fixture-digests.json`, `P0-10E/evidence-manifest.sha256`, `P0-10E/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10E/**` (except `harness/Cargo.toml`), `ws06-platform-oss-qualification/docs/measured-envelope.md`
- **READ-ONLY DEPENDENCIES:** load generator, checkers.
- **PRECONDITIONS:** W06-T020 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10E/harness/src/main.rs`; `claims/P0-10E/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10E/fixtures/** (synthetic only)`; `claims/P0-10E/fixture-digests.json`; `claims/P0-10E/run-manifest.draft.json`; `claims/P0-10E/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Envelope run. 2. 2× run. 3. Injections from reset baselines. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10E):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10E. (2) Run at envelope then 2x admission while racing lease handoffs, outbox replay and graph expansion. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-03 under pressure.
- **FAILURE CASES:** Silent shedding of mandatory units.
- **TESTS REQUIRED:** DRY_RUN at reduced scale. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_slow_provider`; `scenarios::injection_b_db_lock_contention`; `scenarios::injection_c_duplicate_receipts`; `scenarios::injection_d_expiration_while_waiting_for_row_lock`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10E injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Slow provider; (b) DB lock contention; (c) duplicate receipts; (d) expiration while waiting for row lock.
- **ACCEPTANCE CRITERIA:** Within envelope correctness holds; above it shedding is explicit. **Claim expected observations:** Within envelope correctness holds; above envelope shedding is explicit; mandatory units never omitted into PASS or stale completion accepted. **Threshold:** Unauthorized/stale accepts 0; DB connections <=40; 100 events/s sustained for 10 min; report p95/p99 rather than invent SLA. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Explicit shedding.

### W06-T028-P0-10F-HARNESS
- **TASK_ID:** W06-T028-P0-10F-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10F: "Rolling schema/protocol upgrade". It runs one normal case and then 4 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10F harness: two current and two previous-minor server/runner instances, expand → writers → contract migration and a live old-version attempt; injections old reader sees a new critical field, writer downgrade, crash midway, resume an old session with a new version.
- **WHY IT EXISTS:** Rolling schema/protocol upgrade.
- **ARCHITECTURE REQUIREMENTS:** 47; 07; ADR-014/024; INV-18.
- **P0 CLAIMS:** P0-10F (primary)
- **INPUT CONTRACTS:** upgrade orchestrator.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `upgrade-step-log.json`, `version-negotiation.json`, `before-after-record-digests.json`, `P0-10F/run-manifest.json`, `P0-10F/fixture-digests.json`, `P0-10F/evidence-manifest.sha256`, `P0-10F/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10F/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** orchestrator, builds.
- **PRECONDITIONS:** W06-T021 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10F/harness/src/main.rs`; `claims/P0-10F/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10F/fixtures/** (synthetic only)`; `claims/P0-10F/fixture-digests.json`; `claims/P0-10F/run-manifest.draft.json`; `claims/P0-10F/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Four instances. 2. Migration steps. 3. Injections from reset baselines. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10F):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10F. (2) Expand readers/DB, deploy writers, finish old attempt, then contract after compatibility window. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-18.
- **FAILURE CASES:** Mixed semantics accepted.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_old_reader_sees_new_critical_field`; `scenarios::injection_b_writer_downgrade`; `scenarios::injection_c_crash_midway`; `scenarios::injection_d_resume_old_session_with_new_version`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10F injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Old reader sees new critical field; (b) writer downgrade; (c) crash midway; (d) resume old session with new version.
- **ACCEPTANCE CRITERIA:** No mixed-semantic authority accepted; rollback preserves records. **Claim expected observations:** No mixed-semantic authority accepted; downgrade rejects; old supported attempts preserve pinned semantics; rollback preserves records. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Version pinning per attempt.

### W06-T029-P0-10G-HARNESS
- **TASK_ID:** W06-T029-P0-10G-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10G: "PostgreSQL failover fencing". It runs one normal case and then 3 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10G harness on the failover rig: commit lease/effect state, partition the primary, promote the standby through the fenced process, resume both workers; injections old primary still reachable to a worker, ambiguous transaction commit, delayed heartbeat.
- **WHY IT EXISTS:** PostgreSQL failover fencing.
- **ARCHITECTURE REQUIREMENTS:** 29; 35; 50; INV-01, INV-13.
- **P0 CLAIMS:** P0-10G (primary)
- **INPUT CONTRACTS:** failover rig.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `failover-timeline.json`, `fencing-log.json`, `DB-and-journal-diff.json`, `P0-10G/run-manifest.json`, `P0-10G/fixture-digests.json`, `P0-10G/evidence-manifest.sha256`, `P0-10G/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10G/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** rig, W01 doubles.
- **PRECONDITIONS:** W06-T022 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10G/harness/src/main.rs`; `claims/P0-10G/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10G/fixtures/** (synthetic only)`; `claims/P0-10G/fixture-digests.json`; `claims/P0-10G/run-manifest.draft.json`; `claims/P0-10G/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Scenario. 2. Injections from reset baselines. 3. Timeline + DB/journal diff. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10G):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10G. (2) Commit lease/effect state, partition primary, promote standby through fenced process, resume both workers. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** Only the active epoch mutates/sends.
- **FAILURE CASES:** Old primary accepting writes.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_old_primary_remains_reachable_to_worker`; `scenarios::injection_b_ambiguous_transaction_commit`; `scenarios::injection_c_delayed_heartbeat`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10G injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Old primary remains reachable to worker; (b) ambiguous transaction commit; (c) delayed heartbeat.
- **ACCEPTANCE CRITERIA:** Duplicate effect/ledger operations idempotent; unknown commits reconciled. **Claim expected observations:** Only active epoch can mutate/send; duplicate effect/ledger operations remain idempotent; unknown commits reconciled. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Fencing order.

### W06-T030-RESTORE-RIG
- **TASK_ID:** W06-T030-RESTORE-RIG
- **OBJECTIVE:** **In plain words:** Build the restore rig: back up and restore the database, and recover any external calls the restored database forgot by reading them from the journal's witness copies. **Precisely:** Build the P0-10H rig: backup/PITR scripts for the testdb, a witness-range import tool that reads the journal's uncertain window through W01's journal `read_range`, a restore-only-the-DB procedure, and the restore acceptance record template.
- **WHY IT EXISTS:** Finding 9: surviving-row scans are insufficient; recovery must enumerate from the witness range.
- **ARCHITECTURE REQUIREMENTS:** 35 recovery procedure and restore acceptance record; 50 restore/retention; ADR-023.
- **P0 CLAIMS:** supports P0-10H, P0-10I (W01).
- **INPUT CONTRACTS:** I-04, ExternalEffectJournalV1, JournalWitnessReceiptV1, JournalEpochFenceV1.
- **OUTPUT CONTRACTS:** `drills/restore/{backup.sh,restore.sh,witness_import/**,RESTORE_ACCEPTANCE_TEMPLATE.md,README.md}`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/drills/restore/**`
- **READ-ONLY DEPENDENCIES:** failover rig, W01 `mc-journal`.
- **PRECONDITIONS:** W06-T022 INTEGRATED; W01-T017 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `drills/restore/{backup.sh,restore.sh,RESTORE_ACCEPTANCE_TEMPLATE.md,README.md}`; `drills/restore/witness_import/**`. **Steps:** 1. Backup/PITR. 2. Witness range import. 3. Template.
- **SECURITY INVARIANTS:** No new send while the journal window is uncertain.
- **FAILURE CASES:** Import scanning only DB rows.
- **TESTS REQUIRED:** Backup → write → restore → import finds the missing entry. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `backup → write → restore → import finds the missing entry`; `witness unavailable → import refuses completeness`; `restore restores only the DB (journal untouched)`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Witness unavailable → import refuses to declare completeness.
- **ACCEPTANCE CRITERIA:** Rig reproducible.
- **EVIDENCE TO RETURN:** Rig log.
- **OUT-OF-SCOPE:** Harness (T032).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Enumeration from the witness range.

### W06-T031-P0-10B-HARNESS
- **TASK_ID:** W06-T031-P0-10B-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10B: "Transfer invalidates all old authority". It runs one normal case and then 3 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10B harness on the transfer orchestrator: verify that no old authority grants evidence/approval/effect in B, that audit stays tenant-authorized and no private artifact crosses ownership; injections delayed webhook, stale cache, restore of an old epoch snapshot.
- **WHY IT EXISTS:** Transfer invalidates all old authority.
- **ARCHITECTURE REQUIREMENTS:** 15; 18; 28; ADR-013; INV-15.
- **P0 CLAIMS:** P0-10B (primary)
- **INPUT CONTRACTS:** transfer orchestrator.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `transfer-replay-matrix.json`, `epoch-history.json`, `artifact-access-log.json`, `P0-10B/run-manifest.json`, `P0-10B/fixture-digests.json`, `P0-10B/evidence-manifest.sha256`, `P0-10B/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10B/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** orchestrator, doubles.
- **PRECONDITIONS:** W06-T023 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10B/harness/src/main.rs`; `claims/P0-10B/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10B/fixtures/** (synthetic only)`; `claims/P0-10B/fixture-digests.json`; `claims/P0-10B/run-manifest.draft.json`; `claims/P0-10B/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Replay matrix. 2. Injections from reset baselines. 3. Epoch history + artifact access log. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10B):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10B. (2) Replay each old authority type after observed current ownership/epoch transition. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-15.
- **FAILURE CASES:** Model intent accepted under the old epoch.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_delayed_webhook`; `scenarios::injection_b_stale_cache`; `scenarios::injection_c_restore_old_epoch_snapshot`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10B injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Delayed webhook; (b) stale cache; (c) restore old epoch snapshot.
- **ACCEPTANCE CRITERIA:** No old authority grants anything in B. **Claim expected observations:** Old authority cannot grant evidence/approval/effect in B; audit remains tenant-authorized and no private artifacts cross ownership. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7 + two staging orgs).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Matrix completeness.

### W06-T032-P0-10H-HARNESS
- **TASK_ID:** W06-T032-P0-10H-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program ("harness") that will later prove claim P0-10H: "Regional restore recovers intents lost from database". It runs one normal case and then 4 attack/failure cases, each from a clean start, and writes the evidence files. In this task it only runs locally against fakes (DRY_RUN); the real run is the separate RUN task after G2/G3. **Precisely:** Build the P0-10H harness on the restore rig: back up at T0; create and externally confirm an effect at T1 > T0 with the durable journal (two storage domains); restore only the DB to T0; enumerate the witness's uncertain window including the effect absent from the restored DB; reconcile the provider and reconstruct audit linkage before unfreeze; injections delete intent from restored snapshot by RPO, delay provider query, witness unavailable, old region still sending. Hand off to W01-T046 for the fresh-bootstrap portion (P0-10I).
- **WHY IT EXISTS:** Regional restore recovers intents lost from the database.
- **ARCHITECTURE REQUIREMENTS:** 35; 50; ADR-023; INV-01, INV-17.
- **P0 CLAIMS:** P0-10H (primary)
- **INPUT CONTRACTS:** restore rig.
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `backup-manifest.json`, `witness-range-export.json`, `recovered-effect-inventory.json`, `P0-10H/run-manifest.json`, `P0-10H/fixture-digests.json`, `P0-10H/evidence-manifest.sha256`, `P0-10H/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10H/**` (except `harness/Cargo.toml`)
- **READ-ONLY DEPENDENCIES:** restore rig, W02 fake GitHub.
- **PRECONDITIONS:** W06-T030 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `claims/P0-10H/harness/src/main.rs`; `claims/P0-10H/harness/src/{scenarios.rs,report.rs}`; `claims/P0-10H/fixtures/** (synthetic only)`; `claims/P0-10H/fixture-digests.json`; `claims/P0-10H/run-manifest.draft.json`; `claims/P0-10H/README.md (how to run DRY_RUN; which injections; expected outputs)`. **Steps:** 1. Scenario. 2. Injections from reset baselines. 3. Recovered-effect inventory. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-10H):** (1) Provision exactly the sealed cell and empty evidence directory named P0-10H. (2) Enumerate journal witness uncertain window including effect absent from restored DB; reconcile provider and reconstruct audit linkage before unfreeze. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** No new send while uncertain; old region fenced.
- **FAILURE CASES:** Recovery scanning only DB rows.
- **TESTS REQUIRED:** DRY_RUN. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_delete_intent_from_restored_snapshot_by_rpo`; `scenarios::injection_b_delay_provider_query`; `scenarios::injection_c_witness_unavailable`; `scenarios::injection_d_old_region_still_sending`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** All P0-10H injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) Delete intent from restored snapshot by RPO; (b) delay provider query; (c) witness unavailable; (d) old region still sending.
- **ACCEPTANCE CRITERIA:** The recovered effect is discovered without a surviving-row scan. **Claim expected observations:** Recovered effect is discovered without surviving-row scan; no new send while journal uncertain; regional external authority remains fenced. **Threshold:** All journaled T0..freeze effects accounted for (100%); zero blind duplicate sends; unavailable journal leaves gate closed. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-7 + two storage domains).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Witness-range enumeration.

### W06-T033-HOST-LAB-INVENTORY
- **TASK_ID:** W06-T033-HOST-LAB-INVENTORY
- **OBJECTIVE:** **In plain words:** Write down which machines we have for sandbox testing (Linux with KVM, your Mac's lab account, an Azure Windows VM with Hyper-V) and add small scripts that print each machine's exact OS and hypervisor versions. A missing machine is recorded honestly, not hidden. **Precisely:** Document the hardware lab (G-065) and provide capture scripts: Linux x86_64 KVM host (WSL2 nested virtualization or a rented VM), Linux arm64 if available, macOS arm64 (master's Mac, dedicated non-admin `mclab` user account), Windows x86_64 with Hyper-V (an Azure VM with nested virtualization, Azure for Students credit, deallocated when idle; or a Windows 11 Education/Pro PC). Each host's exact OS, kernel, hypervisor and CPU virtualization flags are captured by script for later sealing in run-manifests.
- **WHY IT EXISTS:** P0-02 cells are per OS/architecture/hypervisor; missing hardware must be recorded, not hidden.
- **ARCHITECTURE REQUIREMENTS:** 31 initial matrix; 10 sandbox tiers.
- **P0 CLAIMS:** enables P0-02A–D.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `docs/host-lab.md` (per host: role, access method, availability, decision AVAILABLE/UNAVAILABLE); `lab/capture_linux.sh`, `lab/capture_macos.sh`, `lab/capture_windows.ps1` (print JSON with OS, kernel, hypervisor version, virtualization flags, CPU model).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/docs/host-lab.md`, `ws06-platform-oss-qualification/lab/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** G0, G1; PRESTART: hardware access arranged by the host owners (the Linux KVM VM, the Mac's `mclab` account, the Azure Windows VM), used by the W03 lane, which uses the inventory.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `docs/host-lab.md`; `lab/capture_linux.sh`; `lab/capture_macos.sh`; `lab/capture_windows.ps1`; `lab/README.md`. **Steps:** 1. One capture script per OS; output JSON only, no secrets or hostnames beyond aliases. 2. Run each on its host and paste the JSON into `docs/host-lab.md`. 3. Mark each cell AVAILABLE or UNAVAILABLE with the reason (e.g. "Windows Home: no Hyper-V").
- **SECURITY INVARIANTS:** No serial numbers, usernames or IPs in the doc.
- **FAILURE CASES:** Marking a cell AVAILABLE without a capture.
- **TESTS REQUIRED:** Each script runs on its OS and prints valid JSON (`python3 -m json.tool`). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `bash lab/capture_linux.sh | python3 -m json.tool` succeeds on the Linux host; `bash lab/capture_macos.sh | python3 -m json.tool` succeeds on the Mac; `powershell -File lab/capture_windows.ps1 | python -m json.tool` succeeds on the Windows host; `capture_linux.sh exits non-zero when /dev/kvm is missing`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Script exits non-zero when the hypervisor is missing (e.g. `/dev/kvm` absent).
- **ACCEPTANCE CRITERIA:** Every cell has a capture or an UNAVAILABLE reason.
- **EVIDENCE TO RETURN:** Doc SHA-256; the JSON captures.
- **OUT-OF-SCOPE:** Sandbox code.
- **BLOCKERS:** none (UNAVAILABLE is a valid outcome).
- **A3 REVIEW FOCUS:** Honest availability.

### W06-T034-LOCAL-SERVICES
- **TASK_ID:** W06-T034-LOCAL-SERVICES
- **OBJECTIVE:** **In plain words:** Add the local test servers to docker-compose: a private Git server and private package registries (npm, Python, Cargo). Tests never touch the real internet. **Precisely:** Publish the shared local test services as `local-services/compose.yaml` (its own compose project, started by each consumer with `docker compose --env-file <own .env.local> -f <repo>/ws06-platform-oss-qualification/local-services/compose.yaml up -d`; consumers: the W03 lane's checkout tasks use the Git server, the W04 lane's dependency-acquisition tasks use the registries): a Git smart-HTTP server over HTTPS with a self-signed staging CA, Verdaccio (npm) with one public and two private (tenant A, tenant B) scopes protected by canary tokens, devpi (Python), and a static file server for a Cargo sparse index and toolchain archives — all with health checks and pinned image digests.
- **WHY IT EXISTS:** P0-05 experiments must run against controlled local servers, never the public internet (G-015).
- **ARCHITECTURE REQUIREMENTS:** 55 acquisition classes; 08 checkout; 42 synthetic data.
- **P0 CLAIMS:** enables P0-05A–H.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `local-services/compose.yaml`; `local-services/services/**` (configs); `local-services/README.md` (ports from `.env.local`, how to load fixtures).
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/local-services/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws06-platform-oss-qualification/`):** `local-services/compose.yaml`; `local-services/services/git/{nginx.conf,Dockerfile}`; `local-services/services/verdaccio/config.yaml`; `local-services/services/devpi/README.md`; `local-services/services/static/README.md`; `local-services/services/ca/make_ca.sh (writes into a git-ignored dir)`; `local-services/README.md`; `local-services/healthcheck.sh`. **Steps:** 1. Staging CA generated by `mkcert`-style script into a git-ignored directory; path from `MC_STAGING_CA_PATH`. 2. git-http-backend behind nginx (HTTPS). 3. Verdaccio config with three scopes and canary tokens from `.env.local`. 4. devpi and static server. 5. Health checks.
- **SECURITY INVARIANTS:** Canary tokens only; CA private key never committed.
- **FAILURE CASES:** A service pulling from the public internet at start.
- **TESTS REQUIRED:** `docker compose up -d` then all health checks healthy; `git ls-remote https://localhost:$MC_GIT_HTTP_PORT/...` works with the staging CA. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `docker compose up -d && bash local-services/healthcheck.sh` reports all healthy; `git ls-remote over HTTPS with the staging CA works`; `private scope without token → 401`; `tenant A token on tenant B scope → 403`; `no service pulls from the public internet at start (run with network disabled after image pull)`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Private scope without token → 401; tenant A token on tenant B scope → 403.
- **ACCEPTANCE CRITERIA:** All services healthy from a clean start using only `.env.local`.
- **EVIDENCE TO RETURN:** Health output.
- **OUT-OF-SCOPE:** Fixture content (W05-T010…W05-T012).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No public network; secrets only in env.

---

## RUN tasks (atomic P0 claims; generated from the authoritative claim JSON)

### W06-RUN-P0-06A
- **TASK_ID:** W06-RUN-P0-06A
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-06A ("Canonical positive encoding parity") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-06A — "Canonical positive encoding parity" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Canonical positive encoding parity; failure cannot be hidden by an umbrella gate result. Family gate P0-06. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-06A; TRACEABILITY/atomic-prototype-claims.json record P0-06A; 19 Prototype Evidence; 42; ENCODING.md, REGISTRY.json, ADR-024, INV-18.
- **P0 CLAIMS:** P0-06A (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T016. Topology: Independent Rust and TypeScript harnesses built only after later authorization; all SCHEMAS signed/hashed contracts and TEST_VECTORS fixtures. Fixture: P0-06A fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `rust-vector-results.json`, `typescript-vector-results.json`, `byte-comparison.json`, `P0-06A/run-manifest.json`, `P0-06A/fixture-digests.json`, `P0-06A/evidence-manifest.sha256`, `P0-06A/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-06A/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-06A/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-06A/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-06A/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 mc-wire CLI, W05 wire-ts CLI — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-06; W06-T016 INTEGRATED; integration milestone(s): IM-1; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Independent Rust and TypeScript harnesses built only after later authorization; all SCHEMAS signed/hashed contracts and TEST_VECTORS fixtures. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-06 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-06A/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-06A/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-06A. 5. Encode/decode every positive fixture independently and compare complete bytes, digest and signature input. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-06A/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-06A/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-06A/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-06A/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Canonical positive encoding parity; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Vary host locale, timezone and map insertion order without changing semantic values.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Vary host locale, timezone and map insertion order without changing semantic values.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: 100% positive fixtures byte/digest/signature-input identical; zero unexplained differences. Expected observations: Canonical bytes identical across implementations and environments; signature domain and ordered fields match normative vectors.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** rust-vector-results.json, typescript-vector-results.json, byte-comparison.json, P0-06A/run-manifest.json, P0-06A/fixture-digests.json, P0-06A/evidence-manifest.sha256, P0-06A/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-1; external: NO.
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-06B
- **TASK_ID:** W06-RUN-P0-06B
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-06B ("Canonical rejection parity") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-06B — "Canonical rejection parity" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Canonical rejection parity; failure cannot be hidden by an umbrella gate result. Family gate P0-06. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-06B; TRACEABILITY/atomic-prototype-claims.json record P0-06B; 19 Prototype Evidence; 42; ENCODING.md, ADR-024, INV-18.
- **P0 CLAIMS:** P0-06B (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T017. Topology: Same harnesses and all negative fixtures, including unknown fields, null, integer bounds, malformed bytes and array semantics. Fixture: P0-06B fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `rejection-matrix.json`, `malformed-fixtures.bin`, `decoder-traces.json`, `P0-06B/run-manifest.json`, `P0-06B/fixture-digests.json`, `P0-06B/evidence-manifest.sha256`, `P0-06B/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-06B/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-06B/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-06B/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-06B/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 mc-wire CLI, W05 wire-ts CLI — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-06; W06-T017 INTEGRATED; integration milestone(s): IM-1; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Same harnesses and all negative fixtures, including unknown fields, null, integer bounds, malformed bytes and array semantics. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-06 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-06B/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-06B/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-06B. 5. Decode each negative fixture and record exact rejection category in both languages. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-06B/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-06B/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-06B/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-06B/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Canonical rejection parity; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Duplicate keys; noncanonical integer; wrong domain/type version; unknown critical/noncritical field inside closed signed record.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Duplicate keys; noncanonical integer; wrong domain/type version; unknown critical/noncritical field inside closed signed record.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: 100% normative negative fixtures rejected by both implementations. Expected observations: Both reject every forbidden encoding; no normalizing malformed signed input into acceptable equivalent.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** rejection-matrix.json, malformed-fixtures.bin, decoder-traces.json, P0-06B/run-manifest.json, P0-06B/fixture-digests.json, P0-06B/evidence-manifest.sha256, P0-06B/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-1; external: NO.
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-07D
- **TASK_ID:** W06-RUN-P0-07D
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-07D ("Administrative independence capability truth") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-07D — "Administrative independence capability truth" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Administrative independence capability truth; failure cannot be hidden by an umbrella gate result. Family gate P0-07. BLOCKS_FREEZE: CELL (C3).
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-07D; TRACEABILITY/atomic-prototype-claims.json record P0-07D; 19 Prototype Evidence; 42; 13, 31, ADR-006, INV-08.
- **P0 CLAIMS:** P0-07D (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T026. Topology: Profile A has two runners under same root admin; profile B has separately administered runners, signing roots, enrollment and recovery. Fixture: P0-07D fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `administration-access-matrix.json`, `enrollment-probes.json`, `C-class-decisions.json`, `P0-07D/run-manifest.json`, `P0-07D/fixture-digests.json`, `P0-07D/evidence-manifest.sha256`, `P0-07D/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-07D/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-07D/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-07D/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-07D/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W03 runner pools, W02 verification, W01 trust authority — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-07; W06-T026 INTEGRATED; integration milestone(s): IM-8; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Profile A has two runners under same root admin; profile B has separately administered runners, signing roots, enrollment and recovery. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-07 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-07D/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-07D/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-07D. 5. Evaluate claimed C3 for both; exercise compromised A admin against B enrollment/keys/control plane. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-07D/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-07D/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-07D/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-07D/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Administrative independence capability truth; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Shared cloud root or CI credential; copied runner key; claimed native subagent graph.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Shared cloud root or CI credential; copied runner key; claimed native subagent graph.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: A cannot qualify C3; B qualifies only when all separation checks pass; critical C3-required work on A is non-permitting.. ACCEPTED only with both lens reviews (SEC, CC).
- **EVIDENCE TO RETURN:** administration-access-matrix.json, enrollment-probes.json, C-class-decisions.json, P0-07D/run-manifest.json, P0-07D/fixture-digests.json, P0-07D/evidence-manifest.sha256, P0-07D/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-8; external: YES — two independently administered cloud accounts.
- **A3 REVIEW FOCUS:** Evidence lenses: SEC lens → fresh evidence-A3 session using the W03 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10B
- **TASK_ID:** W06-RUN-P0-10B
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10B ("Transfer invalidates all old authority") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10B — "Transfer invalidates all old authority" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Transfer invalidates all old authority; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10B; TRACEABILITY/atomic-prototype-claims.json record P0-10B; 19 Prototype Evidence; 42; 15, 18, 28, ADR-013, INV-15.
- **P0 CLAIMS:** P0-10B (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T031. Topology: Transfer repository A→B while leases, approval, patch grants, model intents and projection are live. Fixture: P0-10B fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `transfer-replay-matrix.json`, `epoch-history.json`, `artifact-access-log.json`, `P0-10B/run-manifest.json`, `P0-10B/fixture-digests.json`, `P0-10B/evidence-manifest.sha256`, `P0-10B/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10B/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10B/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10B/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10B/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 lease/AA/custody, W02 eligibility/projection, W04 model intents — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T031 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Transfer repository A→B while leases, approval, patch grants, model intents and projection are live. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10B/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10B/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10B. 5. Replay each old authority type after observed current ownership/epoch transition. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10B/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10B/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10B/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10B/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Transfer invalidates all old authority; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Delayed webhook; stale cache; restore old epoch snapshot.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Delayed webhook; stale cache; restore old epoch snapshot.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: Old authority cannot grant evidence/approval/effect in B; audit remains tenant-authorized and no private artifacts cross ownership.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** transfer-replay-matrix.json, epoch-history.json, artifact-access-log.json, P0-10B/run-manifest.json, P0-10B/fixture-digests.json, P0-10B/evidence-manifest.sha256, P0-10B/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: YES — two GitHub staging orgs (transfer).
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10C
- **TASK_ID:** W06-RUN-P0-10C
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10C ("Artifact deletion respects holds and revocation") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10C — "Artifact deletion respects holds and revocation" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Artifact deletion respects holds and revocation; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10C; TRACEABILITY/atomic-prototype-claims.json record P0-10C; 19 Prototype Evidence; 42; 24, 14, ADR-025, INV-16, INV-19.
- **P0 CLAIMS:** P0-10C (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T024. Topology: Tenant artifacts include ordinary expired object, legal hold and active PatchCustodyV1 pin with separate grants. Fixture: P0-10C fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `deletion-receipts.json`, `retention-hold-history.json`, `download-probes.json`, `P0-10C/run-manifest.json`, `P0-10C/fixture-digests.json`, `P0-10C/evidence-manifest.sha256`, `P0-10C/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10C/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10C/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10C/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10C/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 custody — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T024 INTEGRATED; integration milestone(s): —; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Tenant artifacts include ordinary expired object, legal hold and active PatchCustodyV1 pin with separate grants. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10C/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10C/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10C. 5. Request deletion, revoke grants, complete custody reconciliation, release hold with authorized ceremony. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10C/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10C/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10C/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10C/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Artifact deletion respects holds and revocation; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Premature delete under active pin; replay revoked download grant; unauthorized hold release.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Premature delete under active pin; replay revoked download grant; unauthorized hold release.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: Ordinary deletion becomes verifiable unavailable; active pin/hold prevents premature removal; revoked reads fail.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** deletion-receipts.json, retention-hold-history.json, download-probes.json, P0-10C/run-manifest.json, P0-10C/fixture-digests.json, P0-10C/evidence-manifest.sha256, P0-10C/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), —; external: NO (local MinIO).
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10D
- **TASK_ID:** W06-RUN-P0-10D
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10D ("Sensitive logs and immutable audit") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10D — "Sensitive logs and immutable audit" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Sensitive logs and immutable audit; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10D; TRACEABILITY/atomic-prototype-claims.json record P0-10D; 19 Prototype Evidence; 42; 24, 34, INV-16.
- **P0 CLAIMS:** P0-10D (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T025. Topology: Inject distinct canary secret/source strings in prompts, tool output, URL query and exception; audit protected role records effect hashes. Fixture: P0-10D fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `sink-inventory.json`, `canary-search-results.json`, `audit-seal-verification.json`, `P0-10D/run-manifest.json`, `P0-10D/fixture-digests.json`, `P0-10D/evidence-manifest.sha256`, `P0-10D/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10D/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10D/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10D/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10D/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 audit, W04 broker, all component logging — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T025 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Inject distinct canary secret/source strings in prompts, tool output, URL query and exception; audit protected role records effect hashes. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10D/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10D/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10D. 5. Trigger normal/error/timeout paths and export every declared telemetry sink; verify audit seal chain. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10D/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10D/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10D/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10D/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Sensitive logs and immutable audit; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Oversize error; privileged dump; secret in malformed request; audit-row update/delete attempt.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Oversize error; privileged dump; secret in malformed request; audit-row update/delete attempt.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Forbidden canary matches 0; audit mutation success 0; seal lag <=declared 60 s test bound. Expected observations: No canary raw content in policy-forbidden sinks; authoritative audit immutable and sealed within bound.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** sink-inventory.json, canary-search-results.json, audit-seal-verification.json, P0-10D/run-manifest.json, P0-10D/fixture-digests.json, P0-10D/evidence-manifest.sha256, P0-10D/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: NO.
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10E
- **TASK_ID:** W06-RUN-P0-10E
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10E ("Load preserves fencing and mandatory coverage") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10E — "Load preserves fencing and mandatory coverage" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Load preserves fencing and mandatory coverage; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10E; TRACEABILITY/atomic-prototype-claims.json record P0-10E; 19 Prototype Evidence; 42; 33, 17, INV-03, INV-13.
- **P0 CLAIMS:** P0-10E (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T027. Topology: Declared load envelope: 10 tenants, 100 active jobs, 1000 queued jobs, 100 events/s for 10 min; pool max 40 DB connections. Fixture: P0-10E fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `load-config.json`, `latency-histogram.json`, `conservation-and-fence-checks.json`, `P0-10E/run-manifest.json`, `P0-10E/fixture-digests.json`, `P0-10E/evidence-manifest.sha256`, `P0-10E/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10E/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10E/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10E/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10E/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 lease/outbox, W02 planner/reducer, W04 graph — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T027 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Declared load envelope: 10 tenants, 100 active jobs, 1000 queued jobs, 100 events/s for 10 min; pool max 40 DB connections. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10E/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10E/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10E. 5. Run at envelope then 2x admission while racing lease handoffs, outbox replay and graph expansion. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10E/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10E/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10E/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10E/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Load preserves fencing and mandatory coverage; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Slow provider; DB lock contention; duplicate receipts; expiration while waiting for row lock.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Slow provider; DB lock contention; duplicate receipts; expiration while waiting for row lock.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Unauthorized/stale accepts 0; DB connections <=40; 100 events/s sustained for 10 min; report p95/p99 rather than invent SLA. Expected observations: Within envelope correctness holds; above envelope shedding is explicit; mandatory units never omitted into PASS or stale completion accepted.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** load-config.json, latency-histogram.json, conservation-and-fence-checks.json, P0-10E/run-manifest.json, P0-10E/fixture-digests.json, P0-10E/evidence-manifest.sha256, P0-10E/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: NO.
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10F
- **TASK_ID:** W06-RUN-P0-10F
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10F ("Rolling schema/protocol upgrade") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10F — "Rolling schema/protocol upgrade" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Rolling schema/protocol upgrade; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10F; TRACEABILITY/atomic-prototype-claims.json record P0-10F; 19 Prototype Evidence; 42; 47, 07 migrations, ADR-024, INV-18.
- **P0 CLAIMS:** P0-10F (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T028. Topology: Two current and two previous-minor server/runner instances, expand-contract migration plan and a live old-version attempt. Fixture: P0-10F fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `upgrade-step-log.json`, `version-negotiation.json`, `before-after-record-digests.json`, `P0-10F/run-manifest.json`, `P0-10F/fixture-digests.json`, `P0-10F/evidence-manifest.sha256`, `P0-10F/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10F/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10F/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10F/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10F/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 negotiation, W03 runner — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T028 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Two current and two previous-minor server/runner instances, expand-contract migration plan and a live old-version attempt. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10F/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10F/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10F. 5. Expand readers/DB, deploy writers, finish old attempt, then contract after compatibility window. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10F/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10F/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10F/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10F/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Rolling schema/protocol upgrade; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Old reader sees new critical field; writer downgrade; crash midway; resume old session with new version.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Old reader sees new critical field; writer downgrade; crash midway; resume old session with new version.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: No mixed-semantic authority accepted; downgrade rejects; old supported attempts preserve pinned semantics; rollback preserves records.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** upgrade-step-log.json, version-negotiation.json, before-after-record-digests.json, P0-10F/run-manifest.json, P0-10F/fixture-digests.json, P0-10F/evidence-manifest.sha256, P0-10F/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: NO.
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10G
- **TASK_ID:** W06-RUN-P0-10G
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10G ("PostgreSQL failover fencing") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10G — "PostgreSQL failover fencing" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** PostgreSQL failover fencing; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10G; TRACEABILITY/atomic-prototype-claims.json record P0-10G; 19 Prototype Evidence; 42; 29, 35, 50, INV-01, INV-13.
- **P0 CLAIMS:** P0-10G (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T029. Topology: Primary and synchronous local standby, two workers, journal witness and controllable network partition. Fixture: P0-10G fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `failover-timeline.json`, `fencing-log.json`, `DB-and-journal-diff.json`, `P0-10G/run-manifest.json`, `P0-10G/fixture-digests.json`, `P0-10G/evidence-manifest.sha256`, `P0-10G/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10G/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10G/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10G/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10G/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 lease/journal/effects — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T029 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Primary and synchronous local standby, two workers, journal witness and controllable network partition. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10G/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10G/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10G. 5. Commit lease/effect state, partition primary, promote standby through fenced process, resume both workers. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10G/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10G/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10G/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10G/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** PostgreSQL failover fencing; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Old primary remains reachable to worker; ambiguous transaction commit; delayed heartbeat.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Old primary remains reachable to worker; ambiguous transaction commit; delayed heartbeat.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: Only active epoch can mutate/send; duplicate effect/ledger operations remain idempotent; unknown commits reconciled.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** failover-timeline.json, fencing-log.json, DB-and-journal-diff.json, P0-10G/run-manifest.json, P0-10G/fixture-digests.json, P0-10G/evidence-manifest.sha256, P0-10G/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: NO (local HA).
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

### W06-RUN-P0-10H
- **TASK_ID:** W06-RUN-P0-10H
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-10H ("Regional restore recovers intents lost from database") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-10H — "Regional restore recovers intents lost from database" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Regional restore recovers intents lost from database; failure cannot be hidden by an umbrella gate result. Family gate P0-10. BLOCKS_FREEZE: YES.
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-10H; TRACEABILITY/atomic-prototype-claims.json record P0-10H; 19 Prototype Evidence; 42; 35, 50, ADR-023, INV-01, INV-17.
- **P0 CLAIMS:** P0-10H (primary owner W06; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W06-T032. Topology: Recovery backup timestamp T0; create and externally confirm effect at T1>T0 with durable journal, then restore only DB to T0. Fixture: P0-10H fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `backup-manifest.json`, `witness-range-export.json`, `recovered-effect-inventory.json`, `P0-10H/run-manifest.json`, `P0-10H/fixture-digests.json`, `P0-10H/evidence-manifest.sha256`, `P0-10H/independent-review.json`; `ws06-platform-oss-qualification/claims/P0-10H/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws06-platform-oss-qualification/claims/P0-10H/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws06-platform-oss-qualification/claims/P0-10H/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-10H/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 journal/effects, W02 reconciliation reads — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-10; W06-T032 INTEGRATED; integration milestone(s): IM-7; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Recovery backup timestamp T0; create and externally confirm effect at T1>T0 with durable journal, then restore only DB to T0. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-10 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws06-platform-oss-qualification/claims/P0-10H/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws06-platform-oss-qualification/claims/P0-10H/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-10H. 5. Enumerate journal witness uncertain window including effect absent from restored DB; reconcile provider and reconstruct audit linkage before unfreeze. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws06-platform-oss-qualification/claims/P0-10H/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-10H/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws06-platform-oss-qualification/claims/P0-10H/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws06-platform-oss-qualification/claims/P0-10H/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Regional restore recovers intents lost from database; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** Delete intent from restored snapshot by RPO; delay provider query; witness unavailable; old region still sending.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** Delete intent from restored snapshot by RPO; delay provider query; witness unavailable; old region still sending.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: All journaled T0..freeze effects accounted for (100%); zero blind duplicate sends; unavailable journal leaves gate closed. Expected observations: Recovered effect is discovered without surviving-row scan; no new send while journal uncertain; regional external authority remains fenced.. ACCEPTED only with both lens reviews (DS, CC).
- **EVIDENCE TO RETURN:** backup-manifest.json, witness-range-export.json, recovered-effect-inventory.json, P0-10H/run-manifest.json, P0-10H/fixture-digests.json, P0-10H/evidence-manifest.sha256, P0-10H/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 90 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-7; external: YES — two independent storage domains (cloud).
- **A3 REVIEW FOCUS:** Evidence lenses: DS lens → fresh evidence-A3 session using the W04 lane playbook, CC lens → fresh evidence-A3 session using the W02 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.

## Attempt history — W06-T000

- r0: RETURNED NOT_READY: INCOMPLETE_R0_RETURN_REQUESTED_BY_OWNER, 2026-10-12T18:54:29Z. No implementation SHA/tree. Eight scoped dirty files and index preserved; no final A3 target. See RETURNS/W06-T000-ROOT-HARDENING-r0-A2_RETURN.md and DISPOSITIONS/W06-T000-ROOT-HARDENING-r0-DISPOSITION.md.
- r1: fresh bounded remainder prepared 2026-10-12T19:04:51Z at import base; exact r0 bytes recovered without editing/committing r0. Requires fixture-cwd repair, missing runbook and complete local verification. Not launched by A1; no implementation/acceptance/merge yet.

## Owner-requested documentation draft task

- **TASK_ID:** W06-DOCS-REPOSITORY-OVERVIEW
- **OBJECTIVE:** Prepare two accurate, distinct README drafts and proposed GitHub descriptions: product overview for mastercoder-p0, CI evidence explanation for mastercoder-p0-w06-ci-qualification. Explain what Mastercoder is intended to do, how its workflow operates, architecture/stack and actual implementation status. These are local reviewable drafts, not a deployment or production-readiness grant.
- **WHY IT EXISTS:** Owner finds the duplicated READMEs and descriptions misleading and insufficient; A1 must coordinate A2 instead of handing coordination back to another A1.
- **ARCHITECTURE REQUIREMENTS:** Preserve authoritative local-first review/bounded fix-publication design from01; canonical00D/49–57 precedence,00F stack and frozen architecture status. Reader-friendly descriptions do not alter contracts or gates.
- **P0 CLAIMS:** none executed or accepted; all55remainNOT_RUN.
- **INPUT CONTRACTS:** existing normative architecture and actual W01/W06 integration records; no new wire contract.
- **OUTPUT CONTRACTS:** root README draft, distinct qualification README draft, two suggested GitHub descriptions, source/status map.
- **EXACT WRITE SCOPE:** README.md; ws06-platform-oss-qualification/docs/README_CI_QUALIFICATION_DRAFT.md; ws06-platform-oss-qualification/docs/REPOSITORY_DESCRIPTIONS_DRAFT.md; ws06-platform-oss-qualification/docs/REPOSITORY_OVERVIEW_SOURCES.md. Plus only the named absolute A2 RETURN.
- **READ-ONLY DEPENDENCIES:** ARCHITECTURE/V2_1/**; live absolute coordination/memory/task/reviews; retained remoteCI proof; existing lane source/manifests/workflows; original qualification source atb780a3f4fa4f550b76a79bc9483c1f94cd7f2d03.
- **PRECONDITIONS:** G0/G1 build records present; W01-T000/W06-T000 integrated; fresh task worktree at9a7922bf29c0a6689903b96896dab8851d1eeeeb; no other W06 A2 overlaps this draft scope.
- **IMPLEMENTATION STEPS:** Read authoritative sources/current state; draft main README with product overview/workflow/planned capabilities/architecture/stack/repo map/current status/useful verified developer commands/contribution pointers; draft qualification README with original source, rationale, one positive/nine deliberate rejecting cases and real CI/PR links; propose distinct short GitHub descriptions; provide source map separating design claims from implemented facts. Main product purpose must come before internal-program language. Preserve required prototype/qualification status in an appropriately scoped current-status section/footer, without calling the future product inherently disposable. Do not assert production readiness or features currently absent. Fix inherited README references to package-only paths that do not exist in the published repo. Do not place local private paths or credentials in proposed published README text.
- **SECURITY INVARIANTS:** No false production/P0 claim, copied evidence scope expansion, authority change, credential exposure, or fixture failure called a product defect. Architectural trust boundaries are design targets unless implemented evidence supports them.
- **FAILURE CASES:** Two identical READMEs; planned runner/API/UI described as already runnable; nonexistent rootCargo/startup command; placeholder check claimed full product test; changing normative architecture/gates; modifying existing qualifying main/tree history.
- **TESTS REQUIRED:** Documentation-only static review: inspect every source/status claim and relative link/command against actual files; git diff --check; exact changed-path verification. No build, package install, new test suite or broad scan is needed for reversible Markdown drafts; any executable validator/scan must use shared wrapper. Existing code/test results are cited at their actual scope, not rerun or invented.
- **NEGATIVE TESTS:** Manually reject unsupported production-ready/end-to-end claims, broken repo links, machine-specific/private paths or instructions implying all lane workspaces already exist. No implementation tests introduced.
- **ACCEPTANCE CRITERIA:** Two different readable drafts; main product purpose/how-it-works understandable to a newcomer; actual vs planned capabilities explicit; qualification controls and preservation explained; source map and suggested descriptions complete; exactly authorized draft paths changed. DRAFT_READY is not code acceptance/integration.
- **EVIDENCE TO RETURN:** Actual HEAD/tree/dirty draft paths; hashes for each draft; source map, link/command review and diff-check results; proposed descriptions; explicit publishing blockers.
- **OUT-OF-SCOPE:** Code/CI/architecture/license/gates, repository deletion/archive/renaming/settings changes, pushes/PRs/merges or root doc scope registration. No .env.local or credentials needed.
- **BLOCKERS:** Publication has not been authorized by this question/complaint. This new task is not in protected-main WRITE_SCOPES.json, so signed implementation commits/pushes cannot be represented as ready: leave drafts uncommitted, do not stage/bypass hooks. A1 will process the concrete output and arrange publication authorization/scope registration separately if requested.
- **A3 REVIEW FOCUS:** Future documentation review: trace planned capabilities to authoritative architecture, verify current implementation caveats, distinguish CI test repo from canonical product and preserve exact qualifying source history. No A3 is dispatched by this draft task.
