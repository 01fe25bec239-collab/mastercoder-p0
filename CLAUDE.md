# mastercoder-p0 — repo-wide agent entry point (auto-loaded by Codex / Cursor / Claude Code)
PROTOTYPE — DISPOSABLE — NOT PRODUCTION. Production implementation of Mastercoder is NOT AUTHORIZED.
1. Identify your workstream from your branch prefix (ws01/ … ws06/) or the human's prompt, `cd wsNN-*/`, and read
   `wsNN-*/coordination/AGENTS.md` and `wsNN-*/coordination/SKILLS.md` BEFORE doing anything.
2. Print your WORK LOCATION code block first; end every handoff with WORK LOCATION + NEXT AGENT PROMPT code blocks.
3. Only edit your workstream's paths (.github/CODEOWNERS; enforced by .githooks and the write-set-guard CI). Never edit ARCHITECTURE/**.
4. No secrets, no hardcoded configuration (AGENTS.md §0.3). Never approve your own work. No push/PR/merge unless the human says so.
5. Stuck or reported problem: AGENTS.md §0.1 (ask, don't loop) and §0.2 (problem handling, no quick fixes).
6. A1: after every merge, remove finished worktrees and their Rust target/ (wsNN-*/coordination/AGENTS.md §3b; never --force).
7. Normally use 2–3 helpers (§3c): Claude → Haiku 5.5 max / Sonnet 5.5 high for complex work; OpenAI → GPT-6 Luna xhigh / GPT-6.1 Sol high for complex work. Read /SUBAGENT_POLICY.md; verify actual host IDs/efforts with no silent substitutions. Rotate Haiku before 100k context tokens with a handoff; disclose missing telemetry. Read /SKILL.md for bundled Matt Pocock engineering skills; project authority still applies.
