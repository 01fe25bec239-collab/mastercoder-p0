# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# Repository branch protection runbook

W06-T000 documents the existing bootstrap policy for `main`. It does not grant
permission to change GitHub settings, push, open a PR, merge, or execute P0 claims.
Authority: `coordination/AGENTS.md` §§3a, 8, 13–14 and the task dispatch. The
bootstrap source is `COMMON/bootstrap/create_github_repo.sh` in the program
package outside this repository. A1 records live verification in coordination;
this document alone is not evidence that protection is active.

## Required settings

| Setting | Required policy |
| --- | --- |
| Protected branch | `main`; changes through PRs; protection enforced for administrators |
| Merge methods | Merge commits only; squash and rebase disabled; linear-history requirement disabled |
| Required checks | Exactly the eight contexts below, each restricted to the verified GitHub Actions App |
| Up-to-date branch | Disabled (`required_status_checks.strict: false`) so disjoint accepted tasks can merge in parallel |
| GitHub approving reviews | Zero (`required_approving_review_count: 0`); the single owner cannot approve their own PR |
| CODEOWNERS | `.github/CODEOWNERS` must exist and describe ownership/review routing; task scopes remain separately enforced |
| GitHub code-owner approval gate | Disabled (`require_code_owner_reviews: false`), as configured at bootstrap; ownership does not replace independent A3 |
| Stale reviews / conversations | Dismiss stale approvals; require conversation resolution |
| Force pushes / branch deletion | Disabled on `main`; retain task branches after merge until A1 housekeeping |
| Auto-merge | Repository setting enabled; use only after acceptance and explicit human authorization |

The eight unique required contexts are:

| Context | Workflow |
| --- | --- |
| `arch-check` | `.github/workflows/common-arch-check.yml` |
| `write-set-guard` | `.github/workflows/common-write-set-guard.yml` |
| `ws01-ci` | `.github/workflows/ws01-ci.yml` |
| `ws02-ci` | `.github/workflows/ws02-ci.yml` |
| `ws03-ci` | `.github/workflows/ws03-ci.yml` |
| `ws04-ci` | `.github/workflows/ws04-ci.yml` |
| `ws05-ci` | `.github/workflows/ws05-ci.yml` |
| `ws06-ci` | `.github/workflows/ws06-ci.yml` |

The six lane checks are bootstrap placeholders until their owning scaffold tasks
are integrated. Their success does not prove compilation or qualification.
These development checks are separate from the product GitHub profile's
canonical `Mastercoder Review` check (architecture 49).

## Trusted checks and independent review

Only the two common guard workflows may use `pull_request_target`. They check
out the protected base, use `permissions: contents: read` with no repository
secrets, and treat candidate commits as Git data. Candidate guard scripts,
workflow commands and architecture Python are never executed. `arch-check`
lints candidate action metadata, checks every architecture file against the
pinned manifest (including missing/extra files), then runs the base's
specification checker against verified candidate JSON. It must report
`p0_executed:false`. `write-set-guard` enforces both lane ownership and exact
task scopes/exclusions from the protected base's `WRITE_SCOPES.json`, with
branch and PR-title agreement. Every `uses:` must be a full 40-hex commit SHA
with a same-line `# vX.Y` or `# vX.Y.Z` comment; local/Docker actions and
unsupported YAML forms fail closed under this task's lint policy.

A fresh A3 who did not dispatch, author or repair the task reviews its exact
implementation SHA and tree. A1 alone records acceptance. The owner-approved
build/review/CI sequence permits preliminary independent review when a local
RETURN is blocked only on remote CI; it does not permit final PASS or acceptance
until every required CI and scratch-PR result exists. Record real URLs, head
SHAs and check sources. Local tamper tests are not remote CI evidence.

Before an authorized merge, A1 verifies that the PR head still equals the
accepted SHA/tree, that all eight checks succeeded for the intended subject and
trusted source, and that the exact-head A3 verdict and resolved conversations
are recorded. GitHub can accept skipped/neutral checks; this project's evidence
rules require actual applicable work, so inspect those outcomes. Changed heads
need a new attempt and fresh review. Do not amend, rebase, squash, self-approve,
or use an administrator bypass. Conflicts go through a repair dispatch. Merge
commits preserve the reviewed commit, and `strict:false` avoids rewriting it
after unrelated parallel merges. A1 sends the toolchain-pin notice to all lanes
before merging this task and performs §3b cleanup only after integration.

## Read-only operator verification

Run from the intended repository with authenticated `gh`. Resolve the repository
and App identity from the live API; do not copy another account's IDs or accept
an unrestricted/unknown check source. These commands read settings only:

```bash
set -eu
task_repo=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
task_main_sha=$(gh api "repos/$task_repo/commits/main" --jq .sha)
gh api "repos/$task_repo" --jq '{allow_merge_commit, allow_squash_merge, allow_rebase_merge, allow_auto_merge, delete_branch_on_merge}'
gh api "repos/$task_repo/branches/main/protection" --jq '{required_status_checks, enforce_admins, required_pull_request_reviews, required_linear_history, allow_force_pushes, allow_deletions, required_conversation_resolution}'
gh api --paginate "repos/$task_repo/commits/$task_main_sha/check-runs" --jq '.check_runs[] | {name, head_sha, status, conclusion, app: {id: .app.id, slug: .app.slug}}'
```

Compare every required check's `app_id` with the actual trusted `github-actions`
check producer shown by check runs and the operator's verified App identity.
Never set `app_id` to `-1`/null (any source). The PR-only `write-set-guard` may
have no run on `main`; inspect its actual authorized PR head in the same API
instead. Verify all eight contexts individually and record the JSON/URLs in
coordination. Failure, missing checks, wrong sources, unavailable protection or
unknown identity blocks acceptance; request an owner decision rather than
weakening settings. Confirm the ownership map from the protected base, not a
candidate replacement. Provider-setting changes require a separate owner grant.

## Local verification before the CI handoff

Use the A1-managed shared build wrapper for every local test. Resolve its
absolute path from the dispatch and substitute each command below after the
wrapper's `--`; run at the repository root:

```bash
bash scripts/check_write_set.sh --isolation-test
bash scripts/check_write_set.sh --self-test
python3 scripts/verify_architecture.py ARCHITECTURE/V2_1
python3 scripts/verify_architecture.py ARCHITECTURE/V2_1 --self-test
python3 scripts/check_action_pins.py --self-test
python3 scripts/check_action_pins.py --revision "$(git rev-parse HEAD)"
python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py
git diff --check
gitleaks detect --no-banner --redact
```

The guard's disposable fixture covers valid scope, neighbouring tasks,
coordination edits, harness exclusions, absent tasks, title mismatch, foreign
paths, rename escapes, control-character paths, blob limits, candidate scope
forgery, and combined guard weakening plus architecture/foreign changes. Its
sentinels verify that fixture Git leaves the original HEAD/index/files intact.
Verifier and pin-lint self-tests cover positive and hostile inputs. Scratch
objects/copies are removed on exit. A1 must separately arrange authorized real
green/red scratch PR runs and final fresh A3 review; this runbook creates none.

Provider semantics checked against official documentation on 2026-10-13:
[protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches),
[branch-protection API](https://docs.github.com/en/rest/branches/branch-protection),
and [safe pull_request_target use](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target).
