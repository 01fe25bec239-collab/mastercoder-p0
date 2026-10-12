# ADR-009: Root Resource Envelope

Status: ACCEPTED_DIRECTION.

## Context

Producer-asserted agent counts and released reservations do not bound hidden descendants or late provider billing.

## Decision

Every attempt graph shares one broker-enforced ResourceEnvelopeV2 for model calls, tokens/spend, tasks, depth, concurrency, time, events, and bytes. Broker-created child edges are authoritative and cannot mint budget. Opaque native subagents are outer-limited only. Actual late usage always settles.

## Consequences

Fine-grained hidden topology cannot be promised. Provider hard caps remain bounded by per-request intent plus debt/freeze, not atomic recall after send.

Gate: P0-08.

## V2.1 amendment

V2.1 clarification: dependency fetches and visible child work consume root ceilings; missing mandatory units cannot be shed into PASS. Actual lease expiry rejects new intent admission independently of the reaper; late usage settles without authority revival.
