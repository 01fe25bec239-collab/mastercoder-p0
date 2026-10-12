# KNOWN_ISSUES — WORKSTREAM-05-WEB-APPROVAL-UX

Living log of problems already solved in this workstream. **Agents read this before diagnosing any problem** (AGENTS.md §0.2 step 3) and append to it after every fix (step 10). Newest at the top. No secrets.

| ID | Date | Class | Symptom (exact error) | Root cause | Fix (commands / commit SHA) | Prevention | Found by |
|---|---|---|---|---|---|---|---|
| KI-000 | 2026-10-04 | — | (example) `DATABASE_URL must be set` when running tests in a new worktree | `.env.local` symlink is only created in the main clone | `ln -s <LANE>/.env <worktree>/ws05-web-approval-ux/.env.local` (now part of every CREATE command) | CREATE commands include the symlink | program setup |
