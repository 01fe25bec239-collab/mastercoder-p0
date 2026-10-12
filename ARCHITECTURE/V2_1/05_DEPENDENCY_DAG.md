# Dependency DAG

~~~mermaid
flowchart TD
  S[00D invariants and frozen Rust/PostgreSQL stack] --> C[Canonical schemas and state machines]
  C --> TB[Trust and identity boundaries]
  C --> PG[PostgreSQL workflow, time, epochs and journals]
  TB --> RUN[Runner, checkout and dependency acquisition]
  PG --> API[Rust control roles]
  API --> GIT[Git exact subject and eligibility]
  RUN --> DET[Deterministic analysis and expected manifest]
  GIT --> DET
  DET --> VER[Independent verification and oracle trust]
  VER --> FAST[FAST gate and check projection]
  TB --> MODEL[Optional customer-owned model/harness cells]
  PG --> MODEL
  MODEL --> DEEP[Model-assisted findings and graph]
  VER --> DEEP
  FAST --> PATCH[Optional trusted approval and publication capability]
  C --> TEST[Atomic P0 contracts]
  TEST --> QUAL[Future cell qualification]
~~~

There is no MODEL → DET, VER or FAST dependency. Deterministic analysis, evidence completeness, verification and current gate projection are implementable and testable with all model modes disabled. Model-assisted deep review consumes those established contracts as a later optional branch. If a tenant explicitly chooses a policy requiring models, that policy has a model dependency; FAST does not.

Freeze order: invariants and canonical schemas; authority/time/state contracts; deterministic read-only path; optional model and publication paths; independent atomic prototype evidence per capability; explicit later implementation authorization. Schema-ready architecture is not production freeze. No P0 experiment or product work is authorized in this repair.
