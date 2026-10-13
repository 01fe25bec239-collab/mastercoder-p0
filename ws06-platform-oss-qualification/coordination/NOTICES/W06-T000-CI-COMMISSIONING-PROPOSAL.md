# W06-T000 CI commissioning proposal — approval requested

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

STATUS: PROPOSED_NOT_AUTHORIZED
DATE_UTC: 2026-10-13T00:08:46.963482+00:00
OWNER: master
CANDIDATE: b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
TREE: af193b0df3b88e64f0616b8d04d61799ba0d5045
CANONICAL_REPOSITORY: 01fe25bec239-collab/mastercoder-p0
VERIFICATION_REPOSITORY_PROPOSED: 01fe25bec239-collab/mastercoder-p0-w06-ci-qualification (new private disposable repository)

## Problem and evidence

Fresh independent A3 found local SATISFACTORY, final BLOCKED:FINAL_PR_CI_PENDING, no required local repair. It records B-1: canonical protected main at 58b6ab38b0a514d94d8851679ad66ef9bf696c9e has old guards, so a candidate PR there does not execute the new protected-base guard implementation. A1 just read the live API: canonical main is still that exact commit; all eight checks use GitHub Actions App15368, strictfalse, admins enforced, no force/deletion, merge commits only. Read-only name lookup returned404 for the proposed verification repository; this does not guarantee creation rights or visibility, so fail if the exact name already exists at execution. No provider changes made.

N-2 requires discovered event/check association, not an assumption that target-event check SHA equals candidate head. GitHub documents pull_request_target as default/base context, and permits branches filters on target branch; therefore an arbitrary qualification branch would not exercise these unchanged branches:[main] workflows. An isolated repository whose default main contains the exact reviewed candidate preserves workflow bytes.

## Authorization requested now — phase1 only

Authorize creating the one private disposable verification repository; uploading the exact two signed candidate commits/history needed for it; explicitly commissioning b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03/af193b0df3b88e64f0616b8d04d61799ba0d5045 as trusted test-only code on its initial main; configuring its protections/Actions as below; bounded synthetic PR cases and read-only evidence collection; and pushing the exact existing ws06/w06-t000-root-hardening-r1 branch plus creating one draft W06 PR in the canonical repository. Authorization would be for these concrete external actions only. No canonical main push, protection change, merge, auto-merge, final acceptance, downstream unlock, claim execution, paid API/cloud usage or repository deletion. It does not declare isolated main to be canonical main.

$0 additional spend ceiling for this phase. Use only included GitHub Actions quota; check entitlement and estimated remaining quota before enabling/running Actions. Private-repository/protection entitlement and $0 execution ability must be verifiable; otherwise stop with the specific access/quota blocker. Do not enable paid overages, buy credits, publish the repository, install Apps, add collaborators, add secrets or use self-hosted runners. Existing G1 total5USD/model1M budget is unchanged; this phase consumes no model API or cloud spend.

## Concrete procedure after approval

1. Reverify live canonical base/protections, owner identity/access, candidate signatures/tree/clean status and zero-spend quota. Stop on drift, unknown access or an occupied verification repository name. Use a separate scratch Git repository for remote fixtures; immutable product/r0/review worktrees are read-only.
2. Create the verification repository private with no generated README/license/initial commit. Before uploading code, verify it has no repository secrets/environments, no self-hosted runners, no inherited caches/Apps/deployments; configure Actions/token permissions read-only, disallow PR approvals, and verify included-quota $0 execution. Only github-actions hosted runners are permitted. Actions must be enabled for the initial seed event after these checks. Stop if these conditions cannot be proved.
3. Upload exact b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 to its initial main and capture the genuine initial push/main arch-check run. Verify HEAD/tree and all nine implementation path hashes against the reviewed candidate. Protect verification main immediately: same eight named Actions-app contexts, actual verified github-actions App identity (recorded15368, fail if mismatch), strictfalse, enforceadmins, force/deletionfalse, mergecommitonly, conversationresolutiontrue, approvals0/codeownerfalse. Verify privacy/protection and unchanged main before any PR. The initial seed is the one explicit test-only trust designation for this new empty repository; no later main updates or scratch merges. It is not canonical integration or acceptance and bypasses no canonical protection.
4. Open one positive test PR from a signed synthetic branch ws06/w06-t000-root-hardening-r99 based on exact verification main, changing only a prototype comment in ws06-platform-oss-qualification/docs/branch-protection.md; title [W06-T000-ROOT-HARDENING] CI qualification positive. HeadSHA/tree must be computed/recorded after creation, never guessed. Confirm all eight associated contexts succeed; lane placeholders are reporting controls only, not lane builds.
5. Open at most nine signed, nonmerged scratch PRs, one per red case in the table below. All branch/task/title metadata is generated from protected WRITE_SCOPES.json and the exact case, with signed synthetic commits. Current original task scopes remain unchanged. Candidate common guards are never executed: only verification protected main code executes them, candidate objects remain data. No hostile addition to a workflow that executes on pull_request. Close test PRs and delete only their enumerated synthetic branches after URLs/logs/objects are retained; no repository deletion.
6. Push exact canonical task branch at b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03; no amend/rebase/new product commit. Create draft title [W06-T000-ROOT-HARDENING] Harden protected-base root checks using retained draft body. Obtain every actual canonical PR context/subject/URL and associate it with event head/base/trusted workflow revision. Those are bootstrap guard results, explicitly distinct from new guard implementation results in the verification repository. If canonical branch already exists, verify exactSHA or stop; never force push. Attach every created PR to this chat.
7. Collect the evidence bundle and stop for A1 disposition. If any new failure indicates a product defect, use a new bounded repair A2 and fresh A3; a changed implementation invalidates this plan's binding and requires requalification. No continuing to merge or auto-merge.

## Required red-case matrix

| Case | PR change relative to exact verification main | Required specific failure |
|---|---|---|
| byte tamper | change one byte in ARCHITECTURE/V2_1/00F_TECHNOLOGY_STACK.md | arch-check hash mismatch |
| extra architecture file | add ARCHITECTURE/V2_1/W06_CI_EXTRA.txt | arch-check extra-file rejection |
| floating action | add uses: actions/checkout@v6 with versioncomment to candidate common-arch-check.yml | trusted action-pin lint rejects fullSHA violation; candidate common workflow not executed |
| neighbouring task | known W06 task branch modifies another task's file from protected scopes | write-set-guard exact-scope rejection |
| coordination edit | same known task modifies coordination/memory.md | write-set-guard rejects coordination path |
| harness exclusion | harness task branch edits its excluded harness/Cargo.toml | write-set-guard explicit exclusion rejection |
| unknown task | well-formed branch for absent task, prototype scoped file | write-set-guard missing dispatch rejection |
| title mismatch | known task branch, harmless permitted comment, other task in PR title | write-set-guard task/title mismatch |
| combined self-weakening | candidate common workflow/guard/verifier made always-pass plus architecturebyte + foreignpath | protected-base arch-check hash rejection AND write-set-guard foreign/scope rejection; head scripts never run |

All negative PRs are genuine remote runs on the same trusted implementation. Wrong-name branch, invalid YAML, missing runner or unrelated setup error is not the intended rejection. Repair fixture mistakes only in the scratch branch with explicit new recordedSHA; never silently reuse stale URLs or failed unrelated runs. Any candidate YAML change is limited to the common pull_request_target guards, whose execution source remains protected default main.

## Evidence collector contract

For each repository/PR/run retain: repositoryID/name/privacy; main/trustedSHA/tree; immutable candidateSHA/tree; PRnumber/title/headref/headSHA/baseref/baseSHA; workflowID/path/workflow revision/blob hash; trigger event; runID/attempt/URL/headSHA/checksuiteID; every job/checkID/name/URL/actualheadSHA/conclusion; appID/slug; changed-path case; intended rejecting step/log. Query PR-associated checks and actions runs, then both base/head/merge check suites as applicable, correlate actual PR associations and event pair. Do not select a main check solely because its name is green. If provider association cannot be shown or does not satisfy protected required checks, stop; no manual check/status creation or app-source loosening. Proof from verification repo stays labeled there; canonical PR/current-main proof stays labeled canonical. Preserve redacted raw JSON, downloaded logs, object/path hashes and classification manifest; no token values.

## Phase2 gate — explicit unresolved canonical commissioning decision

Phase1 produces remote implementation evidence and canonical draft-PR evidence. It **does not satisfy** the unchanged canonical requirement arch-check green on main / pin lint active there. The protected-base activation circularity therefore still prevents final PASS/ACCEPTED and ordinary canonical merge under the current original gates. No automatic transition follows phase1.

After phase1 succeeds, present a separate concrete owner decision grounded in actual URLs: either a narrowly scoped commissioning transition that assigns isolated exact-implementation qualification to the premerge gate and canonical-main activation/green to a mandatory postmerge-before-INTEGRATED gate, or a separately dispatched trusted rollout design that fits the existing gates. Both would need explicit scope/sequence authorization, a fresh independent premerge review of that decision/evidence, preserved eight canonical protections and a separate merge grant. Under a commissioning transition, READY_FOR_COMMISSIONING is distinct from full acceptance; no ACCEPTED/INTEGRATED before actual canonical green/active checks and fresh required review. This proposal grants none of that transition.

The task specification remains verbatim. No original criterion is waived, no synthetic/local result is relabeled canonical, and no bootstrap check is called a test of the new guard. If the owner declines isolated verification, keep BLOCKED and record the chosen alternative; do not retry promotion to main.

## Source references

- /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/REVIEWS/W06-T000-ROOT-HARDENING-r1-a-A3_REVIEW.md
- /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/BUILD_REVIEW_CI_SEQUENCE_DECISION.md
- Live read-only snapshot /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W06-T000-ROOT-HARDENING-r1-review-processing/live-readonly-github.json
- [GitHub events/default context and branch filters](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target)
- [Protected branch status-source and merge gates](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [Check-run identities and subjects](https://docs.github.com/en/rest/checks/runs#list-check-runs-for-a-git-reference)

REQUESTED_OWNER_REPLY: Authorize W06 phase1 CI verification and canonical draft PR, $0 additional spend, no canonical merge.
