# LANE_02_GIT_ANALYSIS_REVIEW — START HERE

You are **master**, the single operator of all six lanes. This folder is **lane 02: WORKSTREAM-02-GIT-ANALYSIS-REVIEW** — Git, Analysis & Review. Shared local workspace (one clone, all worktrees): `../WORKSPACE/`. Program-level start: `../COMMON/DAY0_RUNBOOK.md` and `../COMMON/A2_PARALLEL_PLAN.md`.

Establish GitHub truth and the deterministic review path: staging org and Apps, provider normalizer and exact MergeSubjectV2, required-check semantics, CheckProjectionV2 lifecycle with withdrawal and emergency freeze, eligibility proofs, analysis planner with the sealed ExpectedAnalysisManifestV1, digest-pinned deterministic analyzers and findings, the pure gate reducer, oracle trust and verification authority, Publication App token residual characterization, checkout grant issuance, and a model-free FAST smoke.

## In 10 steps

1. Read `../COMMON/PROGRAM_README.md` and `../COMMON/MASTER_SCOPE.md` (15 min).
2. Complete `PRESTART_CHECKLIST.md` top to bottom and sign it (this is everything needed before you start vibecoding).
3. Fill the `__SET_ME__` values in `.env` (machine paths are filled by `setup_workspace.py`); never commit it.
4. Run `python3 setup_workspace.py`. It fills `.env` paths, clones the repo once into `../WORKSPACE/repo` (shared by all lanes), enables hooks, creates this lane's COORDINATION worktree `../WORKSPACE/worktrees/COORD-ws02` and symlinks `.env` (PRESTART §D). Re-run it until it shows no `[TODO]`.
5. Read `AGENTS.md` fully (the operating manual for every agent) and skim `SKILLS.md` (the procedures every agent must follow).
6. Skim `SCOPE.md`, `WRITE_SET.md`, `DEPENDENCIES.md`, `ARCHITECTURE_MAP.md`, `P0_CLAIMS.md`.
7. Open a fresh agent session and paste the **First A1** prompt from `AGENTS.md` §5.
8. A1 writes DISPATCHes (up to `MC_MAX_PARALLEL_A2` = 2 for this lane); you paste each **A2** prompt into its own new session; then each **A3** prompt into another new session. Run the six lanes' A1s side by side (round-robin) to keep about 15 A2s busy.
9. When A1 reports ACCEPTED, run the HUMAN COMMANDS it gives: push, open the PR, `gh pr merge --merge --auto` (`GIT_AND_PR_WORKFLOW.md`). GitHub merges each PR on its own when CI is green, so many PRs merge in parallel.
10. Repeat. When an A1 gets long in the tooth it writes a handoff; start a fresh A1 with the successor prompt.

## Files

| File | Purpose |
|---|---|
| AGENTS.md | Agent rules + full inter-agent message protocol + launch prompts |
| SKILLS.md | Mandatory reasoning/testing/security/self-review procedures + this workstream's technical skill pack |
| HUMAN_OPERATOR_GUIDE.md | **For you (the human):** daily loop, which model per role, what to say and never say to agents, problem reports, escalation |
| TROUBLESHOOTING.md | Symptom → cause → correct fix → never-do, plus the SECURITY INCIDENT procedure |
| KNOWN_ISSUES.md / INCIDENTS.md | Living logs: solved problems (agents read these first) / security incidents (metadata only) |
| CLAUDE.md / AGENT_TOOLING/ | Auto-load files for Claude Code, Codex CLI and Cursor + role slash commands (installed into the repo by bootstrap) |
| PRESTART_CHECKLIST.md | Tools, accounts, hardware, access, verification — before any coding |
| setup_workspace.py | Automated, non-destructive setup + status check (`python3 setup_workspace.py [--check]`) |
| .env.example / .env | Environment variables (placeholders; secrets only in your local copy) |
| GIT_AND_PR_WORKFLOW.md / PR_TEMPLATE.md | Branches, worktrees, commits, PRs, merges |
| task.md | Execution-ready BUILD + RUN tasks and the state ledger |
| memory.md | Durable workstream memory |
| SCOPE.md / WRITE_SET.md | What you own / exact paths you may change |
| ACCEPTANCE_CRITERIA.md | When this workstream is done for the freeze-closure block |
| P0_CLAIMS.md | Your atomic claims in full |
| ARCHITECTURE_MAP.md | Which architecture files matter and why |
| DEPENDENCIES.md | Interfaces you provide/consume, doubles, milestones, external needs |
| HANDOFFS/ RETURNS/ REVIEWS/ DISPATCHES/ DISPOSITIONS/ NOTICES/ CONTRACT_CHANGE_REQUESTS/ | Coordination records (seeded into the repo at bootstrap) |
| ARCHITECTURE_READONLY/ | The complete, verified V2.1 package + original ZIP |
