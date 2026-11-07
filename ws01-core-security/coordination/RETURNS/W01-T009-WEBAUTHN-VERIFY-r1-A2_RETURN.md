PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

```
FROM: W01-A2[W01-T009-WEBAUTHN-VERIFY-r1]
TO: Root A1 (acting W01-A1)
MESSAGE_TYPE: RETURN
TASK_ID: W01-T009-WEBAUTHN-VERIFY
ATTEMPT: r1
DISPOSITION: READY_FOR_A3_REVIEW (fresh independent source A3 only; no acceptance/activation)
DATE_UTC: 2026-11-07T08:21:33Z
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo
  BRANCH: ws01/w01-t009-webauthn-verify-r1
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: ws01-core-security/coordination/
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e

TASK_ID: W01-T009-WEBAUTHN-VERIFY
DISPATCH: coordination/DISPATCHES/W01-T009-WEBAUTHN-VERIFY-r1-REGISTRATION-PROOF-REPAIR-DISPATCH.md (SHA256 04efdc959446aefd2d0a0cb95755ec7be7372fddd720ede691c5f8b24cb0bd3b, verified at start)
AUTHORITY READ: CCR-W01-002-OWNER-DECISION.md (SHA256 ba2f80f275e74ba8b623431cfad26f72c50aff9ae8496576e7384090208dd821, verified), NOTICES/CCR-W01-002-ADOPTED-PROFILE-NOTICE.md, CCR-W01-002.md, HANDOFFS/A1_HANDOFF_029.md, r0 RETURN, r0-a A3 REVIEW (F-1), AGENTS.md, SKILLS.md §13.2/§14.4/§15, memory.md.
BASE_SHA: 305cbfd838776a9b894fdd44a2687ad542a59eff
BASE_TREE: bf5535a9036fdf35f659d43eb3f7c549d0e858b5   (verified at start; worktree clean)
IMPLEMENTATION_SHA: b9af738fd9779be0f3e02351a4cfba863db2711a
TREE: 17b33ab470340a11db143c6ccca530c7a74d3eca
SOLE_PARENT: 305cbfd838776a9b894fdd44a2687ad542a59eff
COMMITS:
  b9af738fd9779be0f3e02351a4cfba863db2711a  [W01][W01-T009-WEBAUTHN-VERIFY][r1] Require packed self ES256 new-key proof at registration
  SIGNATURE: git verify-commit → exit 0, Good "git" signature, ED25519 SHA256:4wK1HTnklplUc/y8C9zm8zYvTVwiMVQxGI3HVqdwh/8
CHANGED_FILES (git diff --stat BASE..IMPLEMENTATION_SHA):
  .../crates/mc-webauthn-verify/src/lib.rs           |  23 +-
  .../crates/mc-webauthn-verify/src/registration.rs  |  47 +++-
  .../mc-webauthn-verify/tests/registration.rs       | 297 +++++++++++++++++++--
  .../tests/support/soft_authenticator.rs            |  50 +++-
  4 files changed, 374 insertions(+), 43 deletions(-)
WRITE_SET_CHECK: PASS — exactly the four dispatch-permitted paths. lib.rs: module doc comment only (mod list and pub use re-exports byte-identical). No change to assertion.rs, auth_data.rs, client_data.rs, error.rs, tests/assertion.rs, tests/crosscheck.rs, Cargo.toml, Cargo.lock, .env.example, schemas, ARCHITECTURE, consumers or coordination on the task branch.

WHAT CHANGED:
  - registration.rs: none_attestation_auth_data → packed_self_attestation. attestationObject stays strict CTAP2-canonical {fmt, attStmt, authData}. fmt must be "packed" (else Attestation). attStmt must be exactly {"alg": -7, "sig": bstr} in canonical order. The cases 0 entries, >2 entries (so x5c/certificate-backed), a first key other than "alg", alg ≠ -7, and a second key other than "sig" all return Attestation. {"alg": -7} alone returns Signature. After the unchanged rpIdHash/UP/UV/AT/credential-ID/COSE-ES256 key/trailing-byte/point checks, the DER sig is verified (p256 VerifyingKey::verify, same pattern as assertion.rs) with the NEWLY EXTRACTED credential key over the exact received authenticatorData bytes || SHA-256(exact received clientDataJSON bytes). Bad DER or a verify failure returns Signature. No re-encoding, no caller key, no fallback, no certificate parsing.
  - lib.rs module docs: restricted profile (CCR-W01-002), possession-only trust, no hardware/biometric/provenance/universal-passkey claim, no cell enabled, only the BS-without-BE rejection (no broader backup policy), and the caller duties (state/generation/scope, backup policy, user handle/ID binding, challenge freshness/expiry/one-use, enrollment authorization/quorum, activation).
  - soft_authenticator.rs: additive helpers only (KEY/OTHER_KEY scalars, public_sec1_of, sign_as, cose_key_of, attested_credential_with, packed_stmt). Existing functions keep identical output (now delegating); all 11 assertion tests still pass unchanged.
  - tests/registration.rs: the two owner-authorized r0 expectations were revised prospectively: valid_none_attestation → valid_packed_self_attestation, and non_none_attestation_rejected → unsupported_attestation_rejected plus unsigned_none_rejected. wrong_challenge is kept (now correctly self-signed over the wrong challenge). The r0 originals stay immutable at 305cbfd.

REQUIREMENT → EVIDENCE (amended task step 4 / owner decision / dispatch):
  packed self ES256/P-256, alg=-7, sig, no x5c → registration::valid_packed_self_attestation (plus a follow-on assertion with the registered key)
  new credential key verifies sig over exact authData || SHA256(exact JSON) → registration::self_signature_by_other_key_rejected, registration::signed_transcript_mutations_rejected, registration::missing_or_invalid_self_signature, guard control below
  unsigned none rejected → registration::unsigned_none_rejected (r0-accepted none+{} → Attestation; none + a valid self stmt → Attestation)
  certificate-backed rejected → registration::certificate_backed_packed_rejected ({alg,sig valid,x5c} → Attestation)
  unsupported formats/cells rejected → registration::unsupported_attestation_rejected (fido-u2f, tpm, android-key, android-safetynet, apple, "Packed"; packed+{}; packed+{sig} → Attestation)
  alg must match the credential COSE key → registration::algorithm_mismatch_rejected (stmt alg -8/-35/-257 → Attestation; stmt -7 with a COSE key alg -8, correctly self-signed → Format via the unchanged COSE guard)
  changed authData/key/ID and semantically equal, byte-different JSON vs an old signature → signed_transcript_mutations_rejected (appended "crossOrigin":false; inserted whitespace; sign count 0→1; BE flag added; credential ID changed consistently in authData and raw; credential key swapped → Signature each. The baseline replay of the unchanged transcript is Ok)
  foreign signer / no caller key override → self_signature_by_other_key_rejected (key A attested + B signed → Signature; B attested + A signed → Signature; B attested + B signed → Ok control)
  challenge / existing guards preserved → registration::wrong_challenge (Challenge) + unchanged assertion suite 11/11
  order type → challenge → origin → rpIdHash → flags → signature: the signature is checked last in verify_registration. Statement-shape checks (Attestation) stay before rpIdHash, at the same position as r0's attestation check.
  One named test per WebAuthnError variant: unchanged (assertion tests cover Type…Format; registration covers Attestation and also Signature/Challenge/Format)

TESTS_ADDED / REVISED (crates/mc-webauthn-verify/tests/registration.rs; all PASS at IMPLEMENTATION_SHA):
  - registration::valid_packed_self_attestation — POSITIVE
  - registration::unsupported_attestation_rejected — NEGATIVE (Attestation)
  - registration::wrong_challenge — NEGATIVE (Challenge)
  - registration::unsigned_none_rejected — NEGATIVE (Attestation)
  - registration::missing_or_invalid_self_signature — NEGATIVE (Signature: missing sig, empty, non-DER, bit-flipped DER)
  - registration::self_signature_by_other_key_rejected — NEGATIVE (Signature) + positive control
  - registration::algorithm_mismatch_rejected — NEGATIVE (Attestation; Format for the COSE alg)
  - registration::signed_transcript_mutations_rejected — NEGATIVE (Signature ×6) + baseline Ok
  - registration::certificate_backed_packed_rejected — NEGATIVE (Attestation)
  Unchanged and passing: assertion::{valid_ceremony_verifies, wrong_type, wrong_challenge, origin_prefix_is_not_equal, cross_origin_true, wrong_rp_id_hash, up_missing, uv_missing, signature_bit_flip, counter_regression_flags_anomaly, malformed_auth_data}.
  crosscheck::w05_fixtures_agree — #[ignore], unchanged. W05-T008 fixtures are still unavailable, so the test is NOT RUN and not claimed. No TS fixtures were written; no real-device compatibility is claimed.
  Test independence: ceremonies are hand-assembled byte by byte (CBOR, COSE, authData, JSON) and signed with p256 SigningKey in the helper. The production parser is never used as the oracle.

SIGNATURE-GUARD REMOVAL / RESTORATION CONTROL (after commit, at HEAD b9af738f…, own scratch copy):
  saved registration.rs SHA256 253ae79b7ff4d4a0ddc3142f4596c1179eab698246b7ce5a727c66d9a90b7fee (working file == scratch copy)
  removed `key.verify(&signed, &signature).map_err(..)?;` (replaced with a no-op binding)
  python3 WORKSPACE/run_single_build.py --cwd <ws01> -- cargo test -p mc-webauthn-verify --locked --test registration → exit 101; FAILED: self_signature_by_other_key_rejected, signed_transcript_mutations_rejected, missing_or_invalid_self_signature; 6 others ok. log target/a2-r1-evidence/guard-removed.log SHA256 da9bf80a64ef6eed69dc626bcef0e0b3f5d042d862fe9dbb068c1439f35ac53b
  restored by copying from scratch → SHA256 253ae79b…7fee (match), git status --porcelain empty. Restored-source pass = final-3 below (exit 0, 9/9 registration). Hash record: target/a2-r1-evidence/guard-hashes.txt SHA256 ed7c75d97a7f91c0d9af8b0cc25de80e3e93318142a9137c2d19026c16a97d78. One run, not repeated.

TESTS_RUN (cwd /Users/omkar/.codex/worktrees/w01-webauthn-verify-r1/repo/ws01-core-security unless stated. The lane .env.local was sourced in a subshell with output suppressed and no trace. CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0, RUSTC_WRAPPER=sccache, the worktree's own target/. Every cargo command ran serially through python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd <ws01> -- <cmd>, with no nesting. Machine limits unchanged):
TEST_RESULTS (all at HEAD b9af738fd9779be0f3e02351a4cfba863db2711a / tree 17b33ab470340a11db143c6ccca530c7a74d3eca, status clean; logs in target/a2-r1-evidence/):
  - final-0 docker compose --env-file .env.local -f compose.yaml ps --status running → exit 0; mc-ws01-postgres-1 listed (not started/stopped by A2); log SHA256 9f593fe81fd10d9fd602562764d8315c5f52267c896f4dac934312525081f90e
  - final-1 cargo fmt --all --check → exit 0; SHA256 31c6199dba7eb52ddaa057696a58c1e6d258bf3dd7aed3cac0464e6e28bf2a90
  - final-2 cargo clippy --workspace --all-targets --all-features --locked -- -D warnings → exit 0, 0 warnings; SHA256 e3b05df2db3726694bb232471d8634f5c90d55098bb594903c8362b11670f76d
  - final-3 cargo test -p mc-webauthn-verify --locked → exit 0; 20 passed / 0 failed / 1 ignored (assertion 11, registration 9, crosscheck ignored); SHA256 d0389aad7513c73121998c1b58f503bd1147a00b212f081e02debc792382ec96
  - final-4 cargo test --workspace --locked → exit 0; 102 passed / 0 failed / 1 ignored, 83 result blocks; SHA256 3fdd7d149b2da98484073fb30e27290424ec3a5ac66df83c61097dcfa621b118
  - final-5 (cwd repo root) python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py → exit 0; "p0_executed": false; SHA256 be985494fb2a65dc8ba05da1153c26bce1481b678c79dc67d72d219a97a88b12
  - final-6 gitleaks detect --no-banner --redact --log-opts=305cbfd8..b9af738f (repair range) → exit 0, no leaks; SHA256 1d62a8dd7e14577624a00a2047aee92552a7c9378b17b33f32e2faebd8cd70d7
  - final-7 gitleaks detect --no-banner --redact --log-opts=0ed316f35e1cde00563067132e1b9726a0ddcadb..b9af738f (full task range r0+r1) → exit 0, no leaks; SHA256 9340e400dc326fae6b28e1415ae430a83edad9e1fb69317ee9d7c3aff355343f
  - final-8 gitleaks detect --no-banner --redact (full history, gitleaks default all refs) → exit 1, 826 findings, NOT clean. 820 generic-api-key on 2 distinct lines of ws01-core-security/coordination/task.md (same type-name-list classification as r0/T007; the count is above r0's 816 because later coordination commits re-carry those lines), and 6 generic-api-key in ws03-runner-sandbox-acquisition/coordination/RETURNS/W03-T000-WORKSPACE-SCAFFOLD-r0-A2_RETURN.md (file-digest lines). 0 in mc-webauthn-verify, 0 in commit b9af738f. No exclusion/allowlist was added. log SHA256 a9bc8993a84e0fa930320fc97c9526a6c34d9b565e922b1d67cde36dc181671f
  - dev-1 (pre-commit iteration) cargo fmt --all, then cargo test -p mc-webauthn-verify --locked → exit 0, 20/0/1; SHA256 36fbf6ef89ea6042209d61eba84d02aa132087f9f3d5421a70f13a571287fda9. Iteration only, not final evidence.
  - Scope: git diff --name-only BASE..HEAD = the 4 paths. Hardcoding grep (AGENTS.md §0.3 regex) over added diff lines → no hits (grep exit 1). The only origins are the existing synthetic https://aa.example.test fixtures, unchanged. git verify-commit HEAD → exit 0.
  RECEIPTS: target/a2-r1-evidence/receipts.jsonl, one JSON line per command: command, cwd, exit, log path, log SHA256, head, tree, status. Local and untracked (target/ is ignored). Evidence/RETURN gitleaks dir scan: result reported in the final chat block (it runs after this file is written).

ARCHITECTURE_CONTRACTS_USED:
  - 52_APPROVAL_AUTHORITY_CONTRACT.md:31 (registration verifies webauthn.create, fresh challenge, exact RP/origin, UV, supported algorithm and new-key proof; AA policy declares the supported attestation; unsupported cells reject)
  - SCHEMAS/ENCODING.md WebAuthn (ES256 over exact authenticatorData || SHA256(exact clientDataJSON); COSE ES256/P-256)
  - CCR-W01-002 owner decision (restricted packed self ES256 profile, possession-only trust limit)
  - W3C WebAuthn L3 §8.2 packed attestation, self-attestation verification procedure (alg matches credentialPublicKey; sig over authenticatorData || clientDataHash with the credential key); §8.7 none (empty attStmt, no signature)
  - RFC 9053 COSE ES256 = -7; RFC 8949 strict CBOR; SKILLS.md §14.4
SCHEMAS_USED:
  - WebAuthnRegistrationCeremonyV1 raw fields (credential_id, client_data_json, attestation_object). The signature lives inside attestation_object, so there is no wire/API change.
DOUBLES_USED: NONE

ASSUMPTIONS / INTERPRETATIONS (A3 please confirm):
  - J1 Error mapping (existing closed variants only). Statement-shape problems are Attestation: none, other fmt, attStmt not {alg,sig}, x5c present, alg ≠ -7. A missing sig, non-DER sig or failed verification is Signature. A COSE key alg ≠ -7 stays Format (unchanged r0 guard). Malformed CBOR is Format.
  - J2 attStmt ordering: CTAP2 canonical {"alg","sig"[, "x5c"]}. Other key orders reject (Attestation), consistent with r0 I5 strictness.
  - J3 Statement checks run before rpIdHash (same position as r0's attestation check). Signature verification runs last, after all parser/RP/UP/UV/ID/key checks. No partial result is returned on any failure.
  - J4 High-S signatures are accepted (p256 verify, same as the assertion path and WebAuthn).
  - r0 I1–I7 remain as previously stated, except that I6 is superseded for the attestation step.
KNOWN_LIMITATIONS:
  - PROTOTYPE_ONLY: registration accepts only packed self ES256/P-256. Authenticators or clients that produce none, strip attestation, or use certificate-backed/other formats are refused. That is the accepted compatibility limit; there is no fallback.
  - Possession-only proof at this boundary: no manufacturer provenance, trusted hardware, biometric/UV assurance or universal passkey support. No physical/browser cell is enabled or qualified. Real-device compatibility is UNVERIFIED.
  - Optional W05 cross-check NOT RUN (fixtures unavailable).
  - Sub-agents: none used (deviation from the normal 2–3 helper norm, §3c, disclosed). This was a four-file, tightly coupled crypto repair that I wrote and verified line by line myself. Token telemetry is not observable on this host.
  - Full-history gitleaks is not clean (pre-existing, classified above).
BLOCKERS: NONE
CCR_DRAFTED: NONE (implements the adopted CCR-W01-002 profile; no frozen field, wire, digest, schema or public API change)
SECRETS_CHECK: repair-range and full-task-range gitleaks → exit 0, no leaks. Full history → exit 1, 826 pre-existing non-secret findings, 0 in this change. Synthetic scalars 0x42/0x43 exist only in tests. No .env content was read or printed.
MEMORY_UPDATES:
  - Decisions — T009 r1 implements CCR-W01-002: verify_registration accepts only fmt packed with attStmt {alg:-7, sig}, verified by the newly attested key over the exact authData || SHA256(exact clientDataJSON). none/x5c/other formats → Attestation; missing/bad/foreign sig → Signature (pending A3).
  - Known pitfalls — gitleaks detect scans all refs, so the full-history finding count grows with coordination commits (816 at r0 → 826 at r1) without any new source finding. Compare per-file/per-commit classification, not raw counts.
  - Interface pins — public API and RawRegistration shape are unchanged by r1. Consumers (T011/T028/T036/T046) must expect Attestation for none-only authenticators.
PUSH / PR / MERGE: NONE
READY_FOR_A3_REVIEW: YES
```

§15 checklist: [x] criteria→evidence [x] named negatives assert specific variants [x] guard removal/restore control [x] write set (4 paths) [x] no invented schema/enum/digest [x] gitleaks ranges clean; history classified, not called clean [x] PROTOTYPE headers present (no new files) [x] §0.3 grep no hits; no new variables [x] required commands at the final SHA with exits [x] limitations/UNVERIFIED labeled [x] MEMORY_UPDATES [x] clean; SHA/TREE via command [x] single text block final [x] A1 COORDINATION worktree exists [x] no unresolved guesses (J1–J4 flagged for A3).
