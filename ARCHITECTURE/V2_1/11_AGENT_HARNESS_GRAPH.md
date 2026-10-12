# Agent Harness and Graph

## Two products, one contract

LOCAL_USER_HARNESS and CUSTOMER_MODEL_API are distinct capability modes. They may share ModelTurnV2 but never share assumptions about authentication, entitlements, plugins, skills, hooks, cached sessions, or billing. A mode is enabled only for the exact provider/version/auth/capability cell demonstrated by P0-01.

## ModelTurnV2

The unprivileged harness emits a structured sequence containing:

- root resource-envelope ID and attempt-binding digest
- conversation/turn IDs and ordered role/content parts
- tool_use with correlation ID, schema ID/digest, and canonical arguments
- tool_result with matching correlation ID, status, bounded output digest, and truncation marker
- requested model capability, stop/cancel state, and protocol version

It cannot choose endpoint, account, credential, tenant, billing project, unrestricted headers, or arbitrary URL. The Model Access Broker validates the sequence, overwrites authority fields, meters it, and passes a canonical request to the Secret Dialer.

## Tool execution

Tools are closed, schema-pinned capabilities owned by the broker or runner. A tool call does not become a shell escape. Repository tools operate on the bound workspace and resource envelope. Tool results are untrusted content and return through the same correlation protocol.

## Agent graph

Only broker-created tasks and edges are authoritative. Each child has parent ID, depth, capability subset, and a slice or shared counter from the root envelope. A child cannot create authority or budget. The broker enforces task count, concurrency, model calls, tokens/spend, wall time, event count, and artifact bytes.

Native provider subagents that occur inside an opaque harness are not individually enforceable. V2 labels their topology UNOBSERVABLE and constrains only the outer request/envelope. They cannot satisfy evidence-independence requirements based on claimed agent count or parentage.

## Cancellation and replay

Cancel revokes new turn/tool intents for the root generation and propagates best effort to the provider. Already billed usage is settled. Turn idempotency keys prevent duplicate charge-intent issuance; provider ambiguity becomes a metering reconciliation item, not an assumed failure.

## Parity rule

“Same model name” is not parity. Tool schema, streaming, cache semantics, context handling, plugins/skills/hooks, authentication mode, cancellation, usage accounting, and output normalization are separately recorded. No unsafe proxy or guest-credential fallback exists.

## V2.1 analysis coverage and authorization

Graph Engineering remains broker-authoritative: every child consumes the root's task/depth/concurrency/call/spend/time/event/artifact ceilings and cannot mint a new budget. A model-created child cannot alter ExpectedAnalysisManifestV1 or self-declare mandatory units complete. Agent-authored tests are causality candidates and require independent oracle trust before blocking. All lease-bound new model/tool authority atomically checks actual PostgreSQL expiry through the broker. FAST deterministic review has no model-graph prerequisite; customer-owned LOCAL_USER_HARNESS and CUSTOMER_MODEL_API cells remain separate and optional.

## Structured tool content

ModelMessageV1 resolves bounded custody containing canonical ModelContentV1 and ordered ModelContentBlockV1 records. TEXT, TOOL_USE and TOOL_RESULT have closed conditional fields for name/call ID/arguments/content/error/truncation; malformed or mismatched fields reject. Root envelope and one-use model intent still apply per provider turn. Raw content is not audit payload.
