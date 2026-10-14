// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use sha2::{Digest, Sha256};

#[test]
fn registry_sha256_matches() {
    let raw = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json"
    ));
    assert_eq!(
        hex::encode(Sha256::digest(raw)),
        mc_contracts::REGISTRY_SHA256
    );
}
