# W01-T009-WEBAUTHN-VERIFY r1-a — fresh independent source A3

PROTOTYPE — DISPOSABLE — NOT PRODUCTION. Root remains A1. HUMAN hosts and chooses model/settings; no reviewer launched by Root.

## Immutable target and independence

FOLDER: /Users/omkar/.codex/worktrees/w01-webauthn-r1-review-a/repo/ws01-core-security
WORKTREE: /Users/omkar/.codex/worktrees/w01-webauthn-r1-review-a/repo
BRANCH: DETACHED
IMPLEMENTATION_SHA / HEAD: b9af738fd9779be0f3e02351a4cfba863db2711a
TREE: 17b33ab470340a11db143c6ccca530c7a74d3eca
REPAIR_BASE / sole parent: 305cbfd838776a9b894fdd44a2687ad542a59eff
ORIGINAL_TASK_BASE: 0ed316f35e1cde00563067132e1b9726a0ddcadb
STATUS: existing clean detached checkout, private lane environment linked. CREATE: NOT_NEEDED. Print/verify actual location/HEAD/tree/clean status before and after; never reset/stash/delete on mismatch.

You are a NEW independent A3, not the r0/r1 author, r0 source reviewer, CCR reviewer, dispatcher or any of their helpers. Do not reuse Root's t009_r1_receipt_check/t009_r1_preservation_check teams. Any reviewer helpers must be fresh. Root's foreign-key spotcheck is not source approval.

## Complete inputs and authority

Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md §§13.3/14.4/15, memory.md, amended task T009 and HANDOFFS/A1_HANDOFF_030.md; repository SKILL.md/SUBAGENT_POLICY.md and applicable review skills without overriding project scope.
Read COMPLETE repair dispatch /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T009-WEBAUTHN-VERIFY-r1-REGISTRATION-PROOF-REPAIR-DISPATCH.md, SHA25604efdc959446aefd2d0a0cb95755ec7be7372fddd720ede691c5f8b24cb0bd3b, and COMPLETE RETURN /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T009-WEBAUTHN-VERIFY-r1-A2_RETURN.md, SHA256640602acc5439d78ed004532c50a868a7bf2e77f916e9e1a45dab5b92048f24f.
Read adopted /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/CONTRACT_CHANGE_REQUESTS/CCR-W01-002-OWNER-DECISION.md, SHA256ba2f80f275e74ba8b623431cfad26f72c50aff9ae8496576e7384090208dd821, and NOTICES/CCR-W01-002-ADOPTED-PROFILE-NOTICE.md. Original CCR and cross-lane review stay unchanged; owner adopted a restricted packed-self ES256 profile/task correction, no frozen field/wire/API change. This supersedes the old none-success task expectation prospectively.
Read original r0 RETURN, blocked r0-a source REVIEW and CCR review as history. Source r0 F1 needs current source validation; neither owner adoption nor author controls alone closes it. Do not reopen the already adjudicated unsupported-none compatibility choice merely because it is restrictive; report any actual violation/unsound proof concretely.
Read frozen ENCODING/doc52/registration/enrollment schema and primary W3C references cited in the decision as needed. Original architecture remains immutable. Candidate coordination files are historical main snapshots; live canonical coordination above governs this attempt.

## Review scope and focus

Review the full current ten-file library/test task from 0ed316f35e1cde00563067132e1b9726a0ddcadb..b9af738fd9779be0f3e02351a4cfba863db2711a, with special focus on the four-file repair 305cbfd838776a9b894fdd44a2687ad542a59eff..b9af738fd9779be0f3e02351a4cfba863db2711a. r0 source review stopped after F1, so do not assume its remaining workspace/security validation completed.

1. Actual same-NEW-key proof: verify ES256 DER signature over exact received authenticatorData || SHA256(exact received clientDataJSON), using only the credential key extracted/validated from that same create data. No caller override, unrelated signer, reserialized JSON/authData, prehash confusion, certificate trust or success before verification.
2. Enforce adopted packed SELF attStmt alg=-7/sig and no x5c. Refuse none, certificate-backed and other unsupported formats/algorithms/shape; no fallback or extra ceremony. The public-point-only unsigned control must not succeed under this profile.
3. Preserve all prior checked length/arithmetic/canonical/duplicate/trailing/point/ID/RP/UP/UV/challenge/origin checks. Inspect CBOR/COSE/DER boundary/error paths and statement ordering; keep strictness within the declared profile and give concrete failure examples for findings.
4. Independently assess RETURN J1 error mapping, J2 attStmt ordering, J3 statement/signature check ordering and J4 high-S handling against the closed variants/adopted scope/primary specification. These are implementation interpretations, not new owner policy. Signature failure cannot produce a registration result.
5. Exactly four allowed paths changed, lib.rs executable modules/re-exports/public API unchanged, assertion/auth-data/client-JSON/error implementation plus entire assertion/crosscheck files unchanged. Helper delegation must preserve inherited outputs and independent fixture construction; do not trust a production parser as oracle.
6. All nine registration methods and inherited11 assertion methods remain meaningful. Verify foreign signer, missing/bad signature, signed transcript byte changes (including semantically equal JSON), candidate key/ID substitutions, alg mismatch, unsigned none and certificate rejection, plus valid baseline. Assess the retained signature-guard negative/restored control; add a small independent sensitivity/boundary control in own scratch if needed, no new framework or exhaustive fuzzing.
7. Module policy documentation must stay possession-only, with no hardware/manufacturer/biometric/universal-passkey or real-device enablement claim. Caller state/scope/generation/backup/challenge/authorization/activation duties stay separate. Physical/browser qualification and W05 independence remain held.

## Independent checks and receipts

Follow task and AGENTS.md §11 required checks once at exact source. Existing private .env.local without trace/credential output; verify lane PostgreSQL availability, no service start/stop/delete. Preserve jobs2/incremental0/configured sccache/own target and shared machine limits.
All compiling/testing/Clippy commands serially through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/.codex/worktrees/w01-webauthn-r1-review-a/repo/ws01-core-security -- COMMAND
No nesting or floor bypass; >=20GiB remains required.
Required: fmt --all --check; workspace Clippy --all-targets --all-features --locked -- -D warnings; crate tests --locked; workspace tests --locked; canonical check_spec.py with p0_executed:false; task named negatives and small independent sensitivity check; exact scope/signature/clean/hardcoding and redacted source/report/output scans per rules. Record actual argv/head/tree/exit/log hashes for each own invocation; do not lose exits to pipelines.

Author evidence /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo/ws01-core-security/target/a2-r1-evidence is READ-ONLY. Root pinned ten receipts at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T009-r1-RETURN-PROCESSING-20261107/return-verification.json; root-foreign-key-spotcheck.json records Root's one exact test0/1pass. Existing author crate20/workspace102 passes and1ignored are historical support, not your results. Preserve old r0 missing-receipt qualifications instead of repairing history. No source changes or author finalizers reused.
Author history scan exit1/826 reported public findings differs from r0's816 due to coordination history. Compare actual findings/paths/commit provenance, not a fixed count. No exclusions or globally clean-history claim. Preserve and classify nonzero results; new unknown findings must be reported. Any author final evidence/RETURN scan statements lacking separate receipts remain statements; record your own current scan receipts.
Check W05 fixture availability. Its optional cross-check stays explicitly ignored only under the task's fixture-unavailable exception; never report it passed without fixtures. Do not generate/read-write W05 implementation or test real devices here.

If a concrete blocker is established, retain the repro and stop unnecessary expensive follow-on checks; do not patch the candidate, weaken criteria or rerun blindly. Apply §0.1 for repeated failures/missing prerequisites/scope conflicts.

## Outputs and verdict

Write only NEW /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T009-WEBAUTHN-VERIFY-r1-a-A3_REVIEW.md and own ignored /Users/omkar/.codex/worktrees/w01-webauthn-r1-review-a/repo/ws01-core-security/target/a3-r1-evidence. No candidate/RETURN/old report/Root memory edits. Include exact target/full-base/repair-base, independence, criterion/test mapping, findings and repros, actual exits/counts/skips, policy/resource/scan limitations and MEMORY_UPDATES.
Give SOURCE_ONLY PASS/PASS_WITH_NOTES, REPAIR_REQUIRED or BLOCKED under project rules. Explicitly assess F1 resolution for this exact source; no prior tests or contract recommendation alone equals acceptance. Source pass does not establish device/browser interoperability, remote CI, authority activation or P0 readiness.
No role appointment, source repair, dependencies, frozen contract changes, VM/provider/device operations, cleanup, push/PR/merge or consumer unlock. Root alone adjudicates and routes. Entire final response exactly one text code block with verdict, absolute report path, WORK LOCATION and complete Root/A1-resume prompt.
