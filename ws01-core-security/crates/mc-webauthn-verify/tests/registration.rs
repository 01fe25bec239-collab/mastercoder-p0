// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! `webauthn.create` under the owner-adopted restricted profile (CCR-W01-002):
//! ES256/P-256 packed self attestation only. Ceremonies are built and signed by
//! the in-test software authenticator, never by the code under test.

#[path = "support/soft_authenticator.rs"]
mod soft_authenticator;

use mc_webauthn_verify::{
    ExpectedAssertion, ExpectedRegistration, RawAssertion, RawRegistration, VerifiedRegistration,
    WebAuthnError, verify_assertion, verify_registration,
};
use soft_authenticator::*;

/// CBOR int -7 (COSE ES256).
const ES256: &[u8] = &[0x26];

fn verify(
    credential_id: &[u8],
    client_data_json: Vec<u8>,
    attestation_object: Vec<u8>,
) -> Result<VerifiedRegistration, WebAuthnError> {
    let expected = ExpectedRegistration {
        challenge: CHALLENGE,
        origin: ORIGIN,
        rp_id: RP_ID,
    };
    let raw = RawRegistration {
        credential_id: credential_id.to_vec(),
        client_data_json,
        attestation_object,
    };
    verify_registration(&expected, &raw)
}

fn create_json(challenge: &[u8; 32]) -> Vec<u8> {
    client_data("webauthn.create", challenge, ORIGIN, "")
}

fn good_auth_data() -> Vec<u8> {
    auth_data(RP_ID, UP | UV | AT, 0, &attested_credential(CREDENTIAL_ID))
}

/// packed self attestation signed by `signer` over exactly (auth, cdj).
fn packed_signed_by(signer: u8, auth: &[u8], cdj: &[u8]) -> Vec<u8> {
    let stmt = packed_stmt(ES256, &sign_as(signer, auth, cdj));
    attestation_object("packed", &stmt, auth)
}

#[test]
fn valid_packed_self_attestation() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    let verified = verify(
        CREDENTIAL_ID,
        cdj.clone(),
        packed_signed_by(KEY, &auth, &cdj),
    )
    .expect("valid registration");
    assert_eq!(verified.credential_id, CREDENTIAL_ID);
    assert_eq!(verified.public_key_sec1, public_sec1());

    // The registered key then verifies an assertion from the same authenticator.
    let get = client_data("webauthn.get", &CHALLENGE, ORIGIN, "");
    let get_auth = auth_data(RP_ID, UP | UV, 1, &[]);
    let raw = RawAssertion {
        signature: sign(&get_auth, &get),
        authenticator_data: get_auth,
        client_data_json: get,
        credential_id: verified.credential_id.clone(),
        user_handle: None,
    };
    let expected = ExpectedAssertion {
        challenge: CHALLENGE,
        origin: ORIGIN,
        rp_id: RP_ID,
        credential_public_key: &verified.public_key_sec1,
        prior_sign_count: 0,
    };
    assert!(verify_assertion(&expected, &raw).is_ok());
}

#[test]
fn unsupported_attestation_rejected() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    let stmt = packed_stmt(ES256, &sign(&auth, &cdj));
    for fmt in [
        "fido-u2f",
        "tpm",
        "android-key",
        "android-safetynet",
        "apple",
        "Packed",
    ] {
        let object = attestation_object(fmt, &stmt, &auth);
        assert_eq!(
            verify(CREDENTIAL_ID, cdj.clone(), object),
            Err(WebAuthnError::Attestation),
            "fmt {fmt}"
        );
    }
    // packed with an empty or foreign-shaped statement is not the supported profile.
    let empty = attestation_object("packed", &[0xa0], &auth);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj.clone(), empty),
        Err(WebAuthnError::Attestation)
    );
    let sig_only = [vec![0xa1], cbor_text("sig"), cbor_bytes(&sign(&auth, &cdj))].concat();
    let sig_only = attestation_object("packed", &sig_only, &auth);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj, sig_only),
        Err(WebAuthnError::Attestation)
    );
}

#[test]
fn wrong_challenge() {
    let mut other = CHALLENGE;
    other[0] ^= 1;
    // Correctly self-signed over the wrong challenge: rejection is the challenge check.
    let cdj = create_json(&other);
    let auth = good_auth_data();
    let object = packed_signed_by(KEY, &auth, &cdj);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj, object),
        Err(WebAuthnError::Challenge)
    );
}

#[test]
fn unsigned_none_rejected() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    // The r0-accepted shape: otherwise valid none with an empty attStmt.
    let none = attestation_object("none", &[0xa0], &auth);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj.clone(), none),
        Err(WebAuthnError::Attestation)
    );
    // none carrying a valid self signature is still not the packed profile.
    let stmt = packed_stmt(ES256, &sign(&auth, &cdj));
    let none_with_sig = attestation_object("none", &stmt, &auth);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj, none_with_sig),
        Err(WebAuthnError::Attestation)
    );
}

#[test]
fn missing_or_invalid_self_signature() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    let check = |stmt: Vec<u8>| {
        verify(
            CREDENTIAL_ID,
            cdj.clone(),
            attestation_object("packed", &stmt, &auth),
        )
    };

    // {"alg": -7} without sig.
    let alg_only = [vec![0xa1], cbor_text("alg"), ES256.to_vec()].concat();
    assert_eq!(check(alg_only), Err(WebAuthnError::Signature));
    // Empty and non-DER signatures.
    assert_eq!(
        check(packed_stmt(ES256, &[])),
        Err(WebAuthnError::Signature)
    );
    assert_eq!(
        check(packed_stmt(ES256, &[0x30, 0x00])),
        Err(WebAuthnError::Signature)
    );
    // Well-formed DER whose last s byte is flipped.
    let mut flipped = sign(&auth, &cdj);
    *flipped.last_mut().expect("non-empty signature") ^= 1;
    assert_eq!(
        check(packed_stmt(ES256, &flipped)),
        Err(WebAuthnError::Signature)
    );
}

#[test]
fn self_signature_by_other_key_rejected() {
    let cdj = create_json(&CHALLENGE);
    // Attested key = KEY, signature by OTHER_KEY.
    let auth = good_auth_data();
    assert_eq!(
        verify(
            CREDENTIAL_ID,
            cdj.clone(),
            packed_signed_by(OTHER_KEY, &auth, &cdj)
        ),
        Err(WebAuthnError::Signature)
    );
    // Attested key = OTHER_KEY, signature by KEY (no caller key override exists).
    let other_auth = auth_data(
        RP_ID,
        UP | UV | AT,
        0,
        &attested_credential_with(CREDENTIAL_ID, &cose_key_of(OTHER_KEY, 0x26)),
    );
    assert_eq!(
        verify(
            CREDENTIAL_ID,
            cdj.clone(),
            packed_signed_by(KEY, &other_auth, &cdj)
        ),
        Err(WebAuthnError::Signature)
    );
    // Control: OTHER_KEY self-signing its own attestation is accepted.
    let ok = verify(
        CREDENTIAL_ID,
        cdj.clone(),
        packed_signed_by(OTHER_KEY, &other_auth, &cdj),
    )
    .expect("other key self-signed");
    assert_eq!(ok.public_key_sec1, public_sec1_of(OTHER_KEY));
}

#[test]
fn algorithm_mismatch_rejected() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    let sig = sign(&auth, &cdj);
    // attStmt alg other than ES256: EdDSA -8, ES384 -35, RS256 -257.
    for alg in [&[0x27][..], &[0x38, 0x22], &[0x39, 0x01, 0x00]] {
        let object = attestation_object("packed", &packed_stmt(alg, &sig), &auth);
        assert_eq!(
            verify(CREDENTIAL_ID, cdj.clone(), object),
            Err(WebAuthnError::Attestation),
            "alg {alg:02x?}"
        );
    }
    // attStmt alg -7 but the credential COSE key claims EdDSA (-8), correctly self-signed.
    let eddsa_key = auth_data(
        RP_ID,
        UP | UV | AT,
        0,
        &attested_credential_with(CREDENTIAL_ID, &cose_key_of(KEY, 0x27)),
    );
    assert_eq!(
        verify(
            CREDENTIAL_ID,
            cdj.clone(),
            packed_signed_by(KEY, &eddsa_key, &cdj)
        ),
        Err(WebAuthnError::Format)
    );
}

#[test]
fn signed_transcript_mutations_rejected() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    // One signature over the original exact transcript, replayed against mutations.
    let old_stmt = packed_stmt(ES256, &sign(&auth, &cdj));
    let replay = |id: &[u8], cdj: Vec<u8>, auth: &[u8]| {
        verify(id, cdj, attestation_object("packed", &old_stmt, auth))
    };
    assert!(replay(CREDENTIAL_ID, cdj.clone(), &auth).is_ok());

    // Semantically equal, byte-different clientDataJSON.
    let with_false = client_data(
        "webauthn.create",
        &CHALLENGE,
        ORIGIN,
        r#","crossOrigin":false"#,
    );
    assert_eq!(
        replay(CREDENTIAL_ID, with_false, &auth),
        Err(WebAuthnError::Signature)
    );
    let mut spaced = cdj.clone();
    spaced.insert(1, b' ');
    assert_eq!(
        replay(CREDENTIAL_ID, spaced, &auth),
        Err(WebAuthnError::Signature)
    );
    // Changed authData: sign count, and an extra accepted flag (BE).
    let counted = auth_data(RP_ID, UP | UV | AT, 1, &attested_credential(CREDENTIAL_ID));
    assert_eq!(
        replay(CREDENTIAL_ID, cdj.clone(), &counted),
        Err(WebAuthnError::Signature)
    );
    let be = auth_data(
        RP_ID,
        UP | UV | AT | 0x08,
        0,
        &attested_credential(CREDENTIAL_ID),
    );
    assert_eq!(
        replay(CREDENTIAL_ID, cdj.clone(), &be),
        Err(WebAuthnError::Signature)
    );
    // Changed credential ID (same length, consistent in authData and raw input).
    let other_id = b"synthetic-credential-0002";
    let reid = auth_data(RP_ID, UP | UV | AT, 0, &attested_credential(other_id));
    assert_eq!(
        replay(other_id, cdj.clone(), &reid),
        Err(WebAuthnError::Signature)
    );
    // Changed credential key.
    let rekeyed = auth_data(
        RP_ID,
        UP | UV | AT,
        0,
        &attested_credential_with(CREDENTIAL_ID, &cose_key_of(OTHER_KEY, 0x26)),
    );
    assert_eq!(
        replay(CREDENTIAL_ID, cdj, &rekeyed),
        Err(WebAuthnError::Signature)
    );
}

#[test]
fn certificate_backed_packed_rejected() {
    let cdj = create_json(&CHALLENGE);
    let auth = good_auth_data();
    // {"alg": -7, "sig": valid self signature, "x5c": [synthetic cert bytes]}.
    let stmt = [
        vec![0xa3],
        cbor_text("alg"),
        ES256.to_vec(),
        cbor_text("sig"),
        cbor_bytes(&sign(&auth, &cdj)),
        cbor_text("x5c"),
        vec![0x81],
        cbor_bytes(b"synthetic-not-a-certificate"),
    ]
    .concat();
    let object = attestation_object("packed", &stmt, &auth);
    assert_eq!(
        verify(CREDENTIAL_ID, cdj, object),
        Err(WebAuthnError::Attestation)
    );
}
