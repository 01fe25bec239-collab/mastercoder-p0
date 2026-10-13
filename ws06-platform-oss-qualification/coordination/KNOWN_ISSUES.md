# KNOWN_ISSUES — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION

Living log of problems already solved in this workstream. **Agents read this before diagnosing any problem** (AGENTS.md §0.2 step 3) and append to it after every fix (step 10). Newest at the top. No secrets.

| ID | Date | Class | Symptom (exact error) | Root cause | Fix (commands / commit SHA) | Prevention | Found by |
|---|---|---|---|---|---|---|---|
| KI-000 | 2026-10-04 | — | (example) `DATABASE_URL must be set` when running tests in a new worktree | `.env.local` symlink is only created in the main clone | `ln -s <LANE>/.env <worktree>/ws06-platform-oss-qualification/.env.local` (now part of every CREATE command) | CREATE commands include the symlink | program setup |

## KI-001 — scratch Git targets original task root (TEST)

2026-10-13 IST. Symptom: guard self-test scratch `git commit-tree` exits128 before assertions; signing-disabled synthetic retry also fails. Confirmed cause: Python `cwd=trusted` default captures task root at function definition; self_test changing global trusted does not change the default. Fixture init/add/write-tree touch the wrong repository, and scratch commit-tree reports `fatal: not a git repository (or any of the parent directories): .git`. Read-only extracted-function proof and explicit-cwd positive control: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W06-T000-ROOT-HARDENING-r0-preservation/diagnosis.md. Implemented in r1 b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03: dynamic default directory, original-root/index sentinel regression and safe stderr; A1 wrapped isolation/full self-test pass. Independent review and real CI remain pending. Do not rerun the flawed full self-test in r0; product signing remains enabled.

## KI-002 — trusted checker JSON followed by semantic summary (TEST)

A2 r1 records JSON Extra data when parsing entire trusted checker output. Leading-JSON parse plus explicit p0_executed:false implemented; offline replay passed per return, independent A3/remote evidence pending.

## KI-003 — hidden block-scalar workflow events (SECURITY)

A2 r1 records hidden literal/folded target events in draft lint. Conservative lint rejects these forms with positive/negative controls; independent A3/provider evidence pending. Never count offline replay as protected-main deployment.

## KI-002 — protected-base CI activation and check binding

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

Symptom: locally satisfactory guard implementation cannot be remotely qualified by PR CI against bootstrap protected main; target-event check subjects also unverified. Root cause: guards correctly execute protected-default/base bytes and old main lacks new lint/verifier/guard controls. B-1 is an acceptance blocker; N-2 is remote evidence binding, not a demonstrated permitting defect. Proposed bounded verification phase: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/NOTICES/W06-T000-CI-COMMISSIONING-PROPOSAL.md; owner approval pending, canonical activation still separate. Prevention: record exact trusted workflow/code/base/head and actual provider check association; never call bootstrap greens or isolatedmain greens canonical deployment proof. No waiver, acceptedSHA or merge. N-1 resource bounds remain nonblocking/unqualified; any new requirement gets scoped A2/A3 work.
