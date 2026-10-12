# SCOPE — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION (Platform, OSS & Qualification)

**Mission.** Own the shared platform and cross-component qualification: repo root and CI, architecture tamper check, pinned test DB, supply-chain and secret scanning, evidence kit and generated claim ledger, canary scanner and observability, the Rust↔TS parity harness, admin-domain profiles, artifact store lifecycle, transfer/failover/restore/load/upgrade/logs drills, OSS governance drafts, and the freeze dossier.

## Owns (primary, exclusive)
- Repo root files and common CI (arch-check, supply-chain)
- testdb (I-14)
- evidence-kit + evidence store + generated ledger (I-13)
- canary-scanner + observability
- parity-harness (P0-06A/B)
- drills: admin-domains, deletion, logs-audit, load, upgrade, failover, restore, transfer
- artifact-store
- oss/ governance drafts
- freeze-dossier generator
- ARCHITECTURE/ import stewardship (import-only)
- Host-lab inventory and capture scripts (W06-T033, moved from W03)
- Shared local test services: Git smart-HTTP server, npm/Python registries, static toolchain server (W06-T034, moved from W03)

## Does NOT own (read-only or other workstream)
- Any component under test (owned by W01–W05)
- Codec implementations (W01 Rust, W05 TS)
- The G4 decision (master)

## Primary P0 claims (10)
P0-06A, P0-06B, P0-07D, P0-10B, P0-10C, P0-10D, P0-10E, P0-10F, P0-10G, P0-10H — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
