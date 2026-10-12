# Fix Publication Sequences

~~~mermaid
sequenceDiagram
  participant W as Patch Producer (untrusted)
  participant RC as Runner Control / Evidence API
  participant V as Clean-room Verifier
  participant C as Custody Broker
  participant P as PostgreSQL (trusted roles only)
  participant UI as Ordinary Next.js Browser
  participant AA as Approval Authority origin
  participant PL as Credential-free Planner
  participant GW as Git Capability Gateway
  participant J as Independent Durable Effect Journal
  participant G as Git Provider
  W->>RC: PatchCandidateV2 and bytes, fenced identity
  RC->>C: Store immutable tenant-scoped custody
  RC->>V: Exact candidate and custody references
  V->>C: Read/hash exact bytes and base material
  V->>RC: Signed VerifiedPublicationPlanV2
  RC->>P: Authority-validated immutable plan and pin
  UI->>AA: Top-level navigation with plan ID only
  AA->>P: Read authoritative plan and credential binding
  AA->>C: Read/hash canonical complete display bundle
  AA->>AA: Trusted rendering; fresh WebAuthn UV ceremony
  AA->>P: Atomic nonce and HumanApprovalAssertionV2
  P-->>PL: Outbox approved publication ID
  PL->>GW: Typed publication ID
  loop Each individual blob, tree, commit, ref, PR effect
    GW->>C: Verify pinned exact effect bytes
    GW->>J: Confirm PREPARED record
    GW->>P: Atomic current approval/epoch/lease predicate; STARTED
    GW->>J: Confirm SEND_ARMED and exclusive dispatch claim
    GW->>G: One exact approved request before dispatch deadline
    G-->>GW: Result or ambiguous timeout
    GW->>J: Durable outcome / uncertainty
    GW->>P: CONFIRMED or UNKNOWN, fenced update
    GW->>GW: Require exact OID/tree/prerequisite equality
  end
~~~

All runner/producer/verifier submissions route through authenticated trusted ingress; no runner/sandbox/browser PostgreSQL credential is implied. AA and gateway have constrained trusted roles. Journal stages and limits are defined in 50; a missing durable receipt means DO NOT SEND.

Every effect revalidates current authority. Revocation after commit but before ref forbids starting the ref/PR even though the parent operation has progress. Only an already-started individual call may finish; its late result is reconciled. A returned-tree mismatch stops commit/ref creation. Deterministic identifiers do not grant automatic retry: UNKNOWN remains unresolved until exact completion or terminal non-execution is proved; a momentary missing object/ref/PR is insufficient. Custody stays pinned until all effects are terminal. See 14, 52 and 53.
