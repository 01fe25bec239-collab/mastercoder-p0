// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//! In-test software authenticator: assembles synthetic ceremonies byte by byte,
//! independently of the code under test. Synthetic key and identities only.
// Shared by several test binaries; each uses a subset.
#![allow(dead_code)]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};
use sha2::{Digest, Sha256};

pub const RP_ID: &str = "aa.example.test";
pub const ORIGIN: &str = "https://aa.example.test";
pub const CHALLENGE: [u8; 32] = [0x5a; 32];
pub const CREDENTIAL_ID: &[u8] = b"synthetic-credential-0001";
pub const UP: u8 = 0x01;
pub const UV: u8 = 0x04;
pub const AT: u8 = 0x40;

/// Synthetic scalar of the authenticator's credential key.
pub const KEY: u8 = 0x42;
/// Synthetic scalar of an unrelated ("foreign") key.
pub const OTHER_KEY: u8 = 0x43;

fn key_of(scalar: u8) -> SigningKey {
    SigningKey::from_slice(&[scalar; 32]).expect("synthetic scalar")
}

fn key() -> SigningKey {
    key_of(KEY)
}

/// Uncompressed SEC1 point of the synthetic key.
pub fn public_sec1() -> Vec<u8> {
    public_sec1_of(KEY)
}

pub fn public_sec1_of(scalar: u8) -> Vec<u8> {
    key_of(scalar)
        .verifying_key()
        .to_sec1_point(false)
        .as_bytes()
        .to_vec()
}

/// clientDataJSON as a browser serializes it; `extra` is appended verbatim.
pub fn client_data(ty: &str, challenge: &[u8; 32], origin: &str, extra: &str) -> Vec<u8> {
    let challenge = URL_SAFE_NO_PAD.encode(challenge);
    format!(r#"{{"type":"{ty}","challenge":"{challenge}","origin":"{origin}"{extra}}}"#)
        .into_bytes()
}

pub fn auth_data(rp_id: &str, flags: u8, sign_count: u32, attested: &[u8]) -> Vec<u8> {
    let mut out = Sha256::digest(rp_id.as_bytes()).to_vec();
    out.push(flags);
    out.extend_from_slice(&sign_count.to_be_bytes());
    out.extend_from_slice(attested);
    out
}

/// ES256 DER signature over authenticatorData || SHA-256(clientDataJSON).
pub fn sign(authenticator_data: &[u8], client_data_json: &[u8]) -> Vec<u8> {
    sign_as(KEY, authenticator_data, client_data_json)
}

pub fn sign_as(scalar: u8, authenticator_data: &[u8], client_data_json: &[u8]) -> Vec<u8> {
    let mut message = authenticator_data.to_vec();
    message.extend_from_slice(&Sha256::digest(client_data_json));
    let signature: Signature = key_of(scalar).sign(&message);
    signature.to_der().as_bytes().to_vec()
}

/// CBOR head for definite lengths below 2^16.
pub fn cbor_head(major: u8, len: usize) -> Vec<u8> {
    let m = major << 5;
    match len {
        0..=23 => vec![m | len as u8],
        24..=0xff => vec![m | 24, len as u8],
        _ => {
            let [hi, lo] = (len as u16).to_be_bytes();
            vec![m | 25, hi, lo]
        }
    }
}

pub fn cbor_text(s: &str) -> Vec<u8> {
    [cbor_head(3, s.len()), s.as_bytes().to_vec()].concat()
}

pub fn cbor_bytes(b: &[u8]) -> Vec<u8> {
    [cbor_head(2, b.len()), b.to_vec()].concat()
}

/// COSE_Key {1: 2, 3: -7, -1: 1, -2: x, -3: y}.
pub fn cose_key() -> Vec<u8> {
    cose_key_of(KEY, 0x26)
}

/// COSE_Key {1: 2, 3: alg, -1: 1, -2: x, -3: y}; `alg` is a one-byte CBOR int.
pub fn cose_key_of(scalar: u8, alg: u8) -> Vec<u8> {
    let point = public_sec1_of(scalar);
    [
        vec![0xa5, 0x01, 0x02, 0x03, alg, 0x20, 0x01, 0x21],
        cbor_bytes(&point[1..33]),
        vec![0x22],
        cbor_bytes(&point[33..65]),
    ]
    .concat()
}

/// aaguid (zeros) || credentialIdLength || credentialId || COSE key.
pub fn attested_credential(credential_id: &[u8]) -> Vec<u8> {
    attested_credential_with(credential_id, &cose_key())
}

pub fn attested_credential_with(credential_id: &[u8], cose_key: &[u8]) -> Vec<u8> {
    let mut out = vec![0; 16];
    out.extend_from_slice(&(credential_id.len() as u16).to_be_bytes());
    out.extend_from_slice(credential_id);
    out.extend_from_slice(cose_key);
    out
}

/// packed attStmt {"alg": alg, "sig": sig}; `alg` is pre-encoded CBOR.
pub fn packed_stmt(alg: &[u8], sig: &[u8]) -> Vec<u8> {
    [
        vec![0xa2],
        cbor_text("alg"),
        alg.to_vec(),
        cbor_text("sig"),
        cbor_bytes(sig),
    ]
    .concat()
}

/// attestationObject {"fmt": fmt, "attStmt": att_stmt, "authData": auth}.
pub fn attestation_object(fmt: &str, att_stmt: &[u8], auth: &[u8]) -> Vec<u8> {
    [
        vec![0xa3],
        cbor_text("fmt"),
        cbor_text(fmt),
        cbor_text("attStmt"),
        att_stmt.to_vec(),
        cbor_text("authData"),
        cbor_bytes(auth),
    ]
    .concat()
}
