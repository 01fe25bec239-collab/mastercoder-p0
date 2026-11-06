//! mc-secret-dialer — credential-bearing provider dialer (the only provider-capable crate). PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

pub mod api;
pub mod cell;
// `dial` is the only module that reaches a provider, so it exists only with the `server` feature
// (reqwest/rustls); mc-model-broker builds this crate with default-features = false.
#[cfg(feature = "server")]
pub mod dial;
pub mod guard;
