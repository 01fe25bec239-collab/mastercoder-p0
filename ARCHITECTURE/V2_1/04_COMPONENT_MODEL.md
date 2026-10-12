# Component model and authority

| Component | Owns | Forbidden authority |
|---|---|---|
| Ordinary Next.js Web / Control API | Requests, presentation, nonauthoritative candidate intake | Approval signing/display origin, trusted evidence writes, gate signing, Git/model credentials |
| Scheduler / Runner Control | Protected leases, authenticated sessions, outbox, admission | Gate verdict manufacture, direct provider mutation |
| Rust runner / sandbox / agent | Bounded credential-free execution and claims | PostgreSQL credentials, provider secrets, trusted result acceptance |
| Analysis Plan / Evidence / Verification Authority | Policy-derived manifests, authenticated completion validation, immutable accepted receipts/sets | Human approval or Git mutation |
| Policy / Trust / Provider Subject Authority | Authorized immutable policy/trust/revocation/currentness records | Caller-supplied trusted rows |
| Gate Authority | Independent qualified reduction and proof signing | Unverified API rows, signing arbitrary caller digest |
| Review/Check Broker | Dedicated read/check App, current provider observations, projection reconciliation | Contents mutation, publication App, ruleset administration |
| Checkout / Dependency Acquisition Broker | Finite exact object/package acquisition and credential injection | Scripts, hostile semantic parsing, arbitrary guest egress |
| Model Broker / Secret Dialer | One-use turns, root accounting, approved provider requests | Guest credentials, unrelated endpoint proxying |
| Approval Authority and controlled origin | Canonical display, credential lifecycle, fresh UV, assertion | Ordinary UI-supplied diff truth or email recovery approval |
| Publication Planner | Orchestration using immutable verified plan identifiers | Provider credential/egress, approved-plan mutation |
| Git Publication Capability Gateway | Positive per-call request validation, provider credential, lineage check | Merge/update/delete/force/source-ref endpoints under uncompromised boundary |
| Journal sequencer / independent witness | Durable append chain and unique per-effect send claims | Business authorization or reuse of old send claims |
| Manual Resolution Authority | Evidence-bound decisions, recovery/reconciliation workflow | Converting uncertainty into PASS |
| PostgreSQL | Business truth and protected scoped write roles | Restored old epoch as current authority |
| Custodian | Encrypted exact bytes and scoped pins | Truth without independent digest/currentness checks |

The modular Rust codebase may implement multiple service roles, but API/runner principals must not share signer keys, privileged role credentials or deployment/configuration authority. Gate-critical writes use the complete matrix in 07; only credential-free pure functions may share the ordinary API's process. Privileged roles live in independently constrained workloads. Journal witness copies are independently recovered and synchronously confirmed as described in 50. Additional services exist for authority separation, not speculative scale.
