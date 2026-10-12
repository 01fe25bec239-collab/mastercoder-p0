# SCOPE — WORKSTREAM-04-AGENT-MODEL-GRAPH (Agent, Model & Graph)

**Mission.** Qualify optional customer-owned model cells and build the in-house Graph Engineering engine (no external plugin) with its resource boundary: ModelTurnV2 normalization, the Model Access Broker with one-use ModelIntentV2 and lease checks, the content-blind Secret Dialer, Codex and Claude adapter cells (U-002), entitlement/terms truthfulness, the graph engine core, agent runtime and replayable graph export, the root ResourceEnvelopeV2 with broker-authoritative graph edges, and the append-only ledger with late settlement and restart safety, plus dependency acquisition moved from W03: the credential-free npm/Git/toolchain resolver, the SSRF-safe credential-bearing registry dialer and the DependencyAcquisitionGrantV1 authority (claims P0-05E/F/G).

## Owns (primary, exclusive)
- mc-model-turn (ModelTurnV2 family)
- mc-model-broker (I-11) + mc-secret-dialer
- mc-adapter-openai-compat (the one planned cell: cheapest tool-calling model, e.g. DeepSeek; Claude/Codex cells are not planned — adding one = new tasks)
- In-house Graph Engineering engine: mc-graph (core + store), mc-graph-runtime (agent loop), mc-graph-replay (export/fixtures), mc-graph-envelope (root counters)
- mc-ledger (budget_writer)
- Fake model provider relay (I-15)
- Model compatibility rows + terms confirmation request
- Dependency acquisition (moved from W03): mc-dependency-resolver (credential-free), mc-acquisition-dialer (credential-bearing, SSRF-safe), mc-dependency-authority (dependency_writer, grants + tenant cache), toolchain manifest fixtures, P0-05E/F/G harnesses

## Does NOT own (read-only or other workstream)
- Lease authority and journal (W01)
- Sandbox guest isolation (W03)
- Graph UI (W05)
- Any FAST deterministic path (must not depend on you)

## Primary P0 claims (10)
P0-01A, P0-01B, P0-01C, P0-01D, P0-05E, P0-05F, P0-05G, P0-08B, P0-08C, P0-08D — details in `P0_CLAIMS.md`.

## In scope now
Disposable prototypes, harnesses, fixtures, doubles, DRY_RUNs, sealed run-manifests, claim execution after G2+G3, evidence, CCRs, docs needed for freeze.

## Out of scope now
Production implementation (POST_FREEZE), other lanes' paths, any change to `ARCHITECTURE/`, real customer data or production credentials.
