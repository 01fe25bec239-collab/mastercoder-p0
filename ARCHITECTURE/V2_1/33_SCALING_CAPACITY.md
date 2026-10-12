# Scaling and Capacity

## Scaling units

- API: requests and PostgreSQL connections.
- Provider ingest/check: installations, webhook rate, provider API quotas.
- Scheduler/outbox: ready jobs and effect backlog.
- Runners: concurrent sandbox slots by capability.
- Analysis: file/object bytes, tool CPU/memory, artifact output.
- Model access: calls, tokens, spend, latency, provider rate.
- Verification: duplicate/independent runs by C class.
- Publication: low-volume, high-assurance effects.

## Capacity controls

Admission reserves tenant concurrency, root resource envelope, expected PostgreSQL/provider pressure, and a compatible runner cell. Per-tenant weighted fairness prevents one large scan from monopolizing ready work. Queue age and reason are visible.

PostgreSQL tables with high churn are partitioned only after measured need; likely candidates are inbox/outbox, audit facts, attempts/events, and ledger entries. Indexes follow exact acquisition/currentness queries. Connection pools are bounded per role and globally budgeted.

## Load shedding

Noncritical advisory analyzers and retained artifacts may be disabled by explicit policy before admission. Work already declared mandatory is never silently shed. Publication and approval paths have small dedicated pools so bulk analysis cannot starve security effects.

## Capacity gates

P0-08 covers fault and budget capacity. P0-10 covers sustained ledger/audit/outbox load, graph fan-out, provider rate limits, replay, and DR. Initial product limits remain conservative until measurements exist:

- bounded repository bytes/object count
- bounded changed files and patch size
- bounded graph depth/tasks/concurrency
- bounded model calls/tokens/spend
- bounded event/artifact bytes
- bounded active publications per tenant

Exact numbers are product configuration, not architecture invariants, and must stay within demonstrated envelopes.

## V2.1 completeness under pressure

The root envelope covers dependency packages/bytes/requests and every graph descendant. Once ExpectedAnalysisManifestV1 freezes mandatory units, load shedding cannot omit them and still PASS. If capacity expires, the unit reports FAILED/UNSUPPORTED/CANCELLED as appropriate and the evaluation remains non-permitting. No model capacity is reserved for FAST. Independent-domain C3 capacity is a separately measured profile, never inferred from runner count. Atomic load claim P0-10E predeclares the tested envelope and tests both its limit and limit+1; product limits may not exceed the demonstrated envelope.
