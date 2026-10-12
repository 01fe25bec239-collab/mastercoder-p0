# HUMAN OPERATOR GUIDE — master (WORKSTREAM-05-WEB-APPROVAL-UX, one of six lanes)

You are the **operator**, not the coder. Agents write and review code; you start sessions, answer their questions, approve outward actions (push/PR/merge, accounts, money) and escalate. Good operating stops most agent mistakes.

## 1. Daily loop (one operator, six lanes, ~15 A2s at once)
1. Keep one A1 session per lane open, each in its COORDINATION worktree (`WORKSPACE/worktrees/COORD-ws0N/<ws dir>`). For this lane: `WORKSPACE/worktrees/COORD-ws05/ws05-web-approval-ux`. Start it with `/mc-a1` (or the First/Resume A1 prompt).
2. Each A1 dispatches every ready task up to its lane cap (`MC_MAX_PARALLEL_A2` in the lane `.env`; the six caps add up to 15) and ends with WORK LOCATION + NEXT AGENT PROMPT blocks. Open a **new** session per A2 in the folder named in its WORK LOCATION and paste its prompt.
3. Go round the six A1s (W01 → W06 → W01 …). When an A2/A3 finishes, paste its A1 resume prompt into that lane's A1.
4. On A3 PASS, the A1 gives you HUMAN COMMANDS: push, `gh pr create`, `gh pr merge --merge --auto`. Run them at once; GitHub merges each PR by itself when CI is green, so many PRs merge in parallel. Merged = INTEGRATED, which unblocks the next tasks in every lane.
5. If a lane has nothing ready, move its slots to a busy lane (edit both lanes' `MC_MAX_PARALLEL_A2`), keeping the total within what the machine copes with.
6. End of day: push WIP branches to `origin` (backup, no PR); make sure every A1 committed its coordination files and the six coordination PRs are on auto-merge.

## 2. Which model / tool for which role
- **A1 and A3:** your strongest reasoning model at high effort. They make the judgment calls.
- **A2:** a mid-level model is fine because SKILLS.md gives it procedures; use a strong one for security-critical tasks.
- Prefer a **different model or tool for A3 than for A2** (e.g. A2 in Claude Code, A3 in Codex). Different models catch different mistakes.
- **Sub-agent swarms** (AGENTS.md §3c): every main agent normally uses 2–3 helpers. Claude sessions use Haiku 5.5 max for routine work and Sonnet 5.5 high for complex work; OpenAI sessions use GPT-6 Luna xhigh and GPT-6.1 Sol high. Validate host availability; rotate Haiku before 100k context tokens with a handoff and disclose missing usage telemetry. The names live in the `MC_SUBAGENT_*` values of the lane `.env`. If you hit rate limits, lower `MC_SUBAGENT_MAX_PARALLEL` before lowering the lane caps.
- **One role per session.** A new session for every A2 attempt and every A3 review. Never let an A2 session review its own work.

## 3. What to say, and what never to say

| Never say | Say instead |
|---|---|
| "Just fix it" / "make the tests pass" | `/mc-fix <what I was doing> :: <exact error>`, or "Follow AGENTS.md §0.2 for this problem: <error>" |
| "Skip the review, it's small" | "Send it to A3" (every change gets an A3) |
| "Ignore the architecture for now" | "If the architecture blocks you, write a QUESTION FOR USER or a CCR" |
| "Use my personal token / the real org" | "Use the staging credentials in .env; if missing, ask me" |
| "Force push / reset it" | "Stop and tell me the exact state; don't change history" |
| "Mark it PASS, we know it works" | Evidence decides; NOT_RUN stays NOT_RUN |
| "Do whatever you think" on an unclear request | Give the goal, the task ID and the constraint: "only inside your write set" |

Good problem report (copy and fill):
```
PROBLEM REPORT
Task / role: <TASK_ID> as <A1|A2|A3|none>
Folder: <absolute path>
What I was doing: <command or step>
Exact error / symptom:
<paste the full error>
What changed recently: <pull, new dependency, new worktree, nothing>
Follow AGENTS.md §0.2 (problem handling). Do not apply forbidden quick fixes. Propose before editing.
```

## 4. When an agent asks a QUESTION FOR USER
- Read the options and its recommendation. If you know the answer, reply with the letter and a reason. It will be recorded in memory.md.
- If it's a decision outside your workstream (contract, another workstream's interface, money, gates), ask master first (ask the owning lane's A1 for interface questions).
- If you don't know: say "keep it BLOCKED and record it". Don't guess for the agent.

## 5. When an agent says BLOCKED or REPAIR_REQUIRED
- BLOCKED: fix the listed external thing (account, access, gate), or escalate it.
- REPAIR_REQUIRED: normal. Let the A1 write the repair dispatch. After the 3rd repair round the A1 asks you; consider splitting the task or a stronger A2 model.

## 6. Quick sanity checks on agent output (30 seconds)
- WORK LOCATION block present, with full 40-character SHAs?
- A3: `HEAD_VERIFIED_AT_START` = `AT_END` = `REVIEWED_SHA`, and the independence line says YES?
- RETURN: tests listed with exit codes, WRITE_SET_CHECK PASS, SECRETS_CHECK clean?
- Any word like "should work", "probably", "temporarily disabled", "skipped"? Ask why.

## 7. Escalation ladder
Agent → this lane's A1 → the providing lane's A1 (interfaces/doubles, via INTERFACE_REQUEST) → master (contracts, gates, money, incidents). Security incidents go straight to master (TROUBLESHOOTING.md → SECURITY INCIDENT).

## 8. Housekeeping (weekly)
- `git worktree list`; remove **merged and clean** worktrees with `git worktree remove <path>` (never `--force`); then `git worktree prune`.
- Make sure memory.md is current and short (<400 lines); ask the A1 for a handoff if sessions feel confused.
- Check KNOWN_ISSUES.md for repeated problems, and fix systemic ones through a FIX task.
