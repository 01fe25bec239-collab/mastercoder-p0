# Deep scan sequences

~~~mermaid
sequenceDiagram
  participant U as User/Policy
  participant A as Ordinary Rust API
  participant Q as Protected Analysis Planner
  participant P as PostgreSQL
  participant C as Runner Control / Evidence Ingestion
  participant W as Runner
  participant S as Sandbox
  participant M as Model Broker
  participant X as Secret Dialer
  participant V as Independent Verifier
  U->>A: Request optional deep scan
  A->>P: Admission request and ordinary workflow metadata
  A->>Q: Request policy-bound analysis planning
  Q->>P: Seal logical analysis plan and authorized root envelope
  W->>C: Outbound mTLS lease request
  C->>P: Atomic owner/generation/epoch/actual-expiry predicates
  C->>Q: Allocated immutable AttemptBindingV2
  Q->>P: Seal attempt-bound manifest under current lease before work release
  C-->>W: Exact binding, sealed manifest and capabilities
  W->>S: Materialize verified tree and approved offline dependencies
  S->>M: Structured ModelTurnV2 with bound one-use intent
  M->>P: Validate actual lease expiry and consume root allowance
  M->>X: Canonical typed provider request
  X-->>M: Bounded response and provider usage metadata
  M-->>S: Parsed response with correlated tool request
  S->>M: Correlated tool result
  S->>W: Candidate findings and bounded artifact output
  W->>C: Completion receipts for every expected unit
  C->>P: Verify and insert immutable receipts
  V->>C: Clean-room verification and trusted-oracle evidence
  C->>P: Qualified verification ingress
  M->>P: Settle actual usage including late billing
  W->>C: Final completion
  C->>P: Current attempt transition, or audit-only stale result
~~~

Only trusted control/broker roles contact PostgreSQL. The sandbox has typed audience-bound broker access and no database or reusable model/registry/provider credential. Broker-created graph edges inherit root counters and capability subsets; opaque native descendants are UNOBSERVABLE and cannot claim independent evidence. Customer-owned harness/API cells stay distinct.

Cancellation commits generation/state revocation; new tool/model/dependency intents reject even before reaper cleanup when the lease has expired. Already sent provider usage still settles, and late findings cannot enter a current completion/verification set. Retries use a new generation and immutable state, never assumed model-memory continuation. Agent-authored expectations remain advisory without independently trusted oracle provenance.
