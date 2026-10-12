FROM: STARTUP-A1 (acting W01-A1)
TO: W01-A3[W01-T000-WORKSPACE-SCAFFOLD-r1-a] (fresh independent session)
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W01-T000-WORKSPACE-SCAFFOLD
ATTEMPT: r1
DISPOSITION: PRELIMINARY_LOCAL_REVIEW_REQUESTED; final CI evidence pending
DATE_UTC: 2026-10-12T23:36:00.334809+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r1-review-a
  BRANCH: NONE (detached exact candidate)
  IMPLEMENTATION_BRANCH: ws01/w01-t000-workspace-scaffold-r1
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
IMPLEMENTATION_SHA / REVIEWED_SHA: d926be555e86ad2f382fa2d3f73ab6d6998c8e41
TREE: e3024c237e6fdf3762a1bd874b8e67732f4a3012
SOLE_PARENT: 30f4813838fa779ec55af45cb2892f417e4205b4
REPAIR_BASE_SHA: 30f4813838fa779ec55af45cb2892f417e4205b4
ACCEPTED_SHA: NONE
PRODUCT WRITES: A2 scaffold,162 scoped paths; A1/A3 no implementation edits
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

REVIEW PHASE: PRELIMINARY_LOCAL. Owner-approved sequence: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/BUILD_REVIEW_CI_SEQUENCE_DECISION.md. Assess local correctness and identify repairs before A1 requests separate PR authorization. No final PASS/PASS_WITH_NONBLOCKING_FINDINGS/ACCEPTED while mandatory remote CI is missing. A preliminary satisfactory assessment is not acceptance. Do not push/create PR/merge/enable auto-merge.

INDEPENDENCE: You and every helper must not have authored, repaired or dispatched this candidate. Never reuse A2's dependency_audit/ci_scaffold helpers or A1's w01_return_scope_check/w01_ci_readiness_check helpers. This A3 and any helpers must be fresh. Do not reuse earlier A2/A3 sessions, A2 r1 helpers f1_provenance/regression_inventory, or startup A1/helper sessions w01_r1_return_verify/w05_r1_return_verify. If not independent, return INVALID_REVIEW_TARGET.

WORKTREE: Already created by A1, detached at exact SHA/tree, clean, hooks enabled; lane .env.local symlink exists. Verify before reading and after all checks. CREATE not needed. Stop on any unexpected dirty state or SHA/tree mismatch; never reset/stash/amend. Only your review output at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T000-WORKSPACE-SCAFFOLD-r1-a-A3_REVIEW.md may be written in coordination. Do not edit memory/task/product files. Scratch negative/compile fixtures must run in isolated temporary copies, leaving the immutable review and A2 worktrees unchanged.

READ FIRST:
1. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md and SKILLS.md (full instructions; §13.3/14/15–17).
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/task.md §W01-T000-WORKSPACE-SCAFFOLD, memory.md, NOTICES/INSTRUCTION_AUDIT_2026-10-13.md.
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T000-WORKSPACE-SCAFFOLD-r1-DISPATCH.md.
4. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T000-WORKSPACE-SCAFFOLD-r1-A2_RETURN.md (r1 A2 report; not an A3 verdict).
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/W01-T000-PASSWORD-CHECK-CLARIFICATION.md — owner explicitly approved the concrete classification rule; original criterion preserved.
6. Task architecture in your exact candidate: 00F_TECHNOLOGY_STACK.md,07_POSTGRES_PERSISTENCE_WORKFLOW.md,36_SUPPLY_CHAIN.md, ADR-001-RUST-MODULAR-MONOLITH.md and applicable00D invariants. Root SKILL.md/SUBAGENT_POLICY.md govern optional bundled helpers, never supersede authority.
7. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T000-WORKSPACE-SCAFFOLD-r1-processing/verification.json, a1-key-test.json, artifact-manifest.json and a2-evidence/. Retained A2 artifacts are context, not a substitute for fresh checks. Bundled review template: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md.
8. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T000-WORKSPACE-SCAFFOLD-r0-a-A3_REVIEW.md and original r0 dispatch/RETURN: preserve the exact F-1/F-2 findings and independently determine whether r1 closes them.

AUTHORIZED IMPLEMENTATION SCOPE (protected-base globs):
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
DENIED QUALIFIERS:
- No additional explicit deny globs; original task empty-skeleton qualifiers still apply.

REVIEW FOCUS (task): Dependency table complete for every crate the later units name; CI permissions; no secrets; members exact.
A1 SPECIFIC FOCUS:
- F-2: Verify the package is wire-vector-table, the public library is wire_vector_table, the workspace key, local lock entry and docs ownership row agree, and cargo test -p wire-vector-table --locked and a scratch external import resolve correctly. Exact 41 member paths and external dependency versions/checksums/edges must remain unchanged.
- Review both scopes: five-file repair diff 30f4813838fa779ec55af45cb2892f417e4205b4..d926be555e86ad2f382fa2d3f73ab6d6998c8e41, and all 162 original scaffold paths 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..d926be555e86ad2f382fa2d3f73ab6d6998c8e41. The repair must not introduce source logic or broaden original scaffold qualifiers. Fresh review covers the complete candidate, not only the five lines.
- Independently verify exact41 members (21 crates / 6 doubles / 13 harnesses / 1 tool), all module declarations/import dependencies, correct migration order, no foreign members, SQL or security logic. Audit T011 signatures and later exact APIs rather than trusting a stub of the component under test.
- Reproduce the OpenAPI-only directory membership exclusion and scratch T011 compile without changing returned/reviewed code. The retained parent fixture script hardcodes original A2 paths: inspect it first and use an isolated copy; never blindly execute it against r0 or the immutable review worktree. Confirm empty fixture placeholders establish dependency closure only, not type/authority correctness.
- CI always reports, change gate handles root/workflow/architecture/unusual/deleted paths, permissions contents:read, no pull_request_target, ephemeral PostgreSQL digest/health/dynamic port. Remote Linux/image-pull/service/required-check behavior remains unverified. Reproduce local gate controls independently; don't represent them as remote CI.
- F-1: Independently verify the release-to-commit binding of actions/checkout@d23441a48e516b6c34aea4fa41551a30e30af803 # v6.1.0 from official sources. Retained checkout-provenance.json is a summary, not raw API responses; inspect and substantiate it rather than treating it as independent verification. All other workflow bytes/security/gates/services must remain unchanged. Full-SHA pinning must satisfy the W06 guard requirement; a local checker is not deployed CI.
- Reqwest0.13.5 uses rustls/json with defaults disabled; A2 reports owner approval of that feature-name substitution. Validate the pinned crate's actual feature table, technical equivalence and declared provenance; don't invent approval or silently change the task.
- Password criterion: record full recursive3matches, independently prove the two coordination files unchanged from base, assert only productmatch is `.env.example` placeholder, and inspect compose for literal secrets. The owner-approved rule is specific; do not exclude new findings globally.
- Full-history gitleaks remains nonzero for documented contract-list locations across shared historical refs. Record actual commands/conclusions, independently classify all findings with redaction and scan candidate range. No suppression/no fabricated clean history/no additional truefinding treated as baseline.

REPRODUCE (all tests/builds use the single-build wrapper):
- git -C /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r1-review-a rev-parse HEAD HEAD^ HEAD^{tree}; verify current HEAD=d926be555e86ad2f382fa2d3f73ab6d6998c8e41,TREE=e3024c237e6fdf3762a1bd874b8e67732f4a3012,parent=30f4813838fa779ec55af45cb2892f417e4205b4; clean status at start/end.
- git verify-commit d926be555e86ad2f382fa2d3f73ab6d6998c8e41; git merge-base --is-ancestor 58b6ab38b0a514d94d8851679ad66ef9bf696c9e d926be555e86ad2f382fa2d3f73ab6d6998c8e41; git diff --name-only 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..d926be555e86ad2f382fa2d3f73ab6d6998c8e41; git diff --check 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..d926be555e86ad2f382fa2d3f73ab6d6998c8e41.
- python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r1-review-a/ws01-core-security -- sh -c 'set -a; . ./.env.local; set +a; cargo fmt --all --check && cargo build --workspace --locked && cargo clippy --workspace --all-targets --all-features --locked -- -D warnings && cargo test --workspace --locked'
- Through that wrapper: cargo metadata --format-version 1 --no-deps --locked, assert41 members/exact expectedsets. Verify lockfile stays unchanged and no network paid API call.
- All task-specific local negatives/stub compile and compose config validation in isolated fixtures; use wrapper with actualcwd. No compose up/down or unrelated DB. Owner phase allows DB startup deferred until integration; empty shells have no DB-dependent tests. Report absence honestly.
- python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T000-WORKSPACE-SCAFFOLD-r1-review-a -- python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (must report p0_executed:false; not product-codec evidence).
- Wrapper root: gitleaks detect --no-banner --redact (record actual nonzero result), and gitleaks detect --no-banner --redact --log-opts=58b6ab38b0a514d94d8851679ad66ef9bf696c9e..d926be555e86ad2f382fa2d3f73ab6d6998c8e41 (candidate range). Never print secret values.
- Original git grep criterion plus approved classification; separately reproduce intended failure for a newly added productmatch in an isolated scratch copy so clarified check is not blind to new literals.
- Recheck immutable HEAD/tree/clean at end; no product commit/edits.

OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T000-WORKSPACE-SCAFFOLD-r1-a-A3_REVIEW.md, using full A3 template plus `REVIEW_PHASE: PRELIMINARY_LOCAL` and `PRELIMINARY_LOCAL_STATUS: SATISFACTORY | REPAIR_REQUIRED | BLOCKED | INVALID_REVIEW_TARGET`. Final `VERDICT` must not be PASS while PR-CI criteria are pending: use BLOCKED:FINAL_PR_CI_PENDING for otherwise satisfactory local work; use REPAIR_REQUIRED for actual local defects. List every criterion with local evidence and remote NOT_RUN. No pending mandatory evidence represented as MET.

STOP/COMPLETION: Required materials unavailable, independence/SHA/tree mismatch, scope/contract ambiguity, samefailuretwice, true secret, foreigndirt or insufficient context/disk→stop with exactreason. No securitypredicate/test weakening. Completion is an inspectable preliminary report, not acceptance. A1 later requests authorized PR, records all eight required CI checks and obtains a fresh final exact-head review before acceptance/merge. G2notpassed/G3none/allclaimsNOT_RUN/productionunauthorized;oneexecutingbuild/diskfloor20GiB.

VERBATIM TASK SPECIFICATION (unchanged; approved grep interpretation is an attached decision):

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

FINAL CHAT FORMAT: Per the live owner rule, your entire final response must be in exactly one fenced text code block: preliminary result/verdict, checks/blockers, absolute REVIEW path, WORK LOCATION and complete NEXT AGENT PROMPT for A1. Nothing outside that block; no additional or nested fences. Write the review file first.
