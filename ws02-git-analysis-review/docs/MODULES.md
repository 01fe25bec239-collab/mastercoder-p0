# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# MODULES — crate → owning task, imports and pinned versions

Scaffold owner: W02-T000-WORKSPACE-SCAFFOLD. Every later W02 task edits only its own module files, tests and SQL (see the write scope in `coordination/task.md`); it never edits `Cargo.toml`, `Cargo.lock`, `lib.rs`, `compose.yaml` or CI.

## Crate → modules → owning task

| Crate | Module | Owning task |
|---|---|---|
| mc-git-test-support | `testdb` | W02-T005 |
| mc-git-test-support | `fixtures` | W02-T005 |
| mc-github | `api` | W02-T004 |
| mc-github | `auth` | W02-T008 |
| mc-github | `client` | W02-T008 |
| mc-github | `webhook` | W02-T008 |
| mc-github | `readback` | W02-T008 |
| mc-provider-normalizer | `api` | W02-T004 |
| mc-provider-normalizer | `ingest` | W02-T014 |
| mc-provider-normalizer | `subject` | W02-T014 |
| mc-provider-normalizer | `candidate` | W02-T014 |
| mc-review-broker | `api` | W02-T004 |
| mc-review-broker | `slot` | W02-T021 |
| mc-review-broker | `state` | W02-T021 |
| mc-review-broker | `association` | W02-T021 |
| mc-review-broker | `dispatch` | W02-T030 |
| mc-review-broker | `discovery` | W02-T030 |
| mc-review-broker | `freeze` | W02-T033 |
| mc-eligibility | `api` | W02-T004 |
| mc-eligibility | `identity` | W02-T018 |
| mc-eligibility | `proof` | W02-T018 |
| mc-eligibility | `verify` | W02-T018 |
| mc-eligibility | `transfer` | W02-T018 |
| mc-analysis-planner | `api` | W02-T004 |
| mc-analysis-planner | `policy` | W02-T012 |
| mc-analysis-planner | `plan` | W02-T012 |
| mc-analysis-planner | `units` | W02-T012 |
| mc-analysis-planner | `seal` | W02-T016 |
| mc-analyzers | `api` | W02-T004 |
| mc-analyzers | `registry` | W02-T011 |
| mc-analyzers | `distribution` | W02-T011 |
| mc-analyzers | `adapters` | W02-T015 |
| mc-analyzers | `cache` | W02-T015 |
| mc-findings | `api` | W02-T004 |
| mc-findings | `normalize` | W02-T010 |
| mc-findings | `fingerprint` | W02-T010 |
| mc-findings | `novelty` | W02-T010 |
| mc-findings | `dedupe` | W02-T010 |
| mc-gate-reducer | `api` | W02-T004 |
| mc-gate-reducer | `reduce` | W02-T009 |
| mc-gate-reducer | `precedence` | W02-T009 |
| mc-verification | `api` | W02-T004 |
| mc-verification | `oracle` | W02-T013 |
| mc-verification | `receipts` | W02-T017 |
| mc-verification | `set` | W02-T017 |
| mc-verification | `reproduction` | W02-T017 |
| mc-checkout-grant | `api` | W02-T004 |
| mc-checkout-grant | `issue` | W02-T029 |
| mc-checkout-grant | `terminate` | W02-T029 |
| mc-fast-smoke | `smoke` | W02-T034 |

Doubles and harnesses:

| Path | Package | Owning task |
|---|---|---|
| `doubles/fake-github/v1` | fake-github | W02-T002 (+T007) |
| `doubles/reducer-table/v1` | reducer-table | W02-T003 |
| `doubles/checkout-grant/v1` | checkout-grant | W02-T006 |
| `claims/P0-03A/harness` | w02-p0-03a-harness | W02-T022 |
| `claims/P0-03B/harness` | w02-p0-03b-harness | W02-T020 |
| `claims/P0-03C/harness` | w02-p0-03c-harness | W02-T031 |
| `claims/P0-03D/harness` | w02-p0-03d-harness | W02-T035 |
| `claims/P0-03E/harness` | w02-p0-03e-harness | W02-T032 |
| `claims/P0-04A/harness` | w02-p0-04a-harness | W02-T023 |
| `claims/P0-04B/harness` | w02-p0-04b-harness | W02-T024 |
| `claims/P0-04C/harness` | w02-p0-04c-harness | W02-T025 |
| `claims/P0-07A/harness` | w02-p0-07a-harness | W02-T026 |
| `claims/P0-07C/harness` | w02-p0-07c-harness | W02-T027 |
| `claims/P0-07E/harness` | w02-p0-07e-harness | W02-T028 |
| `claims/P0-09E/harness` | w02-p0-09e-harness | W02-T019 |

T004 owns `src/api.rs` in the ten crates that have one (all except mc-git-test-support and mc-fast-smoke). mc-analyzers `adapters/` submodules are created by W02-T015 itself.

## Crate → imports table (from each W02 task's Exact public API and READ-ONLY DEPENDENCIES)

| Crate | Imports | Dependencies | Dev-dependencies |
|---|---|---|---|
| mc-git-test-support | mc_contracts (fixtures); sqlx `PgPool` in its API; W01 role SQL `ws01-core-security/db/authority/roles/*.sql` (applied by tests, not a crate) | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio | proptest |
| mc-github | mc_contracts: GitOidV1, TreeEntryV1 | mc-contracts, thiserror, serde, serde_json, uuid, reqwest, jsonwebtoken, hmac, sha2, tokio, async-trait | proptest, mc-git-test-support, fake-github |
| mc-provider-normalizer | mc_contracts: ProviderEventV2, MergeSubjectV2, CurrentCandidateV1; mc_github::api: ProviderError, RepoRef; mc_wire | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire, mc-github | proptest, mc-git-test-support, fake-github |
| mc-review-broker | mc_contracts: GateProofV2, RevocationRecordV1; mc_github::api: ProviderError; mc_effects (effects journal); mc_wire | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire, mc-github, mc-effects | proptest, mc-git-test-support, effects-journal-double, gate-authority-double, fake-github |
| mc-eligibility | mc_contracts: EligibilityProofV2, MergeSubjectV2, PolicyVersionV1; mc_github (membership reads); mc_wire | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire, mc-github | proptest, mc-git-test-support, fake-github |
| mc-analysis-planner | mc_contracts: AnalysisPlanV1, AttemptBindingV2, ExpectedAnalysisManifestV1, MergeSubjectV2, PolicyVersionV1; mc_lease (`LeaseAuthority` for `PgSealer<L>`); mc_wire | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire, mc-lease | proptest, mc-git-test-support, lease-double |
| mc-analyzers | mc_contracts: AnalysisUnitV1, FindingCandidateV2; mc_sandbox_core::unit: UnitExecutionSpec, UnitExecutionResult (I-17, never copied into W02); mc_findings; mc_wire (distribution signature verification) | mc-contracts, thiserror, serde, serde_json, uuid, mc-sandbox-core, mc-findings, mc-wire | proptest, mc-git-test-support, fake-sandbox |
| mc-findings | mc_contracts: FindingCandidateV2 | mc-contracts, thiserror, serde, serde_json, uuid, sha2 | proptest, mc-git-test-support |
| mc-gate-reducer | mc_contracts: AnalysisCompletionReceiptV1, AnalysisUnitV1, EvaluationIdentityV2, ExpectedAnalysisManifestV1, FindingCandidateV2, OracleTrustClassV1, PolicyVersionV1, VerificationReceiptV2, VerificationSetV2, GateProofV2Conclusion (data only, no IO) | mc-contracts, thiserror, serde, serde_json, uuid | proptest |
| mc-verification | mc_contracts: ManualResolutionV1, OracleTrustClassV1, SignedEnvelopeV1, VerificationReceiptV2, VerificationSetV2; mc_wire; W01 lease/gate doubles (dev) | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire | proptest, mc-git-test-support, lease-double, gate-authority-double |
| mc-checkout-grant | mc_contracts: AttemptBindingV2, CheckoutGrantV2, MergeSubjectV2; mc_lease (`LeaseAuthority` for `PgGrantIssuer<L>`); mc_wire | mc-contracts, thiserror, serde, serde_json, uuid, sqlx, tokio, async-trait, mc-wire, mc-lease | proptest, mc-git-test-support, lease-double |
| mc-fast-smoke | all other W02 crates (except mc-git-test-support, a dev-dependency); W01/W03 doubles and all three W02 doubles (dev); no mc-model-* / mc-graph-* crate | mc-contracts, thiserror, serde, serde_json, uuid, mc-github, mc-provider-normalizer, mc-review-broker, mc-eligibility, mc-analysis-planner, mc-analyzers, mc-findings, mc-gate-reducer, mc-verification, mc-checkout-grant | proptest, mc-git-test-support, lease-double, effects-journal-double, gate-authority-double, fake-sandbox, fake-github, reducer-table, checkout-grant |

Doubles:

| Double | Dependencies |
|---|---|
| fake-github | axum, tokio, serde_json |
| reducer-table | serde_json, mc-contracts, mc-gate-reducer |
| checkout-grant | mc-contracts, mc-checkout-grant, ed25519-dalek, async-trait, tokio |

Harnesses (each also gets clap, serde_json, tokio, mc-contracts):

| Claim | Crates / doubles named by its task |
|---|---|
| P0-03A (W02-T022) | mc-github, mc-provider-normalizer, fake-github, mc-git-test-support |
| P0-03B (W02-T020) | mc-github, fake-github |
| P0-03C (W02-T031) | mc-github, mc-review-broker, fake-github, mc-git-test-support, lease-double, effects-journal-double, gate-authority-double |
| P0-03D (W02-T035) | mc-github, mc-review-broker, fake-github, mc-git-test-support |
| P0-03E (W02-T032) | mc-github, mc-review-broker, fake-github, mc-git-test-support, lease-double, effects-journal-double, gate-authority-double |
| P0-04A (W02-T023) | mc-github, mc-eligibility, fake-github, mc-git-test-support |
| P0-04B (W02-T024) | mc-github, mc-eligibility, fake-github, mc-git-test-support |
| P0-04C (W02-T025) | mc-github, mc-eligibility, mc-provider-normalizer, fake-github, mc-git-test-support |
| P0-07A (W02-T026) | mc-gate-reducer, mc-analysis-planner, lease-double, mc-git-test-support |
| P0-07C (W02-T027) | mc-verification, fake-sandbox, mc-git-test-support |
| P0-07E (W02-T028) | mc-verification, mc-git-test-support |
| P0-09E (W02-T019) | mc-github, fake-github |

## Dependency rule (AGENTS.md §3a)

Only the scaffold edits `Cargo.toml`/`Cargo.lock` files and `lib.rs`. A task that needs a crate or dependency not listed above returns `DEPENDENCY_REQUEST` to A1 instead of editing a Cargo file. Purity rules enforced by construction: `mc-gate-reducer` depends only on mc-contracts, thiserror, serde, serde_json and uuid (no tokio/sqlx/reqwest, normal or dev); no `mc-model-*`/`mc-graph-*` crate appears anywhere in ws02. Scaffold stub-check finding: the T004 signatures compile against these dependency lists unchanged; `UnitExecutionSpec`/`UnitExecutionResult` did not yet exist in `mc_sandbox_core::unit` on the scaffold base (W03-T003 provides them before T004 starts).

## Pinned versions (`[workspace.dependencies]`, exact `=x.y.z`)

Rule: a crate that `ws01-core-security/Cargo.toml` also pins copies W01's exact version and feature set (path dependencies fail to resolve if two workspaces pin different exact versions); only crates W01 does not use are chosen fresh. Pins were chosen on 2026-11-05.

| Crate | Version | Source |
|---|---|---|
| tokio | =1.53.2 | copied from W01 |
| axum | =0.8.9 | copied from W01 |
| sqlx | =0.9.0 | copied from W01 (postgres, runtime-tokio, tls-rustls, uuid, macros, migrate) |
| serde | =1.0.229 | copied from W01 |
| serde_json | =1.0.154 | copied from W01 |
| uuid | =1.28.0 | copied from W01 |
| sha2 | =0.11.0 | copied from W01 |
| hex | =0.4.3 | copied from W01 |
| ed25519-dalek | =3.0.0 | copied from W01 |
| reqwest | =0.13.5 | copied from W01 (default-features off; `rustls`, `json`; reqwest 0.13 names the feature `rustls`, not `rustls-tls`) |
| thiserror | =2.0.21 | copied from W01 |
| tracing | =0.1.44 | copied from W01 |
| async-trait | =0.1.92 | copied from W01 |
| clap | =4.6.7 | copied from W01 |
| tempfile | =3.27.0 | copied from W01 |
| proptest | =1.12.0 | copied from W01 |
| jsonwebtoken | =11.1.0 | fresh (latest per `cargo info`, 2026-11-05); feature `rust_crypto` (pure Rust backend, no C toolchain) + default `use_pem` |
| hmac | =0.13.0 | fresh (latest per `cargo info`, 2026-11-05); matches sha2 0.11 / digest 0.11 |

Path dependencies: W01 `mc-contracts`, `mc-wire`, `mc-lease`, `mc-effects`, `lease-double`, `effects-journal-double`, `gate-authority-double`; W03 `mc-sandbox-core`, `fake-sandbox`.
