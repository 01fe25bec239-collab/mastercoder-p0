# ADR-024 — Normative wire-schema specification

Status: ACCEPTED_FOR_V2_1_AUDIT; implementation/P0 evidence not granted. Amends ADR-005 and V2 contract-freeze statement. Finding 12 and 19.

V2 specified fixed CBOR arrays and some fields but omitted complete ordered layouts, timestamp units, unknown-field handling and vectors. Implementers could invent different security contracts. Preserve deterministic CBOR and SHA-256; define every signed/hashed envelope in SCHEMAS/REGISTRY.json with strict decoded JSON schemas, explicit options, integer widths, domains, detached signatures and version rejection. TEST_VECTORS supplies independently consumable exact bytes/digests and rejection cases. No production encoder is supplied.

A new field requires an exact schema revision and authenticated supported-registry negotiation; no generic extension bag. Historical aliases are documentation-only. Gate inputs retain protected producer/write authority; valid bytes alone do not establish truth. Contract dependency digests are acyclic, and state snapshots cannot mutate signed intent identity.

Cost: larger architecture specification and mandatory independent interoperability work. Rejected alternatives: freeze-by-prose, JSON Number for u64, implicit nulls, schema-free extensibility and claiming one local fixture generator proves Rust/TypeScript compatibility. P0-06 must independently test all supported types/revisions and rejection behavior before protocol enablement.
