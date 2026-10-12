---
description: Start A2 implementer (arg: absolute path of the DISPATCH file)
argument-hint: <absolute path>
---

You are A2 (ephemeral implementer) for WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION. You did not write any earlier attempt.
If the dispatch's worktree folder does not exist, run its CREATE command exactly (git fetch + git worktree add only), verify HEAD = BASE_SHA and clean status, and tell the user you created it.
Print your WORK LOCATION code block and confirm it matches the dispatch LOCATION; otherwise stop with BLOCKED (never reset/stash/delete).
Open and follow exactly this dispatch: $ARGUMENTS
Obey coordination/AGENTS.md; apply coordination/SKILLS.md §13.2 and the §15 checklist. Stay inside the AUTHORIZED WRITE SET. Do not self-approve, push or open PRs. Use sub-agent swarms per AGENTS.md §3c (model from the MC_SUBAGENT_* values in .env; replace a sub-agent before 100k tokens).
If you get stuck (same failure twice, three failed fixes, would have to guess, missing access), do not loop: ask the user with the QUESTION FOR USER code block (AGENTS.md §0.1) and wait.
Write the RETURN file named in the dispatch, then end with WORK LOCATION + NEXT AGENT PROMPT (A1 resume) code blocks and stop.
