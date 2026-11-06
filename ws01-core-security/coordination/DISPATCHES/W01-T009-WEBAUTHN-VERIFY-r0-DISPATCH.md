# W01-T009-WEBAUTHN-VERIFY r0 — A2 dispatch

PROTOTYPE — DISPOSABLE — NOT PRODUCTION. Root remains A1. Human hosts A2 and chooses model/settings. This message appoints only the human-hosted implementation role; A2 returns to A1 and does not appoint A3.

## Verified work location and authority

WORKTREE: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo
FOLDER: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ws01-core-security
BRANCH: ws01/w01-t009-webauthn-verify-r0
BASE_SHA / initial HEAD: 0ed316f35e1cde00563067132e1b9726a0ddcadb
BASE_TREE: 8971f0679d14152487e3d36e0e2d65ba91b32296
STATUS: created, clean, private lane environment symlink ready. CREATE: NOT_NEEDED. Never reset/stash/delete to reconcile a mismatch.

Owner: “everything is merged lets move to next implementation task.” PRs 11/12/13 are merged and their source ancestors are verified at this base. This task depends only on integrated W01-T000, merge 9a7922bf29c0a6689903b96896dab8851d1eeeeb, verified ancestor of BASE. T007's final CI-inclusive review qualification does not gate T009. Human-hosted roles; three active A2 globally, one per lane and one executing build. Prepared, not launched; no start time or acceptance is claimed.

Why this task first: T011's exact API re-exports RawAssertion from this crate, whose current lib.rs is an empty scaffold. Completing T009 publishes that real type without a temporary duplicate or widening T011's scope.

## Read first

Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md §§13.2/14.4/15, memory.md, task.md T009, and HANDOFFS/A1_HANDOFF_025.md. Read the repository SKILL.md and SUBAGENT_POLICY.md. Human settings supersede stale role-model wording; ordinary helper rules still apply and require honest telemetry. Use the applicable engineering skill, retaining project scope/independence.

Read-only canonical inputs: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ARCHITECTURE/V2_1/SCHEMAS/ENCODING.md, 52_APPROVAL_AUTHORITY_CONTRACT.md and the task's cited requirements. Use exact received clientDataJSON bytes. Follow the frozen trust-boundary checks and error order; never invent a success policy to satisfy a test. Resolve counter/backup/unsupported-cell semantics from the canonical contract; do not silently ignore anomalies. Credential lifecycle/challenge consumption and current credential state are callers' responsibilities, not new services in this crate.

## Original task — copied verbatim

### W01-T009-WEBAUTHN-VERIFY
- **TASK_ID:** W01-T009-WEBAUTHN-VERIFY
- **OBJECTIVE:** **In plain words:** Write one library that checks WebAuthn (fingerprint/security-key) logins and approvals exactly as the standard requires. The approval, credential and resolution services all use this single checker instead of three separate ones. **Precisely:** Implement `mc-webauthn-verify`, one shared verification library used by the AA, credential lifecycle and resolution units: parse and verify `webauthn.get` assertions and `webauthn.create` registrations (attestation "none" only) exactly per ENCODING.md and doc 52.
- **WHY IT EXISTS:** Three authorities verify ceremonies; one reviewed implementation avoids three subtly different ones (G-055 critical parser).
- **ARCHITECTURE REQUIREMENTS:** ENCODING WebAuthn rules (challenge = D(…) as base64url without padding; ES256 over authData || SHA-256(clientDataJSON)); 52 ceremony checks; 36 focused review of crypto parsers.
- **P0 CLAIMS:** supports P0-09B, P0-09C, P0-10J.
- **INPUT CONTRACTS:** raw bytes only (authenticatorData, clientDataJSON, signature, public key COSE/SEC1).
- **OUTPUT CONTRACTS:** `verify_assertion(expected: &ExpectedAssertion { challenge: [u8;32], origin: &str, rp_id: &str, require_uv: true, credential_public_key, prior_sign_count }, raw) -> Result<VerifiedAssertion { sign_count, flags }, WebAuthnError>`; `verify_registration(expected, raw) -> Result<VerifiedRegistration { credential_id, public_key }, WebAuthnError>`; WebAuthnError: Type, Challenge, Origin, CrossOrigin, RpIdHash, UserPresent, UserVerified, Signature, Counter, Format, Attestation. **Exact public API (names and signatures are binding; do not rename):** `pub struct ExpectedAssertion<'a> { pub challenge: [u8; 32], pub origin: &'a str, pub rp_id: &'a str, pub credential_public_key: &'a [u8], pub prior_sign_count: u32 }` (UV always required, no field to disable it); `pub struct RawAssertion { pub authenticator_data: Vec<u8>, pub client_data_json: Vec<u8>, pub signature: Vec<u8>, pub credential_id: Vec<u8>, pub user_handle: Option<Vec<u8>> }`; `pub fn verify_assertion(e: &ExpectedAssertion, r: &RawAssertion) -> Result<VerifiedAssertion, WebAuthnError>`; `pub struct VerifiedAssertion { pub sign_count: u32, pub counter_anomaly: bool }`; `pub struct ExpectedRegistration<'a> { pub challenge: [u8; 32], pub origin: &'a str, pub rp_id: &'a str }`; `pub fn verify_registration(e, raw: &RawRegistration) -> Result<VerifiedRegistration, WebAuthnError>`; `pub struct VerifiedRegistration { pub credential_id: Vec<u8>, pub public_key_sec1: Vec<u8> }`; `pub enum WebAuthnError { Type, Challenge, Origin, CrossOrigin, RpIdHash, UserPresent, UserVerified, Signature, Counter, Format, Attestation }`.
- **EXACT WRITE SCOPE:** `ws01-core-security/crates/mc-webauthn-verify/src/**`, `ws01-core-security/crates/mc-webauthn-verify/tests/**`
- **READ-ONLY DEPENDENCIES:** none. Cross-check fixtures from W05-T008 are used when available, never required.
- **PRECONDITIONS:** W01-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws01-core-security/`):** `crates/mc-webauthn-verify/src/lib.rs (owned)`; `crates/mc-webauthn-verify/src/{client_data.rs,auth_data.rs,assertion.rs,registration.rs,error.rs}`; `crates/mc-webauthn-verify/tests/{assertion.rs,registration.rs,crosscheck.rs}`; `crates/mc-webauthn-verify/tests/support/soft_authenticator.rs`. **Steps:** 1. clientDataJSON: exact `type`, challenge byte-equal, exact origin, `crossOrigin` absent/false. 2. authData: rpIdHash = SHA-256(rp_id), UP and UV bits set, sign count rule (counter anomaly reported as Counter). 3. ES256 signature over authData || SHA-256(clientDataJSON) with p256. 4. Registration: attestation format "none" only; extract credential ID and key. 5. Tests use an in-test software authenticator (p256 key) to build valid ceremonies, then one-field mutations.
- **SECURITY INVARIANTS:** Order: type → challenge → origin → rpIdHash → flags → signature; no partial acceptance.
- **FAILURE CASES:** Origin prefix match instead of exact; UV not required.
- **TESTS REQUIRED:** `cargo test -p mc-webauthn-verify`; when W05-T008 fixtures exist, they verify identically (cross-implementation check). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `assertion::valid_ceremony_verifies`; `assertion::wrong_type`; `assertion::wrong_challenge`; `assertion::origin_prefix_is_not_equal`; `assertion::cross_origin_true`; `assertion::wrong_rp_id_hash`; `assertion::up_missing`; `assertion::uv_missing`; `assertion::signature_bit_flip`; `assertion::counter_regression_flags_anomaly`; `assertion::malformed_auth_data`; `registration::valid_none_attestation`; `registration::non_none_attestation_rejected`; `registration::wrong_challenge`; `crosscheck::w05_fixtures_agree (#[ignore] until W05-T008 fixtures exist; then enabled)`. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** One named test per WebAuthnError variant.
- **ACCEPTANCE CRITERIA:** Valid ceremony verifies; every single-field mutation fails with the right variant.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Credential state, challenges, nonce consumption (callers).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Exact comparisons; check order; no lenient parsing.


## Authorized implementation boundary

Only /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ws01-core-security/crates/mc-webauthn-verify/src/** and tests/**. The public names/signatures in the task bind this attempt. Reuse installed pinned p256/base64/sha2/serde_json/thiserror; the crate declares these already. No Cargo.toml/Cargo.lock changes, generated schema edits, caller rewrites or parallel fake verifier. If a real dependency is absent, return its precise DEPENDENCY_REQUEST rather than bypassing the scope. No network identity/provider service, browser automation, production claims, UI, VM, new configuration surface or extra framework. Keep it small: one shared verifier, minimal parsing helpers and the specified synthetic controls.

Do not implement later API-freeze or authority-state tasks here. Do not stub verification with success, weaken checks, silence errors or manufacture expected bytes with the code under test. Use independently assembled synthetic p256 ceremonies, exact byte mutation controls and one small removed/restored guard check. Do not create repetitive test scaffolding beyond what demonstrates the task criteria.

## Required verification and evidence

Use target/a2-evidence for logs/results; preserve failures. Source the existing private lane .env.local only to run required commands, with no shell trace or credential output. Keep CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0, the configured sccache and this worktree's own target. Do not start/stop shared services. Check the existing lane PostgreSQL service through the command in AGENTS.md §11 before workspace tests; missing service is a concrete environment blocker, not license for fake DB success.

Run every build/test/Clippy command serially through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ws01-core-security -- COMMAND
Do not nest wrapper invocations. It enforces the one shared build slot and 20 GiB free-space floor; do not bypass it. The separate old VM disk/RAM/timing gates are not this Rust library's acceptance criteria.

Required final checks:
1. cargo fmt --all --check
2. cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
3. cargo test -p mc-webauthn-verify --locked (every named task test and negative reason)
4. cargo test --workspace --locked (inherited coverage)
5. python3 /Users/omkar/.codex/worktrees/w01-webauthn-verify-r0/repo/ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py; retain p0_executed:false.
6. Small guard-removal/restored check in own scratch only; restore and verify unchanged candidate before final checks/commit. No blind repeated mutations or exhaustion tests.
7. Task range and final evidence/RETURN redacted gitleaks plus AGENTS.md §11's history scan. Preserve/classify any historical public-schema findings; do not create blanket exclusions or claim an old failed scan was clean. Inspect the diff for hardcoded secrets, paths or per-machine configuration.
8. Exact final BASE..HEAD write set, signed commit verification, HEAD/tree and clean status. Use commit prefix [W01][W01-T009-WEBAUTHN-VERIFY][r0]. Run final required checks on that exact immutable commit; don't amend a returned commit.

The optional W05 cross-check is ignored only under the task's explicit fixture-unavailable rule; record its exact availability and skip, never report it passed without fixtures. Record actual test counts, including ignored cases; do not call BUILD tests P0 execution.

## Stop and completion

STOP if scope/contracts disagree, a required dependency/input is absent, a cap/credential/service is missing, the same symptom fails twice, three hypotheses fail, or a change would weaken a security boundary. Preserve evidence and return the exact blocker per AGENTS.md §0.1; do not retry blindly or expand the design.

COMPLETE means the specified assertion/registration APIs and synthetic checks are implemented within scope, final required checks pass on the signed exact commit, no required negative case is missing, checkout is clean, and a complete RETURN is written. This is READY_FOR_INDEPENDENT_SOURCE_A3, never self-acceptance, push/PR/merge or downstream unlock. No remote action is authorized by this dispatch.

Write /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T009-WEBAUTHN-VERIFY-r0-A2_RETURN.md, using the existing RETURN template. Include requirement/check mapping, exact SHA/tree/base/scope, command exits and logs, failures/limits, optional cross-check status, and MEMORY_UPDATES. Do not edit Root's task/memory or other coordination files. Return to Root/A1 at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination; A1 prepares fresh independent review. Put your entire final chat response in exactly one fenced text code block containing result, absolute RETURN path, WORK LOCATION and complete Root-resume prompt.
