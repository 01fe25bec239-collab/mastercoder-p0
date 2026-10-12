# memory.md — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

Durable workstream memory. Facts only; no secrets; no chat logs. Only the A1 writes this file; A2/A3 facts arrive as MEMORY_UPDATES in RETURN/REVIEW files. Keep < ~400 lines.

## Environment
- ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1 · REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
- ARCH_IMPORT_COMMIT: __RECORD_AT_G0__
- G0 recorded: NO · G1 signed: NO (file SHA-256: —) · G2: NOT_PASSED · G3 families: none
- Tool versions: __RECORD__

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
P0-03A, P0-03B, P0-03C, P0-03D, P0-03E, P0-04A, P0-04B, P0-04C, P0-07A, P0-07C, P0-07E, P0-09E: all NOT_RUN.

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
- G0/G1 not recorded.

## Pending contract change requests
- (none)
