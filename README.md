# mastercoder-p0 — Mastercoder freeze-closure prototype monorepo

> **PRIVATE · PROTOTYPE · DISPOSABLE · NOT PRODUCTION.** Production implementation of Mastercoder is **not authorized**. This repository exists only to prove (or disprove) the 55 atomic P0 claims of architecture V2.1 and to reach a legitimate architecture-freeze decision.

## What is here

| Path | Owner | Purpose |
|---|---|---|
| `ARCHITECTURE/V2_1/` | read-only (W06 import steward) | Authoritative architecture `MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1`. Verified in CI by `arch-check`. Never edited. |
| `ws01-core-security/` | lane W01 | Canonical codec, PostgreSQL authority, leases, journal, approval authority, custody, publication gateway, resolution |
| `ws02-git-analysis-review/` | lane W02 | GitHub truth, check projection, eligibility, analysis planner, reducer, verification |
| `ws03-runner-sandbox-acquisition/` | lane W03 | Runner, sandboxes (Linux, macOS, Windows), checkout, offline install (dependency acquisition is in ws04) |
| `ws04-agent-model-graph/` | lane W04 | Model turns, broker, secret dialer, graph engine + envelope, ledger, dependency acquisition |
| `ws05-web-approval-ux/` | lane W05 | Next.js UI, approval-origin renderer, TS codec, accessibility |
| `ws06-platform-oss-qualification/` + root files | lane W06 | CI, supply chain, evidence kit, drills, OSS drafts, freeze dossier |
| `wsNN-*/coordination/` | each lane | task.md, memory.md, AGENTS.md, SKILLS.md, handoffs, returns, reviews (changed only on branch `ws0N/coordination`) |

Each `wsNN-*/` is its own Cargo/pnpm workspace with its own lockfile. Code from other lanes is consumed read-only. One operator (master) runs all six lanes.

## Start here (master)
1. The program package: `COMMON/PROGRAM_README.md`, `COMMON/DAY0_RUNBOOK.md`, then each `LANE_*/README.md`, `PRESTART_CHECKLIST.md`, `HUMAN_OPERATOR_GUIDE.md`.
2. Run `python3 setup_workspace.py` from your program folder (clones, hooks, COORDINATION worktree, symlinks, checks).
3. Agents start in the right worktree and read `coordination/AGENTS.md` + `coordination/SKILLS.md` automatically (CLAUDE.md / AGENTS.md / Cursor rules).

## Rules everyone follows
- Agent model A1 → A2 → A1 → A3 → A1; independent review on exact SHAs; no self-approval.
- One owner per path (`.github/CODEOWNERS`), enforced locally (`.githooks`) and in CI (`write-set-guard`).
- No secrets, real customer data or production credentials anywhere. No hardcoded configuration (AGENTS.md §0.3).
- Merge with **merge commits only**; `main` is protected; CI must pass.
- Gates: G1 build authorization → G2 external audit PASS → G3 per-family execution authorization → G4 freeze (master).

## CI
| Workflow | What it enforces |
|---|---|
| `common-arch-check.yml` | Architecture snapshot integrity (235 files + pinned hashes) and `p0_executed:false` |
| `common-write-set-guard.yml` | Changed files match the PR branch's workstream write set; coordination only on `ws0N/coordination`; no secret-like or >5 MB files |
| `ws0N-*.yml` | Each workstream's build/test (added in each workstream's T000 task) |
