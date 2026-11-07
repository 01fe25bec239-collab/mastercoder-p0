// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

use p256::ecdsa::VerifyingKey;

use crate::{WebAuthnError, auth_data, client_data};

#[derive(Debug, Clone, Copy)]
pub struct ExpectedRegistration<'a> {
    pub challenge: [u8; 32],
    pub origin: &'a str,
    pub rp_id: &'a str,
}

/// Exact received `webauthn.create` bytes; fields mirror WebAuthnRegistrationCeremonyV1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawRegistration {
    pub credential_id: Vec<u8>,
    pub client_data_json: Vec<u8>,
    pub attestation_object: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedRegistration {
    pub credential_id: Vec<u8>,
    /// Uncompressed SEC1 P-256 point (0x04 || x || y).
    pub public_key_sec1: Vec<u8>,
}

/// Order: type → challenge → origin → crossOrigin → attestation "none" →
/// rpIdHash → UP → UV → attested credential ID and ES256 key.
pub fn verify_registration(
    e: &ExpectedRegistration,
    raw: &RawRegistration,
) -> Result<VerifiedRegistration, WebAuthnError> {
    client_data::check(
        &raw.client_data_json,
        "webauthn.create",
        &e.challenge,
        e.origin,
    )?;
    let auth_bytes = none_attestation_auth_data(&raw.attestation_object)?;
    let auth = auth_data::parse(auth_bytes, e.rp_id)?;
    if auth.flags & auth_data::AT == 0 {
        return Err(WebAuthnError::Format);
    }
    // attestedCredentialData: aaguid(16) || credentialIdLength(2) || credentialId || COSE key.
    let mut c = Cbor::new(auth.rest);
    c.take(16)?;
    let id_len = c.take(2)?;
    let credential_id = c.take(usize::from(u16::from_be_bytes([id_len[0], id_len[1]])))?;
    if credential_id.is_empty() || credential_id.len() > 1023 || credential_id != raw.credential_id
    {
        return Err(WebAuthnError::Format);
    }
    let public_key_sec1 = es256_cose_key(&mut c)?;
    if !c.done() {
        return Err(WebAuthnError::Format);
    }
    VerifyingKey::from_sec1_bytes(&public_key_sec1).map_err(|_| WebAuthnError::Format)?;
    Ok(VerifiedRegistration {
        credential_id: credential_id.to_vec(),
        public_key_sec1,
    })
}

/// attestationObject in CTAP2 canonical key order {"fmt", "attStmt", "authData"};
/// only fmt "none" with an empty attStmt is supported.
fn none_attestation_auth_data(bytes: &[u8]) -> Result<&[u8], WebAuthnError> {
    let mut c = Cbor::new(bytes);
    if c.expect(MAP)? != 3 || c.text()? != "fmt" {
        return Err(WebAuthnError::Format);
    }
    if c.text()? != "none" {
        return Err(WebAuthnError::Attestation);
    }
    if c.text()? != "attStmt" {
        return Err(WebAuthnError::Format);
    }
    if c.expect(MAP)? != 0 {
        return Err(WebAuthnError::Attestation);
    }
    if c.text()? != "authData" {
        return Err(WebAuthnError::Format);
    }
    let auth = c.bytes()?;
    if !c.done() {
        return Err(WebAuthnError::Format);
    }
    Ok(auth)
}

/// COSE_Key EC2 / ES256 / P-256 in CTAP2 canonical order:
/// {1: 2, 3: -7, -1: 1, -2: x, -3: y} (RFC 9053 §7.1). Anything else rejects.
fn es256_cose_key(c: &mut Cbor) -> Result<Vec<u8>, WebAuthnError> {
    if c.expect(MAP)? != 5 {
        return Err(WebAuthnError::Format);
    }
    for (label, value) in [(1, 2), (3, -7), (-1, 1)] {
        if c.int()? != label || c.int()? != value {
            return Err(WebAuthnError::Format);
        }
    }
    let mut point = vec![0x04];
    for label in [-2, -3] {
        if c.int()? != label {
            return Err(WebAuthnError::Format);
        }
        let coordinate = c.bytes()?;
        if coordinate.len() != 32 {
            return Err(WebAuthnError::Format);
        }
        point.extend_from_slice(coordinate);
    }
    Ok(point)
}

const UINT: u8 = 0;
const NINT: u8 = 1;
const BSTR: u8 = 2;
const TSTR: u8 = 3;
const MAP: u8 = 5;

/// Minimal strict CBOR reader (RFC 8949): definite lengths only, shortest-form
/// heads only, no tags/floats/arrays. Every failure is `Format`.
struct Cbor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cbor<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn done(&self) -> bool {
        self.pos == self.buf.len()
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], WebAuthnError> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&end| end <= self.buf.len())
            .ok_or(WebAuthnError::Format)?;
        let out = &self.buf[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    fn head(&mut self) -> Result<(u8, u64), WebAuthnError> {
        let initial = self.take(1)?[0];
        let (major, info) = (initial >> 5, initial & 0x1f);
        let (value, min) = match info {
            0..=23 => (u64::from(info), 0),
            24 => (u64::from(self.take(1)?[0]), 24),
            25 => (u64::from(u16::from_be_bytes(self.array()?)), 0x100),
            26 => (u64::from(u32::from_be_bytes(self.array()?)), 0x1_0000),
            27 => (u64::from_be_bytes(self.array()?), 0x1_0000_0000),
            _ => return Err(WebAuthnError::Format),
        };
        if value < min {
            return Err(WebAuthnError::Format);
        }
        Ok((major, value))
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], WebAuthnError> {
        self.take(N)?.try_into().map_err(|_| WebAuthnError::Format)
    }

    fn expect(&mut self, major: u8) -> Result<u64, WebAuthnError> {
        match self.head()? {
            (m, value) if m == major => Ok(value),
            _ => Err(WebAuthnError::Format),
        }
    }

    fn bytes(&mut self) -> Result<&'a [u8], WebAuthnError> {
        let len = self.expect(BSTR)?;
        self.take(usize::try_from(len).map_err(|_| WebAuthnError::Format)?)
    }

    fn text(&mut self) -> Result<&'a str, WebAuthnError> {
        let len = self.expect(TSTR)?;
        let raw = self.take(usize::try_from(len).map_err(|_| WebAuthnError::Format)?)?;
        std::str::from_utf8(raw).map_err(|_| WebAuthnError::Format)
    }

    fn int(&mut self) -> Result<i64, WebAuthnError> {
        let (major, value) = self.head()?;
        let value = i64::try_from(value).map_err(|_| WebAuthnError::Format)?;
        match major {
            UINT => Ok(value),
            NINT => Ok(-1 - value),
            _ => Err(WebAuthnError::Format),
        }
    }
}
