// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! `webauthn.create` with attestation "none" only.

#[path = "support/soft_authenticator.rs"]
mod soft_authenticator;

use mc_webauthn_verify::{
    ExpectedAssertion, ExpectedRegistration, RawAssertion, RawRegistration, VerifiedRegistration,
    WebAuthnError, verify_assertion, verify_registration,
};
use soft_authenticator::*;

fn register(
    challenge: &[u8; 32],
    attestation_object: Vec<u8>,
) -> Result<VerifiedRegistration, WebAuthnError> {
    let expected = ExpectedRegistration {
        challenge: CHALLENGE,
        origin: ORIGIN,
        rp_id: RP_ID,
    };
    let raw = RawRegistration {
        credential_id: CREDENTIAL_ID.to_vec(),
        client_data_json: client_data("webauthn.create", challenge, ORIGIN, ""),
        attestation_object,
    };
    verify_registration(&expected, &raw)
}

fn good_auth_data() -> Vec<u8> {
    auth_data(RP_ID, UP | UV | AT, 0, &attested_credential(CREDENTIAL_ID))
}

#[test]
fn valid_none_attestation() {
    let verified = register(
        &CHALLENGE,
        attestation_object("none", &[0xa0], &good_auth_data()),
    )
    .expect("valid registration");
    assert_eq!(verified.credential_id, CREDENTIAL_ID);
    assert_eq!(verified.public_key_sec1, public_sec1());

    // The registered key then verifies an assertion from the same authenticator.
    let cdj = client_data("webauthn.get", &CHALLENGE, ORIGIN, "");
    let auth = auth_data(RP_ID, UP | UV, 1, &[]);
    let raw = RawAssertion {
        signature: sign(&auth, &cdj),
        authenticator_data: auth,
        client_data_json: cdj,
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
fn non_none_attestation_rejected() {
    let packed = attestation_object("packed", &[0xa0], &good_auth_data());
    assert_eq!(
        register(&CHALLENGE, packed),
        Err(WebAuthnError::Attestation)
    );
    // "none" must carry an empty attStmt: {"alg": -7} rejects.
    let stmt = [vec![0xa1], cbor_text("alg"), vec![0x26]].concat();
    let none_with_stmt = attestation_object("none", &stmt, &good_auth_data());
    assert_eq!(
        register(&CHALLENGE, none_with_stmt),
        Err(WebAuthnError::Attestation)
    );
}

#[test]
fn wrong_challenge() {
    let mut other = CHALLENGE;
    other[0] ^= 1;
    let object = attestation_object("none", &[0xa0], &good_auth_data());
    assert_eq!(register(&other, object), Err(WebAuthnError::Challenge));
}
