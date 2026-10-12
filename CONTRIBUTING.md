# Contributing (single-operator process)

1. Work only through each lane's agent loop (A1 → A2 → A1 → A3 → A1) described in `wsNN-*/coordination/AGENTS.md`.
2. Branch names: `ws0N/<task-id-lowercase>` for tasks, `ws0N/coordination` for coordination files.
3. Commits: `[W0N][<TASK_ID>][rN] <summary>`, SSH-signed; only the dispatched task's paths (hooks + CI enforce it).
4. PR: use the template, link the A2 RETURN and A3 REVIEW (PASS), all 8 required checks green, then `gh pr merge --merge --auto` (**merge commit**; no approval exists because one operator cannot approve their own PR — the A3 PASS is the independent review).
5. Problems: AGENTS.md §0.2 (no quick fixes), questions: §0.1 (ask, don't loop), configuration: §0.3 (no hardcoding).
6. Contract changes: CCR process (`COMMON/CONTRACT_CHANGE_PROTOCOL.md`), never by editing `ARCHITECTURE/`.
