# Observability

## Signals and privacy

Metrics cover admission/queue age, lease expiry and post-expiry rejects, owner/generation/epoch fencing, required manifest units missing versus completed with zero findings, provenance/writer-role denials, verification/oracle classes, gate verdict and currentness, check desired/confirmed generation lag, emergency freeze duration, per-call effect state/age, witness acknowledgment latency/coverage, journal high-water/restore gap, custody expiry, manual-case age/expiry, model usage/debt and audit export lag.

Traces propagate tenant-safe review/evaluation/attempt/effect IDs, intent/state digest, projection generation and protocol version. Typed logs allowlist metadata; no raw source, prompts, model output, secrets, patch bytes or arbitrary subprocess/provider strings. Detailed diagnostic artifacts require bounded encrypted tenant custody. Privileged SDK stderr/crash dumps are disabled or filtered by default.

## Audit versus telemetry

AuditEnvelopeV1 facts are authoritative metadata committed with state transitions; a per-stream digest chain binds prior event and sequence. Journal entries and independent witness receipts are mandatory before privileged sends. Telemetry can be sampled/dropped and cannot establish lease validity, no-send proof, PASS, trusted oracle origin or human approval. A dashboard links to the actual signed record. General remote audit seal lag may alert without halting review; effect journal acknowledgment loss must stop new sends.

## Alerts and owners

| Urgency | Signal | Owner/action |
|---|---|---|
| Page | PostgreSQL/epoch/clock authority loss, unexpected privileged egress, split journal chain, unjournaled send, cross-tenant authoritative write, signature/key-role failure, publication tree mismatch | Security/on-call contains egress and freezes affected capability |
| Page | Internally revoked success not safely withdrawn during emergency, old gateway not fenced after restore | Repository security owner enforces merge freeze; incident remains open until provider evidence confirms containment |
| Urgent | UNKNOWN/MANUAL_RECONCILIATION backlog, witness degraded copy, missing manifest units, projection lag, near-expiry custody, invalid approval generation, local audit append failure | Named workflow/provider/custody owner resolves underlying authoritative state; no success override |
| Ticket | Advisory analyzer degradation, routine seal lag, capacity trend or unsupported customer cell | Product/operations records scope and remediation |

Track normal policy-change exposure separately from emergency security revocation. Projection latency SLO is an objective, not an upper-bound security guarantee. UNKNOWN age never automatically becomes failure or absence; manual expiry creates escalation without granting retry.

## SLO attribution and tests

Use CONTROL_PLANE, PROVIDER, CUSTOMER_RUNNER, POLICY_BLOCK, UNSUPPORTED, SECURITY_CONTAINMENT and EXTERNAL_AMBIGUITY. Atomic claims require canaries for sensitive log leakage, telemetry-drop independence of safety, zero-send on lost witness acknowledgment and recovery of journal/DB discrepancies. Prototypes are NOT_RUN; metrics definitions do not claim validation evidence.
