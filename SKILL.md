---
name: mastercoder-program
description: Start and coordinate the Mastercoder six-lane disposable-prototype program using its pinned architecture, gated task plan and bundled engineering skills.
---

# Mastercoder program skill

Before G0 in the package, read START_HERE.md, COMMON/A1_START_PROMPT.md, COMMON/PROGRAM_README.md and COMMON/DAY0_RUNBOOK.md. Read COMMON/SUBAGENT_POLICY.md for the owner's helper preferences. After bootstrap in the prototype repo, use /SUBAGENT_POLICY.md and the lane coordination/AGENTS.md, coordination/SKILLS.md and current memory/task/handoff records; do not restart the pre-G0 startup routine or look for absent COMMON paths. Select the lane and read its AGENTS.md, SKILLS.md, task.md, memory.md and latest handoff. The architecture and existing gates remain authoritative; this package does not sign G1 or grant production implementation.

## Bundled Matt Pocock skills

38 upstream skills are available offline under `COMMON/VENDORED_SKILLS/mattpocock/<name>/SKILL.md` in this package, or `/.agents/skills/<name>/SKILL.md` after bootstrap. Read `COMMON/VENDORED_SKILLS/mattpocock/SOURCE_MANIFEST.json` (package) or `/.agents/skills/SOURCE_MANIFEST.json` (bootstrapped repo) for exact names, categories, pinned commit, license and file hashes. Engineering/productivity are the normal catalog; misc/in-progress are optional upstream material, not mandatory project processes.

Use relevant skills only after reading their full SKILL.md and referenced local files. Useful routes: research/domain-modeling for discovery; diagnosing-bugs for investigation; implement/implement-spec/tdd for scoped implementation; code-review for independent A3 review; handoff/writing-for-agents for concise durable returns. Their generic workflow must stay within the dispatched write set, existing task/evidence gates and fresh-review requirements. A skill cannot authorize a push, expand scope, change frozen architecture or replace the A1/A2/A3 authority model.

No upstream setup skill is run automatically. Before using skills that depend on issue-tracker/domain/triage configuration, follow the bundled setup skill within the owner's authorized scope. Existing task ledgers stay authoritative and GitHub remains the development host; GitLab transfer is deferred to submission. The upstream AGENTS.md/CLAUDE.md are not substituted for this project's instructions.

After G0, bootstrap copies this catalog to `/SKILL.md` and the upstream skill folders to `/.agents/skills/` and `/.claude/skills/` in the prototype repository. Avoid installing duplicate plugin copies. Pre-G0 agents can read these bundled paths directly without network or global installation.
