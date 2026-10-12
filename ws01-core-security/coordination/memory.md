# memory.md — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

Durable workstream memory. Facts only; no secrets; no chat logs. Only the A1 writes this file; A2/A3 facts arrive as MEMORY_UPDATES in RETURN/REVIEW files. Keep < ~400 lines.

## Environment
- ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1 · REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
- ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
- G0 recorded: YES (2026-10-12T14:48:06Z; PHASED_BUILD_STARTUP; owner-approved scoped PRESTART) · G1 signed: YES (owner typed signature; file SHA-256: 2b5253f7739ed7b4a434053af256887f8a84b2e6d0def4cc56744eec10692712) · G2: NOT_PASSED · G3 families: none
- Tool versions: git git version 2.54.0 (Apple Git-157); gh gh version 2.96.0 (2026-07-02); python3 Python 3.14.6; rustc rustc 1.97.1 (8bab26f4f 2026-07-14); cargo cargo 1.97.1 (c980f4866 2026-06-30); docker Docker version 29.7.2, build a7dcaa6; gitleaks gitleaks version 8.30.1; sccache sccache 0.18.0; node v24.18.0; pnpm 11.22.0

## Accepted architecture interpretation
- Precedence 00D > SCHEMAS > 49–57 > 01–48 > ADRs; BASELINE_HISTORY non-normative.
- Claim evidence reviewer names `A4-V2_1-*` = review lenses SEC/CC/DS run by fresh A3s from other workstreams (DEFINITION_OF_DONE §3); no A4 agent layer exists.
- FAST deterministic review has no model dependency (05, 47).
- Everything built here is PROTOTYPE — DISPOSABLE — NOT PRODUCTION; production implementation NOT_AUTHORIZED.

## Task / SHA ledger (mirror of task.md; accepted and integrated only)
| TASK_ID | ACCEPTED_SHA | MERGED_SHA | A3 review file |
|---|---|---|---|

## Interface pins
| Interface | Provider | Pinned version | Pinned SHA |
|---|---|---|---|

## Claims
P0-06C, P0-06D, P0-07B, P0-08A, P0-09B, P0-09C, P0-09D, P0-09F, P0-09G, P0-09H, P0-10A, P0-10I, P0-10J: all NOT_RUN.

## Decisions
- (2026-10-05, master) Real-provider model tests (P0-01*, P0-08B–D) use the cheapest tool-calling model via API (e.g. DeepSeek through the OpenAI-compatible adapter), token cap about 1–5M. Each tested model qualifies only its own cell; frontier models are not required.
- (2026-10-04, master) Licensing: UNDECIDED (commercial/proprietary vs open core). The repo stays private; code is 'All rights reserved, internal use' until decided and is owned equally by all six team members (COMMON/OPEN_SOURCE_STRATEGY.md §2).
- (2026-10-03, master) Mandatory handoff output: WORK LOCATION + NEXT AGENT PROMPT code blocks (AGENTS.md §0).
- (2026-10-03) Agent tools in use: Claude Code, Codex CLI, Cursor; auto-load files in AGENT_TOOLING/.
- (2026-10-07, master) Hackfest: not selected; the Hackfest plan is archived (ARCHIVE/HACKFEST_2026_NOT_SELECTED) and is not part of this program.
- (program) Claim ownership per `COMMON/P0_CLAIM_ASSIGNMENT_MATRIX.md`.

## Known pitfalls (from architecture — do not relearn the hard way)
- Acceptance-time clock is `clock_timestamp()` sampled AFTER locks; `now()`/`CURRENT_TIMESTAMP`/`statement_timestamp()` are wrong; equality is expired (ADR-022).
- Not-found never proves absence; UNKNOWN stays UNKNOWN without terminal proof (50).
- Signature = origin only, never authority (INV-06).
- u64/i64 are decimal strings in JSON; never JSON numbers; TS uses BigInt (ENCODING).
- Unknown fields/enums/revisions reject; no extension bags (INV-18).

## Failed approaches
- (none yet)

## Prototype evidence pointers
- (none yet)

## Current blockers
- No construction-startup blocker. Deferred resources and task dependencies remain mandatory before the owning tasks; see NOTICES/STARTUP_2026-10-12.md.

## Pending contract change requests
- CCR-W01-001 (DRAFT, audit F-23: P0-09G parent state STARTED → RUNNING) — triage first; P0-09G RUN blocked until resolved

## Startup verification — 2026-10-12
- Private GitHub repository: https://github.com/01fe25bec239-collab/mastercoder-p0; protected main, 8 required checks pinned to GitHub Actions app 15368, 0 approvals, merge commits only, auto-merge enabled.
- Shared clone/worktree and .env.local symlink verified. ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e.
- G1_SIGNED_AT: 2026-10-12T14:19:54Z; G1_FILE_SHA256: 2b5253f7739ed7b4a434053af256887f8a84b2e6d0def4cc56744eec10692712. Build only: 1,000,000 API tokens / 5 USD total; cloud 0 USD; security keys 0 USD.
- Architecture: 235 files, 0 mismatches; specification check only, p0_executed:false. Seven main bootstrap workflows passed; PR-only write-set-guard was tested locally, not reported as a main CI run.
- Guard smoke tests: invalid scratch scope, valid-task cross-lane path, direct-main push all rejected; no test commit or branch remains.
- GitHub secret scanning unavailable for this private repository (HTTP 422); local gitleaks hook installed. Two baseline full-history scan findings are contract type-name lists at W01 task.md INPUT CONTRACTS, not secrets; no blanket exclusion was added.
- Free disk ~41 GiB, below 100 GB checklist; shared compile-cache cap reduced to 5G, no personal files deleted. Docker reports ~8 GB memory allocation; one active A2 build maximum until 4 GB tuning is verified.
- Staging databases, object-lock evidence storage, provider Apps/keys and lab hosts are unprovisioned; tasks own their creation. Do not fabricate values or sign PRESTART to clear the blanket setup check. Master explicitly approved phased G0/scoped PRESTART; future-resource fields remain deferred prerequisites, not fake provisioned values.
- All BUILD tasks NOT_STARTED, all 55 claims NOT_RUN; G2 NOT_PASSED, no G3, no production grant.

## Active phased G0 — 2026-10-12T14:48:06Z
- Owner explicitly approved phased G0 and scoped PRESTART; G0_PHASE_RECORD_SHA256: 079c1584dca8f51d45cfd10127ff308749736ecc92961765e274f6789b7508cf. See NOTICES/G0_PHASED_BUILD_AUTHORIZATION.json and NOTICES/G0_PHASE_DECISION.md.
- Construction checker PASS: six lanes, zero errors, 39.57 GiB free; live GitHub identity/protection/signatures/freshness/CI, architecture/pins, paths/symlinks/hooks and caps verified. Negative tests PASS; fresh independent GPT-6.1 Sol high review accepted the scoped construction checks.
- Global scheduling: max3 active A2 tasks, max1 per lane, max1 active build until Docker about4GiB verified; compiler jobs2, cache5G; stop dispatch below20GiB free. Shared cap covers all six lanes together.
- Scoped PRESTART signed by Omkar (master) via explicit chat approval. Unchecked access/manual/knowledge and future-resource items retain per-task/phase prerequisites. Lane A1 completes mandatory reads on launch.
- G2 NOT_PASSED; G3 none; all claims NOT_RUN; production NOT_AUTHORIZED.

## Scaffold fallback image pin — 2026-10-12
- Read-only local image inspection: postgres:16.14-alpine3.24 RepoDigest postgres@sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777; image ID sha256:57c72fd2a128e416c7fcc499958864df5301e940bca0a56f58fddf30ffc07777. Permitted W01-T000 fallback until I-14 is published; no unrelated database/container/data reuse. /deny.toml belongs to W06-T007.
