# Normative architecture test vectors

Consumers: independently implemented Rust and TypeScript codecs and authority tests in the later P0-06/P0-09 phase. No Rust/TypeScript product implementation or P0 experiment is included or executed here.

`wire-vectors.json` contains an exact decoded projection, canonical CBOR hex and domain-separated SHA-256 for every registered runtime record, plus malformed bytes/projections that must reject. Positive wire fixtures intentionally cover **structural encoding**, not valid real-world authority: synthetic OIDs/identities and missing runtime evidence cannot authorize an operation. Do not mistake accepting a structurally well-formed record for trusting it. The negative fixtures cover missing/extra fields, unsupported revisions, trailing bytes, malformed/nonminimal CBOR, integer/UUID errors, duplicate units and cross-role capability enums.

`signature-vectors.json` contains six pure Ed25519 signature inputs, public keys, signatures and single-byte-corrupted rejection signatures. The known RFC8032 test seed used for generation is public test material only; never deploy it. Verification must reconstruct the exact signature input, verify digest/key/role/currentness, and reject altered signature/input/domain. Correct cryptography alone does not establish an authorized producer.

`semantic-vectors.json` specifies positive and negative acceptance predicates for the six requested contract families plus enrollment, oracle approval, eligibility, check and manual-case boundaries. They are explicit synthetic fixtures with trusted-state assumptions, not opaque PASS claims. They include changed display/plan, cross-purpose ceremony, registration key/scope changes, duplicate administrator identity, nonce replay, stale epoch, missing zero-finding receipt, invalid oracle, expired lease, per-child revocation, absent witness durability and transient not-found. A future authority implementation must reproduce these outcomes with real schema-valid records and actual authenticated trusted-state setup; the included checker only checks that the declarative examples are consistent.

Run local documentation conformance from the package directory:

```sh
python3 TEST_VECTORS/check_spec.py
```

This standard-library fixture tool checks canonical encodings/digests, strict structural decoding, rejection outcomes, registry references and schema order, and declared semantic predicates. It is an architecture artifact, not a production parser or a claim of independently implemented language parity. Independent implementations must not import this codec or generate expected bytes from their own output. They consume the committed exact vector files as an oracle, check all primitive width/unknown-field/schema conditions, and separately test every semantic rule in REGISTRY.json/CONTRACT_AUTHORITY.md. The accompanying package validator checks declared files, references, claim records and baseline integrity. All P0 claims stay NOT_RUN.
