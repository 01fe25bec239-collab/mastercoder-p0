# W06 canonical commissioning sequence — concrete owner decision required

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

STATUS: PROPOSED_NOT_APPROVED
DATE_UTC: 2026-10-13T00:23:07.218760+00:00
CANDIDATE: b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
TREE: af193b0df3b88e64f0616b8d04d61799ba0d5045
CANONICAL_PR: https://github.com/01fe25bec239-collab/mastercoder-p0/pull/1
CANONICAL_BASE: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e

## Completed precommissioning evidence

- Exact candidate main CI green in the explicitly commissioned private test repository: https://github.com/01fe25bec239-collab/mastercoder-p0-w06-ci-qualification/actions/runs/38326251375. It ran the new action lint/verifier/extraction/spec-check controls and p0_executed:false; it is qualification-main proof, not canonical-main proof.
- Positive qualification PR, all eight contexts success: https://github.com/01fe25bec239-collab/mastercoder-p0-w06-ci-qualification/pull/1. Exact objects/base/App/job/run associations retained.
- All nine named hostile PR tests correctly red for intended reasons, including combined self-weakening: https://github.com/01fe25bec239-collab/mastercoder-p0-w06-ci-qualification/pull/10. Each individual URL and rejection logs appear in /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W06-T000-ROOT-HARDENING-remote-ci/remote-qualified-cases.json.
- Canonical PR https://github.com/01fe25bec239-collab/mastercoder-p0/pull/1 is ready (notdraft), exact-head eight required Actions15368 checks passed. Actual checkheadSHA matches candidate and API associates each with PR1; target-run base is import58b6ab38b0a514d94d8851679ad66ef9bf696c9e. Those guards are bootstrap code; not newimplementationproof. No actionable comments/unresolvedthreads in latest sweep; bot independent review unavailable due quota, not a PASS. Fresh A3 still pending.
- Canonical protections unchanged and no paid Actions usage authorized; availableincludedquota2985minutes at start. No productchanges; candidate/r0/review preserved.

## Concrete circular rule

Original task acceptance clause is verbatim: “arch-check green on main, red on tamper; pin lint active; runbook written.” Original AGENTS requires acceptance/finalA3 before ordinary merge. New protected-base implementation becomes active on canonicalmain only after merge; oldmain PRgreen does not establish that criterion. The prior PR-before-CI ordering decision did not resolve this activation timing.

Owner already explicitly authorized babysitting and conditionalmerge. This proposal asks only which commissioning sequence governs that criterion; it does not ask for another merge grant, waive tests, or let A1 self-review.

## Recommended W06-only staged sequence

1. A fresh independent A3 reviews exactcandidate plus actual canonicalPR/isolatednewguard evidence, unchanged protections, event/check associations, all nine specific negatives, unresolvedcomments and this approved sequence. It records READY_FOR_COMMISSIONING only if the complete precommissioning evidence and code are satisfactory; this is not finalPASS/ACCEPTED. Changed source/base or unresolveddefect stops for bounded repair/newreview.
2. A1 verifies currentexacthead, all eight genuine canonicalchecksuccesses/Appsource, permissible reviews/nodraft/noactionablecomments/nounresolvedthreads, actualmergeability and retained signed provenance. Under owner's existing conditionalmerge authorization and this explicit timing amendment, merge only canonicalPR1 using normal mergecommit with exact-head guard. No adminbypass, squashing, rebasing, forcepush, protectionchange, branchdeletion or automerge. This is a commissioning merge authorized before full acceptance, limited to this immutable W06guardcandidate. No prerequisite unlock yet.
3. Obtain realcanonicalmergedmain arch-check green/newlintactive, exactmergedtree/evaluatedsource provenance and applicable lanepushchecks. Verify allcandidateimplementationfiles remain byte-identical and candidate commit reachable as mergeparent/ancestor. Keep claimsNOT_RUN. A failed/missing maincheck prevents acceptance/INTEGRATED; no fabricatedpass or blindrollback. Any necessary fix gets boundedA2/freshA3; no repeatmerge.
4. A separatefreshfinalindependentA3 reviews the actual commissioning/mainresults against the exactcandidateSHA/tree and actualmergedcontext. Alloriginalcriteria must then be met; only after finalPASS does A1 record ACCEPTED and INTEGRATED with realMERGED_SHA. NoW06downstreamtasks start before that.

Allchecks are retained; canonicalmainactivation proof moves to the postmerge acceptance/integration gate where it can exist. Originaltaskfieldsremainunchanged, with this explicitowneramendmentattached. N-1 resourcebound observation remainsnonblocking/unqualified, and N-2actualproviderassociation is documented rather than inferred.

## Alternative

KeepcanonicalPRopen/unmerged and existingliteralordering unchanged; recordblockedactivationcircularity. Noauthority exists to mergefirstunderordinaryacceptance without the proposed narrowcommissioning exception.

REQUESTED_OWNER_DECISION: Approve W06 commissioning sequence with READY_FOR_COMMISSIONING → already-authorized normal merge → real canonical-main checks → fresh final A3 → acceptance/INTEGRATED; or keep the current hold.
