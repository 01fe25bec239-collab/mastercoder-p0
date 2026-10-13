# Mastercoder

Mastercoder is being built as a local-first code review and bounded fix-publication system. It is designed to help teams analyze a pinned repository revision, inspect grounded findings, and open a narrowly scoped fix pull request only after a person reviews and approves the exact change. The architecture keeps source custody and the runner under customer control by default; any external model use goes through a configured, bounded broker.

This repository contains the V2.1 architecture, an early Rust workspace scaffold, and shared CI integrity controls. It does not yet contain a runnable end-to-end Mastercoder application.

## How Mastercoder is designed to work

1. **Pin the work.** A customer selects a repository and exact commit/tree. A customer-controlled runner obtains the source and runs it in a declared sandbox.
2. **Analyze the change.** A deterministic fast review is designed to work without a model. Optional AI, hybrid, or deeper analysis uses bounded model access. Model output is treated as a claim; blocking test evidence requires an independently trusted oracle and applicable causal reproduction.
3. **Review exact bytes.** The review experience presents the pinned source, findings, and proposed patch. A separate Approval Authority displays the exact candidate and records a fresh, scoped WebAuthn approval.
4. **Publish one bounded change at a time.** A credential-free planner asks a separate Git Publication Gateway to perform only the approved Git operations. The gateway checks current authority and exact object lineage before creating the fix branch and pull request. The designed operation set excludes merging the change.

These are architecture goals, not features available in the current prototype.

## Architecture and stack

The design keeps PostgreSQL as the sole business-state authority. Durable jobs use PostgreSQL transactions, an outbox, leases, and fencing. A separate synchronous journal preserves knowledge of external effects across database recovery.

| Area | Designed technology and role |
|---|---|
| Control plane | Rust with Axum, Tokio, SQLx, and Serde; a modular monolith with separately credentialed service roles |
| Durable state | PostgreSQL for policy, work, budgets, leases, audit metadata, and transactional workflow state |
| Web review | Next.js, React, and TypeScript for the human review and approval experience |
| Execution | A customer-controlled Rust runner and declared sandbox capabilities |
| Contracts | Deterministic CBOR for signed or bound records; JSON for human-facing APIs |
| Provider access | Separate, least-privilege GitHub review and publication identities, with isolated brokers |

Redis, Kafka, NATS, RabbitMQ, and a general workflow engine are not required by the design. Read the [architecture overview](ARCHITECTURE/V2_1/01_EXECUTIVE_ARCHITECTURE.md), [mandatory invariants](ARCHITECTURE/V2_1/00D_V2_1_INVARIANTS.md), [technology stack](ARCHITECTURE/V2_1/00F_TECHNOLOGY_STACK.md), [approval and publication contract](ARCHITECTURE/V2_1/14_PATCH_APPROVAL_PUBLICATION.md), and [Git publication capability](ARCHITECTURE/V2_1/53_GIT_PUBLICATION_CAPABILITY.md). The [architecture guide](ARCHITECTURE/V2_1/00_READ_ME_FIRST.md) explains precedence: 00D invariants, schemas, contracts 49–57, then documents 01–48.

## Security boundaries in the design

- Repository source runs in a declared sandbox and never receives reusable trusted credentials.
- PostgreSQL is the business authority. Separate service roles and provider identities limit who can approve or publish.
- Approval binds to the exact displayed patch and candidate tree. A new effect requires a new authority check.
- The designed publication gateway uses a closed set of Git operations and excludes merges, protected-ref updates, and arbitrary HTTP requests. Provider permissions remain the outer limit; a compromised gateway or stolen token retains provider-granted authority.
- Unknown, stale, or incomplete evidence fails closed. Revocation and provider-check delays are described explicitly rather than treated as instantaneous.

These are design requirements. The current integrated code has not demonstrated them end to end.

## Repository map

- [ARCHITECTURE/V2_1](ARCHITECTURE/V2_1/00_READ_ME_FIRST.md) — the authoritative architecture package, contracts, schemas, and synthetic vectors.
- [ws01-core-security](ws01-core-security/README.md) — the first integrated Rust workspace scaffold, with crate shells and test scaffolding.
- [ws02-git-analysis-review](ws02-git-analysis-review/README.md), [ws03-runner-sandbox-acquisition](ws03-runner-sandbox-acquisition/README.md), [ws04-agent-model-graph](ws04-agent-model-graph/README.md), and [ws05-web-approval-ux](ws05-web-approval-ux/README.md) — planned ownership areas and their current workspace notes.
- [ws06-platform-oss-qualification](ws06-platform-oss-qualification/README.md) — shared CI, supply-chain, and qualification work. The separate [W06 CI qualification repository](https://github.com/01fe25bec239-collab/mastercoder-p0-w06-ci-qualification) holds evidence for the root CI controls.

## Developer setup

The root [Rust toolchain pin](rust-toolchain.toml) selects Rust 1.97.1 with rustfmt and Clippy. The integrated Rust workspace is under `ws01-core-security/`; run its checks from that directory:

```sh
cd ws01-core-security
cargo fmt --all --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace
```

The `ws01-ci` workflow provisions an isolated PostgreSQL service for its full checks. For local database-backed runs, follow the setup in [the W01 workspace README](ws01-core-security/README.md); local Compose uses untracked lane configuration. The repository also checks architecture vectors from the root:

```sh
python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py
```

That checker validates specification fixtures; it does not run the product, prove Rust/TypeScript parity, or execute a P0 claim. There is no root Cargo workspace or UI package yet.

## Current implementation status

**Current stage: PROTOTYPE — DISPOSABLE — NOT PRODUCTION.** This describes the present prototype state; it does not define the intended product.

As of 2026-10-13, W01's integrated contribution is a Rust workspace scaffold: its own README states that it implements no security behavior or SQL tables. W06's integrated root work provides architecture-integrity and write-set CI controls; its canonical change and main checks are recorded in [W06 PR #1](https://github.com/01fe25bec239-collab/mastercoder-p0/pull/1) and the [main activation run](https://github.com/01fe25bec239-collab/mastercoder-p0/actions/runs/38328484236). W01's scaffold and main CI are recorded in [W01 PR #2](https://github.com/01fe25bec239-collab/mastercoder-p0/pull/2) and the [W01 main run](https://github.com/01fe25bec239-collab/mastercoder-p0/actions/runs/38351662370).

The other product workspaces are not yet integrated, and the W02–W06 lane CI workflows remain bootstrap placeholders. Green scaffold and repository-guard checks do not establish runtime behavior. All 55 atomic P0 claims remain NOT_RUN; production implementation is not authorized, and the production architecture freeze has not been granted.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the task and review process. The [license](LICENSE) applies to this repository.
