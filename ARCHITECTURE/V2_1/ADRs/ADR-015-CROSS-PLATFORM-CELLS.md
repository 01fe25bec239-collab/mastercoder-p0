# ADR-015: Cross-Platform Capability Cells

Status: ACCEPTED_DIRECTION.

## Context

A cross-platform Rust runner does not prove VM containment, process semantics, checkout features, or native project builds on every OS.

## Decision

Advertise and bind independent cells for host OS/architecture, sandbox tier/backend, checkout features, toolchains, native project classes, model modes, and evidence independence. Schedule only exact matches. EXPERIMENTAL is advisory; UNSUPPORTED mandatory work is non-pass.

## Consequences

The product matrix is more explicit and initially narrower. No silent fallback creates misleading coverage.

Gate: P0-02/P0-05.

## V2.1 amendment

V2.1 clarification for finding 14: host/platform separation does not imply C3 administrative independence. Same-domain teams and individuals advertise C0-C2; only independently administered trust/access/recovery domains can qualify C3. Dependency ecosystem cells are separately bounded and tested.
