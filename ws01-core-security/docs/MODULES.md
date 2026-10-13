# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# Workspace modules and dependency ownership

Only the scaffold edits Cargo.toml/Cargo.lock; a unit that needs a new crate returns BLOCKED with DEPENDENCY_REQUEST. W01-T052 is the explicitly authorized later cross-workspace manifest task; standing DEPENDENCY-UPDATE/LOCKFILE-SYNC dispatches require their own scope and independent review.

Sources: authoritative coordination/task.md W01-T000 steps 1–10; every W01 BUILD task's Exact public API and READ-ONLY DEPENDENCIES. Task references below use the W01 prefix. T011 owns all 14 api.rs files; the skeleton provides declarations only. No API or security logic is implemented here.

## Member ownership (41 total)

| Member path | Package | Owning task(s) |
|---|---|---|
| crates/mc-approval-authority | mc-approval-authority | T011, T025, T032 |
| crates/mc-authority-db | mc-authority-db | T007 |
| crates/mc-bootstrap | mc-bootstrap | T011, T037, T046 |
| crates/mc-cbor | mc-cbor | T005 |
| crates/mc-contracts | mc-contracts | T004 |
| crates/mc-credential-lifecycle | mc-credential-lifecycle | T011, T028, T036 |
| crates/mc-custody | mc-custody | T010, T011, T021 |
| crates/mc-effects | mc-effects | T011, T016, T030 |
| crates/mc-gate-authority | mc-gate-authority | T011, T031, T052 |
| crates/mc-git-gateway | mc-git-gateway | T011, T043, T047 |
| crates/mc-ingestion | mc-ingestion | T011, T019 |
| crates/mc-journal | mc-journal | T011, T017 |
| crates/mc-lease | mc-lease | T011, T014 |
| crates/mc-negotiation | mc-negotiation | T011, T023 |
| crates/mc-plan-verifier | mc-plan-verifier | T011, T033 |
| crates/mc-publication-planner | mc-publication-planner | T011, T042 |
| crates/mc-resolution | mc-resolution | T011, T027, T035 |
| crates/mc-runner-control | mc-runner-control | T003 protocol, T029 server |
| crates/mc-test-support | mc-test-support | T008 |
| crates/mc-webauthn-verify | mc-webauthn-verify | T009 |
| crates/mc-wire | mc-wire | T012 |
| doubles/wire-vector-table/v1 | wire-vector-table | T006 (library: `wire_vector_table`) |
| doubles/lease/v1 | lease-double | T015 |
| doubles/effects-journal/v1 | effects-journal-double | T018 |
| doubles/gate-authority/v1 | gate-authority-double | T020 |
| doubles/custody/v1 | custody-double | T022 |
| doubles/runner-control/v1 | runner-control-double | T026 |
| claims/P0-06C/harness | p0-06c-harness | W01-T024 |
| claims/P0-06D/harness | p0-06d-harness | W01-T038 |
| claims/P0-07B/harness | p0-07b-harness | W01-T040 |
| claims/P0-08A/harness | p0-08a-harness | W01-T039 |
| claims/P0-09B/harness | p0-09b-harness | W01-T045 |
| claims/P0-09C/harness | p0-09c-harness | W01-T041 |
| claims/P0-09D/harness | p0-09d-harness | W01-T048 |
| claims/P0-09F/harness | p0-09f-harness | W01-T034 |
| claims/P0-09G/harness | p0-09g-harness | W01-T049 |
| claims/P0-09H/harness | p0-09h-harness | W01-T051 |
| claims/P0-10A/harness | p0-10a-harness | W01-T013 |
| claims/P0-10I/harness | p0-10i-harness | W01-T050 |
| claims/P0-10J/harness | p0-10j-harness | W01-T044 |
| tools/codegen | codegen | T004 |

## Crate imports

Every named public peer/type has a direct import. Standard-library types need no Cargo dependency. All 21 crates have proptest as a dev dependency; all except mc-contracts, mc-cbor, mc-authority-db and mc-test-support also have mc-test-support. mc-runner-control adds rcgen for dev use. mc-test-support exposes its optional mc-authority-db dependency through `authority`.

| Crate | Direct normal imports | Binding public API / later task rationale |
|---|---|---|
| mc-approval-authority | async-trait, axum, ed25519-dalek, mc-authority-db, mc-contracts, mc-credential-lifecycle, mc-custody, mc-webauthn-verify, mc-wire, serde, serde_json, sqlx, thiserror, tokio, tracing, uuid | T011 RawAssertion/DisplayModel Serde; T025 CustodyStore, Axum routes and receipt signing; T032 CredentialRegistry |
| mc-authority-db | mc-contracts, sqlx, thiserror, tokio, uuid | T007 migrations, PgPool and verified context |
| mc-bootstrap | async-trait, mc-authority-db, mc-contracts, mc-credential-lifecycle, mc-effects, mc-journal, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 entitlement/epoch API; T037 JournalSequencer; T046 CredentialRegistry/EffectAuthority |
| mc-cbor | thiserror, unicode-normalization | T005 typed errors and NFC checks |
| mc-contracts | hex, serde, serde_json, sha2, thiserror, unicode-normalization, uuid | T004 Serde primitives/projections, UUIDs and registry SHA-256; W01-DEPENDENCY-UPDATE supplies the existing =0.1.25 Unicode pin for T004 to reject non-NFC text rather than normalize it (ENCODING.md, Exact bytes) |
| mc-credential-lifecycle | async-trait, mc-authority-db, mc-contracts, mc-webauthn-verify, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 lifecycle contract chain; T028 WebAuthn registration |
| mc-custody | async-trait, hex, mc-authority-db, mc-contracts, mc-effects, mc-wire, sha2, sqlx, thiserror, tokio, tracing, uuid | T011 contract bundles/Uuid; T010 raw object SHA-256; T021 EffectAuthority |
| mc-effects | async-trait, mc-authority-db, mc-contracts, mc-journal, mc-lease, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 LeaseRef, contract effects/certificates; T030 JournalSequencer |
| mc-gate-authority | async-trait, ed25519-dalek, mc-authority-db, mc-contracts, mc-ingestion, mc-lease, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 proofs/revocations; T031 ingestion, lease checks and gate signing |
| mc-git-gateway | async-trait, clap, mc-approval-authority, mc-authority-db, mc-contracts, mc-custody, mc-effects, mc-plan-verifier, mc-wire, reqwest, sqlx, thiserror, tokio, tracing, uuid | T011 capability/provider records; T043 reqwest::Client/Url, EffectAuthority, plans, approvals, CustodyStore and CLI; T047 reconcile |
| mc-ingestion | async-trait, ed25519-dalek, mc-authority-db, mc-contracts, mc-lease, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 signed records; T019 LeaseAuthority and signing fixtures |
| mc-journal | async-trait, clap, ed25519-dalek, mc-authority-db, mc-contracts, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 journal contract records/Uuid; T017 witness signing and CLI |
| mc-lease | async-trait, mc-authority-db, mc-contracts, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 LeaseRef/Uuid, CapabilityProfileV1, AttemptBindingV2 and re-exported PgConnection |
| mc-negotiation | mc-cbor, mc-contracts, mc-wire, sha2, thiserror | T011 pure tuple API; T023 canonical CBOR tuple-array SHA-256 |
| mc-plan-verifier | async-trait, ed25519-dalek, mc-authority-db, mc-contracts, mc-custody, mc-wire, sha1, sqlx, thiserror, tokio, tracing, uuid | T011 contract tree/plan; T033 CustodyStore, Git SHA-1 object hashing and plan signing |
| mc-publication-planner | async-trait, clap, mc-authority-db, mc-contracts, mc-plan-verifier, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 PublicationOperationV1; T042 PlanVerifier and CLI |
| mc-resolution | async-trait, axum, mc-authority-db, mc-contracts, mc-effects, mc-webauthn-verify, mc-wire, sqlx, thiserror, tokio, tracing, uuid | T011 RawAssertion/ResolutionKind/contracts; T027 Axum routes; T035 EffectAuthority |
| mc-runner-control | axum, clap, mc-contracts, mc-lease, mc-negotiation, rustls, serde, serde_json, tokio, tokio-rustls, uuid | T029 LeaseAuthority/Negotiator, Tokio mTLS with Rustls, HTTP/Serde DTOs, UUID/contract fields and CLI |
| mc-test-support | ed25519-dalek, mc-authority-db (optional authority), p256, serde_json, sha2, sqlx, tokio, uuid | T008 PgPool/clock, random test keys, JSON and raw fixture SHA-256; optional mc-authority-db |
| mc-webauthn-verify | base64, p256, serde_json, sha2, thiserror | T009 ES256, raw assertion JSON, SHA-256 and base64 |
| mc-wire | clap, ed25519-dalek, mc-cbor, mc-contracts, serde_json, sha2, thiserror, uuid | T012 contract canonicalization, Ed25519 verification, vector JSON CLI |

Extra peer dev imports: mc-ingestion and mc-gate-authority use lease-double (T019/T031); mc-approval-authority uses custody-double (T025). The runner-control double directly imports lease-double and serde (T026). Every Rust double imports tokio, serde_json, async-trait, mc-contracts and uuid alongside its named fake interfaces; these allow it to name the exact trait parameters directly. Harnesses import the W01 crates named in their task plus clap, serde_json and tokio; member-specific manifests are authoritative.

W02 reducer crate/table imports are deferred to T052. The fake GitHub and W05 test IdP are external test services, not fabricated local Cargo members. No chrono dependency is needed by the frozen API: time values are integer microseconds. Native Git tree hashing is required by T033, so the scaffold adds sha1 rather than asking the later task to change a manifest.

## Exact registry pins

Verified from the official crates.io API on the scaffold day. These are latest non-prerelease stable versions, not guessed versions. Cargo.lock pins the resolved transitive graph; all subsequent checks use --locked. Serde derive is enabled. SQLx disables default database drivers and uses only the required PostgreSQL features. Reqwest disables defaults; master explicitly authorized 0.13.5's renamed `rustls` feature in place of the dispatch's old `rustls-tls` name.

| Dependency | Exact version | Checked UTC | Registry source |
|---|---|---|---|
| async-trait | =0.1.92 | 2026-10-12T18:49:41.472652+00:00 | [official API](https://crates.io/api/v1/crates/async-trait) |
| axum | =0.8.9 | 2026-10-12T18:49:09.987251+00:00 | [official API](https://crates.io/api/v1/crates/axum) |
| base64 | =0.23.1 | 2026-10-12T18:49:23.533648+00:00 | [official API](https://crates.io/api/v1/crates/base64) |
| clap | =4.6.7 | 2026-10-12T18:49:37.921483+00:00 | [official API](https://crates.io/api/v1/crates/clap) |
| ed25519-dalek | =3.0.0 | 2026-10-12T18:49:21.014483+00:00 | [official API](https://crates.io/api/v1/crates/ed25519-dalek) |
| hex | =0.4.3 | 2026-10-12T18:49:20.203721+00:00 | [official API](https://crates.io/api/v1/crates/hex) |
| p256 | =0.14.0 | 2026-10-12T18:49:22.755443+00:00 | [official API](https://crates.io/api/v1/crates/p256) |
| proptest | =1.12.0 | 2026-10-12T18:49:39.726500+00:00 | [official API](https://crates.io/api/v1/crates/proptest) |
| rcgen | =0.14.10 | 2026-10-12T18:49:34.317108+00:00 | [official API](https://crates.io/api/v1/crates/rcgen) |
| reqwest | =0.13.5 | 2026-10-12T18:49:35.584202+00:00 | [official API](https://crates.io/api/v1/crates/reqwest) |
| rustls | =0.23.45 | 2026-10-12T18:49:31.103403+00:00 | [official API](https://crates.io/api/v1/crates/rustls) |
| serde | =1.0.229 | 2026-10-12T18:49:13.724712+00:00 | [official API](https://crates.io/api/v1/crates/serde) |
| serde_json | =1.0.154 | 2026-10-12T18:49:15.710615+00:00 | [official API](https://crates.io/api/v1/crates/serde_json) |
| sha1 | =0.11.0 | 2026-10-12T18:53:36.046496+00:00 | [official API](https://crates.io/api/v1/crates/sha1) |
| sha2 | =0.11.0 | 2026-10-12T18:49:18.924772+00:00 | [official API](https://crates.io/api/v1/crates/sha2) |
| sqlx | =0.9.0 | 2026-10-12T18:49:11.715881+00:00 | [official API](https://crates.io/api/v1/crates/sqlx) |
| tempfile | =3.27.0 | 2026-10-12T18:49:38.242951+00:00 | [official API](https://crates.io/api/v1/crates/tempfile) |
| thiserror | =2.0.21 | 2026-10-12T18:49:26.775917+00:00 | [official API](https://crates.io/api/v1/crates/thiserror) |
| tokio | =1.53.2 | 2026-10-12T18:49:08.248061+00:00 | [official API](https://crates.io/api/v1/crates/tokio) |
| tokio-rustls | =0.26.6 | 2026-10-12T18:49:32.834493+00:00 | [official API](https://crates.io/api/v1/crates/tokio-rustls) |
| tracing | =0.1.44 | 2026-10-12T18:49:28.295826+00:00 | [official API](https://crates.io/api/v1/crates/tracing) |
| tracing-subscriber | =0.3.23 | 2026-10-12T18:49:29.366270+00:00 | [official API](https://crates.io/api/v1/crates/tracing-subscriber) |
| unicode-normalization | =0.1.25 | 2026-10-12T18:49:25.033272+00:00 | [official API](https://crates.io/api/v1/crates/unicode-normalization) |
| uuid | =1.28.0 | 2026-10-12T18:49:17.436987+00:00 | [official API](https://crates.io/api/v1/crates/uuid) |

## Declared modules and migrations

All declared module files contain only a doc comment. mc-resolution's flows is a directory module (`src/flows/mod.rs`). mc-lease re-exports sqlx::PgConnection without behavior. The module list below comes verbatim from T000.

| Crate | Empty declared modules |
|---|---|
| mc-approval-authority | api, prepare, display, challenge, http_prepare, finish, consume, anomaly, http_finish |
| mc-bootstrap | api, fence, deny, bootstrap, unfreeze |
| mc-credential-lifecycle | api, chain, registry, operations, recovery, break_glass |
| mc-custody | api, manifest, store, pins, snapshot, reader, retention, service |
| mc-effects | api, store, dispatch, authority |
| mc-gate-authority | api |
| mc-git-gateway | api, gateway, reconcile |
| mc-ingestion | api |
| mc-journal | api |
| mc-lease | api |
| mc-negotiation | api |
| mc-plan-verifier | api |
| mc-publication-planner | api |
| mc-resolution | api, case, action, challenge, commit, http, flows |

Migration order: roles → base → lease → effects → journal → ingestion → gate → custody → approval → credential → resolution → publication → bootstrap. The machine-readable `db/authority/MODULES` intentionally contains only these module names, one per line. SQL numbering is local to each directory; there are no SQL files or tables.
