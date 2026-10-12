# WRITE SET — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION

Extracted from `COMMON/WRITE_OWNERSHIP_MATRIX.md` (authoritative). Each task's EXACT WRITE SCOPE narrows this further.

## WS06 — `ws06-platform-oss-qualification/` (+ root files above)

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `local-services/**` (shared local Git smart-HTTP server, npm/Python registries, static toolchain server; moved from W03) | W06 | W03, W04 | IMPL(config), TEST | — |
| `docs/host-lab.md`, `lab/**` (host-lab inventory + capture scripts; moved from W03) | W06 | W03 | DOC, TEST | — |
| `evidence-kit/**` (run-manifest sealer, evidence manifest, review record, ledger generator) | W06 | ALL (run) | IMPL, TEST | AtomicPrototypeClaimV1 |
| `parity-harness/**` | W06 | W01, W05 | IMPL, TEST | wire/signature vectors; I-01/I-02/I-10 |
| `canary-scanner/**` | W06 | ALL (run) | IMPL, TEST | — |
| `testdb/**` (pinned PostgreSQL images, compose fragments) | W06 | ALL (include) | IMPL(config) | — |
| `observability/**` | W06 | ALL | IMPL(config) | AuditEnvelopeV1 (read) |
| `artifact-store/**` | W06 | W01 | IMPL, TEST | ArtifactCustodyEntryV1 (read) |
| `drills/{transfer,failover,restore,load,upgrade,logs-audit,admin-domains,deletion}/**` | W06 | ALL | IMPL, TEST | via I-03/I-04/I-05/I-07 doubles then real components |
| `supply-chain/**` | W06 | ALL | IMPL(config) | 36 |
| `oss/**` (public-repo governance drafts) | W06 | ALL | DOC | — |
| `freeze-dossier/**` | W06 | ALL | DOC, GEN | 00E, 42 |
| `claims/{P0-06A,P0-06B,P0-07D,P0-10B,P0-10C,P0-10D,P0-10E,P0-10F,P0-10G,P0-10H}/` | W06 | evidence A3s | IMPL, TEST, DOC | claim JSON |
| `coordination/**` | W06 | master | COORD | — |

Coordination files `wsNN-*/coordination/**` change **only** on branch `ws0N/coordination` (COORDINATION worktree); task branches never touch them (enforced by `.githooks/pre-commit`).

Each workstream additionally owns its own agent auto-load files inside its directory: `wsNN-*/AGENTS.md`, `wsNN-*/CLAUDE.md`, `wsNN-*/.claude/**`, `wsNN-*/.cursor/**` (installed by bootstrap from `PERSON_*/AGENT_TOOLING/ws-root/`).


## Root and program-level paths

| PATH / MODULE | PRIMARY | SECONDARY READERS | ALLOWED CHANGE TYPE | CONTRACTS CONSUMED |
|---|---|---|---|---|
| `/README.md`, `/LICENSE`, `/NOTICE`, `/.gitignore`, `/.gitattributes`, `/.editorconfig` | W06 | all | DOC, IMPL(config) | — |
| `/rust-toolchain.toml`, `/deny.toml`, `/.cargo/config.toml` | W06 | all | IMPL(config); toolchain bump needs all six A1s notified 48h ahead | 00F stack |
| `/.github/CODEOWNERS`, `/.github/pull_request_template.md`, `/.github/ISSUE_TEMPLATE/**`, `/.github/dependabot.yml` | W06 | all | IMPL(config) | — |
| `/.github/workflows/common-*.yml` | W06 | all | IMPL(CI) | — |
| `/.github/workflows/ws0N-*.yml` | WS0N (each its own) | W06 | IMPL(CI) within W06 CI policy | — |
| `/scripts/**` | W06 | all | IMPL | — |
| `/.githooks/**` (guardrail hooks: write set, coordination-branch rule, secrets, no push to main) | W06 | all | IMPL; changes announced to all A1s | — |
| `/AGENTS.md`, `/CLAUDE.md`, `/.cursor/rules/mastercoder-repo.mdc` (repo-wide agent entry points) | W06 | all | DOC (pointer only; rules live in each `wsNN-*/coordination/AGENTS.md`) | — |
| `/ARCHITECTURE/V2_1/**` | W06 as **steward** (IMPORT only) | all | IMPORT of a Program-Contract-Board-adjudicated release; no content edits ever | whole package |
| `/evidence-ledger/**` | W06 | all | GEN (from `wsNN-*/claims/*/cells/*/current.json` + the runs they name) | AtomicPrototypeClaimV1 |
| Program folder `COMMON/**` (this planning tree) | master (program owner) | all | DOC; changes announced to all lane A1s | — |
| Program folder `LANE_0N_*/**` | that lane (operator: master) | all lanes (read) | COORD, DOC | — |
| Program folder `POST_FREEZE/**` | master | all | DOC | — |


## Always forbidden
- `ARCHITECTURE/**` (import-only by W06 after a Program Contract Board release)
- Any `wsNN-*/` other than `ws06-platform-oss-qualification/`
- Root files (`/README.md`, `/.github/CODEOWNERS`, `/rust-toolchain.toml`, `/deny.toml`, `common-*` workflows) unless you are W06
- Generated files by hand (`mc-contracts` generated code, ledger, dossier)

## Allowed outside `ws06-platform-oss-qualification/`
- `.github/workflows/ws06-*.yml` only (plus the root paths listed above as W06)
