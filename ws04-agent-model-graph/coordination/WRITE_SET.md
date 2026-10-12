# WRITE SET — WORKSTREAM-04-AGENT-MODEL-GRAPH

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS04 — `ws04-agent-model-graph/`

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `crates/mc-dependency-resolver/` (credential-free; moved from W03) | W04 | W03, W06 | IMPL, TEST | DependencyPackageV1 |
| `crates/mc-acquisition-dialer/` (credential-bearing; `policy` module also used by W03's checkout broker; moved from W03) | W04 | W03, W06 | IMPL, TEST | DependencyAcquisitionGrantV1, AcquisitionRequestV1 |
| `crates/mc-dependency-authority/` (dependency_writer; moved from W03) | W04 | W03, W06 | IMPL, TEST | DependencyAcquisitionGrantV1, ResourceEnvelopeV2 (read), I-03 |
| `db/dependency_grants/**`, `db/dependency_cache/**` (moved from W03 `db/runner/**`) | W04 | W06 | IMPL(SQL) | W01 role framework |
| `fixtures/toolchains/**` (moved from W03) | W04 | ALL | TEST | — |
| `claims/{P0-05E,P0-05F,P0-05G}/` (moved from W03) | W04 | W06, evidence A3s | IMPL, TEST, DOC | claim JSON |
| `Cargo.toml`, `Cargo.lock`, `compose.yaml`, `.env.example`, every crate's `Cargo.toml` and `src/lib.rs` | W04 (scaffold task W04-T000 only) | — | IMPL(config) | — |
| `crates/mc-model-test-support/` (dev-only) | W04 | — | TEST | — |
| `doubles/model-broker/**` (I-11 double) | W04 | W03, W06 | IMPL, TEST (versioned) | I-11 |
| `crates/mc-model-turn/` | W04 | W03 (guest harness side), W06 | IMPL, TEST | ModelTurnV2, ModelMessageV1, ModelContentV1, ModelContentBlockV1, ModelToolV1 |
| `crates/mc-model-broker/` | W04 | W06 | IMPL, TEST | ModelIntentV2, ResourceEnvelopeV2, EffectIntentV2 (MODEL_REQUEST) via I-04, lease via I-03 |
| `crates/mc-secret-dialer/` | W04 | W06 | IMPL, TEST | ModelTurnV2 canonical request only |
| `crates/mc-adapter-openai-compat/`, `crates/mc-adapter-claude/`, `crates/mc-adapter-codex/` | W04 | — | IMPL, TEST | ModelTurnV2 |
| `crates/mc-graph/`, `db/graph/**`, `docs/GRAPH_ENGINE.md` (in-house Graph Engineering engine core) | W04 | W06 | IMPL, TEST | ResourceEnvelopeV2 (read) |
| `crates/mc-graph-runtime/`, `profiles/**` (agent loop + agent profiles) | W04 | W03 (runs inside the sandbox), W06 | IMPL, TEST | ModelTurnV2, ModelToolV1, ModelMessageV1, I-11 |
| `crates/mc-graph-envelope/` (root counters) | W04 | W06 | IMPL, TEST | ResourceEnvelopeV2 |
| `crates/mc-graph-replay/`, `fixtures/graphs/**` (export format v1 + fixtures) | W04 | W05 (graph view), W06 | IMPL, TEST | ResourceEnvelopeV2 |
| `crates/mc-ledger/`, `db/model/**`, `db/MODULES` (schema `mc_model`, budget_writer) | W04 | W06 | IMPL, TEST | ResourceEnvelopeV2, ModelIntentV2 |
| `doubles/fake-model-provider/**` | W04 | W03, W06 | IMPL, TEST | — |
| `compat/**` (provider/auth/version/terms rows) | W04 | ALL | DOC | 47 |
| `claims/{P0-01A..D,P0-08B,P0-08C,P0-08D}/` | W04 | W06, evidence A3s | IMPL, TEST, DOC | claim JSON |
| `coordination/**` | W04 | master | COORD | — |



## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws04-agent-model-graph/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws04-agent-model-graph/`
- `.github/workflows/ws04-*.yml` only
