# ADR-007: Sandbox Tiers and No Downgrade

Status: ACCEPTED_DIRECTION.

## Context

Linux shared-kernel isolation cannot protect reusable host authority against the declared hostile repository attacker, and semantic path denylists are incomplete.

## Decision

Define S0 no execution, S1 low-risk process/container, S2 VM/microVM hostile default, and S3 dedicated/stronger isolation. Bind the required tier and actual capability digest into the attempt. Credentials remain outside every guest. Unknown execution-control semantics raise tier or block.

## Consequences

Cost and platform coverage may be lower. Shared-kernel Linux cannot be relabeled S2.

Gate: P0-02.

## V2.1 amendment

Amended by ADR-026: dependencies arrive through bounded authenticated grants/pre-resolved artifacts; package scripts run inside the bound sandbox, never credential-bearing acquisition code. Runner receipt and lease operations route through Runner Control, with actual database-time expiry.
