# W01-T009-WEBAUTHN-VERIFY r1 — registration proof repair

PROTOTYPE — DISPOSABLE — NOT PRODUCTION. Root remains A1. Fresh HUMAN-hosted A2; human selects model/settings. No role launched.

FOLDER: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo/ws01-core-security
WORKTREE: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo
BRANCH: ws01/w01-t009-webauthn-verify-r1
BASE / initial HEAD: 305cbfd838776a9b894fdd44a2687ad542a59eff
BASE_TREE: bf5535a9036fdf35f659d43eb3f7c549d0e858b5
STATUS: created clean, existing private lane environment linked. CREATE: NOT_NEEDED. Verify actual pins/status first; never reset/stash/delete on mismatch.

## Authority and inputs

Read COMPLETE /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/CONTRACT_CHANGE_REQUESTS/CCR-W01-002-OWNER-DECISION.md/SHA256ba2f80f275e74ba8b623431cfad26f72c50aff9ae8496576e7384090208dd821 and /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/CCR-W01-002-ADOPTED-PROFILE-NOTICE.md. Owner explicitly answered “Approve restricted profile (Recommended)” after independent CCR review. This adopts a restricted task/profile correction, not frozen-contract changes or source acceptance.
Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md §§13.2/14.4/15, memory.md, HANDOFFS/A1_HANDOFF_029.md, this full dispatch and amended task. Read original r0 RETURN/blocked r0-a source REVIEW, exact unchanged CCR and independent contract review at their decision pins. F1 is an absent new-key proof; no binding caller proof was found. Read frozen ENCODING/doc52/schema and primary W3C packed-self section. Use live canonical coordination; candidate main-snapshot coordination is historical. Human role settings and existing helper policy apply.

## Prospectively amended task — verbatim

### W01-T009-WEBAUTHN-VERIFY
- **TASK_ID:** W01-T009-WEBAUTHN-VERIFY
- **OBJECTIVE:** **In plain words:** Write one library that checks WebAuthn (fingerprint/security-key) logins and approvals exactly as the standard requires. The approval, credential and resolution services all use this single checker instead of three separate ones. **Precisely:** Implement `mc-webauthn-verify`, one shared verification library used by the AA, credential lifecycle and resolution units: parse and verify `webauthn.get` assertions and `webauthn.create` registrations (owner-adopted restricted ES256/P-256 packed self attestation; unsigned none rejected) exactly per ENCODING.md and doc 52.
- **WHY IT EXISTS:** Three authorities verify ceremonies; one reviewed implementation avoids three subtly different ones (G-055 critical parser).
- **ARCHITECTURE REQUIREMENTS:** ENCODING WebAuthn rules (challenge = D(…) as base64url without padding; ES256 over authData || SHA-256(clientDataJSON)); 52 ceremony checks; 36 focused review of crypto parsers.
- **P0 CLAIMS:** supports P0-09B, P0-09C, P0-10J.
- **INPUT CONTRACTS:** raw bytes only (authenticatorData, clientDataJSON, signature, public key COSE/SEC1).
- **OUTPUT CONTRACTS:** `verify_assertion(expected: &ExpectedAssertion { challenge: [u8;32], origin: &str, rp_id: &str, require_uv: true, credential_public_key, prior_sign_count }, raw) -> Result<VerifiedAssertion { sign_count, flags }, WebAuthnError>`; `verify_registration(expected, raw) -> Result<VerifiedRegistration { credential_id, public_key }, WebAuthnError>`; WebAuthnError: Type, Challenge, Origin, CrossOrigin, RpIdHash, UserPresent, UserVerified, Signature, Counter, Format, Attestation. **Exact public API (names and signatures are binding; do not rename):** `pub struct ExpectedAssertion<'a> { pub challenge: [u8; 32], pub origin: &'a str, pub rp_id: &'a str, pub credential_public_key: &'a [u8], pub prior_sign_count: u32 }` (UV always required, no field to disable it); `pub struct RawAssertion { pub authenticator_data: Vec<u8>, pub client_data_json: Vec<u8>, pub signature: Vec<u8>, pub credential_id: Vec<u8>, pub user_handle: Option<Vec<u8>> }`; `pub fn verify_assertion(e: &ExpectedAssertion, r: &RawAssertion) -> Result<VerifiedAssertion, WebAuthnError>`; `pub struct VerifiedAssertion { pub sign_count: u32, pub counter_anomaly: bool }`; `pub struct ExpectedRegistration<'a> { pub challenge: [u8; 32], pub origin: &'a str, pub rp_id: &'a str }`; `pub fn verify_registration(e, raw: &RawRegistration) -> Result<VerifiedRegistration, WebAuthnError>`; `pub struct VerifiedRegistration { pub credential_id: Vec<u8>, pub public_key_sec1: Vec<u8> }`; `pub enum WebAuthnError { Type, Challenge, Origin, CrossOrigin, RpIdHash, UserPresent, UserVerified, Signature, Counter, Format, Attestation }`.
- **EXACT WRITE SCOPE:** `ws01-core-security/crates/mc-webauthn-verify/src/**`, `ws01-core-security/crates/mc-webauthn-verify/tests/**`
- **READ-ONLY DEPENDENCIES:** none. Cross-check fixtures from W05-T008 are used when available, never required.
- **PRECONDITIONS:** W01-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws01-core-security/`):** `crates/mc-webauthn-verify/src/lib.rs (owned)`; `crates/mc-webauthn-verify/src/{client_data.rs,auth_data.rs,assertion.rs,registration.rs,error.rs}`; `crates/mc-webauthn-verify/tests/{assertion.rs,registration.rs,crosscheck.rs}`; `crates/mc-webauthn-verify/tests/support/soft_authenticator.rs`. **Steps:** 1. clientDataJSON: exact `type`, challenge byte-equal, exact origin, `crossOrigin` absent/false. 2. authData: rpIdHash = SHA-256(rp_id), UP and UV bits set, sign count rule (counter anomaly reported as Counter). 3. ES256 signature over authData || SHA-256(clientDataJSON) with p256. 4. Registration: require owner-adopted packed self ES256/P-256 alg=-7/sig with no x5c. Verify the new credential signature over exact authenticatorData || SHA-256(exact clientDataJSON); retain all ID/key/parser/RP/UV checks and reject unsigned none/certificate/unsupported formats. Document the restricted profile and lack of hardware/biometric assurance in the module; caller policy/authority remains separate. 5. Tests use an in-test software authenticator (p256 key) to build valid ceremonies, then one-field mutations.
- **SECURITY INVARIANTS:** Order: type → challenge → origin → rpIdHash → flags → signature; no partial acceptance.
- **FAILURE CASES:** Origin prefix match instead of exact; UV not required.
- **TESTS REQUIRED:** `cargo test -p mc-webauthn-verify`; when W05-T008 fixtures exist, they verify identically (cross-implementation check). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `assertion::valid_ceremony_verifies`; `assertion::wrong_type`; `assertion::wrong_challenge`; `assertion::origin_prefix_is_not_equal`; `assertion::cross_origin_true`; `assertion::wrong_rp_id_hash`; `assertion::up_missing`; `assertion::uv_missing`; `assertion::signature_bit_flip`; `assertion::counter_regression_flags_anomaly`; `assertion::malformed_auth_data`; `registration::valid_packed_self_attestation`; `registration::unsupported_attestation_rejected`; `registration::wrong_challenge`; `registration::unsigned_none_rejected`; `registration::missing_or_invalid_self_signature`; `registration::self_signature_by_other_key_rejected`; `registration::algorithm_mismatch_rejected`; `registration::signed_transcript_mutations_rejected`; `registration::certificate_backed_packed_rejected`;  `crosscheck::w05_fixtures_agree (#[ignore] until W05-T008 fixtures exist; then enabled)`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** One named test per WebAuthnError variant.
- **ACCEPTANCE CRITERIA:** Valid ceremony verifies; every single-field mutation fails with the right variant.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Credential state, challenges, nonce consumption (callers).
- **BLOCKERS:** r0 F1 requires authorized r1 proof repair and fresh independent source validation; no activation.
- **A3 REVIEW FOCUS:** Exact comparisons/order/strict parsing and actual same-new-key create signature over exact bytes; no unsigned fallback or hardware-trust overclaim. CCR-W01-002 owner decision applies prospectively; r0 criteria/results stay historical.


## EXACT narrower write set (relative to worktree)

ws01-core-security/crates/mc-webauthn-verify/src/registration.rs
ws01-core-security/crates/mc-webauthn-verify/src/lib.rs — module policy docs only, unchanged public API/re-exports
ws01-core-security/crates/mc-webauthn-verify/tests/registration.rs
ws01-core-security/crates/mc-webauthn-verify/tests/support/soft_authenticator.rs

All assertion/auth-data/client-JSON/error implementations, all11 inherited assertion methods and crosscheck.rs stay unchanged. No manifests/lock/dependencies, generated schema/architecture, consumer/UI/server/config or new ceremony. Need outside scope: return a precise request, no workaround.

## Smallest complete repair

Add only supported packed-self statement parsing and signature verification to existing registration validation. Require ES256 alg matching the credential COSE key and verify DER signature with the newly extracted credential key over exact authenticatorData || SHA256(exact JSON). No unrelated signer, caller key override, certificate trust/chain parsing or retrieval. Reject x5c/unsigned none/unsupported profile. Preserve checked lengths, canonical/duplicate/trailing/point/ID/RP/origin/UP/UV guards and use the existing closed error variants consistently.
Reuse p256/sha2 patterns and the synthetic builder. Module docs state restricted profile, possession-only trust, unsupported cells disabled, and caller backup/credential/challenge/authority duties. No hardware/biometric or broader backup-policy claim.
Prospectively revise the two registration expectations explicitly authorized by the owner; immutable r0 originals remain. Keep wrong_challenge and all assertion negatives. Add named small in-memory signed-self, unsigned-none, missing/bad/foreign signature, alg/COSE, exact-transcript and certificate negatives. Cover changed authData/key/ID and semantically equal but byte-different JSON against an old signature. Assert specific existing error reasons. Independent test construction; production parser is not its own oracle. No big framework/exhaustive fuzzing.
One small removed/restored signature-guard control must make a bad/foreign-signature negative fail when verification is removed, then restored source pass with matching saved hash. Own scratch only; actual commands/exits/hashes retained. Stop on same symptom twice or three failed hypotheses under §0.1.

## Required final validation

Use own target/a2-r1-evidence; preserve old failures. Record command/HEAD/tree/actual exit/log hash, including restore proof, to avoid repeating missing raw receipts. Run final required checks on signed exact commit. Existing private .env.local without trace/output; PostgreSQL availability per §11, no start/stop/delete. Keep jobs2/incremental0/configured sccache/own target and unchanged machine limits.
All compiling/testing/Clippy commands serially through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo/ws01-core-security -- COMMAND
No nested wrapper; >=20GiB floor remains.
Required: cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings; cargo test -p mc-webauthn-verify --locked; cargo test --workspace --locked; canonical check_spec.py with p0_executed:false; named task negatives and signature-guard control; redacted repair-range/full-task-range/history/evidence/RETURN gitleaks per rules with actual exits/classification, no exclusions/false clean-history; hardcoding/scope/signature/clean checks.
Optional W05 cross-check retains the task's fixture-unavailable exception only; report current availability/skip, do not write TS fixtures or claim real-device compatibility. Focused implementation iterations within stop rules are allowed; don't repeat a passing full suite without a new change/failure justifying it.

## Stop/completion/return

STOP if proof needs fallback, certificate trust, new ceremony/API/schema/authority, missing deps, unapproved scope/resources or a contract disagreement. Preserve evidence; owner decision cannot waive security or frozen scope.
COMPLETE: minimal four-file repair, required named controls/docs, final checks and signature sensitivity pass; exact in-scope signed commit clean with receipts and full RETURN. READY_FOR_INDEPENDENT_SOURCE_A3 only; no acceptance/activation/push/PR/merge/role launch.
Write /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T009-WEBAUTHN-VERIFY-r1-A2_RETURN.md using existing template: criteria mapping, exact base/head/tree/scope, actual command exits/log hashes, failures/skips, policy limits and MEMORY_UPDATES. Do not edit other coordination. Return to Root/A1 at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination; Root routes fresh independent source A3. Entire final response exactly one text block including result, RETURN, WORK LOCATION and complete Root-resume prompt.
