# Git Publication Capability

## V2.1 decision and defect

V2 08 and ADR-003 incorrectly treated “create-only” as a property of the Publisher App permission. GitHub's Contents write permission is used by both reference creation and update/deletion; endpoint restrictions must therefore be enforced outside the planner. The preserved decision is separate Review and Publication Apps. ADR-016 adds an independent credential-bearing gateway.

## Positive authority boundary

The Publication Planner has no Git provider token, App private key, token mint permission, provider egress, gateway configuration permission, approval signing permission, or direct write permission to authoritative plan/approval/effect rows. It submits only typed publication IDs to the Git Publication Capability Gateway. The gateway has a separate workload identity, deployment/configuration authority, KMS policy, database role and fixed provider egress. The ordinary API and planner cannot load code or change its policy. The gateway independently fetches signed canonical plan, current approval/binding, custody bytes and current authority; it never trusts request bodies or URLs supplied by a caller.

GitPublicationCapabilityV2 is an internal authorization envelope for exactly one EffectIntentV2, not a bearer provider token or authorization for the whole parent operation. SCHEMAS defines its complete wire shape. It binds tenant, stable repository/installation/App identity, ownership epoch, authority epoch, exact publication/plan/approval/custody digests, effect identity and request digest, gateway generation, destination ref, expiry and one-use nonce. A capability cannot be retargeted or expanded; the gateway constructs HTTP itself from the verified plan.

## Closed GitHub mutation vocabulary

| Effect kind | Allowed operation | Exact body constraint |
|---|---|---|
| CREATE_BLOB | POST repository Git blobs endpoint | Blob bytes and encoding from custody; hash equals plan object |
| CREATE_TREE | POST repository Git trees endpoint | Exact approved base_tree and canonical changed entries only |
| CREATE_COMMIT | POST repository Git commits endpoint | Approved tree, one exact approved parent, fixed identity/message/timestamps from plan; no caller-added parents |
| CREATE_REF | POST repository Git refs endpoint | Exact `refs/heads/mastercoder/fix/<publication_id>` and confirmed approved commit |
| CREATE_FIX_PR | POST repository pulls endpoint | Exact confirmed fix ref, approved target base, title/body/draft/provenance from plan |

The schema effect discriminant and request digest select one row only. Read-only object/ref/PR queries are separately bounded by repository, pagination, identity and deadline for reconciliation; they never authorize writes. Provider endpoint owner/name is resolved from stable repository ID and checked before send; renamed/transferred or mismatched identity rejects. No arbitrary host, port, path, query, redirect, header, GraphQL document or raw HTTP passthrough exists. The gateway verifies provider TLS and permits only the configured API origin; token minting and HTTP redirect handling never forward credentials to another origin.

Deny merge, any existing-ref update (including fast-forward), force update, delete, source/default/protected-branch mutation, arbitrary Contents API writes, ruleset edits/bypass, arbitrary PR mutation, actions/workflow administration and every endpoint absent from the table. Existing deterministic ref with the exact expected OID is reconciled as the prior effect, never updated; conflicting contents stop in MANUAL_RECONCILIATION. Unsupported Git features and workflow-file changes requiring additional permissions remain disabled until separately approved policy and capability tests support them; no permission escalation occurs during publication.

## Authorization and send boundary

For every effect, lock its row plus publication authorization/current epoch/revocation rows in the trusted transaction. Require PENDING, correct gateway identity/generation, live lease (`lease_expires_at > database_current_time`), unchanged plan and custody, current tenant/repository scope, active approver binding/generation, unused effect capability, unexpired approval, current policy and authority epoch, confirmed prerequisites, and no publication/repository freeze. Consume capability nonce uniquely with `(tenant_id, capability_id)` and record the exact canonical request digest. STARTED and send follow the durable journal protocol in 50; no provider send occurs without confirmed journal durability. Check the bounded send deadline again immediately before egress. Each future effect requires this transaction again.

After blob/tree confirmation require algorithm-qualified returned OID equality. CREATE_COMMIT cannot start until the provider tree equals approved candidate tree. CREATE_REF cannot start until commit tree/parent equality is confirmed. CREATE_FIX_PR requires confirmed exact ref. Revocation after CREATE_COMMIT may leave immutable orphan objects; it forbids not-yet-started ref/PR effects. A started provider call can still complete; deadlines bound local dispatch, not provider execution. UNKNOWN is reconciled, never replayed on a single “not found” read.

## Provider defense and honest residual authority

The App is selected-repository scoped, short-lived-token scoped and granted only necessary Contents write and Pull requests write permissions, with no Checks write or repository administration. Repository rulesets must protect source/default/protected branches and exclude this App from bypass actors. Setup and periodic drift checks record effective rulesets and deny publication when required posture is absent. Gateway cannot repair rulesets itself; a repository security administrator must do so.

Compromised planner: independent gateway confines it to currently human-approved exact effects. Compromised gateway: its own validator is no longer a boundary; provider permissions/rulesets, token TTL and outer egress still limit damage, but broader Git/PR mutation within that provider grant is residual authority. Stolen token: the gateway is bypassed entirely and no create-only claim is made; provider-enforced permissions/rulesets and revocation are the remaining controls. Loss of token/App integrity triggers publication freeze, provider credential revocation, read-only impact scan and manual reconciliation. No claim says compromise of both Git authorities is necessary to misuse the Publication token's own permissions.

## Acceptance and official sources

P0-03 and P0-09 atomic claims must attempt every denied endpoint with a compromised planner, independently test actual stolen-token permissions and no-bypass rulesets, mutate body/ref/tenant/epoch/digest/expiry, and revoke between commit and ref. Gateway deployment independence is part of evidence. Capability remains gated until those tests pass.

GitHub documents Contents write for create/update/delete reference operations: [Git references REST API](https://docs.github.com/en/rest/git/refs). Bypass actors are configurable repository policy: [Creating rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository). Installation tokens support repository/permission narrowing and have provider-defined expiry: [Installation access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-an-installation-access-token-for-a-github-app). These facts support the need for an independent gateway; they do not prove the proposed gateway implementation.

## Eligibility at the protected boundary

ResourceEnvelopeV2 and GitPublicationCapabilityV2 bind eligibility_proof_digest to EligibilityProofV2. Before reservation and every publication effect STARTED, the protected authority resolves that signed proof, verifies its issuer, tenant/repository/epoch, exact PR and stable opener/linked user, membership generation, policy digest and DB-time expiry, and obtains current provider/membership confirmation when required by the policy freshness bound. MY_PRS_ONLY/ORG_MEMBERS restrictions cannot be replaced by approver scope. Mismatch, unavailable required read or changed eligibility blocks the effect; a renewed proof must bind the same exact permitted subject and current policy.
