# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# Workspace modules and dependency ownership

Only the scaffold edits `Cargo.toml` / `Cargo.lock`. A task that needs a crate its skeleton does not list returns BLOCKED with
`DEPENDENCY_REQUEST: <crate> <exact version> <why>`; the standing `W04-DEPENDENCY-UPDATE` task then edits manifests, lockfile and this file.
A stale lockfile is `W04-LOCKFILE-SYNC`. Sources: the authoritative `coordination/task.md` W04-T000 steps 1-9 and every W04 BUILD task's
"Exact public API" and READ-ONLY DEPENDENCIES. Task references use the W04 prefix (T005 = W04-T005). No API or security logic lives
in the scaffold: every declared module is an empty file and every `main.rs` is `fn main() {}`.

## Member ownership (25 members: 13 crates + 2 doubles + 10 harnesses)

| Member path | Package | Owning task(s) | Declared modules |
|---|---|---|---|
| crates/mc-acquisition-dialer | mc-acquisition-dialer | T002, T027, T028 | api, policy, connect, redirect, stream |
| crates/mc-adapter-openai-compat | mc-adapter-openai-compat | T010, T024 | map, stream, usage |
| crates/mc-dependency-authority | mc-dependency-authority | T002, T029 | api, mint, consume, cache |
| crates/mc-dependency-resolver | mc-dependency-resolver | T002, T026, T030 | api, npm, git_dep, toolchain |
| crates/mc-graph | mc-graph | T002, T007 | api, model, store, lifecycle, cancel, events |
| crates/mc-graph-envelope | mc-graph-envelope | T002, T016 | api, counters, admit |
| crates/mc-graph-replay | mc-graph-replay | T012 | export, replay |
| crates/mc-graph-runtime | mc-graph-runtime | T002, T014 | api, run_loop, tools, profiles, spawn |
| crates/mc-ledger | mc-ledger | T002, T008, T013 | api, entries, reserve, settle, cancel, debt |
| crates/mc-model-broker | mc-model-broker | T002, T011, T015, T016, T017 | api, intent, authority, effect, http, graph, recovery |
| crates/mc-model-test-support | mc-model-test-support | T003 | testdb, fixtures |
| crates/mc-model-turn | mc-model-turn | T002, T005 | api, validate, correlate, stream, custody |
| crates/mc-secret-dialer | mc-secret-dialer | T002, T009 | api, cell, dial, guard (+ main.rs: binary entry) |
| doubles/fake-model-provider/v1 | fake-model-provider | T004 | NONE (library; T004 adds `src/main.rs` for the binary, auto-discovered) |
| doubles/model-broker/v1 | model-broker-double | T006 | NONE (library) |
| claims/P0-01A/harness | p0-01a-harness | T019 | NONE (`src/main.rs` binary) |
| claims/P0-01B/harness | p0-01b-harness | T020 | NONE (`src/main.rs` binary) |
| claims/P0-01C/harness | p0-01c-harness | T021 | NONE (`src/main.rs` binary) |
| claims/P0-01D/harness | p0-01d-harness | T025 | NONE (`src/main.rs` binary) |
| claims/P0-08B/harness | p0-08b-harness | T022 | NONE (`src/main.rs` binary) |
| claims/P0-08C/harness | p0-08c-harness | T018 | NONE (`src/main.rs` binary) |
| claims/P0-08D/harness | p0-08d-harness | T023 | NONE (`src/main.rs` binary) |
| claims/P0-05E/harness | p0-05e-harness | T031 | NONE (`src/main.rs` binary) |
| claims/P0-05F/harness | p0-05f-harness | T032 | NONE (`src/main.rs` binary) |
| claims/P0-05G/harness | p0-05g-harness | T033 | NONE (`src/main.rs` binary) |

`mc-model-broker` also gets a `src/main.rs` binary from T015; Cargo auto-discovers it, so no manifest change is needed.
Harness `Cargo.toml` files are scaffold-owned (each claim task writes `claims/<claim>/**` except `harness/Cargo.toml`).

## Crate -> imports

Normal imports are declared package dependencies, including libraries needed by the listed future modules. Dev imports are
test-only. All entries are `workspace = true` pins; sibling crates and W01/W03 doubles resolve to path dependencies.

| Package | Normal imports | Development imports |
|---|---|---|
| mc-acquisition-dialer | async-trait, bytes, futures, hex, hickory-resolver, ipnet, mc-contracts, mc-effects, reqwest, rustls, serde_json, sha2, thiserror, tokio, tracing | effects-journal-double, flate2, proptest, tar, tempfile |
| mc-adapter-openai-compat | bytes, eventsource-stream, futures, mc-contracts, mc-model-turn, serde, serde_json, thiserror | fake-model-provider, proptest, tokio |
| mc-dependency-authority | async-trait, hex, mc-contracts, mc-graph-envelope, mc-lease, serde_json, sha2, sqlx, thiserror, tokio, tracing, uuid | lease-double, mc-model-test-support, proptest |
| mc-dependency-resolver | ed25519-dalek, mc-contracts, serde_json | proptest, tempfile |
| mc-graph | async-trait, mc-contracts, serde_json, sqlx, thiserror, tokio, uuid | mc-model-test-support, proptest |
| mc-graph-envelope | async-trait, mc-contracts, mc-lease, sqlx, thiserror, tokio, uuid | lease-double, mc-model-test-support, proptest |
| mc-graph-replay | hex, mc-contracts, mc-graph, serde, serde_json, sha2, sqlx, thiserror, tokio, uuid | mc-model-test-support, proptest, tempfile |
| mc-graph-runtime | async-trait, futures, mc-contracts, mc-graph, mc-model-turn, serde, serde_json, sha2, thiserror, tokio, toml, tracing, uuid | fake-model-provider, mc-model-test-support, model-broker-double, proptest, tempfile |
| mc-ledger | async-trait, mc-contracts, mc-model-turn, sqlx, thiserror, tokio, uuid | mc-model-test-support, proptest |
| mc-model-broker | async-trait, axum, bytes, futures, mc-adapter-openai-compat, mc-contracts, mc-effects, mc-graph, mc-graph-envelope, mc-lease, mc-ledger, mc-model-turn, mc-secret-dialer, mc-wire, serde, serde_json, sqlx, thiserror, tokio, tracing, uuid | effects-journal-double, fake-model-provider, lease-double, mc-model-test-support, proptest, tempfile |
| mc-model-test-support | sqlx, thiserror, tokio, uuid | proptest |
| mc-model-turn | bytes, futures, mc-contracts, mc-secret-dialer, mc-wire, serde, serde_json, thiserror | proptest, tokio |
| mc-secret-dialer | async-trait, bytes, futures, thiserror, reqwest (feature `server`), rustls (feature `server`), tokio (feature `server`), tracing (feature `server`) | fake-model-provider, proptest, tokio |
| fake-model-provider | axum, futures, serde, serde_json, tokio | NONE |
| model-broker-double | axum, futures, mc-contracts, mc-model-turn, serde, serde_json, tokio, uuid | NONE |
| p0-01a-harness | clap, mc-adapter-openai-compat, mc-contracts, mc-graph-runtime, mc-model-broker, mc-model-turn, serde_json, tokio | fake-model-provider, mc-model-test-support |
| p0-01b-harness | clap, mc-contracts, mc-model-broker, mc-secret-dialer, serde_json, tokio | fake-model-provider, fake-sandbox, mc-model-test-support |
| p0-01c-harness | clap, mc-contracts, mc-ledger, mc-model-broker, serde_json, tokio | fake-model-provider, lease-double, mc-model-test-support |
| p0-01d-harness | clap, mc-adapter-openai-compat, mc-contracts, mc-graph-runtime, mc-model-turn, serde_json, tokio | fake-model-provider |
| p0-08b-harness | clap, mc-contracts, mc-graph, mc-graph-envelope, mc-model-broker, serde_json, sqlx, tokio, uuid | mc-model-test-support |
| p0-08c-harness | clap, mc-contracts, mc-ledger, mc-model-broker, serde_json, sqlx, tokio, uuid | fake-model-provider, mc-model-test-support |
| p0-08d-harness | clap, mc-contracts, mc-ledger, mc-model-broker, serde_json, sqlx, tokio, uuid | effects-journal-double, fake-model-provider, mc-model-test-support |
| p0-05e-harness | clap, mc-acquisition-dialer, mc-contracts, mc-dependency-authority, mc-dependency-resolver, mc-graph-envelope, serde_json, sqlx, tokio, uuid | lease-double, mc-model-test-support |
| p0-05f-harness | clap, mc-acquisition-dialer, mc-contracts, serde_json, tokio | NONE |
| p0-05g-harness | clap, mc-checkout-broker, mc-contracts, mc-dependency-resolver, serde_json, tokio | NONE |

## Dependency rule and the edges behind it

- Normative edges (audit F-6): `mc-model-broker` -> mc-lease, mc-effects, mc-graph-envelope, mc-secret-dialer (client), mc-model-turn (+ dev
  lease-double and effects-journal-double); `mc-graph-envelope` -> sqlx, tokio, mc-lease; `mc-model-test-support` -> sqlx, tokio;
  `mc-dependency-authority` -> mc-lease, mc-graph-envelope (+ dev lease-double); `mc-acquisition-dialer` -> mc-effects (+ dev
  effects-journal-double); `mc-dependency-resolver` -> mc-contracts, ed25519-dalek, serde_json only (normal edges).
- Edges revealed by the stub compile check (the T002 signatures pasted into a scratch copy, `cargo check --workspace --all-targets --locked`):
  `mc-model-turn` -> `mc-secret-dialer` (`ModelAdapter::from_provider_stream` takes `BoxStream<Result<Bytes, DialError>>`), plus the
  `bytes`, `futures`, `async-trait`, `sqlx`, `uuid`, `serde_json`, `toml` and `mc-contracts` imports the signatures use.
  Removing the `mc-model-turn` -> `mc-secret-dialer` edge fails the check (E0433), so it is required.
- Edges taken from READ-ONLY DEPENDENCIES of later tasks (not in the F-6 list): `mc-model-broker` -> mc-ledger (T017 recovery reads T013),
  mc-graph (T016 `graph.rs` mints children), mc-adapter-openai-compat (T015 wires the adapter); `mc-ledger` -> mc-model-turn (`UsageUnits`);
  `mc-graph-runtime` -> mc-model-turn, mc-graph (T014). The adapter never depends on the broker, so there is no cycle.
- Harness doubles (lease, effects-journal, fake-sandbox, fake-model-provider) are dev-dependencies: doubles are never evidence (P0 runs use
  the real components). `p0-05g-harness` path-depends on ws03 `mc-checkout-broker`, which brings its own HTTP client (reqwest); that harness
  is not a model-provider path and holds no provider credential.

## Provider reachability (task SECURITY INVARIANTS)

Only `mc-secret-dialer` may reach a model provider. Its `reqwest` and `rustls` sit behind the `server` feature, which is **off by default** and is
enabled only to build the dialer binary (`cargo build -p mc-secret-dialer --features server`, which `required-features` also demands). The workspace
dependency entry is declared with `default-features = false`, no member enables `server`, and the `dial` module (the only one that opens
connections) is compiled only with it. So `cargo build/test --workspace` compile the client-side API only, and a broker source file that
referenced `mc_secret_dialer::dial` would fail to compile. `clippy --workspace --all-features` does turn `server` on for every member; it is a lint
pass, not a shipped build. Expected `cargo tree -e normal` results (checked for all 25 members in `.github/workflows/ws04-ci.yml`):

| Package | reqwest | hyper | rustls |
|---|---|---|---|
| mc-dependency-resolver | absent | absent | absent |
| mc-model-broker | absent | present (axum, server side only) | present (sqlx `tls-rustls`, PostgreSQL TLS) |
| mc-graph-runtime | absent | absent | present (sqlx `tls-rustls`, PostgreSQL TLS) |
| mc-secret-dialer with `--features server` | present | present | present |

HTTPS-capable by design and allowlisted in CI: `mc-secret-dialer` (`server`), `mc-acquisition-dialer` (registry downloads; it reads registry canary
tokens only, never a provider key) and the harnesses that depend on an HTTPS-capable crate: `p0-05e-harness` and `p0-05f-harness` (via
`mc-acquisition-dialer`) and `p0-05g-harness` (via ws03 `mc-checkout-broker`). Every other member must have none of reqwest, hyper-rustls,
hyper-tls, native-tls, openssl, ureq, isahc, attohttpc, surf or curl in its normal tree. `rustls` reaching the broker through sqlx is the
PostgreSQL TLS client, not a provider client.

## Pinned versions

`=x.y.z` pins in `[workspace.dependencies]`. Crates that `ws01-core-security/Cargo.toml` also pins use W01's exact version (path dependencies on W01
crates fail to resolve if two workspaces pin different exact versions); `flate2` and `tar` copy W03's. Fresh pins are the latest stable
on the scaffold day (2026-11-05, `cargo info`). `reqwest` uses W01's version with this lane's features (`rustls`, `json`, `stream`, no defaults).

| Crate | Pin | Source |
|---|---|---|
| tokio | 1.53.2 | W01 (same exact version) |
| axum | 0.8.9 | W01 (same exact version) |
| sqlx | 0.9.0 | W01 (same exact version) |
| serde | 1.0.229 | W01 (same exact version) |
| serde_json | 1.0.154 | W01 (same exact version) |
| uuid | 1.28.0 | W01 (same exact version) |
| sha2 | 0.11.0 | W01 (same exact version) |
| hex | 0.4.3 | W01 (same exact version) |
| ed25519-dalek | 3.0.0 | W01 (same exact version) |
| reqwest | 0.13.5 | W01 (same exact version) |
| rustls | 0.23.45 | W01 (same exact version) |
| hickory-resolver | 0.26.3 | fresh |
| ipnet | 2.12.2 | fresh |
| flate2 | 1.1.10 | W03 |
| tar | 0.4.46 | W03 |
| eventsource-stream | 0.2.3 | fresh |
| futures | 0.3.34 | fresh |
| bytes | 1.12.1 | fresh |
| thiserror | 2.0.21 | W01 (same exact version) |
| tracing | 0.1.44 | W01 (same exact version) |
| async-trait | 0.1.92 | W01 (same exact version) |
| clap | 4.6.7 | W01 (same exact version) |
| toml | 1.2.0 | fresh |
| tempfile | 3.27.0 | W01 (same exact version) |
| proptest | 1.12.0 | W01 (same exact version) |

Path dependencies: W01 `mc-contracts`, `mc-wire`, `mc-lease`, `mc-effects` and doubles `lease/v1`, `effects-journal/v1` (dev); W03 double
`fake-sandbox/v1` (dev, used by the P0-01B harness) and `mc-checkout-broker` (P0-05G harness).
