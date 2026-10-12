# AGENTS.md — WORKSTREAM-01-CORE-SECURITY-AUTHORITY (Core Security Authority (HARD AUTHORITY))

Operator: **master** (one human runs all six lanes). Lane folder: `LANE_01_CORE_SECURITY_AUTHORITY/`. Shared local workspace: `WORKSPACE/` (one clone `WORKSPACE/repo`, all worktrees in `WORKSPACE/worktrees/`). Repo dir: `mastercoder-p0/ws01-core-security/`. Branch prefix: `ws01/`. This lane's A2 cap: `MC_MAX_PARALLEL_A2=4` (all lanes together: 15).

**You are the HARD AUTHORITY workstream.** You own the most security-critical freeze-closure scope. Hard authority means *scope ownership*, not approval power: your code needs independent A3 review on exact SHAs, your claims need two independent lens reviews (fresh evidence-A3 sessions that never touched the claim), and you may not casually change contracts other lanes consume — all contract changes go through `COMMON/CONTRACT_CHANGE_PROTOCOL.md`. master is the single operator and program owner (G1/G3/G4 signatures, CCR adjudication): record it as `HUMAN:master (program owner)`; because there is only one human, every CCR decision first needs a fresh A3 review of the CCR from another lane's playbook, and the decision record states SINGLE_OPERATOR_DECISION.

## 0. MANDATORY HANDOFF OUTPUT RULE (read first, applies to every role)

**Owner rule — A2/A3 single code box (2026-10-13):** An A2 or A3 (including evidence A3) must put its **entire final chat response in exactly one fenced `text` code block**. Put the result/verdict, concise changes and check results, remaining blockers, absolute RETURN/REVIEW file path, WORK LOCATION and complete NEXT AGENT PROMPT inside that same block. Include HUMAN COMMANDS or QUESTION FOR USER there when required. No prose or links outside the block, and no additional or nested code fences. The human must be able to copy the whole response with one copy button. Write the full RETURN/REVIEW file first using its existing template; the chat block points to that file rather than replacing it. This rule also covers stopped, incomplete and blocked final responses. Progress commentary and the initial session-location check are separate from the final response. For A2/A3, this rule overrides any older instruction or example asking for separate handoff code blocks. A1's output format, required fields, scope, checks, gates and review independence remain unchanged.

Whenever you hand work to the next agent (A1 → A2, A2 → A1, A1 → A3, A3 → A1, A1 → successor A1, A1 → evidence A3) or to the human operator (master), the **end of your chat output** must contain, in this order:

1. A code block **WORK LOCATION**: the absolute folder/worktree the *next* agent works in, plus branch, BASE/HEAD/TREE SHAs, the absolute path of the message file you just wrote, and a ready `cd` line.
2. A code block **NEXT AGENT PROMPT**: the complete prompt to paste into a fresh session of the next agent, with every path, TASK_ID, ATTEMPT and SHA filled in. No `<placeholders>`.
3. A code block **HUMAN COMMANDS**, only when master must act (push, PR, merge, gate signature, account).

**No location, no handoff:** before you give the user any prompt, the next agent's folder must exist. If it is missing, **create it** (A1 creates A2 task worktrees with `git worktree add`; A3 creates its own detached review worktree from the CREATE line), verify HEAD + clean status, and record `STATUS` and `CREATE` in the WORK LOCATION block. If you are the receiving agent and your folder is missing, run the given CREATE command exactly, verify it, tell the user you created it, then continue. If it exists but is wrong (other HEAD/branch, dirty) or cannot be created, stop with BLOCKED + HUMAN COMMANDS; never reset, stash or delete to make it fit. If the main clone itself is missing, give the user the `git clone` command as HUMAN COMMANDS.

Also, at the **start** of every session, print your own WORK LOCATION block (`pwd`, `git rev-parse --show-toplevel`, branch, `git rev-parse HEAD`) before doing anything. If it doesn't match the message you were given, stop with BLOCKED. A handoff without blocks 1 and 2 is incomplete and will be sent back. The full rule and the who-prompts-whom table are in §4 (protocol section 0.5).

Example: A1 has just dispatched `W01-T004-CONTRACT-TYPES-CODEGEN` r0. **EXAMPLE VALUES ONLY. Never copy these SHAs or paths;** real outputs use values computed with `pwd` / `git rev-parse` (§0.3):

```
WORK LOCATION
ROLE NEXT:  A2
FOLDER:     /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security
WORKTREE:   /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0
REPO:       mastercoder-p0  (remote: git@github.com:mastercoder-org/mastercoder-p0.git)
BRANCH:     ws01/w01-t004-contract-types-codegen
BASE_SHA:   3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80
HEAD_SHA:   3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80
TREE:       NONE
MESSAGE:    /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r0-DISPATCH.md
STATUS:     CREATED BY ME (verified: HEAD=3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80, clean)
CREATE:     cd /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/repo && git fetch origin && git worktree add /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0 -b ws01/w01-t004-contract-types-codegen 3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80 && ln -s /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/LANE_01_CORE_SECURITY_AUTHORITY/.env /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security/.env.local
cd /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security
```

```
You are A2 (ephemeral implementer) for WORKSTREAM-01-CORE-SECURITY-AUTHORITY. You did not write any earlier attempt.
Work ONLY in: /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security  (branch ws01/w01-t004-contract-types-codegen, base 3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80)
If that folder does not exist, create it with: cd /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/repo && git fetch origin && git worktree add /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0 -b ws01/w01-t004-contract-types-codegen 3f9c2a7d1e4b5c6a7b8c9d0e1f2a3b4c5d6e7f80 && ln -s /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/LANE_01_CORE_SECURITY_AUTHORITY/.env /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security/.env.local (if the branch already exists elsewhere: stop BLOCKED).
First print your WORK LOCATION block and confirm it matches.
Open and follow exactly: /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r0-DISPATCH.md
Obey coordination/AGENTS.md and apply coordination/SKILLS.md (§13.2 A2 playbook, §15 checklist). Stay inside the AUTHORIZED WRITE SET.
Do not self-approve, push, or open PRs. When done or stopped, write
/Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md,
then put your entire final response, including the result, report path, WORK LOCATION and NEXT AGENT PROMPT (A1 resume), in exactly one text code block and stop.
```

Resume prompts for A1 (used by A2 and A3 in their Block 2):

```
You are A1 (coordinator) for WORKSTREAM-01-CORE-SECURITY-AUTHORITY, resuming. Work ONLY in: /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security  (COORDINATION worktree, branch ws01/coordination).
Print your WORK LOCATION block first. Re-read coordination/AGENTS.md, SKILLS.md §13.1, memory.md, task.md ledger row for W01-T004-CONTRACT-TYPES-CODEGEN.
Process: /Users/master/mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md   (or the A3_REVIEW file for a verdict)
Verify SHA/TREE/write-set, then write the next message (REVIEW_REQUEST or DISPOSITION/REPAIR) and end with the WORK LOCATION + NEXT AGENT PROMPT code blocks.
```

## 0.1 MANDATORY: ASK, DON'T LOOP

When you hit difficulty, **stop and ask the human** instead of retrying blindly, guessing or piling up changes. Looping costs time and causes silent mistakes in security-critical code.

**You MUST stop and ask when any of these happens:**
1. The same command, test or error fails **twice** with the same symptom, or **three** different fix hypotheses have failed.
2. You are about to **guess**: a requirement, architecture rule, schema field, SHA, path, API behavior, provider fact or the user's intent is unclear.
3. Two sources disagree (task.md vs architecture vs memory.md vs the human's message), or the task seems to need something outside your write set.
4. You need something you don't have: an account, credential, hardware, access, a peer's interface, a gate (G1/G2/G3) or a decision only the human can make.
5. A step would be destructive or irreversible (delete, reset, force-push, provider mutation, spending money) and the human hasn't explicitly approved it in this session.
6. A tool or environment keeps failing (install, network, permissions, auth, Docker, database) after one careful retry.
7. You notice you are going in circles, undoing your own changes, or your context is getting long and confused.

**How to ask.** Stop editing and leave the work in a safe state (commit WIP only if the dispatch allows it; otherwise leave the files untouched and describe them). Then output exactly this code block and **wait for the answer**:

```
QUESTION FOR USER
WHO / ROLE:      <agent id, e.g. W03-A2[W03-T002-...-r0]>
TASK:            <TASK_ID> <ATTEMPT>
WORK LOCATION:   <absolute folder, branch, HEAD sha>
PROBLEM:         <one or two sentences: what is blocking>
WHAT I TRIED:    1. <action> → <exact result/error (short)>
                 2. <action> → <exact result/error>
WHAT I KNOW:     <facts with file:line or command evidence>
WHAT I DON'T KNOW / WHY I CAN'T DECIDE: <the exact unknown>
OPTIONS:         A) <option> (risk: ...)   B) <option> (risk: ...)   C) <option>
MY RECOMMENDATION: <A/B/C + one-line reason>
WHAT I NEED FROM YOU: <the exact answer, file, credential setup, approval or command result>
SAFE STATE:      <what is committed/uncommitted; nothing half-applied>
```

Rules:
- Ask **one clear question** (or a short numbered list). Never ask "what should I do?" without options.
- Do **not** continue other risky work while waiting. Read-only investigation that doesn't change anything is fine.
- Never "fix" the problem by weakening a test, a security check or the write set, or by inventing a value, to avoid asking.
- When the human answers, the decision goes into memory.md (Decisions): the A1 writes it directly; an A2/A3 puts it under MEMORY_UPDATES in its RETURN/REVIEW. If it changes a task, tell the A1.
- If the human isn't available and nothing safe is left to do: return `NOT_READY` / `BLOCKED` with this block included, then stop.

## 0.2 MANDATORY: WHEN THE HUMAN REPORTS A PROBLEM (or says "just fix it")

"Just fix it" **never** means "skip the rules". Most agent damage happens here: the agent patches the symptom, weakens a check, or edits files it doesn't own. Follow these steps in order and show the result of each in your output.

| Step | Do | Output |
|---|---|---|
| 1. LOCATE | Print your WORK LOCATION block. State which role you act as. With no role given you are a **FIX agent**: you may investigate, but code changes go through a task (step 7). | WORK LOCATION block |
| 2. RESTATE | One sentence: what is broken + the exact symptom (command, error text, file:line). No error text given → reproduce it yourself (read-only) or ask for it. | Problem statement |
| 3. CHECK KNOWN | Search `KNOWN_ISSUES.md`, `TROUBLESHOOTING.md`, and memory.md (pitfalls, failed approaches) for this symptom. | Match or "new" |
| 4. CLASSIFY | Exactly one: ENV · GIT · DB · BUILD · TEST · CODE · INTERFACE · CONTRACT · SCOPE · PROVIDER · SECURITY · GATE | Class |
| 5. REPRODUCE | The smallest command that shows the problem. Capture the output. **No edits yet.** | Repro command + output |
| 6. DIAGNOSE | Root-cause hypothesis + evidence; test it cheaply. At most 3 hypotheses, then §0.1 (ask). | Root cause with evidence |
| 7. PROPOSE | Before editing anything: what changes, which files, why this is the root cause and not the symptom, risk, how it will be verified. Route by class (table below). | Fix plan |
| 8. APPLY | Only if the route allows it. Minimal change, one at a time. | Diff |
| 9. VERIFY | Re-run the repro (now fixed) + the full required commands, with outputs. | Evidence |
| 10. RECORD | A1: append to `KNOWN_ISSUES.md` (symptom → root cause → fix → prevention) and memory.md. Any other role: put the same entry in your RETURN/REVIEW (MEMORY_UPDATES) for the A1 to merge. | Entry |

**Routing (step 7):**

| Class | Who fixes / how |
|---|---|
| ENV / GIT / DB local setup, no repo file changes | You may apply after showing the exact commands. Destructive commands → ask first (§0.1) |
| CODE / TEST / BUILD inside the **current** task's write set | The current A2 fixes it as part of the task; A3 reviews it with the task |
| CODE in **already accepted/merged** code | A1 creates a FIX task `W0N-FIX-NNN` in task.md (all 20 fields, short is fine) → fresh A2 → fresh A3. Never patch accepted code ad hoc |
| INTERFACE (another workstream's double/component misbehaves) | BUG_REPORT to the owner's A1 (protocol §11.6). Keep using your pinned version. Never edit their code |
| CONTRACT (architecture wrong/ambiguous) | CCR (AGENTS.md §10). Interim: fail closed |
| SCOPE (needs a path outside your write set) | INTERFACE_REQUEST / CCR / ask the human. Never edit it yourself |
| PROVIDER / GATE (account, quota, approval, G1/G2/G3) | QUESTION FOR USER (§0.1) |
| SECURITY (secret exposed, suspicious behavior, escape) | Stop everything and follow `TROUBLESHOOTING.md` → SECURITY INCIDENT now |

**Forbidden "quick fixes".** Never do these, even if the human says "just make it work":
- Deleting, skipping (`#[ignore]`, `.skip`, `xit`) or weakening tests or assertions; lowering thresholds; editing expected values to match wrong output.
- Silencing problems: `#[allow(..)]`, `@ts-ignore`, `eslint-disable`, `unwrap_or_default()`, empty `catch`, or `let _ =` on security paths. Retry loops around failures you don't understand. Sleeps to hide races.
- Disabling or bypassing protections: CI jobs, git hooks (`--no-verify`), gitleaks, RLS, TLS verification, sandbox limits, egress denies, signature/expiry/epoch checks.
- `git reset --hard`, `git clean -fdx`, `git push --force`, rebasing or amending reviewed commits, stashing or discarding someone else's changes, `git worktree remove --force` on a dirty worktree, deleting branches.
- Editing `ARCHITECTURE/**`, another workstream's paths, root files (unless you are W06), or generated files.
- Adding or upgrading dependencies or the toolchain without listing it in the plan.
- Hardcoding secrets, tokens, SHAs, or test-only bypass flags in non-test code. "Temporarily" commenting code out.
- Changing an interface or double version that other workstreams pin.

If the human insists on one of these: decline in one line, explain the risk, offer the safe alternative, and ask (QUESTION FOR USER).

## 0.3 MANDATORY: NO HARDCODING

Never hardcode a value that differs per person, machine, environment, account or run, or that is secret. Hardcoded values break on another machine, leak data, and silently pin the prototype to one setup.

**Must come from configuration:** environment variables from `.env` → `.env.local`, read through **one config module per crate/app** that validates every value and **fails fast with a clear error** when one is missing.
- secrets, tokens, keys, passwords: never in the repo at all, not even in tests (use canaries in fixtures)
- database URLs, hosts, ports; absolute paths (`/Users/...`, `C:\...`); temp directories
- GitHub owner/org/repo names, App IDs, installation IDs, usernames, emails
- provider endpoints/base URLs, model IDs and model names, cloud accounts, regions, bucket names
- domains, origins, RP IDs; spend caps, budgets, rate limits
- timeouts, retries and limits that the architecture does **not** fix; capability-cell enablement flags

**Allowed constants**, each defined **once** with a citation comment (`// ENCODING.md §Digests`):
- normative architecture values: digest/signature domain strings, wire revision 1, 5 s dispatch / 60 s request deadlines, 5-minute challenge TTL, 24 h recovery hold, check name `Mastercoder Review`, fix-ref prefix `refs/heads/mastercoder/fix/`
- schema type names, field order and enums: **only** from the generated `mc-contracts` / generated TS types, never retyped by hand
- synthetic test fixtures and vectors under `fixtures/` or `tests/`
- non-secret local defaults documented in `.env.example`

**Rules:**
- A new config variable goes into `.env.example` (with a comment) in the **same commit**, and is listed in the RETURN.
- No silent fallback for security-relevant settings: missing means fail fast. No `if hostname == ...` or per-person branches.
- In WORK LOCATION blocks, prompts, scripts and docs, use real **computed** values (`pwd`, `git rev-parse HEAD`, `$MC_WORKSPACE_ROOT`). **Never copy the example SHAs or paths shown in AGENTS.md.**
- A3 checks every review for hardcoding: `grep -rnE '/Users/|/home/|C:\\\\|https?://|@[a-z0-9-]+\.(com|io)|:5540[0-9]|AKIA|BEGIN .*PRIVATE KEY|claude-|gpt-'` over the diff. Any hit outside fixtures, tests or the cited constants module is REPAIR_REQUIRED.

## 1. Authority and architecture precedence

- **Bundled engineering skills:** read `/SKILL.md` after G0 or `../SKILL.md` in the planning package; select relevant Matt Pocock SKILL.md files without overriding scope, gates or A3 independence.

- **Read `SKILLS.md` immediately after this file.** It contains the mandatory reasoning, testing, security, anti-hallucination and self-review procedures plus this workstream's technical skill pack. Every role applies its playbook (SKILLS.md §13).

- Authoritative architecture: `MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1` — `ARCHITECTURE/V2_1/` in the repo and `ARCHITECTURE_READONLY/` in this folder (identical, read-only; verify with `python3 COMMON/verify_architecture.py <path>`).
- REGISTRY_SHA256 `b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787` · PACKAGE_MANIFEST_SHA256 `97c604ecf6aefeec481b1bc8770a8a4c62d7643104d64bfdca4d8b5be5eb9c9d` · ZIP SHA-256 `60dd37b16eaa47c59274e371c02e10677d4c7a045a7716d8a371df0975a7b689`.
- Precedence: 00D invariants > SCHEMAS (REGISTRY.json, ENCODING.md, CONTRACT_AUTHORITY.md) > docs 49–57 > docs 01–48 > ADRs as amended. BASELINE_HISTORY is history only. Disagreement → CCR, never pick the weaker reading.
- Status you must never misstate: PRODUCTION_ARCHITECTURE_FREEZE = NOT_GRANTED; PRODUCTION_IMPLEMENTATION = NOT_AUTHORIZED; all 55 claims NOT_RUN until evidence is ACCEPTED. Everything you build is `PROTOTYPE — DISPOSABLE — NOT PRODUCTION`.
- Gates: G0 boot → G1 build authorization (prototype construction allowed) → G2 external audit PASS → G3 per-family execution authorization (claims may RUN) → G4 freeze (master). See `COMMON/PROGRAM_README.md`.

## 2. Roles (A1 → A2 → A1 → A3 → A1; there is NO A4)

| Role | Is | Does | Never |
|---|---|---|---|
| **A1** coordinator | One agent session at a time, succeeded when its context degrades | Reads AGENTS.md, memory.md, latest handoff, task.md, architecture, git state; picks the next bounded task; writes the DISPATCH; receives the RETURN; binds it to an exact SHA; writes the REVIEW_REQUEST for a fresh A3; interprets the verdict; sets ACCEPTED_SHA or dispatches a repair or marks BLOCKED; updates task.md ledger + memory.md (A1 is the **sole** memory.md writer: it merges the MEMORY_UPDATES of every RETURN/REVIEW it processes); writes DISPOSITION to the human | Implement code; approve without A3; exceed `MC_MAX_PARALLEL_A2` in-flight A2 tasks or run two in-flight tasks with overlapping write scopes (§3a); push/merge without the human |
| **A2** implementer | Fresh, ephemeral, no permanent identity | Reads AGENTS.md, task.md, memory.md, architecture, schemas, vectors, repo state; implements only the dispatched task inside the write set; adds tests incl. every listed negative test; runs required commands; commits; writes the RETURN, listing durable facts under MEMORY_UPDATES | Expand scope silently; touch other paths; edit memory.md, task.md or any coordination file except its own RETURN; self-approve; amend/force-push after RETURN; use real secrets |
| **A3** reviewer | Fresh, independent, did not write or dispatch the code | Reviews exactly one IMPLEMENTATION_SHA in its own worktree; verifies HEAD/TREE at start and end; checks correctness, architecture, security, tests, scope, contracts, failure and race behavior; returns PASS / PASS_WITH_NONBLOCKING_FINDINGS / REPAIR_REQUIRED / BLOCKED / INVALID_REVIEW_TARGET | Edit code; review its own work; review a moving SHA |

Repair loop: REPAIR_REQUIRED → A1 writes REPAIR_DISPATCH (rN+1, base = reviewed SHA, findings verbatim, narrowed write set) → fresh A2 → new SHA → **fresh** A3. After r3 still failing → DISPOSITION to the human before r4.

## 3. The loop, concretely

1. A1: `git fetch` (read-only), read the ledger and the In-flight board in task.md, pick every task that may start now under §3a (up to `MC_MAX_PARALLEL_A2` in flight). Steps 2–7 run independently for each of them.
2. A1 (working in this lane's COORDINATION worktree `<WORKSPACE>/worktrees/COORD-ws01`, branch `ws01/coordination`): `git fetch origin`; BASE_SHA = `origin/main`; for every code prerequisite check `git merge-base --is-ancestor <MERGED_SHA> <BASE_SHA>` (not an ancestor → the task is not ready). If the task worktree does not exist, create it from the shared clone (`git -C <WORKSPACE>/repo fetch origin && git -C <WORKSPACE>/repo worktree add <WORKSPACE>/worktrees/<TASK_ID>-r0 -b ws01/<task-id-lowercase> <BASE_SHA> && ln -s <LANE>/.env <WORKSPACE>/worktrees/<TASK_ID>-r0/ws01-core-security/.env.local`), verify HEAD + clean status (no location, no handoff); write `coordination/DISPATCHES/<TASK_ID>-r0-DISPATCH.md`; tell the human the A2 launch prompt (§5).
3. A2 (fresh session): implement, test, commit, write RETURN, stop.
4. A1: verify RETURN (SHA exists, tree matches, write-set check: `git diff --name-only <BASE>..<IMPL>`), write `REVIEWS/<TASK_ID>-r0-A3_REQUEST.md`; give the human the A3 launch prompt.
5. A3 (fresh session): review, write `REVIEWS/<TASK_ID>-r0-A3_REVIEW.md`, stop.
6. A1: PASS → ACCEPTED_SHA in task.md/memory.md + DISPOSITION asking the human to push/PR/merge (merge commit). REPAIR_REQUIRED → step 2 with r1. BLOCKED → DISPOSITION with exact blocker.
7. After merge, A1 records MERGED_SHA (INTEGRATED), removes that task's worktrees (§3b) and moves on. Interfaces you provide: send INTERFACE_NOTICE to consumers.

## 3a. Several A2s at once (parallel dispatch)

A1 keeps up to **`MC_MAX_PARALLEL_A2`** A2 tasks in flight (value in this lane's `.env`; default 4 for this lane). One operator runs all six lanes on one machine, so the six lane caps add up to the machine-wide budget (default 15 = W01 4 + W02 2 + W03 2 + W04 2 + W05 3 + W06 2). When a lane has no ready task, master may move its slots to another lane by editing both `.env` files (never exceed the total the machine copes with: about 2–3 GB RAM and 2 CPU cores per concurrent Rust A2; use `sccache`, §shared resources). Repair attempts count as in flight. A3 reviews do not count, but every RETURN gets its own fresh A3.

A task may start in parallel only if **all** of these hold:
1. Its PRECONDITIONS hold now: every named task is **INTEGRATED** (merged; its MERGED_SHA is an ancestor of the BASE_SHA you dispatch from). ACCEPTED alone is not enough, because the new worktree starts from `origin/main` and would not contain the code (audit F-10). A published double counts only once the double's own task is INTEGRATED.
2. Its EXACT WRITE SCOPE does not overlap the scope of any in-flight task (same crate, app or directory = overlap). Overlap → it waits.
3. At most one in-flight task edits workspace-level files (`Cargo.toml` at the workspace root, `compose.yaml`, `.env.example`, `package.json`, `pnpm-workspace.yaml`, the CI workflow). Lockfiles are handled by the lockfile rule below.
4. It does not need an interface that another in-flight task is still producing.

**Dependency rule.** Only the workspace scaffold task edits `Cargo.toml`/`Cargo.lock` (or `package.json`/`pnpm-lock.yaml`). If an A2 needs a crate or package that its skeleton does not list, it does not add it: it returns BLOCKED with `DEPENDENCY_REQUEST: <crate> <exact version> <why>`. A1 then dispatches the standing task `W01-DEPENDENCY-UPDATE` (pre-registered in `coordination/WRITE_SCOPES.json`: manifests, lockfile and `docs/MODULES.md` only; branch `ws01/w01-dependency-update-rN`, N increasing per use). It runs alone, gets a fresh A3 and is merged before the blocked task is re-dispatched.

**Exact write-scope enforcement (audit F-1/F-2).** `coordination/WRITE_SCOPES.json` (generated from every task's EXACT WRITE SCOPE) is the dispatch record the server-side guard reads **from `main`**, never from the PR. A task branch `ws01/<task-id-lowercase>[-rN]` may change only its task's paths; the PR title must contain `[<TASK_ID>]`. A FIX task (protocol §11) gets its scope added to WRITE_SCOPES.json in the coordination PR that also adds it to task.md; that PR is merged before the FIX PR can pass.

The **Parallel waves** table in task.md shows which tasks become ready together. It is a guide; the four rules above decide.

**In-flight board.** A1 keeps the "In flight" table at the top of task.md current (TASK_ID, attempt, worktree, BASE_SHA, write scope, started UTC) and copies it into every A1 handoff, so a successor A1 knows every running A2. Each A2 has its own worktree and branch; the human opens one agent session per A2.

**Shared machine resources (the usual cause of "random" failures when running in parallel):**
- **Rust build output:** every worktree builds into its own `target/`. Do not set a shared `CARGO_TARGET_DIR`: cargo locks it and parallel A2s wait for each other ("Blocking waiting for file lock"). Share compiled crates instead with `sccache` (`RUSTC_WRAPPER=sccache` in every lane `.env`), which works across worktrees without a lock. After a task is INTEGRATED, master removes its worktree to free disk (each Rust worktree can take several GB).
- **PostgreSQL:** one container per lane (all on master's machine, different ports), started once from that lane's COORDINATION worktree (`docker compose up -d`; `COMPOSE_PROJECT_NAME` in `.env` makes every worktree address the same container). A2s never run `docker compose down` or `-v`. Every test creates its own uniquely named database (e.g. `#[sqlx::test]`); never a fixed database name or shared tables.
- **Ports:** a test that starts a server binds port 0 and uses the port it got. Fixed ports in `.env` are only for the long-running shared dev services.
- **Model API:** real-provider calls are RUN-only (G3) and share one token cap; parallel BUILD A2s use the fake provider.

**Merging (parallel).** PRs use merge commits and branch protection does not require "up to date", so accepted PRs with disjoint scopes merge in any order and ACCEPTED_SHA never changes. Master merges in parallel: for every ACCEPTED task the DISPOSITION gives `gh pr create … && gh pr merge <PR> --merge --auto`; GitHub merges each PR by itself the moment its required checks pass, independently of the others. A1 records each MERGED_SHA when it sees the merge (`git fetch`). No PR approval is required (one operator cannot approve their own PR); the independent review is the A3 on the exact SHA plus the required checks.

**Lockfile rule.** Task branches never change a lockfile (it is outside every non-scaffold scope, so the guard rejects it). All commands run with `--locked` / `--frozen-lockfile`; if one fails because the lockfile on `main` is stale (for example after another workstream's crates you path-depend on gained a dependency), the A2 returns BLOCKED `LOCKFILE_SYNC`. A1 dispatches the standing task `W01-LOCKFILE-SYNC` (scope: the lockfile only; branch `ws01/w01-lockfile-sync-rN`), which regenerates it from `main` (`cargo metadata --format-version 1 >/dev/null` or `pnpm install --lockfile-only`), runs the tests and commits; a fresh A3 reviews only that delta; after its merge the blocked task is re-dispatched. A merge conflict anywhere else → normal REPAIR with findings.

## 3b. Housekeeping: worktrees and Rust build output (A1, after every merge)

Every A2/A3 worktree carries its own Rust `target/` folder (often several GB). On one machine with up to 15 A2s this fills the disk within days unless the A1 cleans up. The A1 does this **every time it records a MERGED_SHA** and at the start of every A1 session:

1. `git -C <WORKSPACE>/repo worktree list` and compare with the In-flight board and the ledger.
2. For every task now **INTEGRATED**, and for every attempt or A3 review worktree whose verdict is already processed: verify it is clean and its HEAD is the recorded SHA (`git -C <path> status --porcelain` empty, `git -C <path> rev-parse HEAD`), then `git -C <WORKSPACE>/repo worktree remove <path>`. This also deletes that worktree's ignored `target/` and `node_modules/` (tested: a plain remove deletes ignored files).
3. Then `git -C <WORKSPACE>/repo worktree prune` and delete merged local task branches with `git -C <WORKSPACE>/repo branch -d <branch>` (`-d` refuses unmerged branches).
4. Never remove: a worktree of an in-flight task, a worktree with uncommitted or untracked files (stop and ask master: they may be unreturned work), the COORDINATION worktrees, or anything with `--force`, `rm -rf`, `git clean` or `branch -D`.
5. Shared caches are bounded by `.env` (`SCCACHE_CACHE_SIZE`); if the disk still fills, ask master to run `sccache --zero-stats` / clear the cache. Never delete another lane's worktrees: each lane's A1 cleans its own tasks.
6. Record what was removed in the DISPOSITION (one line per worktree), so master can see the cleanup.

## 3c. Sub-agent swarms (A1, A2, A3 and evidence A3)

Every main agent normally uses 2–3 independent sub-agents (maximum 3 concurrently) for parallelizable work. A trivial indivisible task may use fewer. Read COMMON/SUBAGENT_POLICY.md in the package or /SUBAGENT_POLICY.md after bootstrap before spawning; the same policy is repeated in the root instructions. The role itself stays responsible: it plans, gives each sub-agent a bounded job, checks every result, and alone writes the DISPATCH, RETURN or REVIEW.

**Which sub-agent model.** Check which model family you are (Claude or OpenAI) and take the values from `.env` (never hardcode a model name; §0.3):

| Work type | Claude session | OpenAI session |
|---|---|---|
| Routine: searching code, reading architecture sections, listing tests, running and triaging test output, boilerplate inside the write set, formatting, doc lookups | `MC_SUBAGENT_CLAUDE_DEFAULT` at effort `MC_SUBAGENT_CLAUDE_DEFAULT_EFFORT` (Haiku 5.5, max) | `MC_SUBAGENT_OPENAI_DEFAULT` at effort `MC_SUBAGENT_OPENAI_DEFAULT_EFFORT` (GPT-6 luna, xhigh) |
| Complex: security-critical logic, concurrency and locking, codec/crypto/signatures, state machines and protocols, multi-file design reasoning, root-causing a failure that survived two fixes | `MC_SUBAGENT_CLAUDE_COMPLEX` at effort `MC_SUBAGENT_CLAUDE_COMPLEX_EFFORT` (Sonnet 5.5, high) | `MC_SUBAGENT_OPENAI_COMPLEX` at effort `MC_SUBAGENT_OPENAI_COMPLEX_EFFORT` (GPT-6.1 sol, high) |

Resolve requested model and effort against the actual host model list. If unavailable, disclose it and obtain the owner's alternative; never silently substitute. Record the resolved exact ID/effort. If token telemetry is unavailable, proactively rotate at bounded milestones and disclose that exact usage is not observable.

**How to use a swarm effectively.**
- Split the task into independent pieces with an explicit file list each (for example: one sub-agent per test file, per module, per architecture section to read). Run independent pieces in parallel, up to `MC_SUBAGENT_MAX_PARALLEL` at once per session (15 A2s × a wide swarm will hit your agent plan's rate limit).
- Give each sub-agent exactly what it needs: the task's objective, its slice of the AUTHORIZED WRITE SET, the contract names it must use, the forbidden reads, and the expected output (diff, list, report). Never give it secrets; it inherits the `.env` read denial.
- No two sub-agents edit the same file. Sub-agents never commit, push, open PRs, edit coordination files, widen the write set, or change architecture/contracts.
- Verify, don't trust: review every sub-agent diff yourself and re-run the tests yourself before you commit or report anything.
- Replace a sub-agent before its context reaches `MC_SUBAGENT_CONTEXT_LIMIT_TOKENS` (100k): ask it for a short handover (done, remaining, files touched, open doubts) and start a fresh one with that handover.
- Independence still holds: an A3 and its sub-agents never reuse the A2's sub-agents or sessions, and the A3 alone decides the verdict. The W05 `wire-ts` sub-agents never open the Rust codec, and the W01 codec sub-agents never open `wire-ts`.

## 4. Inter-agent message protocol (full, normative)


Flow: **A1 → A2 → A1 → A3 → A1**. There is no A4. Every hop is a **file** plus a **launch prompt**. Agents never rely on chat memory; the next agent reads only the files named in the message.

## 0. Where messages live

After G0, all coordination files live in the **COORDINATION worktree** `<WORKSPACE>/worktrees/COORD-ws0N/wsNN-*/coordination/` on branch `ws0N/coordination` (never on task branches; the git hook enforces this). The A1 commits them after each event; the human merges a coordination-only PR after each accepted task (at least daily). A2 and A3 write their RETURN/REVIEW files there by absolute path. Before G0 they live in your `PERSON_*` folder.

| Message | File path (relative to `wsNN-*/coordination/`) |
|---|---|
| DISPATCH / REPAIR_DISPATCH (A1→A2) | `DISPATCHES/<TASK_ID>-<ATTEMPT>-DISPATCH.md` |
| RETURN (A2→A1) | `RETURNS/<TASK_ID>-<ATTEMPT>-A2_RETURN.md` |
| REVIEW_REQUEST (A1→A3) | `REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REQUEST.md` |
| REVIEW_RESULT (A3→A1) | `REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REVIEW.md` |
| DISPOSITION (A1→Human) | `DISPOSITIONS/<YYYYMMDD>-<TASK_ID>-<ATTEMPT>.md` |
| A1_HANDOFF (A1→next A1) | `HANDOFFS/A1_HANDOFF_<NNN>.md` |
| CCR | `CONTRACT_CHANGE_REQUESTS/CCR-<WS>-<NNN>.md` |
| INTERFACE_NOTICE / INTERFACE_REQUEST | `NOTICES/<YYYYMMDD>-<I-xx>-<vN>.md` |
| EVIDENCE_REVIEW_REQUEST/RESULT | `../claims/<CLAIM_ID>/reviews/<LENS>-<NNN>.md` |

`ATTEMPT` = `r0` for the first implementation and `r1`, `r2`, … for each repair. A3 review files add a suffix `-a`, `-b` when a fresh A3 re-reviews the same SHA after an INVALID result.

Agent identity strings: `W02-A1#3` (the 3rd A1 of workstream 02), `W02-A2[W02-T030-r1]`, `W02-A3[W02-T030-r1-a]`. A2 and A3 are ephemeral and identified only by the task they serve.

## 0.5 MANDATORY handoff output rule (code blocks)

**Owner rule — A2/A3 single code box (2026-10-13):** An A2 or A3 (including evidence A3) must put its **entire final chat response in exactly one fenced `text` code block**. Put the result/verdict, concise changes and check results, remaining blockers, absolute RETURN/REVIEW file path, WORK LOCATION and complete NEXT AGENT PROMPT inside that same block. Include HUMAN COMMANDS or QUESTION FOR USER there when required. No prose or links outside the block, and no additional or nested code fences. The human must be able to copy the whole response with one copy button. Write the full RETURN/REVIEW file first using its existing template; the chat block points to that file rather than replacing it. This rule also covers stopped, incomplete and blocked final responses. Progress commentary and the initial session-location check are separate from the final response. For A2/A3, this rule overrides any older instruction or example asking for separate handoff code blocks. A1's output format, required fields, scope, checks, gates and review independence remain unchanged.

Every time an agent passes work on (A1→A2, A2→A1, A1→A3, A3→A1, A1→successor A1, A1→evidence A3, A1→human), its **final chat output must end with these blocks, in this order**. The human copies them; nothing is retyped.

**Block 1: WORK LOCATION** (where the *next* agent works), always a code block:

```
WORK LOCATION
ROLE NEXT:  A2 | A3 | A1 | A1-SUCCESSOR | EVIDENCE-A3 | HUMAN
FOLDER:     /absolute/path/the/next/agent/must/work/in      (cd target)
WORKTREE:   /absolute/worktree/path  | COORDINATION WORKTREE (A1) | REVIEW WORKTREE TO CREATE: /abs/path
REPO:       mastercoder-p0  (remote: <url>)
BRANCH:     ws0N/<task-id-lowercase>  | NONE
BASE_SHA:   <40-hex>
HEAD_SHA:   <40-hex> | NONE
TREE:       <40-hex> | NONE
MESSAGE:    /absolute/path/to/the/DISPATCH|RETURN|REVIEW|HANDOFF file just written
STATUS:     EXISTS (verified: HEAD=<40-hex>, clean) | CREATED BY ME (verified) | NEXT AGENT MUST CREATE (command below)
CREATE:     <exact command(s) that create this folder if it is missing, or "not needed">
cd /absolute/path/the/next/agent/must/work/in
```

**No location, no handoff (worktree guarantee).** The WORK LOCATION must be real before the human gets the prompt.

1. **The sending agent ensures the folder exists**, creating it if missing, then verifies it:

   | Missing thing | Who creates it | Exact command (run from the main clone `<WORKSPACE>/repo`) |
   |---|---|---|
   | Main clone `<WORKSPACE>/repo` | Human (needs auth). Agent gives it as HUMAN COMMANDS | `git clone <MC_REPO_URL> <WORKSPACE>/repo` |
   | A2 task worktree (first attempt) | A1, before writing the A2 prompt | `git fetch origin && git worktree add <WORKSPACE>/worktrees/<TASK_ID>-r0 -b ws0N/<task-id-lowercase> <BASE_SHA> && ln -s <LANE>/.env <WORKSPACE>/worktrees/<TASK_ID>-r0/wsNN-*/.env.local` |
   | A2 repair worktree (rN+1) | A1 | reuse the task worktree if it exists, is clean and HEAD = previous IMPLEMENTATION_SHA; else `git worktree add <WORKSPACE>/worktrees/<TASK_ID>-r<N+1> ws0N/<task-id-lowercase>` |
   | A3 review worktree | A3 itself (A1 puts the exact command in CREATE) | `git worktree add --detach <WORKSPACE>/worktrees/<TASK_ID>-<rN>-review-<a> <IMPLEMENTATION_SHA>` |
   | A1 COORDINATION worktree | Human at PRESTART (A1 gives the command if missing) | `git -C <WORKSPACE>/repo worktree add <WORKSPACE>/worktrees/COORD-ws0N -b ws0N/coordination origin/main && ln -s <LANE>/.env <WORKSPACE>/worktrees/COORD-ws0N/wsNN-*/.env.local` |

   Verify with `git worktree list`, `git -C <folder> rev-parse HEAD` (must equal the expected SHA) and `git -C <folder> status --porcelain` (must be empty). Record the result in the STATUS line.
2. **The receiving agent checks before doing anything.** If FOLDER is missing, it runs the CREATE command **exactly as given** (only `git fetch` / `git worktree add`, nothing else), verifies HEAD and clean status, prints its WORK LOCATION block with `STATUS: CREATED BY ME (verified)`, tells the user it created the folder, and then continues.
3. **If the folder exists but is wrong** (different HEAD, different branch, uncommitted changes) or cannot be created (no clone, auth failure, branch already checked out elsewhere), the agent **stops with BLOCKED** and a HUMAN COMMANDS block. It never resets, stashes, deletes or overwrites anything to "make it fit".
4. The A1 works only in the COORDINATION worktree. A2 and A3 never work in the main clone or the COORDINATION worktree (they only write their RETURN/REVIEW file there by absolute path). The A1 never implements in a task worktree. The main clone `<WORKSPACE>/repo` stays on `main` and is used only to fetch and create worktrees.

**Block 2: NEXT AGENT PROMPT.** The complete, ready-to-paste prompt for a fresh session of the next agent, always a code block. All absolute paths, TASK_ID, ATTEMPT and SHAs are filled in. **No `<placeholders>`** may remain. The prompt names the WORK LOCATION folder first, then the message file.

**Block 3: HUMAN COMMANDS** (only when the human must act: push, PR, merge, account, gate signature), as a code block of exact shell commands or exact actions.

Additionally, **at the start of every session**, each agent prints its own WORK LOCATION block (where *it* is working: `pwd`, `git rev-parse --show-toplevel`, branch, `git rev-parse HEAD`) before doing anything else. If the location doesn't match the message it was given, it stops with BLOCKED.

Who prompts whom:

| Finishing agent | Writes | Block 2 is the prompt for |
|---|---|---|
| A1 after DISPATCH | DISPATCHES/…-DISPATCH.md | fresh **A2** (FOLDER = the task worktree's `ws0N-*/` dir) |
| A2 after RETURN | RETURNS/…-A2_RETURN.md | **A1** resume ("process RETURN …"; FOLDER = A1 COORDINATION worktree `<WORKSPACE>/worktrees/COORD-ws0N/ws0N-*/`) |
| A1 resume prompts | — | FOLDER = `<WORKSPACE>/worktrees/COORD-ws0N/ws0N-*` |
| A1 after REVIEW_REQUEST | REVIEWS/…-A3_REQUEST.md | fresh **A3** (FOLDER = review worktree path to create at IMPLEMENTATION_SHA) |
| A3 after REVIEW_RESULT | REVIEWS/…-A3_REVIEW.md | **A1** resume ("process A3 verdict …") |
| A1 after verdict PASS | DISPOSITIONS/… | Block 3 HUMAN COMMANDS (push/PR/merge) + Block 2 for A1's next step |
| A1 after REPAIR_REQUIRED | DISPATCHES/…-r(N+1)-DISPATCH.md | fresh **A2** for the repair |
| A1 at succession | HANDOFFS/A1_HANDOFF_NNN.md | **successor A1** |
| A1 for claim evidence | claims/…/reviews/… | **evidence A3** (fresh session launched by master, reviewer lane's playbook) |
| Any agent BLOCKED | BLOCKED message | Block 3 for the human, plus Block 2 to resume once unblocked |

A handoff without Blocks 1 and 2 is incomplete. The receiving human sends it back unanswered.

## 1. Common header (every message starts with this block, in this order)

```
FROM: <agent id>
TO: <agent id | HUMAN:<name>>
MESSAGE_TYPE: DISPATCH | REPAIR_DISPATCH | RETURN | REVIEW_REQUEST | REVIEW_RESULT | DISPOSITION | A1_HANDOFF | CCR_NOTICE | INTERFACE_NOTICE | INTERFACE_REQUEST | EVIDENCE_REVIEW_REQUEST | EVIDENCE_REVIEW_RESULT | BLOCKED
TASK_ID: <e.g. W02-T030-CHECK-PROJECTION-DISPATCH>
ATTEMPT: r0 | r1 | ...
DISPOSITION: <one line, e.g. "READY_FOR_A3_REVIEW", "REPAIR_REQUIRED (2 HIGH)", "ACCEPTED", "BLOCKED: G2 not passed">
DATE_UTC: <ISO-8601>
WORKSTREAM: WORKSTREAM-0N-<NAME>
LOCATION:
  REPO: mastercoder-p0 (<remote URL>)
  WORKTREE: <absolute path of the worktree used>
  BRANCH: ws0N/<TASK_ID-lowercase-slug>[-rN]
  WORKSTREAM_DIR: ws0N-<name>/
  COORDINATION_DIR: ws0N-<name>/coordination/
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: <git sha that imported ARCHITECTURE/V2_1, from memory.md>
BASE_SHA: <40-hex commit the work starts from>
IMPLEMENTATION_SHA / REVIEWED_SHA: <40-hex | NONE>
TREE: <git rev-parse <sha>^{tree} | NONE>
SOLE_PARENT: <parent sha if single-commit delivery | MULTI | NONE>
ACCEPTED_SHA: <40-hex | NONE>
PRODUCT WRITES / NEW TESTS / SCANS: <e.g. "ws02/crates/mc-review-broker (+412/-0); 23 tests; cargo-deny+gitleaks clean" | NONE>
PUSH / PR / MERGE: <NONE | "branch pushed" | "draft PR #12" | "PR #12 merged by HUMAN:master at <sha>">
```

Rules:
- Every SHA is a full 40-hex SHA. Never write "latest", "HEAD" or a short SHA.
- `TREE` lets a reviewer prove that what they read is what was returned. Compute it with `git rev-parse <IMPLEMENTATION_SHA>^{tree}`.
- `ACCEPTED_SHA` is set only by A1, and only after an A3 verdict of PASS or PASS_WITH_NONBLOCKING_FINDINGS on that exact SHA.
- PUSH/PR/MERGE are human-authorized actions. An agent records them; it never does them unless the human said so in this session.

## 2. DISPATCH (A1 → A2)

```
<COMMON HEADER>   (IMPLEMENTATION_SHA: NONE, ACCEPTED_SHA: NONE)
OBJECTIVE: <copied verbatim from task.md, including the "In plain words" line>
READ FIRST (in order, stop if any is missing):
  1. ws0N-*/coordination/AGENTS.md
  2. ws0N-*/coordination/SKILLS.md  (§1–12, §13.2 A2 playbook, §14 workstream pack, §15 checklist)
  3. ws0N-*/coordination/task.md  §<TASK_ID>
  4. ws0N-*/coordination/memory.md
  5. ARCHITECTURE/V2_1/<docs listed in task.md ARCHITECTURE REQUIREMENTS>
  6. ARCHITECTURE/V2_1/SCHEMAS/<schemas listed>, TEST_VECTORS/<vectors listed>
AUTHORIZED WRITE SET (exact globs; anything else is out of scope):
  - ws0N-*/crates/<crate>/**
  - .github/workflows/ws0N-*.yml   (only if the task says so)
FORBIDDEN:
  - Any path outside the write set (including other workstreams' dirs and /ARCHITECTURE/**)
  - Inventing schema names, enum values, digest rules, authority rules or state transitions
  - Secrets in files, commits, logs or the return
  - git push / PR / merge / provider mutation unless the human authorized it in this session
  - Running a P0 claim (NOT_RUN→RUNNING) unless this dispatch is a RUN task and G2+G3 are recorded
INPUT CONTRACTS: <list>
OUTPUT CONTRACTS: <list>
DOUBLES / FIXTURES TO USE: <I-xx@vN paths>
STEPS: <copied verbatim from task.md IMPLEMENTATION STEPS, including the exact file list>
EXACT TESTS: <copied verbatim from task.md TESTS REQUIRED: every named test function must exist and pass>
NEGATIVE TESTS: <copied verbatim from task.md>
SECURITY INVARIANTS: <copied verbatim from task.md>
FAILURE CASES TO AVOID: <copied verbatim from task.md>
EVIDENCE TO RETURN: <copied verbatim from task.md>
OUT OF SCOPE: <copied verbatim from task.md>
Rule: A1 copies every task.md field word for word. A1 never summarises, shortens or reinterprets a task; if a field looks wrong, A1 asks the human (QUESTION FOR USER) instead of editing it.
REQUIRED COMMANDS (run all; paste exit codes in RETURN):
  - REPO_ROOT="$(git rev-parse --show-toplevel)"; then the root-safe commands from AGENTS.md §11 (each in a subshell, absolute paths, `.env.local` loaded explicitly, `--locked`)
  - <task-specific commands>
  - (cd "$REPO_ROOT" && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)   (must still print p0_executed:false)
STOP CONDITIONS (stop immediately, commit nothing further, return with READY_FOR_A3_REVIEW: NO and BLOCKERS filled):
  - A needed change falls outside the write set
  - Architecture is ambiguous or contradictory for this task → draft CCR, do not choose
  - A required double/interface is missing or behaves differently from its contract
  - A test needs a real secret, a real provider mutation, or claim execution not authorized here
  - The base SHA is not an ancestor of the branch, or the worktree is dirty with foreign changes
  - The session is running out of context before the task is complete
COMPLETION CONDITIONS (all must hold to return READY_FOR_A3_REVIEW: YES):
  - <task ACCEPTANCE CRITERIA copied verbatim>
  - All REQUIRED COMMANDS exit 0
  - Every NEGATIVE TEST listed in task.md exists and passes
  - Durable facts listed under MEMORY_UPDATES in the RETURN (facts/pitfalls only, no secrets, no chat logs). A2 never edits memory.md; the A1 is its only writer
  - Work committed on BRANCH; worktree clean; IMPLEMENTATION_SHA and TREE recorded
RETURN: write RETURNS/<TASK_ID>-<ATTEMPT>-A2_RETURN.md using COMMON/A2_RETURN_TEMPLATE.md, then stop.
SIZE RULE: this task is sized for one A2 session. There is no time limit. If it cannot be finished in this session, stop and return NOT_READY listing exactly what is done and what remains; A1 dispatches the remainder as attempt rN+1.
```

Launch prompt the human pastes into a **fresh** A2 session:

```
You are A2 (ephemeral implementer) for WORKSTREAM-0N. You did not write any earlier attempt.
Open and follow exactly: <abs path>/ws0N-*/coordination/DISPATCHES/<TASK_ID>-<ATTEMPT>-DISPATCH.md
Obey AGENTS.md and apply SKILLS.md (A2 playbook). Stay inside the AUTHORIZED WRITE SET. Do not self-approve.
When done or stopped, write the RETURN file named in the dispatch and stop.
```

## 3. RETURN (A2 → A1)

Use `COMMON/A2_RETURN_TEMPLATE.md` exactly. The DISPOSITION line is `READY_FOR_A3_REVIEW` or `NOT_READY: <reason>`.

## 4. REVIEW_REQUEST (A1 → A3)

```
<COMMON HEADER>   (REVIEWED_SHA = the A2 IMPLEMENTATION_SHA; TREE = its tree)
REVIEW TARGET (immutable):
  BASE_SHA: <sha>   IMPLEMENTATION_SHA: <sha>   TREE: <tree>
  Diff command: git diff <BASE_SHA>..<IMPLEMENTATION_SHA> -- <write set>
READ FIRST:
  1. ws0N-*/coordination/AGENTS.md (A3 section) and SKILLS.md (§13.3 A3 playbook, §14 pack, §15–17)
  2. task.md §<TASK_ID> (ACCEPTANCE CRITERIA, SECURITY INVARIANTS, NEGATIVE TESTS, A3 REVIEW FOCUS)
  3. RETURNS/<TASK_ID>-<ATTEMPT>-A2_RETURN.md
  4. Architecture docs/schemas listed in the task
AUTHORIZED WRITE SET OF THE IMPLEMENTATION: <same globs as dispatch>
REVIEW FOCUS: <copied from task.md A3 REVIEW FOCUS + any A1 concerns>
REPRODUCE (A3 must run in its own clean worktree checked out at IMPLEMENTATION_SHA):
  - git -C <review worktree> rev-parse HEAD   → must equal IMPLEMENTATION_SHA
  - <required commands>
STOP CONDITIONS:
  - HEAD or TREE differs from the request at any time → verdict INVALID_REVIEW_TARGET
  - You authored or repaired any part of this implementation → refuse (INVALID_REVIEW_TARGET, reason: not independent)
  - Required materials missing → BLOCKED
COMPLETION CONDITIONS:
  - Every acceptance criterion marked MET/NOT_MET with evidence (file:line or command output)
  - Write-set compliance checked with: git diff --name-only <BASE_SHA>..<IMPLEMENTATION_SHA>
  - Verdict chosen from: PASS | PASS_WITH_NONBLOCKING_FINDINGS | REPAIR_REQUIRED | BLOCKED | INVALID_REVIEW_TARGET
RETURN: REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REVIEW.md using COMMON/A3_REVIEW_TEMPLATE.md. Do not edit code.
```

Launch prompt for a **fresh** A3 session (never reuse the A2 or A1 session):

```
You are A3 (independent reviewer) for WORKSTREAM-0N. You did not write this code.
Open and follow: <abs path>/ws0N-*/coordination/REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REQUEST.md
Apply SKILLS.md (A3 playbook). Create your own worktree at the exact IMPLEMENTATION_SHA. Do not modify product code.
Verify HEAD and TREE before and after review. Write the A3_REVIEW file and stop.
```

## 5. REVIEW_RESULT (A3 → A1)

Use `COMMON/A3_REVIEW_TEMPLATE.md`. The result is valid only if `REVIEWED_SHA` and `TREE` match the request, were verified at start and end, and the reviewer declares independence.

## 6. REPAIR_DISPATCH (A1 → new A2)

A DISPATCH with:
- `ATTEMPT: r(N+1)`, `BASE_SHA = previous IMPLEMENTATION_SHA` (repairs stack on the reviewed commit; never rewrite it)
- `REPAIR SCOPE:` the A3 finding IDs, copied verbatim, each with its required repair
- `AUTHORIZED WRITE SET:` narrowed to the files needed for those findings
- `COMPLETION CONDITIONS:` each finding closed + no regression in previously met criteria
- A fresh A2 session, which may be different from the r(N) A2

The new IMPLEMENTATION_SHA goes to a **fresh** A3 (`-a` suffix resets per attempt). A REPAIR_REQUIRED after r3 needs a human DISPOSITION before r4.

## 7. DISPOSITION (A1 → Human)

The human-facing status record. The A1 writes one after every A3 verdict, every BLOCKED, every gate event, and before any action that needs the human (push, PR, merge, provider mutation, account creation, claim RUN, CCR filing).

```
<COMMON HEADER>   (TO: HUMAN:<name>)
COMPLETED WORK:
  <what is now true, with exact SHAs, test counts, review verdicts>
PRECISE BLOCKER: <NONE | exact missing thing, who owns it, which file/gate>
PRESERVATION AND RECOVERY:
  <where evidence/returns/reviews are stored; hashes of any evidence bundle; nothing fabricated>
COMPONENT DISPOSITION:
  <per component/claim: ACCEPTED at <sha> | REPAIR_REQUIRED (findings) | HELD (reason)>
CURRENT STATE:
  <git state verified: branch, HEAD, clean/dirty; remote state if read-only checked>
DELEGATION:
  <which A2/A3 sessions ran, what each did, what A1 personally verified>
REQUESTED HUMAN DISPOSITION / NEXT GATE:
  <exact decision requested, e.g. "push branch ws02/w02-t004 and open draft PR", "sign G3 for P0-03 family", "approve CCR-W02-001 filing">
  <then the chain: A3 PASS → A1 ACCEPTED_SHA → human PR → CI on PR head → human merge (merge commit) → main CI>
HELD ITEMS: <things explicitly not inferred: no reruns, no waivers, no automatic dispatch>
```

Worked example (W02, after a successful review):

```
FROM: W02-A1#1
TO: HUMAN:master
MESSAGE_TYPE: DISPOSITION
TASK_ID: W02-T009-GATE-REDUCER
ATTEMPT: r1
DISPOSITION: ACCEPTED; READY FOR HUMAN PUSH/PR
DATE_UTC: 2026-11-04T10:12:00Z
WORKSTREAM: WORKSTREAM-02-GIT-ANALYSIS-REVIEW
LOCATION:
  REPO: mastercoder-p0 (git@github.com:<org>/mastercoder-p0.git)
  WORKTREE: /Users/the W02 lane/mc/LANE_02_GIT_ANALYSIS_REVIEW/worktrees/W02-T009-r1
  BRANCH: ws02/w02-t008-gate-reducer-r1
  WORKSTREAM_DIR: ws02-git-analysis-review/
  COORDINATION_DIR: ws02-git-analysis-review/coordination/
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 1f0c…(40-hex)
BASE_SHA: 7a91…(40-hex)
IMPLEMENTATION_SHA / REVIEWED_SHA: c44e…(40-hex)
TREE: 09bd…(40-hex)
SOLE_PARENT: 5e20…(40-hex, the r0 implementation)
ACCEPTED_SHA: c44e…(40-hex)
PRODUCT WRITES / NEW TESTS / SCANS: ws02/crates/mc-gate-reducer (+611/-14); 58 tests incl. 21 negative; clippy/deny/gitleaks clean
PUSH / PR / MERGE: NONE
COMPLETED WORK:
  r0 returned READY; A3 W02-A3[W02-T009-r0-a] REPAIR_REQUIRED (F-1 HIGH: PARTIALLY_AVAILABLE ranked above FAIL, violating doc 13 precedence).
  r1 repaired F-1 only; fresh A3 W02-A3[W02-T009-r1-a] PASS on c44e…, HEAD/TREE verified start+end.
PRECISE BLOCKER: NONE
PRESERVATION AND RECOVERY: returns/reviews committed under coordination/; no evidence bundle (BUILD task, not a claim RUN).
COMPONENT DISPOSITION: mc-gate-reducer ACCEPTED at c44e…; I-06@v1 double unchanged.
CURRENT STATE: worktree clean; branch local only; main at 7a91… (read-only fetch 10:05Z).
DELEGATION: one A2 per attempt; A1 personally re-ran `cargo test -p mc-gate-reducer` at c44e…: 58 passed.
REQUESTED HUMAN DISPOSITION / NEXT GATE:
  Push branch and open PR "[WS02][W02-T009] Pure gate reducer" → CI on PR head → CODEOWNERS approval → merge (merge commit, keeps c44e… reachable) → main CI.
HELD ITEMS: P0-07A remains NOT_RUN (needs IM-6, G2, G3). No automatic dispatch of W02-T026 (P0-07A harness) until human acknowledges.
```

## 8. A1_HANDOFF (A1 → successor A1)

Format: `COMMON/A1_SUCCESSION_PROTOCOL.md`. Launch prompt for the successor:

```
You are the new A1 (#<n+1>) for WORKSTREAM-0N. The previous A1 stopped for context limits.
Read in this order before deciding anything: AGENTS.md, SKILLS.md, memory.md, HANDOFFS/A1_HANDOFF_<NNN>.md, task.md,
the architecture files listed in the handoff, then run `git status`, `git log --oneline -20`, `git worktree list`.
Do not reset history. Confirm or correct the handoff's facts, then perform its EXACT NEXT ACTION.
```

## 9. INTERFACE_NOTICE / INTERFACE_REQUEST

```
<COMMON HEADER> (TO: all consumer A1s, TASK_ID of the providing task)
INTERFACE: I-xx@vN   PROVIDER: W0N   CHANGE: NEW | ADDITIVE_DOC | BREAKING(new vN+1)
DOUBLE PATH: ws0N-*/doubles/<name>/vN/
CONTRACTS: <registered types used>
CONSUMER ACTION REQUIRED: <none | pin vN+1 by <date>>
```
An INTERFACE_REQUEST from a consumer asks for an addition or clarification. If it would change a canonical contract, the provider answers with a CCR instead.

## 10. EVIDENCE_REVIEW_REQUEST / RESULT (claim acceptance)

```
<COMMON HEADER> (TASK_ID: W0N-RUN-<CLAIM>)
CLAIM: <P0-xxY>   LENS: SEC | CC | DS   (from claim JSON evidence_reviewer)
EVIDENCE BUNDLE: <evidence store URI>  EVIDENCE_MANIFEST_SHA256: <hex>
RUN_MANIFEST_SHA256: <hex>   EXECUTOR: <agent/human>   REPAIR AUTHORS: <list>
RESULT (reviewer fills): ACCEPTED | REJECTED ; MEASURED_OUTCOME: PASS | FAIL | INCONCLUSIVE
INDEPENDENCE DECLARATION: reviewer did not execute, author or repair this claim or its harness
```

## 11. BLOCKED

Any role, any time:
```
<COMMON HEADER> (DISPOSITION: BLOCKED: <short>)
BLOCKER: <exact>   OWNER OF UNBLOCK: <WS/human/gate>   SAFE STATE LEFT: <commits, worktree, nothing half-applied>
OPTIONS: <bounded list, no choice made on the human's behalf>
```

## 11.5 QUESTION FOR USER (ask, don't loop)

Any role, any time it is stuck: the same failure twice, three failed fixes, about to guess, conflicting sources, missing access, a destructive action, or going in circles. It stops, leaves a safe state, outputs the QUESTION FOR USER code block (problem, what was tried with exact errors, what is known/unknown, options A/B/C, recommendation, what is needed, safe state) and waits for the human's answer. The answer is recorded in memory.md. Full rule and template: every workstream's AGENTS.md §0.1. Looping, guessing or weakening a check instead of asking is a protocol violation.

## 11.6 BUG_REPORT (to another workstream) and FIX tasks

**BUG_REPORT.** When a problem is caused by another workstream's component or double (problem class INTERFACE), the A1 writes `NOTICES/<date>-BUG-<owner WS>-<NNN>.md` and the human forwards it to the owner:
```
<COMMON HEADER> (TO: W0X-A1, MESSAGE_TYPE: BUG_REPORT)
COMPONENT / INTERFACE: <crate or double + I-xx@vN + pinned SHA>
SYMPTOM: <exact error/output>      REPRO: <smallest command, fixture>
EXPECTED (per contract): <doc § / schema / interface sketch>
IMPACT: <which of our tasks are blocked; workaround in use>
```
The reporter never edits the owner's code. The owner fixes it through its own task flow and answers with an INTERFACE_NOTICE.

**FIX tasks.** A defect found in already ACCEPTED/INTEGRATED code of your own workstream becomes `W0N-FIX-<NNN>-<SLUG>` in task.md: all 20 fields (short is fine), its own ledger row, then a normal dispatch to a fresh A2 and review by a fresh A3. Ad-hoc patches outside a task are forbidden (AGENTS.md §0.2).

## 12. Global stop rule

If any message would require you to guess a SHA, invent architecture, use a secret, act outside your write set, or approve your own work: stop and send BLOCKED.


## 5. Launch prompts (paste into a fresh session; replace `<ABS>` with your absolute path)

**First A1 of this workstream (after G0):**
```
You are A1 (coordinator) #1 for WORKSTREAM-01-CORE-SECURITY-AUTHORITY. You do NOT implement code and you do NOT approve without an independent A3.
Read in order: <ABS>/mastercoder-p0/ws01-core-security/coordination/AGENTS.md, SKILLS.md (§1–12, §13.1, §14), memory.md, HANDOFFS/A1_HANDOFF_000.md, task.md,
then the architecture files in ARCHITECTURE_MAP.md (at least the "MUST READ" rows), then run git status / git log --oneline -20 / git worktree list.
Confirm G0 and G1 are recorded in memory.md (else write BLOCKED). Then perform the EXACT NEXT ACTION in A1_HANDOFF_000.md:
write a DISPATCH for every task that may start now under AGENTS.md §3a (up to MC_MAX_PARALLEL_A2 from .env), using COMMON/INTER_AGENT_MESSAGE_PROTOCOL.md §2, update the In-flight board, and give me one A2 launch prompt per dispatch.
Stop for my confirmation before any push, PR, merge, provider mutation, account action, or claim RUN.
First print your WORK LOCATION code block. End every handoff with the WORK LOCATION and NEXT AGENT PROMPT code blocks (AGENTS.md §0).
If you get stuck, do not loop: ask me with the QUESTION FOR USER code block (AGENTS.md §0.1) and wait.
```

**A2:**
```
You are A2 (ephemeral implementer) for WORKSTREAM-01-CORE-SECURITY-AUTHORITY. You did not write any earlier attempt.
Open and follow exactly: <ABS>/mastercoder-p0/ws01-core-security/coordination/DISPATCHES/<TASK_ID>-<ATTEMPT>-DISPATCH.md
Obey coordination/AGENTS.md and apply coordination/SKILLS.md (§13.2 A2 playbook, §15 checklist). Work only in the worktree named in LOCATION. Stay inside the AUTHORIZED WRITE SET.
Do not self-approve, push, or open PRs. First print your WORK LOCATION code block.
If you get stuck (same failure twice, three failed fixes, would have to guess), do not loop: ask me with the QUESTION FOR USER code block (AGENTS.md §0.1) and wait.
When done or stopped, write the RETURN file named in the dispatch, then put your entire final response, including the result, report path, WORK LOCATION and NEXT AGENT PROMPT (A1 resume), in exactly one text code block (AGENTS.md §0) and stop.
```

**A3:**
```
You are A3 (independent reviewer) for WORKSTREAM-01-CORE-SECURITY-AUTHORITY. You did not write, repair, or dispatch this code.
Open and follow: <ABS>/mastercoder-p0/ws01-core-security/coordination/REVIEWS/<TASK_ID>-<ATTEMPT>-A3_REQUEST.md
Apply coordination/SKILLS.md (§13.3 A3 playbook, §17 rubric). Create your own worktree at the exact IMPLEMENTATION_SHA (git worktree add --detach <path> <SHA>). Verify HEAD and TREE before and after.
Do not modify product code. First print your WORK LOCATION code block.
Write the A3_REVIEW file using COMMON/A3_REVIEW_TEMPLATE.md, then put your entire final response, including the result, report path, WORK LOCATION and NEXT AGENT PROMPT (A1 resume), in exactly one text code block (AGENTS.md §0) and stop.
```

**Successor A1:** see `COMMON/A1_SUCCESSION_PROTOCOL.md` (prompt in §8 of the protocol above).

**Evidence A3 (claim acceptance; a fresh session that never touched this claim, using the reviewer lane's playbook):**
```
You are an evidence A3, lens <SEC|CC|DS>, for claim <P0-xxY>. You did not execute, author or repair this claim or its harness. Apply the owner workstream's SKILLS.md §13.4.
Open <ABS>/mastercoder-p0/<owner ws>/claims/<P0-xxY>/reviews/<LENS>-<NNN>-REQUEST.md, fetch the bundle, verify evidence-manifest.sha256,
check every artifact against the claim JSON (procedure, injections, expected observations, threshold, pass/fail conditions, pinning rule).
Write EVIDENCE_REVIEW_RESULT: ACCEPTED|REJECTED and MEASURED_OUTCOME: PASS|FAIL|INCONCLUSIVE with reasons. Stop.
```

## 6. Worked example — the dispatch for this workstream's first technical task

```
FROM: W01-A1#1
TO: W01-A2[W01-T004-CONTRACT-TYPES-CODEGEN-r0]
MESSAGE_TYPE: DISPATCH
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r0
DISPOSITION: DISPATCHED
DATE_UTC: <ISO-8601>
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (<remote URL from .env MC_REPO_URL>)
  WORKTREE: <abs>/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0
  BRANCH: ws01/w01-t004-contract-types-codegen
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: ws01-core-security/coordination/
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: <from memory.md>
BASE_SHA: <origin/main 40-hex at dispatch time; every code prerequisite is INTEGRATED and `git merge-base --is-ancestor <its MERGED_SHA> <BASE_SHA>` exits 0>
IMPLEMENTATION_SHA / REVIEWED_SHA: NONE
TREE: NONE
SOLE_PARENT: NONE
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE
PUSH / PR / MERGE: NONE
OBJECTIVE: **In plain words:** Generate Rust types for all 85 data records defined in the architecture, straight from the registry file, so every lane uses exactly the same names, fields and rules. This is the first thing the other workstreams wait for: do it as early as possible. **Precisely:** Generate the `mc-contracts` crate (I-01@v1): Rust types for all 85 registered contracts (serde JSON projection, registry field order, enum code tables), generated mechanically from `REGISTRY.json` by `tools/codegen`. **This is the program's critical path**: W02, W03 and W04 start their first real task after it. Dispatch it the moment T000 is ACCEPTED.
READ FIRST (in order, stop if any is missing):
  1. ws01-core-security/coordination/AGENTS.md
  2. ws01-core-security/coordination/SKILLS.md  (§1–12, §13.2, §14, §15) and AGENTS.md §0.1 ask-don't-loop
  3. ws01-core-security/coordination/task.md  §W01-T004-CONTRACT-TYPES-CODEGEN
  4. ws01-core-security/coordination/memory.md
  5. ARCHITECTURE/V2_1/ — SCHEMAS/REGISTRY.json (exact names, order, primitives, enums, options/sets); ENCODING.md (u64/i64 as decimal strings in JSON, UUID lowercase hyphenated, bytes lowercase hex, option = 0/1-element array, sets sorted); 49 glossary (no aliases).
AUTHORIZED WRITE SET: `ws01-core-security/crates/mc-contracts/src/**`, `ws01-core-security/crates/mc-contracts/tests/**`, `ws01-core-security/tools/codegen/src/**`
FORBIDDEN: every other path (incl. ARCHITECTURE/** and other wsNN dirs); inventing schema/enum/digest/authority/state; secrets in files/commits/logs; push/PR/merge; claim execution.
INPUT CONTRACTS: REGISTRY.json (REGISTRY_SHA256 b516004c…); `*.schema.json`.
OUTPUT CONTRACTS: crate `mc-contracts`: `src/primitives.rs` (hand-written), `src/generated.rs` (generated, committed), one struct per contract, one enum per registered enum, `const TYPE_NAME`, `const FIELD_ORDER: &[&str]`, enum `code()`/`from_code()` = zero-based registry position, `REGISTRY_SHA256` constant checked at build time. **Exact public API (names and signatures are binding; do not rename):** `pub struct U64Str(pub u64)`, `pub struct I64Str(pub i64)`, `pub struct Hex<const N: usize>(pub [u8; N])`, `pub struct Bytes(pub Vec<u8>)`, `pub struct GitPath(pub Vec<u8>)`, `pub struct Opt<T>(pub Option<T>)`, `pub struct SetOf<T>(Vec<T>)` (constructor `SetOf::new(Vec<T>) -> Result<Self, SetError>` sorts? NO: rejects unsorted/duplicates), each with Serialize/Deserialize per ENCODING.md; per contract `pub struct <Name> { … }` + `impl <Name> { pub const TYPE_NAME: &'static str; pub const FIELD_ORDER: &'static [&'static str]; }`; per enum `pub enum <Enum> { … }` + `fn code(&self) -> u64` + `fn from_code(u64) -> Option<Self>`; `pub const REGISTRY_SHA256: &str`; codegen CLI `codegen --registry <path> --out <path> [--check]`.
DOUBLES / FIXTURES TO USE: `ARCHITECTURE/V2_1/SCHEMAS/**`, `ARCHITECTURE/V2_1/TEST_VECTORS/wire-vectors.json`.
STEPS: **Files to create/edit (paths under `ws01-core-security/`):** `crates/mc-contracts/src/lib.rs (owned by this task: `pub mod primitives; pub mod generated; pub use generated::*;`)`; `crates/mc-contracts/src/primitives.rs`; `crates/mc-contracts/src/generated.rs (generated)`; `crates/mc-contracts/build.rs or tests/registry_hash.rs`; `crates/mc-contracts/tests/{primitives.rs,accept_vectors.rs,reject_vectors.rs,registry_hash.rs}`; `tools/codegen/src/main.rs`. **Steps:** 1. `primitives.rs`: `U64Str`/`I64Str` (string (de)serialization; reject leading zeros, plus, exponent, overflow, JSON numbers), `Uuid` lowercase-hyphenated only, `Hex<N>` exact-length lowercase hex (digest/bytes32/bytes64), `Bytes` variable hex, `GitPath`, `Opt<T>` as `[]`/`[x]`, `SetOf<T>` validated sorted+unique on deserialize. 2. `tools/codegen`: read REGISTRY.json, emit `generated.rs` with `#[serde(deny_unknown_fields)]` on every struct; `type` and `wire_revision` validated equal to the registered name and 1; no `Default` derives. 3. `build.rs` or a test asserts SHA-256(REGISTRY.json) == REGISTRY_SHA256. 4. CI freshness test: running codegen produces byte-identical `generated.rs`. 5. Vector tests: every ACCEPT projection deserializes and re-serializes to an equal JSON value; every projection-level REJECT vector rejects.
EXACT TESTS: `cargo test -p mc-contracts`; `cargo run -p codegen -- --check`; clippy/fmt clean. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `primitives::u64_rejects_json_number`; `primitives::u64_rejects_leading_zero`; `primitives::u64_rejects_plus_and_exponent`; `primitives::u64_rejects_overflow`; `primitives::hex_rejects_uppercase`; `primitives::hex_rejects_wrong_length`; `primitives::uuid_rejects_uppercase`; `primitives::opt_rejects_two_elements`; `primitives::set_rejects_unsorted`; `primitives::set_rejects_duplicate`; `accept_vectors::all_85_projections_round_trip`; `reject_vectors::projection_level_rejects_all_reject`; `reject_vectors::unknown_field_rejected`; `reject_vectors::null_rejected`; `reject_vectors::wrong_type_rejected`; `reject_vectors::wrong_wire_revision_rejected`; `registry_hash::registry_sha256_matches`; `cargo run -p codegen -- --check` exits 0. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
NEGATIVE TESTS: Unknown field, `null`, numeric u64, leading-zero decimal, wrong `type`/`wire_revision`, unsorted/duplicate set, 2-element option, uppercase hex — each rejected by a named test.
SECURITY INVARIANTS: INV-18; no aliases (49); no defaults for missing fields; no JSON Number for u64/i64.
FAILURE CASES TO AVOID: Silent default on a missing field; numeric JSON accepted for u64; enum order drift; alias type names.
EVIDENCE TO RETURN: Test counts per vector class; codegen command; `generated.rs` SHA-256. A1 then sends INTERFACE_NOTICE I-01@v1 to W02–W06.
OUT OF SCOPE: CBOR (T005/T012); semantic authority checks.
REQUIRED COMMANDS: REPO_ROOT="$(git rev-parse --show-toplevel)" ; docker compose --env-file "$REPO_ROOT/ws01-core-security/.env.local" -f "$REPO_ROOT/ws01-core-security/compose.yaml" ps --status running   # PostgreSQL (port 55401) must be listed; if not, ask the human to start it from the COORDINATION worktree — A2/A3 never run up/down ; (set -a && . "$REPO_ROOT/ws01-core-security/.env.local" && set +a && cd "$REPO_ROOT/ws01-core-security" && cargo fmt --all --check && cargo clippy --workspace --all-targets --all-features --locked -- -D warnings && cargo test --workspace --locked) ; (cd "$REPO_ROOT" && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)   # must print p0_executed:false ; (cd "$REPO_ROOT" && gitleaks detect --no-banner)
STOP CONDITIONS: change needed outside write set; architecture ambiguous (draft CCR); double/interface missing or contradicts its contract; real secret/provider mutation/claim execution needed; base SHA not ancestor; session running out of context before completion.
COMPLETION CONDITIONS: 85 types; 85/85 ACCEPT projections round-trip; all projection-level REJECT vectors reject; regeneration byte-identical; no hand edits in `generated.rs`. + all REQUIRED COMMANDS exit 0 + every EXACT TEST and NEGATIVE TEST present and passing + durable facts listed under MEMORY_UPDATES in the RETURN (A2 never edits memory.md) + committed, clean worktree, SHA and TREE recorded.
RETURN: ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md (COMMON/A2_RETURN_TEMPLATE.md), then stop.
SIZE RULE: one A2 session, no time limit. If it cannot be finished, return NOT_READY listing what is done and what remains; A1 dispatches the remainder as r1.
```

The matching A3 REVIEW_REQUEST copies the same header with `IMPLEMENTATION_SHA`/`TREE` from the RETURN, `REVIEW FOCUS: Primitive mapping vs ENCODING.md; deny_unknown_fields everywhere; enum positions; no aliases.`, and the reproduce commands above. The DISPOSITION to master follows the worked example in §4 (section 7 of the protocol).

## 7. A1 succession

Stop at ~60% context, on a compaction notice, after ~6 cycles, or when you notice drift. Write `HANDOFFS/A1_HANDOFF_<NNN>.md` exactly per `COMMON/A1_SUCCESSION_PROTOCOL.md`, update memory.md, stop. Successors never reset or rewrite history.

## 8. Exact-SHA binding and no self-approval

- Reviews bind to one 40-hex IMPLEMENTATION_SHA + TREE. If either changes, the review is INVALID_REVIEW_TARGET.
- After a RETURN, the task branch receives **no further commits at all**. The PR head must equal ACCEPTED_SHA (or be a later attempt that was itself reviewed).
- **Coordination files live only on `ws01/coordination`**, in this lane's COORDINATION worktree (`<WORKSPACE>/worktrees/COORD-ws01/ws01-core-security/coordination/`): task.md, memory.md, DISPATCHES, RETURNS, REVIEWS, DISPOSITIONS, HANDOFFS, NOTICES, CCRs, KNOWN_ISSUES, INCIDENTS. A2 and A3 write their RETURN/REVIEW files there by absolute path; the A1 commits them after each event. Master merges a coordination-only PR (auto-merge) after each accepted task (at least daily). Task branches never contain coordination changes; the git hook enforces this.
- Merge with a **merge commit** so ACCEPTED_SHA stays reachable. No squash/rebase merges.
- No agent ever reviews or accepts its own work; A1 never acts as A3 for a task it dispatched.

## 9. Scope and write-set control

Your write set is in `WRITE_SET.md` (from `COMMON/WRITE_OWNERSHIP_MATRIX.md`). Each task narrows it further (EXACT WRITE SCOPE). Anything else: STOP. Need something from another workstream → INTERFACE_REQUEST to its A1, or use its published double. Never "quickly fix" another workstream's file.

## 10. Contract changes

Canonical schemas, invariants, state machines and vectors are read-only. If evidence shows a contract is wrong: draft `coordination/CONTRACT_CHANGE_REQUESTS/CCR-W01-<NNN>.md` (all fields in `COMMON/CONTRACT_CHANGE_PROTOCOL.md`), the current A1 triages it, notifies the affected lanes' A1s and master; security-sensitive CCRs need a fresh A3 (another lane's playbook) before amendment. Use the INTERIM SAFE BEHAVIOR (fail closed) meanwhile.

## 11. Required testing

Every A2 runs, and every A3 re-runs:
```
    REPO_ROOT="$(git rev-parse --show-toplevel)"
    docker compose --env-file "$REPO_ROOT/ws01-core-security/.env.local" -f "$REPO_ROOT/ws01-core-security/compose.yaml" ps --status running   # PostgreSQL (port 55401) must be listed; if not, ask the human to start it from the COORDINATION worktree — A2/A3 never run up/down
    (set -a && . "$REPO_ROOT/ws01-core-security/.env.local" && set +a && cd "$REPO_ROOT/ws01-core-security" && cargo fmt --all --check && cargo clippy --workspace --all-targets --all-features --locked -- -D warnings && cargo test --workspace --locked)
    (cd "$REPO_ROOT" && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)   # must print p0_executed:false
    (cd "$REPO_ROOT" && gitleaks detect --no-banner)
```
Plus the task's TESTS REQUIRED and every NEGATIVE TEST. A test that would still pass with the guarded logic removed does not count (A3 spot-checks this).

## 12. memory.md discipline

Durable facts only: accepted interpretations, task/SHA ledger, decisions, pitfalls, failed approaches, evidence pointers, blockers, pending CCRs. No secrets, no raw source, no chat logs. Keep under ~400 lines. **The A1 is the only writer** (audit F-13): A2s and A3s never edit memory.md; they list durable facts under `MEMORY_UPDATES` in their RETURN/REVIEW, and the A1 merges the accepted ones when it processes that file.

## 13. Security rules

- No secret in git, memory.md, handoffs, returns, reviews, evidence or chat. Secrets live in `.env.local` (symlink to this folder's `.env`) or a password manager. Evidence uses canaries.
- Only synthetic repositories and identities; only disposable staging accounts.
- Every new file carries `PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)`.
- Prototype signing keys (AA, gate, journal witnesses, staging CA) are STAGING keys stored outside the repo; AA key ≠ gate key ≠ witness keys; never reuse a key across roles.
- The Publication App private key is readable only by the gateway process; the planner runs with no token, no provider egress, no DB write role on plan/approval/effect rows.
- Never weaken a predicate (expiry, epoch, generation, scope, eligibility) to make a test pass — that invalidates the evidence; file BLOCKED/CCR instead.
- Independence: never open `ws05-web-approval-ux/packages/wire-ts/**`; never use `check_spec.py` output as a codec oracle.
- Hard authority = scope only. Your code still needs a fresh independent A3 PASS; your claims still need two lens reviews from other workstreams.

## 14. Git rules

- Branches: `ws01/<task-id-lowercase>[-rN]`; worktrees under `WORKSPACE/worktrees/` (shared by all lanes; task IDs are unique). One task per branch.
- Commits: `[W01][<TASK_ID>][rN] <summary>`; body states PROTOTYPE and the task; signed commits (SSH signing).
- Never force-push, rebase or amend a commit after it was named in a RETURN. Never delete others' branches.
- `git push`, PR creation and merge are done by master (or by an agent only when master explicitly says so in that session). See `GIT_AND_PR_WORKFLOW.md`.

## 15. Prohibited actions

Production implementation; running a claim before G2+G3; relabeling NOT_RUN/INCONCLUSIVE/FAIL as PASS; self-acceptance; editing `ARCHITECTURE/**`; editing other workstreams' paths; inventing contracts; adding Redis/Kafka/NATS/RabbitMQ/workflow engines; changing the frozen stack; real customer data; real secrets in any artifact; provider mutations outside disposable staging; `pull_request_target` (sole exception: W06's two `common-*` guard workflows, which never check out or execute PR content); disabling CI checks; merging without A3 PASS.

## 16. Global stop conditions

Before giving up, apply §0.1: ask the user a precise QUESTION FOR USER instead of looping. Stop and send BLOCKED if you would have to guess a SHA, invent architecture, use a secret, act outside the write set, approve your own work, run an unauthorized claim, or proceed while G0/G1 are not recorded in memory.md.
