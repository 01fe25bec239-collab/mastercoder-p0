# DEPENDENCIES — WORKSTREAM-04-AGENT-MODEL-GRAPH

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-11 (double W04-T006, real W04-T011 + W04-T015)
- I-15 fake model provider (W04-T004)
- graph fixtures for W05 (W04-T012)
- lease-bound dependency endpoint for IM-2 (W04-T029)
- endpoint policy crate for W03 LFS/submodule checks (W04-T027)
- dependency grants and staged packages for W03 offline install (W04-T029)

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-01/I-02 (W01)
- I-03 lease (W01)
- I-04 effects/journal (W01)
- I-13/I-14 (W06)
- fake-sandbox (W03)
- checkout broker for Git commit dependencies (W03-T016)
- local registries (W06-T034)
- registry/archive fixtures (W05-T012)
- canary-scanner (W06)

## Integration milestones your claim RUNs need
IM-2, IM-2, IM-3, IM-3 — definitions in the integration contract.

## Supporting components per claim
- P0-01A: W03 sandbox (guest side), W01 lease/journal
- P0-01B: W03 S2 sandbox guest, W06 canary-scanner
- P0-01C: W01 lease + journal, W03 sandbox
- P0-01D: —
- P0-05E: W01 lease, W04 root envelope ceilings (same workstream), W06 local registries
- P0-05F: W06 canary-scanner, W06 local registries
- P0-05G: W03 checkout broker (Git commit dependencies)
- P0-08B: W01 lease
- P0-08C: W01 effects
- P0-08D: W01 journal

## External / human dependencies
- Create ONE cheap API account for the first test cell (e.g. DeepSeek API, or the cheapest tool-calling model of OpenAI/Anthropic). The model must support multi-turn tool/function calling, streaming and usage reporting. Set a HARD provider-side spend limit; total test budget about 1–5M tokens. Only synthetic fixtures are ever sent. Each tested model qualifies only its own cell; other providers stay DISABLED until tested.
- Dependency acquisition (moved from W03): start the shared local Git server and registries from `ws06-platform-oss-qualification/local-services/` (W06-T034) on your machine; never let tests reach the public internet.
- Do NOT use personal subscription logins (ChatGPT/Claude subscriptions) for any cell until the terms confirmation (G-060) is answered in writing.
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
