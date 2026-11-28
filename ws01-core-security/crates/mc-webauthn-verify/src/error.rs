// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

/// Why a ceremony was rejected. No variant carries partial acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum WebAuthnError {
    #[error("clientDataJSON type mismatch")]
    Type,
    #[error("clientDataJSON challenge mismatch")]
    Challenge,
    #[error("clientDataJSON origin mismatch")]
    Origin,
    #[error("cross-origin ceremony denied")]
    CrossOrigin,
    #[error("rpIdHash mismatch")]
    RpIdHash,
    #[error("user presence flag not set")]
    UserPresent,
    #[error("user verification flag not set")]
    UserVerified,
    #[error("signature invalid")]
    Signature,
    #[error("sign counter anomaly")]
    Counter,
    #[error("malformed ceremony bytes")]
    Format,
    #[error("unsupported attestation")]
    Attestation,
}
