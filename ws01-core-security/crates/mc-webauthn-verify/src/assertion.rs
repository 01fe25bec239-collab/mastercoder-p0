// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::{WebAuthnError, auth_data, client_data};

/// What the caller expects. UV is always required; there is no field to disable it.
#[derive(Debug, Clone, Copy)]
pub struct ExpectedAssertion<'a> {
    pub challenge: [u8; 32],
    pub origin: &'a str,
    pub rp_id: &'a str,
    /// Uncompressed SEC1 P-256 point, as returned by `verify_registration`.
    pub credential_public_key: &'a [u8],
    pub prior_sign_count: u32,
}

/// Exact received `webauthn.get` bytes. `credential_id` and `user_handle` are
/// matched against credential state by the caller, not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawAssertion {
    pub authenticator_data: Vec<u8>,
    pub client_data_json: Vec<u8>,
    pub signature: Vec<u8>,
    pub credential_id: Vec<u8>,
    pub user_handle: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedAssertion {
    pub sign_count: u32,
    /// Always false on success: a counter anomaly suspends authority (doc 52),
    /// so it is returned as `WebAuthnError::Counter` and never as an accepted result.
    pub counter_anomaly: bool,
}

/// Order: type → challenge → origin → crossOrigin → rpIdHash → UP → UV →
/// signature → counter. The counter is judged only after the signature, so an
/// unsigned blob cannot trigger an anomaly.
pub fn verify_assertion(
    e: &ExpectedAssertion,
    r: &RawAssertion,
) -> Result<VerifiedAssertion, WebAuthnError> {
    client_data::check(&r.client_data_json, "webauthn.get", &e.challenge, e.origin)?;
    let auth = auth_data::parse(&r.authenticator_data, e.rp_id)?;
    if auth.flags & auth_data::AT != 0 || !auth.rest.is_empty() {
        return Err(WebAuthnError::Format);
    }
    if e.credential_public_key.len() != 65 {
        return Err(WebAuthnError::Format);
    }
    let key = VerifyingKey::from_sec1_bytes(e.credential_public_key)
        .map_err(|_| WebAuthnError::Format)?;
    let signature = Signature::from_der(&r.signature).map_err(|_| WebAuthnError::Signature)?;
    let mut signed = r.authenticator_data.clone();
    signed.extend_from_slice(&Sha256::digest(&r.client_data_json));
    key.verify(&signed, &signature)
        .map_err(|_| WebAuthnError::Signature)?;
    // W3C WebAuthn §7.2 step 22: if either counter is nonzero it must increase.
    if (auth.sign_count != 0 || e.prior_sign_count != 0) && auth.sign_count <= e.prior_sign_count {
        return Err(WebAuthnError::Counter);
    }
    Ok(VerifiedAssertion {
        sign_count: auth.sign_count,
        counter_anomaly: false,
    })
}
