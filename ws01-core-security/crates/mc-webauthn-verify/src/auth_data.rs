// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

use sha2::{Digest, Sha256};

use crate::WebAuthnError;

// authenticatorData flag bits (W3C WebAuthn Level 3 §6.1).
const UP: u8 = 0x01;
const UV: u8 = 0x04;
const BE: u8 = 0x08;
const BS: u8 = 0x10;
pub(crate) const AT: u8 = 0x40;
const ED: u8 = 0x80;

pub(crate) struct AuthData<'a> {
    pub flags: u8,
    pub sign_count: u32,
    /// Bytes after the 37-byte fixed header (attested credential data, if any).
    pub rest: &'a [u8],
}

/// Checks rpIdHash, then UP and UV. Extensions are not supported and reject;
/// BS without BE is an invalid flag combination and rejects.
pub(crate) fn parse<'a>(bytes: &'a [u8], rp_id: &str) -> Result<AuthData<'a>, WebAuthnError> {
    if bytes.len() < 37 {
        return Err(WebAuthnError::Format);
    }
    if bytes[..32] != Sha256::digest(rp_id.as_bytes())[..] {
        return Err(WebAuthnError::RpIdHash);
    }
    let flags = bytes[32];
    if flags & UP == 0 {
        return Err(WebAuthnError::UserPresent);
    }
    if flags & UV == 0 {
        return Err(WebAuthnError::UserVerified);
    }
    if flags & ED != 0 || (flags & BS != 0 && flags & BE == 0) {
        return Err(WebAuthnError::Format);
    }
    let sign_count = u32::from_be_bytes([bytes[33], bytes[34], bytes[35], bytes[36]]);
    Ok(AuthData {
        flags,
        sign_count,
        rest: &bytes[37..],
    })
}
