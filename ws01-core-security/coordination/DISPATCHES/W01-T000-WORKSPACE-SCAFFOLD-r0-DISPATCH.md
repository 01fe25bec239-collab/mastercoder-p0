FROM: STARTUP-A1 (acting W01-A1)
TO: W01-A2[W01-T000-WORKSPACE-SCAFFOLD-r0]
MESSAGE_TYPE: DISPATCH
TASK_ID: W01-T000-WORKSPACE-SCAFFOLD
ATTEMPT: r0
DISPOSITION: DISPATCHED — prepared for a fresh A2 session; agent not launched
DATE_UTC: 2026-10-12T18:13:11Z
WORKSTREAM: W01
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r0
  BRANCH: ws01/w01-t000-workspace-scaffold-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
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

OBJECTIVE: **In plain words:** Set up the empty Rust project for the whole security core: every crate, folder, database-migration folder, docker-compose file and CI job, with no logic inside. Later tasks then only fill in their own files, so five or more A2s can work at the same time without touching the same file. **Precisely:** Create the complete `ws01-core-security/` skeleton in one go, so every later W01 unit only edits its own `src/**`, tests and SQL files and never touches `Cargo.toml`, `Cargo.lock`, `compose.yaml` or CI. That covers: the Cargo workspace; every W01 crate, double, tool and claim-harness crate as a compiling empty skeleton with its final dependency list; the migration module layout; compose; CI; and the docs index.

READ FIRST (authoritative live coordination records, absolute paths):
1. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/SKILLS.md
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/task.md
4. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/memory.md
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/STARTUP_ROUTING_2026-10-12.md
6. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/G0_PHASED_BUILD_AUTHORIZATION.json
Then read the task's architecture requirements in /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r0/ARCHITECTURE/V2_1 and root SKILL.md/SUBAGENT_POLICY.md. Follow SKILLS sections 1–12, 13.2, 14 and 15. The local task-worktree coordination snapshot was inherited from main before startup records were committed; do not mistake its old G0/G1 values for the authoritative coordinator records above. Do not copy or commit coordination changes onto this task branch.

AUTHORIZED WRITE SET — repository-relative expanded globs from the protected-base task scope:
- .github/workflows/ws01-ci.yml
- ws01-core-security/.env.example
- ws01-core-security/.gitignore
- ws01-core-security/Cargo.lock
- ws01-core-security/Cargo.toml
- ws01-core-security/README.md
- ws01-core-security/claims/*/harness/Cargo.toml
- ws01-core-security/claims/*/harness/src/main.rs
- ws01-core-security/claims/README.md
- ws01-core-security/compose.yaml
- ws01-core-security/crates/*/Cargo.toml
- ws01-core-security/crates/*/src/*.rs
- ws01-core-security/crates/*/src/lib.rs
- ws01-core-security/crates/mc-resolution/src/flows/mod.rs
- ws01-core-security/db/authority/*/.keep
- ws01-core-security/db/authority/MODULES
- ws01-core-security/docs/MODULES.md
- ws01-core-security/docs/README.md
- ws01-core-security/doubles/README.md
- ws01-core-security/doubles/custody/v1/Cargo.toml
- ws01-core-security/doubles/custody/v1/README.md
- ws01-core-security/doubles/custody/v1/src/lib.rs
- ws01-core-security/doubles/effects-journal/v1/Cargo.toml
- ws01-core-security/doubles/effects-journal/v1/README.md
- ws01-core-security/doubles/effects-journal/v1/src/lib.rs
- ws01-core-security/doubles/gate-authority/v1/Cargo.toml
- ws01-core-security/doubles/gate-authority/v1/README.md
- ws01-core-security/doubles/gate-authority/v1/src/lib.rs
- ws01-core-security/doubles/lease/v1/Cargo.toml
- ws01-core-security/doubles/lease/v1/README.md
- ws01-core-security/doubles/lease/v1/src/lib.rs
- ws01-core-security/doubles/runner-control/v1/Cargo.toml
- ws01-core-security/doubles/runner-control/v1/README.md
- ws01-core-security/doubles/runner-control/v1/src/lib.rs
- ws01-core-security/doubles/wire-vector-table/v1/Cargo.toml
- ws01-core-security/doubles/wire-vector-table/v1/README.md
- ws01-core-security/doubles/wire-vector-table/v1/src/lib.rs
- ws01-core-security/tools/codegen/Cargo.toml
- ws01-core-security/tools/codegen/src/main.rs

Scope remains constrained by the verbatim task qualifiers below (empty modules/skeleton only where stated). The only coordination write exception is this attempt's RETURN at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T000-WORKSPACE-SCAFFOLD-r0-A2_RETURN.md; do not edit memory.md, task.md, other returns or notices.

PREREQUISITES VERIFIED BY A1:
- Fetched origin/main: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e; fresh task branch exists, clean, hooks active and .env.local symlink verified.
- G0 PASS_PHASED_BUILD_STARTUP and G1 signature/digests recorded in authoritative coordination memory. No upstream INTEGRATED task is required by this task's PRECONDITIONS.
- Three disjoint tasks prepared globally, one per lane. No other task implementation worktree/in-flight task existed at dispatch.
- Owner caps: three active A2 tasks globally, one per lane; one active build; compiler jobs two, cache five G; stop new build/dispatch below twenty GiB free. Model budget one million API tokens/five USD total; cloud and security keys zero.
- Deferred resources retain their owning task preconditions. No claims have run; G2 not passed, no G3, production unauthorized.
- Prior helper attempts hit the account usage limit; verify availability and never silently substitute models. Independent work may continue without unavailable helpers.

SINGLE BUILD SLOT (mandatory for all dependency resolution/install, Rust builds/tests, frontend builds/tests and browser installs):
Use the local A1-managed wrapper, outside the product repository:
    python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd "/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r0/ws01-core-security" -- COMMAND ARGUMENTS
Replace COMMAND ARGUMENTS with the actual command, e.g. cargo build --workspace --locked or pnpm -r typecheck. The wrapper waits for the shared OS lock, checks the twenty-GiB floor, and releases the slot on exit. For grouped commands use sh -c with proper quoting. Load lane env explicitly for commands that need it without printing it. Do not run build commands around the lock or set a shared CARGO_TARGET_DIR. Do not change the wrapper or any other task's files.

VERBATIM TASK SPECIFICATION — every original field retained unchanged:

### W01-T000-WORKSPACE-SCAFFOLD
- **TASK_ID:** W01-T000-WORKSPACE-SCAFFOLD
- **OBJECTIVE:** **In plain words:** Set up the empty Rust project for the whole security core: every crate, folder, database-migration folder, docker-compose file and CI job, with no logic inside. Later tasks then only fill in their own files, so five or more A2s can work at the same time without touching the same file. **Precisely:** Create the complete `ws01-core-security/` skeleton in one go, so every later W01 unit only edits its own `src/**`, tests and SQL files and never touches `Cargo.toml`, `Cargo.lock`, `compose.yaml` or CI. That covers: the Cargo workspace; every W01 crate, double, tool and claim-harness crate as a compiling empty skeleton with its final dependency list; the migration module layout; compose; CI; and the docs index.
- **WHY IT EXISTS:** Parallel A2s conflict only on shared files. When the scaffold owns all of them once, the other 51 units have disjoint write scopes and can run 5+ at a time (AGENTS.md §3a). ADR-001 modular monolith.
- **ARCHITECTURE REQUIREMENTS:** 00F frozen stack (Rust, Axum, Tokio, SQLx, Serde; PostgreSQL); ADR-001; 36 pinned lockfiles; 07 (one `mc_authority` schema, module-owned tables).
- **P0 CLAIMS:** none directly; enables all 13 W01 claims.
- **INPUT CONTRACTS:** I-14 test DB harness (W06); if not yet published, the PostgreSQL image is pinned by digest (recorded in memory.md) and switched to I-14 when it lands.
- **OUTPUT CONTRACTS:** Workspace layout below; `docs/MODULES.md` (crate → owning task table, copied from this task); `db/authority/MODULES` (ordered module list).
- **EXACT WRITE SCOPE:** `ws01-core-security/{Cargo.toml,Cargo.lock,compose.yaml,.env.example,README.md,.gitignore}`, `ws01-core-security/docs/{MODULES.md,README.md}`, `ws01-core-security/crates/*/{Cargo.toml,src/lib.rs}`, `ws01-core-security/crates/*/src/*.rs` (empty declared modules only), `ws01-core-security/crates/mc-resolution/src/flows/mod.rs` (empty directory module only), `ws01-core-security/doubles/{wire-vector-table,lease,effects-journal,gate-authority,custody,runner-control}/v1/{Cargo.toml,src/lib.rs,README.md}`, `ws01-core-security/doubles/README.md`, `ws01-core-security/claims/*/harness/{Cargo.toml,src/main.rs}`, `ws01-core-security/claims/README.md`, `ws01-core-security/tools/codegen/{Cargo.toml,src/main.rs}`, `ws01-core-security/db/authority/{MODULES,*/.keep}`, `.github/workflows/ws01-ci.yml`
- **READ-ONLY DEPENDENCIES:** `/rust-toolchain.toml`, `/deny.toml` (W06), `ARCHITECTURE/V2_1/**`.
- **PRECONDITIONS:** G0 (repo exists, ARCH_IMPORT_COMMIT in memory.md); G1 signed.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws01-core-security/`):** `Cargo.toml`; `Cargo.lock`; `compose.yaml`; `.env.example`; `README.md`; `.gitignore`; `docs/README.md`; `docs/MODULES.md`; `db/authority/MODULES`; `db/authority/{roles,base,lease,effects,journal,ingestion,gate,custody,approval,credential,resolution,publication,bootstrap}/.keep`; `crates/<21 crates>/{Cargo.toml,src/lib.rs,src/<declared module>.rs}`; `crates/mc-resolution/src/flows/mod.rs (empty directory module)`; `doubles/<6 doubles>/v1/{Cargo.toml,src/lib.rs,README.md}`; `doubles/README.md`; `claims/<13 claims>/harness/{Cargo.toml,src/main.rs}`; `claims/README.md`; `tools/codegen/{Cargo.toml,src/main.rs}`; `../.github/workflows/ws01-ci.yml`. **Steps:** 1. `Cargo.toml`: virtual workspace, `resolver = "2"`, `members = ["crates/*", "doubles/wire-vector-table/v1", "doubles/lease/v1", "doubles/effects-journal/v1", "doubles/gate-authority/v1", "doubles/custody/v1", "doubles/runner-control/v1", "claims/*/harness", "tools/*"]` — the six Rust doubles are listed by name because `doubles/aa-api/v1` and `doubles/resolution-api/v1` are OpenAPI-only directories with no Cargo.toml (audit F-3); `[workspace.dependencies]` pins exact versions (`=x.y.z`, latest stable on crates.io on the scaffold day, recorded in `docs/MODULES.md`) for: tokio (rt-multi-thread, macros, time, sync, net), axum, sqlx (postgres, runtime-tokio, tls-rustls, uuid, macros, migrate), serde, serde_json, uuid (v4, serde), sha2, hex, ed25519-dalek, p256 (ecdsa), base64, unicode-normalization, thiserror, tracing, tracing-subscriber, rustls, tokio-rustls, rcgen (dev), reqwest (rustls-tls, json, no default features), clap (derive), tempfile, proptest, async-trait. 2. Create these crates under `crates/`, each with `src/lib.rs` containing only `//! <one-line purpose> — PROTOTYPE — DISPOSABLE — NOT PRODUCTION` and the dependencies listed: mc-contracts (serde, serde_json, uuid, hex, sha2, thiserror); mc-cbor (thiserror, unicode-normalization); mc-wire (mc-contracts, mc-cbor, sha2, ed25519-dalek, uuid, thiserror, clap); mc-test-support (sqlx, tokio, uuid, ed25519-dalek, p256, serde_json); mc-authority-db (sqlx, tokio, uuid, thiserror, mc-contracts); mc-lease, mc-effects, mc-journal, mc-ingestion, mc-gate-authority, mc-custody, mc-approval-authority, mc-credential-lifecycle, mc-resolution, mc-plan-verifier, mc-publication-planner, mc-git-gateway, mc-bootstrap (each: mc-contracts, mc-wire, mc-authority-db, sqlx, tokio, uuid, thiserror, async-trait, tracing); add mc-lease to mc-effects, mc-ingestion and mc-gate-authority (their APIs name `LeaseRef`/`LeaseAuthority`); add mc-effects to mc-custody (`Arc<dyn EffectAuthority>`); mc-lease re-exports `pub use sqlx::PgConnection;` so every consumer names the same connection type; mc-negotiation (mc-contracts, mc-wire, thiserror); mc-runner-control (mc-lease, mc-negotiation, axum, tokio-rustls, rustls); mc-webauthn-verify (p256, sha2, base64, serde_json, thiserror); add `reqwest` to mc-git-gateway; add mc-journal to mc-effects; add mc-custody to mc-approval-authority and mc-plan-verifier; add mc-webauthn-verify to mc-approval-authority, mc-credential-lifecycle and mc-resolution; add mc-effects to mc-resolution, mc-git-gateway and mc-bootstrap; add mc-journal to mc-bootstrap; add mc-plan-verifier to mc-publication-planner and mc-git-gateway; add mc-approval-authority to mc-git-gateway; add mc-credential-lifecycle to mc-approval-authority and mc-bootstrap; add mc-ingestion to mc-gate-authority. Every crate except mc-contracts, mc-cbor, mc-authority-db and mc-test-support gets `mc-test-support` as a dev-dependency; every crate gets `proptest`; mc-runner-control also gets `rcgen` (dev). mc-test-support depends on mc-authority-db as an optional dependency behind feature `authority`. **Module declarations:** `src/lib.rs` declares every module that later tasks fill, each created as an empty file with a one-line doc comment, so no later task edits `lib.rs`: mc-custody (api, manifest, store, pins, snapshot, reader, retention, service); mc-approval-authority (api, prepare, display, challenge, http_prepare, finish, consume, anomaly, http_finish); mc-effects (api, store, dispatch, authority); mc-resolution (api, case, action, challenge, commit, http, flows — `flows` is a directory module `src/flows/mod.rs`); mc-credential-lifecycle (api, chain, registry, operations, recovery, break_glass); mc-bootstrap (api, fence, deny, bootstrap, unfreeze); mc-git-gateway (api, gateway, reconcile); mc-lease, mc-journal, mc-ingestion, mc-gate-authority, mc-plan-verifier, mc-publication-planner, mc-negotiation (api only; their single implementing task adds further modules). 3. Doubles under `doubles/<name>/v1/` (lib crates), each with tokio, serde_json and async-trait plus every crate whose interface it fakes: wire-vector-table (mc-wire), lease (mc-lease, sqlx), effects-journal (mc-effects, mc-journal), gate-authority (mc-ingestion, mc-gate-authority, mc-wire, ed25519-dalek), custody (mc-custody), runner-control (mc-lease, mc-negotiation, axum). The W02 reducer crate/double these later call are added by W01-T052, not here (W02's scaffold does not exist yet). 4. Claim harness binaries `claims/<claim>/harness/` for P0-06C, P0-06D, P0-07B, P0-08A, P0-09B, P0-09C, P0-09D, P0-09F, P0-09G, P0-09H, P0-10A, P0-10I, P0-10J: `src/main.rs` with `fn main() {}`, dependencies = the crates named in that claim's harness task plus clap, serde_json, tokio. 5. `tools/codegen` binary (serde_json, clap). 6. `db/authority/MODULES` lists migration modules in apply order, one per line: roles, base, lease, effects, journal, ingestion, gate, custody, approval, credential, resolution, publication, bootstrap; create each directory with `.keep`. Each module's migrations are `db/authority/<module>/NNNN_<name>.sql`, numbered inside the module only, so parallel units never collide. 7. `compose.yaml`: one PostgreSQL service, image pinned by digest, port from `${MC_WS01_PG_PORT}`, password from `${POSTGRES_PASSWORD}` (no default in git). 8. `.github/workflows/ws01-ci.yml` (job `ws01-ci`; change gate over `ws01-core-security/**` + itself); `permissions: contents: read`; jobs `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` with a PostgreSQL service container, `python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py`; no `pull_request_target`. 9. `docs/MODULES.md`: the crate/double/harness → owning task table (from this task list), a **crate → imports table** built by reading every W01 task's "Exact public API" and READ-ONLY DEPENDENCIES in task.md (each type a crate's public API names must come from a listed dependency; audit F-6), and the rule "only the scaffold edits Cargo.toml/Cargo.lock; a unit that needs a new crate returns BLOCKED with DEPENDENCY_REQUEST". 10. **Stub compile check:** in a scratch branch that is not committed, paste the W01-T011 signatures into the api.rs files and run `cargo check --workspace`; every missing dependency it reveals is added to this scaffold before RETURN (record the result in the RETURN). CI runs every cargo command with `--locked`. **Required-check rule (one operator, parallel auto-merge):** the workflow is named and has one job named `ws01-ci` (a required check on main), triggers on **every** pull request to main and every push to main (no workflow-level `paths:` filter, so it always reports), and its first step decides with `git diff --name-only <merge-base> HEAD` whether this lane's paths or the upstream paths listed here changed; if not, the remaining steps are skipped by `if:` and the job still reports success. Without this, auto-merge could merge a PR whose lane tests never ran.
- **SECURITY INVARIANTS:** No secrets in compose/env example (placeholders only); CI token read-only; no `pull_request_target`; no crate contains logic. No workspace-level `rust-toolchain.toml`: the repository root pin (W06-T000) is the single toolchain source (rustup finds it from every workspace).
- **FAILURE CASES:** A skeleton crate missing a dependency that a later unit needs (that unit returns DEPENDENCY_REQUEST); a documentation-only directory matched as a Cargo member; CI triggering on other workstreams' paths; compose embedding a password.
- **TESTS REQUIRED:** `cargo build --workspace` and `cargo test --workspace` green on the empty skeleton; CI green; `cargo metadata --format-version 1` lists exactly the members named in steps 2–5; after creating empty `doubles/aa-api/v1/` and `doubles/resolution-api/v1/` directories (as W01-T001/T002 will), `cargo metadata` still succeeds with the same members; the step-10 stub compile check passes. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `cargo build --workspace` exits 0; `cargo test --workspace` exits 0 (no tests yet); `cargo metadata --format-version 1 --no-deps | python3 -c` prints exactly 41 members (21 crates + 6 doubles + 13 harnesses + 1 tool); `docker compose config` shows no literal password; `with empty `doubles/aa-api/v1/` and `doubles/resolution-api/v1/` directories present, `cargo metadata` still lists the same 41 members`; `stub compile check: the W01-T011 signatures pasted into api.rs compile with `cargo check --workspace --locked` (scratch, not committed)`; `CI run green on the PR`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** A deliberately misformatted scratch file makes CI fail (shown in the return, then deleted); `git grep -n "POSTGRES_PASSWORD=" ws01-core-security` finds only the placeholder in `.env.example`.
- **ACCEPTANCE CRITERIA:** All 21 crates, 6 doubles, 13 harnesses and 1 tool exist and compile; no foreign members; module directories exist in the listed order; CI job `ws01-ci` always reports and skips heavy steps when nothing relevant changedly ws01 + itself.
- **EVIDENCE TO RETURN:** CI run URL; `cargo metadata` member list; pinned version table.
- **OUT-OF-SCOPE:** Any logic, any SQL table, any test beyond compile.
- **BLOCKERS:** G0/G1 not done → BLOCKED.
- **A3 REVIEW FOCUS:** Dependency table complete for every crate the later units name; CI permissions; no secrets; members exact.


A1 APPLICATION NOTES (do not replace the task's exact acceptance criteria):
- All task TESTS REQUIRED, NEGATIVE TESTS and acceptance criteria above remain mandatory. Keep a criterion-to-evidence checklist.
- The owner-approved phase defers database startup until the owning scaffold is integrated. Do not run compose up/down or borrow existing unrelated databases. Report database-dependent checks as pending when no genuine database exists; never fabricate a pass.
- /deny.toml is W06-T007's future deliverable, not W06-T000's. Do not fabricate it or edit outside this task scope. If a required command actually needs an absent dependency, record the exact blocker in RETURN.
- A task requiring an unpublished shared interface uses only its explicitly allowed fallback. W01-T000 may use the PostgreSQL digest recorded in authoritative W01 memory until I-14 lands.
- W06-T000 is a root-file task, before W06-T001 creates a Cargo workspace. Record any inapplicable/unavailable generic workspace command accurately; do not invent a manifest or run against another task's workspace. No unavailable command may be represented as passed.
- The canonical verifier source named COMMON/verify_architecture.py by the task is available read-only at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/verify_architecture.py; the product repository has no COMMON directory.
- All new files carry the prototype marking in syntax appropriate for that file format. Do not add invalid comments to JSON or lockfiles merely to satisfy a generic marking rule.
- Verify exact current dependency versions from official registries/documentation when implementing; do not guess versions. Build tasks do not call paid model APIs.
- Remote scratch PRs and PR CI remain required where specified. This dispatch authorizes local implementation/tests/commits only. Do not push, open PRs or mutate provider settings. If remote acceptance evidence is still pending, return NOT_READY with that precise blocker so A1 can arrange the outward step with master. Never invent a green CI URL or treat local tests as remote CI.
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
All 21 crates, 6 doubles, 13 harnesses and 1 tool exist and compile; no foreign members; module directories exist in the listed order; CI job `ws01-ci` always reports and skips heavy steps when nothing relevant changedly ws01 + itself.
All required applicable commands/test evidence and negative cases must be accounted for. READY_FOR_A3_REVIEW requires all mandatory acceptance evidence, clean signed committed product changes, exact IMPLEMENTATION_SHA/TREE, scope compliance, and no true new secret. Pending mandatory remote CI/resource evidence means NOT_READY with explicit blockers, not an invented PASS. A1 alone records acceptance after fresh independent A3 review.

RETURN:
Write /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T000-WORKSPACE-SCAFFOLD-r0-A2_RETURN.md using the A2 RETURN TEMPLATE below. List durable facts under MEMORY_UPDATES; never edit memory.md. Do not commit the RETURN in the task worktree. End with a one-line absolute coordinator worktree path and an A1 resume prompt pointing to that RETURN, then stop. Do not self-approve, push, open PRs, merge, amend a commit after RETURN, or continue to another task.

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
