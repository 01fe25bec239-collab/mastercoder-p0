# CCR-W01-002 — reviewed owner decision proposal

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

STATUS: AWAITING_OWNER
DECISION_KIND: SINGLE_OPERATOR_DECISION
OWNER: HUMAN:master (program owner)
OWNER_RESPONSE: PENDING
EFFECTIVE_ADOPTION: FALSE

Exact proposal: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/CONTRACT_CHANGE_REQUESTS/CCR-W01-002.md
Target SHA256: f55eee3e4906b73cbbd9a8cbf3cc6c4c97da0d08af3d11ac65b18e99ee154701
Fresh independent W04-playbook review: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws04/ws04-agent-model-graph/coordination/REVIEWS/CCR-W01-002-A3_REVIEW.md
Review SHA256: a382999a6a8ca1932413cbfdd608f69f7cb108f2057200492423de109922fa3f
Verdict: SUPPORTED for the restricted profile; no source acceptance/implementation permission from A3. Root verified pins/source clean state at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/CCR-W01-002-A3-PROCESSING-20261107/verification.json. Historical report/proposal are unchanged.

## Concrete owner choice proposed

Adopt the restricted disposable-prototype registration profile: ES256/P-256 packed SELF attestation only, no x5c/certificate path. The new credential key must verify the signature over the exact create authenticatorData and SHA256 of exact clientDataJSON. Preserve type/challenge/origin/RP/UP/UV, key/algorithm/ID validation and existing caller authorization/freshness/one-use checks. Actual unsigned none responses cannot produce proof-bearing registration success. No silent fallback or extra ceremony.

Accept the compatibility/trust scope: none-only, certificate-backed packed, other formats/algorithms and unsupported client cells are refused. A request for attestation does not guarantee a usable response. No manufacturer provenance, trusted hardware, genuine biometric UV or broad passkey support is established by self signature. No real-device/browser cell is enabled by this decision.

Record the declared policy prospectively in live T009 task/repair dispatch and the mc-webauthn-verify module documentation within its existing source scope, with an interface notice to enrollment/fixture consumers. Record actual returned format and authenticator/backup characteristics in W05's planned docs/authenticator-matrix.md (T032) only after its authorized cell tests. Existing parser BS-without-BE refusal stays; broader backup/UV/hardware suitability remains an explicit AA admission policy, not an automatic property of cryptographic parsing. Every physical/browser cell remains UNVERIFIED/disabled until separately qualified. No new fallback, permissive backup policy or configuration switch is adopted.

Adopt the reviewed disposition as a task/declared-profile correction under doc52's existing AA policy delegation. Existing public API/wire fields/digest identities and frozen architecture stay unchanged. If a later implementation proves a frozen amendment is required, stop and return to the original-preserving CCR amendment/import route; this decision cannot waive it.

On approval, Root may prepare a fresh HUMAN-hosted narrow T009 r1 A2 repair, followed by fresh exact-SHA source A3. Write set remains the registered mc-webauthn-verify source/tests; no dependency, schema, authority-matrix, resource-cap, UI or real-device changes. No current source is accepted. Old r0 source/tests/reviews, partial successes and qualifications stay immutable. T009/T011/activation stay held until the remaining normal gates pass. No push/PR/merge, VM/provider action or role launch is included.

## Affected-lane input for owner consideration

W01 coordinator assessment (Root/A1): existing raw attestation_object can carry the self signature and existing p256 code can verify it. The source/test profile changes are inside T009's registered write scope; no new crypto/service or public-type shape is needed. T011 and enrollment consumers must receive the prospective semantic notice, and r1 must get independent validation. This is implementation planning, not source approval.

W05 plan-based assessment (Root acting coordinating A1; not a separate peer approval or compatibility result): live T008 requires independent TypeScript get/create/mutation fixtures and forbids reading Rust verifier source; those fixtures must use the adopted profile from its contract notice, not reverse-engineer Rust output. T019 is the virtual browser harness. Physical/browser supported cells belong to T032 and are not proven here. No material runtime data or tested-cell support was found/provided for this decision. Existing real-device gates remain; no hardware purchase or UX change authorized now.

Independent A3 input: supports restricted profile as the smallest one-create correction, preserves unsupported-cell and trust limits; profile correction sufficient without frozen change. No dissent on mechanism. Reviewer explicitly rejects universal compatibility, hardware/biometric assurance, automatic fallback and immediate activation.

## Alternatives

Keep registration blocked without adopting a correction; or request a separately reviewed none-plus-new-key-assertion protocol for broader create compatibility. That alternative needs explicit proof/audit/challenge/activation binding and additional browser interaction; it is not already implemented or adopted.

## Required decision record after the owner's response

Record the exact chat answer, time, selected scope, rationale, dissent/limits and SINGLE_OPERATOR_DECISION additively. Do not edit the reviewed CCR to pretend it was already accepted. Until then this proposal has NO effect and authorizes no repair or policy change.
