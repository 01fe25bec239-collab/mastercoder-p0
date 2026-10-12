# Compatibility Matrix

## Protocol roles

| Boundary | N ↔ N-1 requirement | Critical behavior |
|---|---|---|
| Web ↔ Rust API | One prior minor during rolling deploy | Unknown critical response/action disabled |
| API ↔ Runner | One prior minor after session negotiation | Version pinned in AttemptBinding; no mid-attempt change |
| Runner ↔ Checkout Broker | One prior minor | Grant vocabulary/capabilities exact |
| Runner ↔ Model Broker | One prior minor | ModelTurn schema/tool correlation exact |
| Control ↔ Review Broker | One prior minor | MergeSubject/check conclusion critical |
| Control ↔ Approval Authority | Exact supported proof major; one minor if vectors pass | Tuple fields, nonce, origin/RP/UV critical |
| Control ↔ Publication Broker | Exact supported effect major; one minor if vectors pass | Target/effects/tree/generation critical |

Unknown type, field, enum or revision rejects. Authenticate exact supported (type, wire_revision, registry_digest) sets, intersect with policy minimum and pin the chosen set in the session/capability digest. V2.1 has no min_reader_minor or critical-field bitmap. Prior minor support exists only when both exact schema sets are explicitly implemented and independently vector-qualified; never infer compatibility or weaken semantics during an attempt.

## Capability rows

| Capability | State |
|---|---|
| GitHub check/merge modes | DISABLED_PENDING_P0-03 per merge-mode/ruleset row |
| GitHub publication | DISABLED_PENDING_P0-03_AND_P0-09 |
| Codex API | DISABLED_PENDING_P0-01 |
| Codex subscription/native harness | DISABLED_PENDING_P0-01 |
| Claude API/gateway | DISABLED_PENDING_P0-01 |
| Claude subscription/native harness | DISABLED_PENDING_P0-01_AND_TERMS_CONFIRMATION |
| Linux S2 | DISABLED_PENDING_P0-02 |
| macOS S2/native | DISABLED_PENDING_P0-02 |
| Windows S2/native | DISABLED_PENDING_P0-02 |
| LFS/submodules | DISABLED_PENDING_P0-05 per feature |

## Upgrade order

Expand database and readers; deploy brokers/consumers that understand both; deploy writers; update runners/web; observe; then contract after the compatibility window. Key/proof/effect protocol majors require an explicit migration ceremony, not opportunistic negotiation.

## Evidence expiry

Provider/model rows expire on documented semantic/permission/auth changes or at the review interval selected after P0. Sandbox rows expire on kernel/hypervisor/backend or runner changes. Canonical rows change only through a new protocol version and golden vector set.

## V2.1 additional capability rows

| Capability | State and limitation |
|---|---|
| FAST deterministic review | Independent of every P0-01 model cell; still gated by its runner/Git/evidence/dependency/core claims |
| Individual/same-domain C3 | UNSUPPORTED_BY_DEPLOYMENT; C0-C2 only |
| Independent-domain C3 | DISABLED_PENDING_P0-07D and current admin-domain attestation |
| Public/private dependency ecosystem cells | DISABLED_PENDING_P0-05E/F; exact registry/auth/adapter/version |
| Git/toolchain dependency cells | DISABLED_PENDING_P0-05G |
| Dependency lifecycle/offline extraction | DISABLED_PENDING_P0-05H |
| Complete analysis coverage | DISABLED_PENDING_P0-07A and P0-06 vectors |
| Trusted oracle promotion | DISABLED_PENDING_P0-07C |

Family gates mean the exact applicable atomic claims in 57, never a single umbrella experiment. No current row is enabled by this architecture-only package. Unknown wire fields follow the closed canonical schema and version policy in 49/SCHEMAS; inherited critical-field language does not authorize accepting unknown fields in a signed record.
