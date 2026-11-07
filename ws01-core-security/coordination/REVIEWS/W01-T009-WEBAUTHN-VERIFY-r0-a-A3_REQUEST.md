# W01-T009-WEBAUTHN-VERIFY r0-a — independent source A3 request

PROTOTYPE — DISPOSABLE — NOT PRODUCTION. Root remains A1. Fresh HUMAN-hosted A3; human chooses model/settings. This written request does not launch a reviewer. No acceptance or remote action is granted.

## Exact immutable review target

FOLDER: /Users/omkar/.codex/worktrees/w01-webauthn-r0-review-a/repo/ws01-core-security
WORKTREE: /Users/omkar/.codex/worktrees/w01-webauthn-r0-review-a/repo
HEAD / IMPLEMENTATION_SHA: 305cbfd838776a9b894fdd44a2687ad542a59eff
TREE: bf5535a9036fdf35f659d43eb3f7c549d0e858b5
BASE / sole parent: 0ed316f35e1cde00563067132e1b9726a0ddcadb
BASE_TREE: 8971f0679d14152487e3d36e0e2d65ba91b32296
BRANCH: DETACHED
STATUS: existing, verified clean; private lane environment symlink ready. CREATE: NOT_NEEDED. Never reset/stash/delete on a mismatch. Verify HEAD/tree and cleanliness at the beginning and end.

Independence: you did not implement or dispatch this candidate. Do not reuse A2's session or Root's processing helpers t009_return_evidence/t009_contract_pointers; any reviewer helpers are fresh and read-only or bounded to your own review evidence. A2 disclosed no helpers. Root's one UV spotcheck is not an independent verdict.

## Read first and pins

Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md §§13.3/14.4/15, memory.md, task.md T009 and HANDOFFS/A1_HANDOFF_026.md. Read repository SKILL.md/SUBAGENT_POLICY.md and applicable review skills; their generic routines cannot override the human-hosted role or project scope.

Read the COMPLETE original dispatch /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T009-WEBAUTHN-VERIFY-r0-DISPATCH.md (SHA256 1276bde71b556742ddc67346c4f5946c7cc0a1e8656689738ecea663fa87f908) and COMPLETE author RETURN /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T009-WEBAUTHN-VERIFY-r0-A2_RETURN.md (SHA256 023ac41ed775733217221573292d8babf7eb6d9433437538b2268c1d8d18b6f4). The dispatch contains the original task fields verbatim. Verify both pins, the signed source commit, sole parent and exact ten-file write set. Review all source and tests, not only the changed lines.

Read frozen ENCODING.md, doc 52, schemas WebAuthnCeremonyV1/WebAuthnRegistrationCeremonyV1 and cited requirements under /Users/omkar/.codex/worktrees/w01-webauthn-r0-review-a/repo/ARCHITECTURE/V2_1. Do not edit architecture or generated records. Consult primary WebAuthn/CBOR/COSE specifications only as needed; keep project precedence explicit.

Root provenance/source hashes, fourteen retained author-artifact hashes and single exact UV spotcheck: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T009-r0-RETURN-PROCESSING-20261107/return-verification.json and root-uv-spotcheck.json. Author raw evidence: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ws01-core-security/target/a2-evidence. Never modify these files.

## Scope and review focus

Only the ten source/test paths under crates/mc-webauthn-verify are implemented. No dependency/manifest/lockfile change, caller implementation or production/P0 execution. Review:

1. Exact public API and all named task/negative tests. RawAssertion must remain the one shared exported type for later T011. No option may disable UV or replace verification with a caller verdict.
2. Exact received-byte signature input, unpadded base64url challenge equality, exact origin and cross-origin rejection, RP hash, UP/UV, signature verification, then counter handling. Check malformed and multi-fault paths, not just valid synthetic fixtures.
3. Handwritten CBOR/COSE reader: checked arithmetic and slice lengths, shortest heads, duplicate/missing/extra keys, invalid points/algorithm, trailing bytes, integer boundaries and absence of panic/unchecked allocations. Independently assess whether imposed map order and extension refusal are supported restrictions or reject required valid inputs. No mass fuzzing or exhaustive new framework is requested.
4. Client JSON duplicate detection, escaped keys/nested values and exact raw bytes; SEC1/DER handling and limits of the synthetic authenticator as an independent oracle.
5. Author interpretations are NOT yet adopted: I1 RawRegistration fields from the registered ceremony; I2 counter regression returns Err(Counter), so counter_anomaly is always false on Ok; I3 exact API fields override the introductory sketch; I4 uncompressed SEC1 key convention; I5 strict CBOR/order/extensions; I6 registration check order; I7 backup-policy and credential/session/challenge state left to callers. Map each to the binding task/canonical requirement; identify actual incompatibility or security defects concretely. Do not invent a second policy or approve a contract change.
6. Registration attestation none, credential-ID matching, ES256/P-256 key validation, and caller versus library responsibilities. Distinguish the library's verification result from authority minting or credential enrollment.

## Independent checks and honest receipts

Rerun the task and AGENTS.md §11 required checks once on the exact immutable target, plus small independent guard/boundary controls where needed. Keep code unchanged; broken/restored variants belong only in your own scratch. If a concrete blocker is established, preserve it and stop unnecessary costly follow-on checks rather than repairing the candidate.

Use existing private .env.local without trace or credential output. Do not start/stop shared PostgreSQL. Verify its availability as §11 specifies. Keep CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0, configured sccache and this worktree's own target. Every compiling/testing/Clippy command uses the shared wrapper, serially, never nested:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/.codex/worktrees/w01-webauthn-r0-review-a/repo/ws01-core-security -- COMMAND
The wrapper's >=20 GiB free-space floor stays unchanged.

Required: cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings; cargo test -p mc-webauthn-verify --locked; cargo test --workspace --locked; canonical check_spec.py with p0_executed:false; task negative tests and one small independent guard sensitivity control; write-set/hardcoding/signature/clean checks; redacted scans per AGENTS.md. Record exact command, HEAD/tree, exit and log hash for your own invocations. Do not lose exits through pipelines or fabricate historical receipts.

A2's crate log records14pass/1ignored, workspace log96pass/1ignored. Logs lack raw terminal exits; separate final-head.txt binds only HEAD, not each log/tree. guard-removed.log shows UV and origin controls failing as intended, but the restoration/hash receipt is only stated in RETURN. Current clean Git bytes and Root UV spotcheck are verified separately. Missing receipts are qualifications, not invented test failures; independent checks address current validation.

Full-history author scan is retained exit1/816 reported findings:810 pre-existing public type-name-list occurrences,6 W03 file-digest occurrences; zero in task range. Independently verify/classify evidence and required scans without exclusions or calling history clean. The author evidence/RETURN scan successes are reported but lack separate retained raw scan receipts. Retain nonzero scanner outcomes and distinguish public artifacts from secrets; no blanket allowlist.

W05-T008 fixtures were absent for A2. Verify current availability; the optional cross-check may remain explicitly ignored only under the task's own fixture-unavailable exception. Never claim it passed. No power/performance investigation is requested; A2's energy explanation is an unverified host-attribution claim, not acceptance evidence.

## Outputs, stop and handoff

Write ONLY a NEW report /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T009-WEBAUTHN-VERIFY-r0-a-A3_REVIEW.md and your ignored /Users/omkar/.codex/worktrees/w01-webauthn-r0-review-a/repo/ws01-core-security/target/a3-evidence outputs. Do not edit candidate files, either RETURN, Root memory/task or historical evidence. Use the existing REVIEW template. Include exact target/baseline, independence, criterion/test mapping, concrete findings, actual exits/counts/skips, limitations and MEMORY_UPDATES. PASS/PASS_WITH_NOTES, REPAIR_REQUIRED or BLOCKED must be scoped to SOURCE_ONLY; no remote CI, browser/device interoperability or production/P0 acceptance is inferred.

Stop under AGENTS.md §0.1 on repeated failures, missing prerequisites, conflicting requirements or scope pressure. No helpers/roles launched on Root's behalf, no source fixes, new dependencies, VM/native operations, cleanup, push/PR/merge or consumer unlock. Root alone processes findings and acceptance. Entire final response exactly one fenced text code block including verdict, absolute report path, WORK LOCATION and complete Root/A1-resume prompt.
