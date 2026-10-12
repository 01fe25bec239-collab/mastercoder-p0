# TROUBLESHOOTING — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

For agents **and** humans. Agents follow AGENTS.md §0.2 (problem handling) and use this file in step 3. Humans see also HUMAN_OPERATOR_GUIDE.md. PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

## SECURITY INCIDENT (do this immediately)

Triggers: a **real** secret (token, private key, password, API key) was committed, pushed, pasted into an agent chat, logged or put into evidence. Or a laptop/VM with staging credentials is lost. Or unexpected egress or a sandbox escape is observed. Or someone else's data shows up where it shouldn't.

1. **Stop** all agent sessions of this workstream.
2. **Revoke/rotate first:** regenerate the GitHub App private key / revoke the token / revoke the API key at the provider **now**. Rotation matters more than cleaning git history.
3. **Record it within 1 hour (master):** what, where (file/commit/chat), when, already rotated (yes/no).
4. **Don't hide it:** no force-push or history rewrite on your own; master decides (with a fresh A3 from the W06 lane's playbook) whether a rewrite is needed.
5. **Record** in `coordination/INCIDENTS.md` (metadata only, never the secret): date, type, exposure window, rotation time, affected evidence.
6. **Invalidate** any claim evidence produced with the exposed credential (status → INVALIDATED).
7. **Sandbox escape / unexpected egress (W03/W04 experiments):** stop the experiment, disconnect the lab host from the network, preserve logs and pcaps, record it for master and the W03 lane owner.

## How to use this file

Find the symptom, apply the **correct fix**, and never apply the **never do** column. If nothing matches, follow AGENTS.md §0.2 (problem-handling steps). After solving anything new, add it to `KNOWN_ISSUES.md`.

## A. Environment

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| `DATABASE_URL must be set` / auth failed in a worktree | `.env.local` symlink missing in this worktree | `ln -s <LANE>/.env <worktree>/<wsdir>/.env.local` | Hardcode a URL/password in code or compose |
| `port is already allocated` on `docker compose up` | Your own old container, or another service | `docker ps`, stop your own stale container; or change your `MC_WS0N_PG_PORT` in `.env` | Edit another workstream's compose file; `docker system prune -a` |
| Wrong Rust/Node version | Not using the repo pins | `rustup show` (uses `/rust-toolchain.toml`); `node -v` vs `.nvmrc` | Change the toolchain pin (W06 owns it; 48 h notice) |
| Builds very slow in every new worktree | Each worktree has its own `target/` (needed so parallel A2s don't block each other) | Normal for the first build. Optional: install `sccache` and set `RUSTC_WRAPPER=sccache` in your shell to share compiled crates | A shared `CARGO_TARGET_DIR` (parallel builds then wait on its lock) |
| `Blocking waiting for file lock on build directory` | Several A2s share one `CARGO_TARGET_DIR` | Unset `CARGO_TARGET_DIR`; each worktree uses its own `target/` (AGENTS.md §3a) | Killing the other agent's build |
| Disk filling up | One `target/` per worktree | master removes worktrees of INTEGRATED tasks: `git -C WORKSPACE/repo worktree remove WORKSPACE/worktrees/<TASK_ID>-rN`; keep `sccache` on so rebuilding is cheap | Deleting worktrees of in-flight tasks |
| Tests fail only when another A2 is running (`duplicate key`, `database … already exists`, wrong row counts) | Tests share a fixed database or tables | Each test creates its own uniquely named database (e.g. `#[sqlx::test]`) | Running A2s one at a time to hide it |
| `address already in use` in tests | A test binds a fixed port that another A2's test also uses | Bind port 0 and read the assigned port | Changing the fixed port in `.env` |
| PostgreSQL suddenly gone for every A2 | Someone ran `docker compose down` from a worktree | Human restarts it from COORDINATION (`docker compose up -d`); A2s never run `down` | `down -v` (deletes data) |
| PR shows a conflict only in `Cargo.lock` / `pnpm-lock.yaml` | Another accepted PR merged first | A1 dispatches a `LOCKFILE_SYNC` repair attempt (AGENTS.md §3a lockfile rule) | Hand-editing the lockfile; merging main into the accepted branch without review |
| `python: command not found` | Only `python3` exists | Use `python3` | Install random Python distributions system-wide |
| Hidden files missing after unzip (`.env`, `.claude`, `.cursor`) | macOS Finder hides dotfiles | Finder: Cmd+Shift+. ; terminal: `ls -la` | Recreate them by hand |
| Hooks fail with `bad interpreter` / `\r` errors (Windows) | Working in native Windows or files with CRLF | Work inside WSL2 Ubuntu under `~/`; `.gitattributes` keeps LF. Re-clone inside WSL if the clone was made on Windows | Convert files by hand or disable hooks |
| Everything is very slow / permission or symlink errors (Windows) | Project lives under `/mnt/c/...` | Move it to the WSL home (`~/mastercoder`) and re-run `setup_workspace.py` | Run from `/mnt/c` |
| `docker: command not found` inside WSL | Docker Desktop WSL integration off | Docker Desktop → Resources → WSL integration → enable Ubuntu | Install a second Docker daemon blindly |

## B. Git and worktrees

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| `fatal: '<branch>' is already checked out at <path>` | The branch lives in another worktree | Work in that worktree (`git worktree list`), or ask the A1 | `--force`, deleting the other worktree |
| `'<path>' already exists` on `git worktree add` | Worktree created earlier | Verify `git -C <path> rev-parse HEAD` and `status --porcelain`; reuse it if correct, else ask | `rm -rf` the folder |
| Commit blocked by hook: "outside write set" | Editing a path you don't own, or on the wrong branch | Move the change to the right place: INTERFACE_REQUEST / CCR, or switch to the correct branch | `git commit --no-verify` |
| Commit blocked: "coordination files on a task branch" | Coordination edits belong on `ws0N/coordination` | Save them in the COORDINATION worktree instead | Disable the hook |
| `main` moved after your BASE_SHA | Normal | A1 decides: continue (no conflict) or new attempt with a new BASE. Never rewrite reviewed commits | Rebase/amend a returned commit |
| COORDINATION worktree is behind `main` | Other merges landed | A1: `git -C <coord worktree> merge origin/main` (merge, not rebase) | Rebase or reset the coordination branch |
| Detached HEAD in an A3 review worktree | Expected | Review only; never commit there | Commit review "fixes" |
| Merge conflict in a PR | Should be impossible with the ownership matrix | Stop; ask master (an ownership bug) | Resolve by editing another workstream's file |
| Lost work fear / laptop risk | Unpushed branches | The human pushes WIP branches to `origin` at the end of each day (no PR) | Keep weeks of work local only |

## C. Database (PostgreSQL)

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| `permission denied` / RLS returns 0 rows | Often the **intended** role/RLS behavior | Check whether the test expects denial; use the correct writer role from W01's role framework | Grant SUPERUSER/BYPASSRLS, disable RLS |
| `sqlx::query!` macro fails to compile without DB | Compile-time checked queries need DATABASE_URL | Use runtime `sqlx::query(...)` in prototypes, or run `cargo sqlx prepare` and commit `.sqlx/` | Point at another person's DB |
| Migration fails half-way | Non-transactional DDL or ordering | Fix the migration in a new file (expand/contract); reset only your own local DB (`docker compose down -v` for your ws) | Edit an already-merged migration |
| Expiry/lease tests flaky | Using wall clock / `now()` | Inject the clock; acceptance uses `clock_timestamp()` after locks | Add sleeps or widen tolerances |

## D. Build and dependencies

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| Path dependency on another workstream's crate fails / API changed | Their crate moved past your pinned version | Check NOTICES/INTERFACE_NOTICE; pin the double version; tell their A1 (BUG_REPORT) | Edit their crate; vendor a modified copy |
| `cargo deny` / audit fails | Advisory or license problem | Pick another version/crate inside the policy; note it in the plan | Add an ignore entry without the human's approval |
| Lockfile churn | Running `cargo update` broadly | Only `cargo update -p <crate>` when the task requires it | Touch another workstream's lockfile |

## E. Tests and CI

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| Flaky test | Time, ordering, shared state, real network | Find the nondeterminism; inject clock/RNG; isolate DB per test; use barriers | Retries, sleeps, `#[ignore]` |
| Passes locally, fails in CI | Env diff (DB version, locale, TZ, missing service) | Reproduce with the CI image/versions; fix the real difference | Mark the CI job optional |
| A workflow doesn't trigger | Path filters | Check `.github/workflows/ws0N-*.yml` paths | Remove path filters globally |
| Another workstream's CI is red | Not yours | Tell its owner; continue your work | "Fix" their files |

## F. Agent behavior

| Symptom | Likely cause | Correct fix |
|---|---|---|
| Agent ignores the rules / doesn't mention WORK LOCATION | Started in the wrong folder, so AGENTS.md/CLAUDE.md wasn't loaded | Restart the session in the correct folder (see AGENT_TOOLING/README.md) |
| Agent repeats itself, contradicts earlier facts | Context too long | Ask the A1 for a handoff; start a fresh session |
| Agent claims "tests pass" without output | Hallucinated or stale result | Ask for the exact command + exit code at the current SHA |
| Agent wants to push/merge/delete | Not allowed | Deny; the human does outward actions |
| Agent proposes deleting or weakening a test | Symptom-patching | Reject; require §0.2 diagnosis |
| Agent keeps trying fixes | Looping | Tell it: "Stop. Apply AGENTS.md §0.1 and §0.2." |

## G. Providers

| Symptom | Likely cause | Correct fix | Never do |
|---|---|---|---|
| GitHub 403 / rate limited | Permissions or quota | Check `x-ratelimit-*` headers and App permissions; wait or ask | Use a personal token or a real org |
| Model API 429 / spend cap reached | Quota / G1 cap | Stop; QUESTION FOR USER | Switch provider/key/model |
| Cloud permission errors | IAM | Ask the account owner | Use root credentials |

## H. Workstream-specific (Git, Analysis & Review)

| Symptom | Likely cause | Correct fix |
|---|---|---|
| Webhook signature mismatch | Body parsed/re-serialized before HMAC | HMAC the raw body bytes; constant-time compare |
| Check run not blocking merge in staging | Ruleset not pinned to the Review App, or bypass actor present | Fix the ruleset in staging config; rerun the drift check |
| GitHub 404 right after creating something | Eventual consistency | Treat as UNKNOWN; read again later; never infer absence |
| Analyzer reports 0 findings but scanned 0 files | Empty scope / wrong path | Coverage denominator check fails the unit; never count as COMPLETED |
