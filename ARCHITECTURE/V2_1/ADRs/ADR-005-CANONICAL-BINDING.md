# ADR-005: Canonical Binding and Epoch Types

Status: ACCEPTED_DIRECTION.

## Context

The V1 full binding was circular/inconsistent and encoded its epoch as both integer and UUID.

## Decision

Use RFC 8949 deterministic CBOR fixed arrays and domain-separated SHA-256. WorkflowSpecCoreV2 excludes a digest of itself. AttemptBindingV2 contains workflow_spec_digest but no self digest. authority_epoch is exactly a 16-byte UUID; lease_generation and credential_generation are unsigned counters.

## Consequences

Human-facing JSON is not signature input. Independent Rust/TypeScript encoders are mandatory; unknown critical fields and incompatible majors fail closed.

Gate: P0-06.


## V2.1 amendment
ADR-024 supersedes incomplete wire-freeze claims. SCHEMAS/ENCODING.md and REGISTRY.json define exact field order, types, options, signature domains and revision rejection. Historical critical-field bitmap forwarding is replaced by exact revision negotiation.
