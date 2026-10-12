# Threat Model

## Protected assets

Private source and derived artifacts; provider/model credentials; approval/gate/publication signing keys; tenant identity; exact gate truth; patch destination and bytes; budget; audit evidence; component enrollment; availability of merge gating.

## Adversaries

- Malicious pull request author controlling repository files, build scripts, dependencies, output, timing, and proposed tests.
- Compromised runner, host admin, component workload, API session, provider installation, or supply-chain dependency.
- Cross-tenant attacker exploiting identifiers, caches, artifacts, logs, restore, or repository transfer.
- Network/provider faults that create duplicate, stale, reordered, late, or ambiguous state.
- Insider with one administrative plane attempting to impersonate another authority.

## Top abuse cases and controls

| Abuse | Control | Residual |
|---|---|---|
| Guest steals provider/model secret | No secret in guest; typed host broker; S2/S3 | Broker/dialer compromise |
| API forges approval | Separate WebAuthn Approval Authority | Approval service/key or user authenticator compromise |
| Compromised Publication Planner retargets patch | Independent credential gateway reconstructs exact approved effect, endpoint/body and custody | Gateway or stolen provider token retains broader provider permissions (53) |
| Provider delay/reordering leaves stale success | CheckProjectionV2 generation, confirmation/withdrawal, journal and freeze (54) | Explicit unsafe interval until provider withdrawal/freeze; P0-03 required |
| Runner fabricates evidence | Origin-only signature, C3/C4 reproduction | Colluding independent domains |
| Agent manufactures test | OracleTrustClassV1 independently qualifies expected behavior; base-pass/candidate-fail proves only causality | Untrusted oracle remains non-blocking/human review even if independently reproduced |
| Budget released while provider bills | Broker intents, actual late settlement, debt/freeze | Provider usage reporting gaps |
| Repo transfer crosses tenants | repository_tenant_epoch, closed historical ownership | Provider ID reuse/behavior; test P0-10 |
| Audit redaction breaks chain | Metadata-only immutable facts, erasable artifact references | Legal conflict requires separate policy, never chain rewrite |
| Revocation races mutation | Generation check and STARTED boundary | In-flight call may complete; reconcile and disclose |
| Version downgrade changes meaning | Critical fields and pinned negotiation | Decoder implementation defect; P0-06 |
| Shared kernel escape | S2 VM default | Hypervisor/platform zero-day; S3 for high risk |

## Explicit non-claims

V2.1 does not claim perfect sandboxing, truthful runners from signatures, atomic provider revocation, complete semantic path classification, zero source-derived cloud data, complete typed-log coverage, hard provider cost prevention after a sent request, or safe subscription integration before P0-01.

## Release posture

P0-01 through P0-10 are the threat-model closure program. A failed test narrows or disables a capability; it does not change this model to call the risk acceptable.


## V2.1 attacker containment

| Attacker/fault | Expected containment boundary | Residual / response |
|---|---|---|
| Compromised ordinary API | No authoritative evidence/plan/approval registry writes, no signer input fabrication; independently authenticated producers and constrained DB roles | API denial of service; authority/database compromise is a separate failure |
| Compromised ordinary browser UI | AA separate origin independently fetches and renders plan/custody and owns UV ceremony | Browser/OS/extension or AA renderer compromise remains endpoint threat |
| Forged enrollment or login recovery | AA administrator bootstrap, scoped credential registry, dual-authorized recovery and lineage | Compromised recovery custodians; suspend and rebootstrap with new epoch |
| Compromised credential-bearing Git gateway | Provider selected repositories, least permissions, no-bypass rulesets, TTL and external egress | Broader Git/PR authority within actual grant; freeze/revoke token and reconcile |
| Stolen GitHub Publication App token | Provider controls only; gateway cannot constrain off-path use | Broader mutations possible; no claim of provider create-only permission |
| Compromised runner / malicious agent | No reusable secret/Git authority; fenced evidence ingress and independent oracle/verification policy | Fabricated output cannot be gate-qualified merely by signatures |
| Malicious repository or dependency metadata | Sandbox plus exact authenticated dependency broker requests/hash/lockfile/policy; no credentialed arbitrary egress | Supply-chain code still hostile and executes only in sandbox |
| Database restore loses intents | Independent synchronous durable effect journal before send, authority freeze, replay and complete uncertainty accounting | Loss of witness quorum/durability blocks sends/recovery, never scans only surviving rows |
| Revoked analyzer/plugin | Trust-set invalidation propagates to evaluation and provider check withdrawal/freeze | In-flight provider success can remain effective during disclosed unsafe interval |
| Patch custody replacement/loss | Immutable tenant bundle, digest/size on each read, retention pins | Publication disabled; uncertain sent effects remain manual if bytes unavailable |
| Duplicate or late provider request | Per-effect dispatch claim, no blind resend; generation and terminality proof on reconciliation | Provider lacks atomic cancellation; freeze/manual uncertainty may persist |

Findings 1–4 and 13 are contract repairs in 52–54, 14 and 24. All enforcement claims remain bounded by the atomic provider, origin/lifecycle, custody and restore evidence; NOT_RUN is not a demonstrated containment test.
