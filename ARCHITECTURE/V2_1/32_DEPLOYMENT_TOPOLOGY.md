# Deployment Topology

## Managed control region

- Multi-AZ PostgreSQL primary with synchronous local standby and encrypted backups.
- Stateless Rust API replicas behind an ingress/load balancer.
- Rust scheduler, inbox, outbox, and reconciliation replicas using database leases.
- Separately identified Review, Checkout, Model Access, Secret Dialer, Gate, Approval, and Publication workloads.
- Next.js web deployment.
- Optional immutable artifact store with per-tenant encryption grants.
- Observability pipeline isolated from raw source and secrets.

## Customer execution

Rust runners connect outbound over mTLS. Each runner hosts a credential-free checkout parser and selected sandbox backends. It reaches managed services only through typed control, checkout, model, and artifact endpoints. No inbound customer firewall opening is required.

## Why not microservices

Ordinary business domains remain one Rust codebase and PostgreSQL schema because their transactions are coupled and separate queues add failure modes. Separate deployment roles exist only where a credential/signing/egress/hostile-workload boundary is mandatory. This is the smallest topology that preserves the authority split.

## Network policy

- API cannot reach Git write or model-provider endpoints.
- Review Broker cannot reach Git contents-write endpoints.
- Publisher cannot reach check endpoints and is restricted to approved provider API families.
- Secret Dialer reaches only pinned model-provider endpoints/accounts.
- Approval Authority has no Git/model access.
- Runner sandbox egress defaults deny; brokers are audience-bound destinations.
- PostgreSQL roles and network access are least privilege by workload.

## Regions

V2 starts with one write region. A warm recovery region receives backups/replication but has no concurrent external-effect authority. Active-active writes are out of scope because they conflict with simple, provable fencing.

## V2.1 explicit security roles and profiles

Separate protected Analysis Planner/Evidence Ingestion/Candidate/Policy/Trust authorities own gate-critical inputs; ordinary API writes cannot manufacture evidence. Runner Control is the only lease/receipt ingress; customer runners, sandboxes and agents have neither PostgreSQL credentials nor its network access. A credential-free Publication Planner calls an independently administered credential-bearing Git Publication Capability Gateway. The trusted Approval Authority serves its own top-level approval origin and retrieves canonical bytes itself. Dependency acquisition uses a typed credential-free resolver and credential-bearing bounded dialer, with only approved offline bytes reaching guests.

A same-administrator customer installation cannot advertise C3 even with multiple runners. Profiles and honest unavailable-C3 behavior are specified in 13; adding an independent verifier domain is optional and must be proven. Model/harness services may be entirely absent for deterministic FAST. No Redis/Kafka/NATS/RabbitMQ or backend language replacement is introduced.

External effect senders require the independent durable journal/witness availability defined in 50 before send. Recovery region activation is fenced and reconciles surviving and lost-intent windows before external authority is reopened.
