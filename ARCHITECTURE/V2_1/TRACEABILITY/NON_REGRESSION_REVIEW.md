# Non-regression review

This is the integrating author's evidence map, not self-issued independent approval. Independent reviewer conclusions are in REVIEWS. Every runtime assertion remains subject to its atomic P0 evidence.

| Preserved V2 decision | Current concrete boundary | Architectural check |
|---|---|---|
| Rust backend | 00F technology,06 Control Plane and04 scoped workload identities | Rust/Axum/Tokio/SQLx/Serde retained; no alternative backend introduced |
| PostgreSQL authority | 07 transactional roles/outbox/leases,50 recovery journal | Journal knows possible sends only; it cannot mint business authorization; no required Redis/Kafka/NATS/RabbitMQ |
| Next.js/React/TypeScript frontend | 16 ordinary UI and52 independent AA origin | Web stack retained; ordinary UI cannot mint approval |
| Rust runner | 10/31/32 | Customer runner and supported cross-platform capability cells retained |
| No direct agent Git authority | 11/22/53 | Credential-free execution/planner; typed independent gateway only |
| No reusable trusted credential in hostile execution | 10/12/22/55 | Checkout/model/dependency dialers separate; scripts execute in secret-free sandbox |
| Exact SHA/tree lineage | 30, VerifiedPublicationPlanV2 and per-call request schemas | Algorithm-qualified OIDs and exact candidate/provider tree equality |
| Tenant isolation | 07/18/29,ScopeV1 | Composite keys, authenticated tenancy,RLS,repository epoch,typed scoped signatures |
| Human-authorized publication | 14/52/53 | Independent canonical display, fresh UV,current credential,exact plan and per-effect authorization |
| Review/publication authority separation | 04/08/22/53/54 | Separate Apps, keys, roles and permitted APIs; no shared bypass |
| Customer-owned agent/API modes | 11/12/31/47 | Native/API/subscription cells remain separately qualified; unsupported modes visible |
| Graph Engineering resource control | 11/33,ResourceEnvelopeV2 | One root bounds calls/events/dependencies/model tokens/CPU/memory/time/nodes/depth/concurrency/artifacts/spend |
| FAST without model availability | 05/09/17/38/39 | Deterministic analysis/verification path has no model dependency |
| Explicit unsupported states | 13/19/31/47/56 | No silent sandbox/independence/provider fallback; manual acceptance never deterministic PASS |
| Fail-closed security | 00D/29/35/50/54 | Actual expiry,witness-before-send,unknown tombstones,unsafe external interval disclosed,post-restore fresh authority bootstrap |

V2 semantics deliberately amended: INV-01 independent intent witness durability; INV-07 trusted oracle in addition to causality; INV-10 trusted display/credential lifecycle; INV-12 gateway-versus-token distinction; INV-17 one STARTED per provider call. New INV-21 and22 state projection exposure and non-promoting manual resolution. These amendments preserve intent while repairing the accepted defects; ADR-016–028 record why.
