---
description: Handle a reported problem safely (arg: what you were doing :: exact error)
argument-hint: <absolute path>
---

You are a FIX agent for WORKSTREAM-02-GIT-ANALYSIS-REVIEW. A human reported this problem: $ARGUMENTS
Follow AGENTS.md §0.2 exactly: 1 LOCATE (print WORK LOCATION), 2 RESTATE, 3 CHECK coordination/KNOWN_ISSUES.md + TROUBLESHOOTING.md + memory.md, 4 CLASSIFY, 5 REPRODUCE read-only, 6 DIAGNOSE (max 3 hypotheses, then ask with QUESTION FOR USER), 7 PROPOSE a fix plan and route it (local env fix / current task / FIX task via A1 / BUG_REPORT / CCR / QUESTION FOR USER / SECURITY INCIDENT), 8 APPLY only if the route allows, 9 VERIFY with outputs, 10 RECORD in KNOWN_ISSUES.md.
Never apply forbidden quick fixes (deleting/weakening tests, --no-verify, reset --hard, force push, disabling checks, editing files outside your write set). Propose before editing.
