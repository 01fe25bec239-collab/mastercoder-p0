# KNOWN_ISSUES — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

Living log of problems already solved in this workstream. **Agents read this before diagnosing any problem** (AGENTS.md §0.2 step 3) and append to it after every fix (step 10). Newest at the top. No secrets.

| ID | Date | Class | Symptom (exact error) | Root cause | Fix (commands / commit SHA) | Prevention | Found by |
|---|---|---|---|---|---|---|---|
| KI-000 | 2026-10-04 | — | (example) `DATABASE_URL must be set` when running tests in a new worktree | `.env.local` symlink is only created in the main clone | `ln -s <LANE>/.env <worktree>/ws01-core-security/.env.local` (now part of every CREATE command) | CREATE commands include the symlink | program setup |

## KI-001 — password grep matches its own baseline task (TEST)

2026-10-13 IST. Original recursive POSTGRES_PASSWORD= grep returns3matches before and after scaffold: productplaceholder, coordinationplaceholder, taskquotation. Root cause is baseline coordination copied into product lane, not a new literal password. Master approved classification rule in NOTICES/W01-T000-PASSWORD-CHECK-CLARIFICATION.md: preserveallmatches, requiretwo unchanged coordination files, requireonly productplaceholder and inspectcompose. Originaltask unchanged; anynewmatch fails. No secretcheckwaived.
