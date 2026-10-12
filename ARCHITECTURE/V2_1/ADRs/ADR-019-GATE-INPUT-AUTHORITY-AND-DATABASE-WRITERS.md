# ADR-019: Gate Input Authority and Database Writers

Status: ACCEPTED_V2_1_ARCHITECTURE; capability validation remains pending.

V2 protected the gate signer without enumerating who could create every input it signed. A compromised ordinary API could therefore fabricate trusted-looking records in an underspecified deployment.

Decision: document 07's per-input producer/authentication/signature/writer/immutability/consumer/scope/attempt/epoch/revocation matrix is normative. Gate Authority verifies source signatures and current pointer/revocation state, not merely database row shape. Runtime roles cannot own protected tables, bypass RLS or inherit another authority role. Ordinary API writes requests only. Signed runner claims still require qualifying analyzer/verifier capability and policy evidence.

Consequence: separate authority pools and producer keys are required even in a shared Rust build. Database superuser compromise remains residual privileged risk; input signatures and separated authorities reduce ordinary API compromise impact. P0 evidence must attempt each forbidden writer and forged-origin receipt. Amends ADR-013; finding 5. Production capability gated pending atomic P0 validation.
