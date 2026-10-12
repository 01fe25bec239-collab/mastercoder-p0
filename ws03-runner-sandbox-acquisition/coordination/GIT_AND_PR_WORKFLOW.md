# Git and PR workflow — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION

## Branch, worktree, commit
| Item | Rule |
|---|---|
| Base | `origin/main` at dispatch time = BASE_SHA; every code prerequisite INTEGRATED and its MERGED_SHA an ancestor of BASE_SHA |
| Branch | `ws03/<task-id-lowercase>` (repairs: same branch, new commits; or `-rN` suffix if A1 prefers a fresh branch) |
| Worktree | `WORKSPACE/worktrees/<TASK_ID>-<ATTEMPT>` — one per attempt; A3 uses its own detached worktree |
| Commit message | `[W03][<TASK_ID>][rN] <imperative summary>` + body `PROTOTYPE — DISPOSABLE — NOT PRODUCTION` + `Task: <TASK_ID>` `Attempt: rN` |
| Signing | SSH-signed commits required |
| After RETURN | No commits at all on the task branch |
| Coordination files | Only on `ws03/coordination` (COORDINATION worktree); merged via a coordination-only PR after each accepted task / daily |

## When the A1 reports ACCEPTED (human steps)
```bash
cd WORKSPACE/worktrees/<TASK_ID>-<ATTEMPT>
git log -1 --format=%H            # must equal ACCEPTED_SHA
git diff --name-only <ACCEPTED_SHA>..HEAD   # must be empty
git push -u origin HEAD
gh pr create --title "[WS03][<TASK_ID>] <title>" --body-file <(sed -n '1,200p' ws03-runner-sandbox-acquisition/coordination/PR_TEMPLATE.md)   # then fill the table
gh pr merge --merge --auto        # GitHub merges it by itself as soon as the required checks pass
```
Do this for every ACCEPTED task as soon as A1 reports it, in any lane, in any order: PRs with disjoint write scopes never conflict, so they all merge in parallel (`gh pr list --search "is:open auto-merge:enabled"` shows the queue).
1. CI on the PR head must be green (your `ws03-*` workflow + `common-*`).
2. No approval click is needed: you are the only human (GitHub never lets an author approve their own PR), so protection requires the checks, not a review. The independent review already happened: the A3 PASS on this exact SHA.
3. Auto-merge uses **Create a merge commit** (squash/rebase are disabled). A1 records the merge SHA as MERGED_SHA in the task.md ledger → state INTEGRATED (`gh pr view <PR> --json mergeCommit`). If auto-merge stops (a red check, or a lockfile conflict), A1 handles it: REPAIR or the standing LOCKFILE-SYNC task.
4. Tell consumer A1s (INTERFACE_NOTICE) if the task published/changed an interface.

## Coordination PR (after each accepted task, at least daily)
```bash
cd WORKSPACE/worktrees/COORD-ws03
git status                                   # only ws03-runner-sandbox-acquisition/coordination/** changed; A1 has committed
git merge origin/main                        # keep current (merge, never rebase)
git push -u origin ws03/coordination
gh pr create --title "[WS03][coordination] <date>" --body "Coordination records only (task ledger, returns, reviews, memory)."
gh pr merge --merge --auto
```
The branch stays and is reused (branches are not deleted on merge). The six lanes' coordination PRs touch disjoint folders and merge in parallel too.

## Never
Force-push after RETURN · squash/rebase merge · merge without A3 PASS · PR touching another lane's paths · committing `.env`/keys · `--no-verify` · turning off a required check to get a PR merged.
