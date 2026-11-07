//! mc-webauthn-verify — PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//!
//! One shared WebAuthn ceremony verifier (ENCODING.md WebAuthn rules; doc 52):
//! `webauthn.get` assertions and `webauthn.create` registrations, ES256/P-256 only.
//!
//! Registration profile (owner-adopted restricted prototype profile, CCR-W01-002):
//! only `packed` self attestation is accepted — attStmt exactly `{alg: -7, sig}`,
//! no `x5c`. The newly attested credential key itself must verify `sig` over the
//! exact received authenticatorData || SHA-256(exact received clientDataJSON).
//! Unsigned `none`, certificate-backed `packed` (x5c), other formats, other
//! algorithms and every other authenticator/attestation cell reject; there is no
//! fallback and no additional ceremony.
//!
//! Trust limit: a successful registration proves possession of the new private key
//! at this boundary only. It is not manufacturer provenance, trusted hardware,
//! biometric assurance or universal passkey support, and no physical/browser
//! authenticator cell is enabled or qualified by this crate. UP/UV are checked as
//! flags; their hardware meaning is not attested. Only the invalid BS-without-BE
//! flag combination rejects; no broader backup-eligibility policy is applied.
//!
//! Caller duties (not done here): credential state/generation and scope, backup
//! policy, user handle and credential-ID binding, challenge freshness, expiry and
//! one-use consumption, enrollment authorization/quorum and authority activation.

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
