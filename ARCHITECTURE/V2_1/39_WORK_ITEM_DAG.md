# Work Item DAG

~~~mermaid
flowchart TD
  P006[P0-06 Canonical vectors] --> CORE[Canonical core + DB schema]
  P008[P0-08 Budget/fencing] --> CORE
  P010[P0-10 RLS/DR/load] --> CORE
  P003[P0-03 Git truth] --> GIT[Review Broker + MergeSubject]
  P004[P0-04 Eligibility] --> GIT
  P005[P0-05 Checkout] --> GIT
  CORE --> GIT
  P002[P0-02 Sandbox] --> RUN[Runner + sandbox cells]
  P007[P0-07 Evidence red team] --> VER[Verifier + gate]
  GIT --> RUN
  RUN --> VER
  P001[P0-01 Model harness] --> MODEL[Model broker + graph]
  CORE --> MODEL
  VER --> DEEP[Optional model-assisted review]
  MODEL --> DEEP
  VER --> REVIEW[Read-only production review]
  P009[P0-09 Approval/publication] --> PUB[Approval + Publication brokers]
  VER --> PUB
  CORE --> PUB
  REVIEW --> QUAL[Production qualification]
  PUB --> QUAL
  P010 --> QUAL
~~~

## Work-package rules

- Each node has one A2 manager, explicit contracts, threats, tests, and rollback/disable path.
- Cross-package identifiers come from 18-21; managers may not create local variants.
- A provider/prototype gate unlocks only its exact compatibility cell.
- Read-only review may ship before publication if its own gates pass.
- Model-assisted review may remain disabled while deterministic review operates.
- Publication is last because it combines human and provider mutation authority.

Current state: every implementation node is NOT_AUTHORIZED. Only architecture/evidence design is authorized in this pass. Prototype execution and implementation need later authorization.

