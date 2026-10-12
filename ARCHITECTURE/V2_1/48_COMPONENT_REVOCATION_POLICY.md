# Component Revocation Policy

| Component | Revocation action | New work | In-flight work | Recovery |
|---|---|---|---|---|
| Runner | Quarantine identity, rotate generation/session | Rejected | Results fenced; spend settles | Re-enroll/attest; new generation |
| Review Broker/App | Revoke token/key, deny egress | No ingest/check | Ambiguous check reconciled later | Rotate App key/session; read current subjects |
| Checkout Broker | Revoke App/session/grants | No grants/calls | Ambiguous grant terminated | New credential generation and explicit grants |
| Model Broker | Rotate generation, cancel intents | No turns | Provider may bill; settle | Reconcile usage, requalify cell if compromise |
| Secret Dialer/provider credential | Revoke/rotate credential, deny egress | No dials | Provider call may finish | Reconcile and rotate broker generation |
| Gate signer | Revoke key/generation | No decisions accepted | Affected projections invalidated/withdrawn; emergency freeze (54) | New key; impact analysis |
| Approval Authority/key | Revoke registrations/key/generation | No proofs | Each not-yet-STARTED effect rejected; only already-STARTED individual call may finish | New ceremony/key and impact analysis |
| Git Capability Gateway/Publication App | Revoke key/token, deny egress | No effects | STARTED effect may finish/unknown | Provider read-back, key rotation, manual reconcile |
| API/admin identity | Revoke session/role | No intents | Cannot revoke other authority’s STARTED call atomically | Audit, rotate affected generations |
| Tool/runner release | Revoke capability bundle | No scheduling | Affected evidence invalid; projections withdrawn/frozen; fresh evaluation required | Upgrade and reverify |

## Rules

Revocation is committed in PostgreSQL with database time and a new generation/state. Brokers check immediately before EACH individual EffectIntentV2 STARTED, including live lease expiry against database time. Parent publication progress grants no future effects. A provider call already sent is outside the transaction; V2 records that limit rather than promising atomic cancellation.

An authority_epoch rotation is used for broad split-brain or systemic compromise. It invalidates sessions/intents for new authority and forces reconciliation. Credential rotation alone does not rewrite old evidence. Impact analysis links superseded verdicts/publications and must withdraw every affected potentially authoritative success under CheckProjectionV2 and 54; normal policy changes use staged freeze, emergencies expose the unsafe interval until provider freeze/withdrawal confirms. Immutable history remains.

## Kill switches

Kill switches exist per provider installation, tenant, component role/generation, model cell, sandbox cell, analyzer/tool bundle, publication, and global external egress. A kill switch disables capability; it cannot create PASS or route through a weaker fallback.


## Approval and custody impact

Credential binding suspension/revocation/replacement or approver-scope changes increment generation, invalidate challenges/assertions and block not-yet-started publication effects. Login recovery cannot restore these rights. A stolen Publication token freezes publication and requires provider-side credential revocation because gateway rules do not constrain a token used elsewhere. Custody loss/deletion revokes plan authority and preserves uncertainty for already-started effects. Check withdrawal uses the current Review Broker identity and a revocation-purpose authorization over historical projection identity; invalidating the old gate proof does not prevent its safe withdrawal, but never permits reuse of that proof for SUCCESS.

## Business-database rollback containment

There is no claimed lossless independent log of every security grant/revocation in V2.1. Following a business-DB restore, document 35 therefore requires full fresh independently reviewed security bootstrap: no restored key, authenticator, policy, trust/analyzer permission, membership or role is automatically current or re-signed under the new epoch. Independent offline recovery roots and fresh provider ownership establish new credentials/inventory; old records remain historical. This denies resurrected grants even when a revocation itself was lost. Failure to complete bootstrap keeps privileged authority frozen.
