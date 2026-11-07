# CCR-W01-002 — owner adjudication

PROTOTYPE — DISPOSABLE — NOT PRODUCTION.

DECISION_KIND: SINGLE_OPERATOR_DECISION
OWNER: HUMAN:master (program owner)
RECORDED_AT_UTC: 2026-11-07T07:59:45.390116+00:00
EXACT_OWNER_RESPONSE: Approve restricted profile (Recommended)
DECISION: ACCEPTED — RESTRICTED PROTOTYPE PROFILE/TASK CORRECTION ONLY
EFFECTIVE: PROSPECTIVE PROFILE AND NARROW REPAIR PREPARATION
SOURCE_ACCEPTANCE: NONE
DEVICE_ENABLEMENT: NONE

Owner explicitly answered the adoption question after independent review. Exact owner proposal SHA256a9760334184f4a773b9ad191ef909fdbe17e53e6485c17352b46af69a5fa762e; CCR SHA256f55eee3e4906b73cbbd9a8cbf3cc6c4c97da0d08af3d11ac65b18e99ee154701; independent W04 review SHA256a382999a6a8ca1932413cbfdd608f69f7cb108f2057200492423de109922fa3f. Full files are preserved at the paths in the owner proposal.

Adopt restricted ES256/P-256 packed SELF registration: alg=-7/sig, no x5c. The new credential key verifies the DER signature over exact create authenticatorData || SHA256(exact received clientDataJSON). Keep type/challenge/origin/RP/UP/UV, algorithm/key/ID/parser checks and caller authorization/freshness/one-use requirements. Unsigned none, certificate-backed and unsupported formats/cells cannot yield proof-bearing success. No fallback or extra ceremony.

Accepted trust/compatibility limit: possession only, not manufacturer provenance, trusted hardware, biometric assurance or universal passkey support. No physical/browser cell enabled; all remain unverified/disabled until qualified. Existing invalid BS-without-BE rejection stays; no broader backup/UV admission policy is invented. W05 plan-based fixture/harness/matrix impact and W01 scope are recorded in the proposal, not claimed as separate peer/device approval.

Adopt the reviewed task/declared-profile correction under doc52's AA policy delegation. No frozen field, wire, digest or public API change. If a real frozen amendment becomes necessary, stop for the original-preserving CCR release/import process. Declare policy in live T009 task/dispatch, the adopted notice and prospective module docs. W05 T008 remains independent of Rust source; T019/T032 qualification gates remain.

Root may prepare a fresh HUMAN-hosted narrow A2 repair followed by fresh exact-SHA source A3. This is not source acceptance, consumer unlock, UI/hardware change, new dependency/resource budget, role launch, push/PR/merge or VM/provider authority. Original CCR/review/proposal/r0 source/evidence remain unchanged; this sidecar supersedes pending adoption status prospectively. No dissent on mechanism; all reviewed limitations retained. A1 does not implement.
