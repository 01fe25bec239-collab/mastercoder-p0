//! mc-webauthn-verify — PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//!
//! One shared WebAuthn ceremony verifier (ENCODING.md WebAuthn rules; doc 52):
//! `webauthn.get` assertions and `webauthn.create` registrations with attestation
//! "none" only, ES256/P-256 only. Credential state, challenge freshness and
//! consumption are the caller's responsibility.

mod assertion;
mod auth_data;
mod client_data;
mod error;
mod registration;

pub use assertion::{ExpectedAssertion, RawAssertion, VerifiedAssertion, verify_assertion};
pub use error::WebAuthnError;
pub use registration::{
    ExpectedRegistration, RawRegistration, VerifiedRegistration, verify_registration,
};
