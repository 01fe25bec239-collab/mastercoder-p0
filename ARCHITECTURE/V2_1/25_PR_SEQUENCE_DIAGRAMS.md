# Pull request review sequences

All arrows denote authenticated calls or actual trusted server database access; no runner, sandbox or agent has PostgreSQL access.

~~~mermaid
sequenceDiagram
  participant G as Git Provider
  participant R as Review Broker / projection writer
  participant C as Protected Candidate and Analysis Authorities
  participant P as PostgreSQL
  participant I as Runner Control / Evidence Ingestion
  participant W as Rust Runner
  participant V as Independent Verifier
  participant D as Gate Decision
  G->>R: Webhook hint
  R->>G: Read exact current candidate and repository posture
  R->>C: Authenticated current subject observation
  C->>P: Freeze candidate, policy and logical analysis plan
  W->>I: mTLS lease request
  I->>P: Atomic lease owner/generation/epoch/expiry transaction
  I->>C: Allocated AttemptBindingV2 and locked current candidate
  C->>P: Seal attempt-bound ExpectedAnalysisManifestV1 before work release
  I-->>W: Bound lease and sealed expected unit set
  W->>R: Bounded checkout grant
  R-->>W: Credential-free exact Git objects
  W->>W: Bound tier; run every required unit
  W->>I: AnalysisCompletionReceiptV1 for each unit, including zero findings
  I->>P: Verify provenance, manifest, actual expiry; immutable receipts
  V->>I: Independently bound verification and oracle provenance
  I->>P: Protected verified ingress only
  D->>P: Exact completeness/trust/currentness reduction and gate proof
  P-->>R: Outbox requests CheckProjectionV2 generation
  R->>G: Re-read currentness; reconcile/write projection
  R->>P: Provider readback confirmation or UNKNOWN
~~~

Head/base/queue, tenant epoch, policy, authority epoch, analyzer revocation or trust changes invalidate the evaluation and schedule withdrawal/replacement of any prior provider success under 54. Refusing a future stale write alone does not revoke prior success. Normal policy change follows the staged activation and confirmation rules in 54; emergency security revocation immediately blocks internal use and engages the defined repository freeze procedure while provider reconciliation remains uncertain. There is a real external revocation interval; instantaneous withdrawal is not claimed.

Duplicate webhooks reuse exact subject/policy identity. Concurrent/delayed projection writes use per-projection generation and readback reconciliation, never resurrecting a superseded generation as internally authoritative. Unsupported mandatory analyzer/sandbox/independence/coverage yields a precise non-permitting status. FAST contains no model call.
