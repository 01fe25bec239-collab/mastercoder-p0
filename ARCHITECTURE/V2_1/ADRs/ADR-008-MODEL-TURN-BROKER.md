# ADR-008: Structured Model Turn Broker

Status: ACCEPTED_DIRECTION.

## Context

An opaque one-shot text envelope cannot express provider-native tool loops, while a proxy or guest credential defeats confinement.

## Decision

Use ModelTurnV2 structured messages, correlated tool use/results, schema-pinned capabilities, a policy/metering Model Access Broker, and a minimal Secret Dialer that injects endpoint/credential but does not parse content. API, subscription, and native harness modes are separate compatibility cells.

## Consequences

Adapters are provider/version specific. Some subscription modes may be disabled for technical or legal reasons. There is no unrestricted proxy fallback.

Gate: P0-01.

## V2.1 amendment

V2.1 clarification: model integration is optional for deterministic FAST. Every new model authorization checks actual lease expiry, and external request accounting follows per-effect intent rules; customer API and harness modes remain distinct.
