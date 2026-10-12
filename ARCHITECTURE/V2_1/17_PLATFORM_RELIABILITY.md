# Platform Reliability

## Reliability model and objectives

PostgreSQL is the authoritative business write plane with HA and tested regional restore. Rust roles are replaceable; customer runners hold no durable control authority. Initial targets remain pending atomic P0-10 evidence: API/control availability 99.9% monthly, accepted provider event to durable inbox p95 <30 seconds while provider is available, ready work to eligible connected runner p95 <60 seconds at declared capacity, normal check projection p95 <60 seconds, control-plane RPO <=5 minutes, infrastructure regional RTO target <=4 hours. Authoritative capability recovery can exceed four hours: document 35 requires complete fresh security bootstrap, and provider uncertainty has no guaranteed completion bound.

External-effect intent coverage is stricter: RPO zero for every send across the declared recovery failure envelope, using document 50's synchronously witnessed journal in a separate restore domain. Loss of its required witness blocks new sends regardless of the ordinary API SLO. This is a durability contract pending P0 proof, not a claim of invulnerability to simultaneous loss of every independent copy.

## Failure behavior

| Dependency | Required behavior |
|---|---|
| PostgreSQL or authority clock unhealthy | Stop new authoritative transitions and effects; never borrow an old lease |
| Journal/witness cannot confirm PREPARED or SEND_ARMED | DO NOT SEND; keep pending or uncertain according to durable record |
| Artifact/custody unavailable | Mandatory evidence is unavailable/non-pass; missing approved patch bytes prevent publication |
| Model unavailable | FAST deterministic review proceeds; mandatory model units remain incomplete/non-pass |
| Git provider unavailable/delayed | Persist projection lag or UNKNOWN effect; no blind mutation retry |
| Audit exporter unavailable | Required local audit still commits; alert and backfill; no fleet-wide review halt |

Admission uses tenant concurrency, budget, runner/artifact capability, database pools, witness health and provider limits. Only unsent work is backoff-retryable. A provider-rate error after dispatch is a recorded result; a replacement call needs proven terminal non-execution or provider-guaranteed idempotency and fresh authority.

## Time bounds and incident posture

Each individual effect has a default maximum 5-second dispatch window from STARTED and a 60-second client request deadline. A stricter approved plan/grant wins. Exceeding either denies a not-yet-sent call or marks an in-flight call UNKNOWN; it does not prove the provider stopped. Every additional call has a new recorded effect attempt. Parent publication state cannot extend these limits.

Normal projection lag is measured separately from security revocation exposure. Emergency revocation freezes repository merge posture under document 54 and pages an operator until provider confirmation; the 60-second projection target never promises instantaneous provider revocation. Operational runbooks name owners for keys, egress, witness durability, restore, stale success and unresolved effects. Metrics retain distinct platform/provider/customer/policy/unsupported/security causes so availability pressure cannot turn non-pass into PASS.
