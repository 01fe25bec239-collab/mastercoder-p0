// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

use std::collections::HashSet;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::Value;

use crate::WebAuthnError;

/// Checks the exact received clientDataJSON bytes in the order
/// type → challenge → origin → crossOrigin (ENCODING.md WebAuthn; doc 52).
pub(crate) fn check(
    bytes: &[u8],
    ceremony_type: &str,
    challenge: &[u8; 32],
    origin: &str,
) -> Result<(), WebAuthnError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| WebAuthnError::Format)?;
    let map = value.as_object().ok_or(WebAuthnError::Format)?;
    if has_duplicate_top_level_key(bytes)? {
        return Err(WebAuthnError::Format);
    }
    let text = |key: &str| map.get(key).and_then(Value::as_str);
    if text("type") != Some(ceremony_type) {
        return Err(WebAuthnError::Type);
    }
    // Compare against the canonical unpadded base64url spelling, so padded or
    // non-canonical encodings of the same bytes are also rejected.
    if text("challenge") != Some(URL_SAFE_NO_PAD.encode(challenge).as_str()) {
        return Err(WebAuthnError::Challenge);
    }
    if text("origin") != Some(origin) {
        return Err(WebAuthnError::Origin);
    }
    match map.get("crossOrigin") {
        None | Some(Value::Bool(false)) => {}
        Some(_) => return Err(WebAuthnError::CrossOrigin),
    }
    if map.contains_key("topOrigin") {
        return Err(WebAuthnError::CrossOrigin);
    }
    Ok(())
}

/// serde_json keeps the last of duplicate keys; a second "type" or "origin"
/// must reject instead. Runs only on bytes serde_json already accepted as an object.
fn has_duplicate_top_level_key(bytes: &[u8]) -> Result<bool, WebAuthnError> {
    let mut keys = HashSet::new();
    let (mut depth, mut expect_key, mut i) = (0usize, false, 0usize);
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                let start = i;
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    i += if bytes[i] == b'\\' { 2 } else { 1 };
                }
                let raw = bytes.get(start..=i).ok_or(WebAuthnError::Format)?;
                if depth == 1 && expect_key {
                    let key: String =
                        serde_json::from_slice(raw).map_err(|_| WebAuthnError::Format)?;
                    if !keys.insert(key) {
                        return Ok(true);
                    }
                    expect_key = false;
                }
            }
            b'{' | b'[' => {
                depth += 1;
                expect_key = depth == 1;
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            b',' if depth == 1 => expect_key = true,
            _ => {}
        }
        i += 1;
    }
    Ok(false)
}
