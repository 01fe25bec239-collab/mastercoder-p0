// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! Valid `webauthn.get` ceremony plus one-field mutations. Every mutated
//! ceremony is re-signed, so each rejection comes from the targeted check.

#[path = "support/soft_authenticator.rs"]
mod soft_authenticator;

use mc_webauthn_verify::{
    ExpectedAssertion, RawAssertion, VerifiedAssertion, WebAuthnError, verify_assertion,
};
use soft_authenticator::*;

fn verify(
    client_data_json: Vec<u8>,
    authenticator_data: Vec<u8>,
    signature: Vec<u8>,
    prior_sign_count: u32,
) -> Result<VerifiedAssertion, WebAuthnError> {
    let key = public_sec1();
    let expected = ExpectedAssertion {
        challenge: CHALLENGE,
        origin: ORIGIN,
        rp_id: RP_ID,
        credential_public_key: &key,
        prior_sign_count,
    };
    let raw = RawAssertion {
        authenticator_data,
        client_data_json,
        signature,
        credential_id: CREDENTIAL_ID.to_vec(),
        user_handle: None,
    };
    verify_assertion(&expected, &raw)
}

/// Signs and verifies; prior sign count 5.
fn signed(
    client_data_json: Vec<u8>,
    authenticator_data: Vec<u8>,
) -> Result<VerifiedAssertion, WebAuthnError> {
    let signature = sign(&authenticator_data, &client_data_json);
    verify(client_data_json, authenticator_data, signature, 5)
}

fn good_client_data() -> Vec<u8> {
    client_data("webauthn.get", &CHALLENGE, ORIGIN, "")
}

fn good_auth_data(sign_count: u32) -> Vec<u8> {
    auth_data(RP_ID, UP | UV, sign_count, &[])
}

#[test]
fn valid_ceremony_verifies() {
    assert_eq!(
        signed(good_client_data(), good_auth_data(6)),
        Ok(VerifiedAssertion {
            sign_count: 6,
            counter_anomaly: false
        })
    );
    // crossOrigin:false is the browser's normal same-origin value.
    let cdj = client_data(
        "webauthn.get",
        &CHALLENGE,
        ORIGIN,
        r#","crossOrigin":false"#,
    );
    assert!(signed(cdj, good_auth_data(6)).is_ok());
    // Zero-counter authenticators stay at zero; not an anomaly (and not clone-proof).
    let (cdj, auth) = (good_client_data(), good_auth_data(0));
    let sig = sign(&auth, &cdj);
    assert!(verify(cdj, auth, sig, 0).is_ok());
}

#[test]
fn wrong_type() {
    let cdj = client_data("webauthn.create", &CHALLENGE, ORIGIN, "");
    assert_eq!(signed(cdj, good_auth_data(6)), Err(WebAuthnError::Type));
}

#[test]
fn wrong_challenge() {
    let mut other = CHALLENGE;
    other[31] ^= 1;
    let cdj = client_data("webauthn.get", &other, ORIGIN, "");
    assert_eq!(
        signed(cdj, good_auth_data(6)),
        Err(WebAuthnError::Challenge)
    );
}

#[test]
fn origin_prefix_is_not_equal() {
    for origin in [
        "https://aa.example.test.evil.test",
        "https://aa.example.test:443",
        "https://aa.example.tes",
    ] {
        let cdj = client_data("webauthn.get", &CHALLENGE, origin, "");
        assert_eq!(
            signed(cdj, good_auth_data(6)),
            Err(WebAuthnError::Origin),
            "{origin}"
        );
    }
}

#[test]
fn cross_origin_true() {
    let cdj = client_data("webauthn.get", &CHALLENGE, ORIGIN, r#","crossOrigin":true"#);
    assert_eq!(
        signed(cdj, good_auth_data(6)),
        Err(WebAuthnError::CrossOrigin)
    );
}

#[test]
fn wrong_rp_id_hash() {
    let auth = auth_data("example.test", UP | UV, 6, &[]);
    assert_eq!(
        signed(good_client_data(), auth),
        Err(WebAuthnError::RpIdHash)
    );
}

#[test]
fn up_missing() {
    let auth = auth_data(RP_ID, UV, 6, &[]);
    assert_eq!(
        signed(good_client_data(), auth),
        Err(WebAuthnError::UserPresent)
    );
}

#[test]
fn uv_missing() {
    let auth = auth_data(RP_ID, UP, 6, &[]);
    assert_eq!(
        signed(good_client_data(), auth),
        Err(WebAuthnError::UserVerified)
    );
}

#[test]
fn signature_bit_flip() {
    let (cdj, auth) = (good_client_data(), good_auth_data(6));
    let mut sig = sign(&auth, &cdj);
    *sig.last_mut().unwrap() ^= 0x01;
    assert_eq!(verify(cdj, auth, sig, 5), Err(WebAuthnError::Signature));
}

#[test]
fn counter_regression_flags_anomaly() {
    for sign_count in [5, 4, 0] {
        assert_eq!(
            signed(good_client_data(), good_auth_data(sign_count)),
            Err(WebAuthnError::Counter),
            "sign_count {sign_count} after prior 5"
        );
    }
}

#[test]
fn malformed_auth_data() {
    let mut truncated = good_auth_data(6);
    truncated.pop();
    let mut trailing = good_auth_data(6);
    trailing.push(0);
    let duplicate_key = client_data(
        "webauthn.get",
        &CHALLENGE,
        ORIGIN,
        r#","type":"webauthn.get""#,
    );
    for (cdj, auth) in [
        (good_client_data(), truncated),
        (good_client_data(), trailing),
        (duplicate_key, good_auth_data(6)),
    ] {
        assert_eq!(signed(cdj, auth), Err(WebAuthnError::Format));
    }
}
