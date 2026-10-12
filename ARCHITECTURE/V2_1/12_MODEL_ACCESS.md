# Model Access

## Boundary

Model access is split into:

1. Credential-free harness/parser in the runner or host.
2. Model Access Broker that validates ModelTurnV2, policy, resource intent, correlation, and usage.
3. Secret Dialer with provider endpoint and credential, limited to canonical provider requests and response byte forwarding.

The Secret Dialer never interprets repository content or tool semantics. The parser never receives the provider secret. An unrestricted HTTP forward proxy is forbidden.

## Authentication modes

API-key, cloud-provider, and subscription-backed modes are different products. Each provider/mode/version has a compatibility row in 47_COMPATIBILITY_MATRIX.md. No row is production-enabled until P0-01 confirms:

- credentials remain outside hostile execution
- real multi-turn tool use and tool results are expressible
- cancellation and timeout behavior are observable
- usage and late billing can be reconciled
- advertised entitlement, skills/plugins/hooks, and context behavior are truthful
- provider terms permit the integration

Official Codex documentation describes programmatic thread/turn event control and external sandbox/tool integration through the app server, which makes a safe adapter plausible but not proven. Claude documentation supports external gateways and recommends keeping proxies outside the agent boundary; subscription OAuth use by a third-party service requires explicit terms/provider confirmation.

## Spend enforcement

Every provider request requires a one-use ModelIntentV2 bound to tenant, attempt, turn, model cell, maximum tokens/spend, deadline, and broker generation. The broker atomically consumes remaining root-envelope allowance before dialing. Actual provider usage settles later even when the attempt is canceled or fenced.

## Data handling

Prompts and responses are ephemeral by default and are not canonical audit payloads. Retention is an explicit tenant policy, encrypted per tenant, size bounded, and disclosed. Secret and source filters run at ingress and egress but are defense in depth, not permission to transmit data outside the bound policy.

## Failure

Provider timeout, invalid structure, unknown usage, entitlement drift, or cancellation ambiguity yields PARTIALLY_AVAILABLE or BLOCKED according to required policy. It never silently selects another provider, credential type, model, or local harness.

## Official references

- [Codex app server](https://learn.chatgpt.com/docs/app-server)
- [Codex authentication](https://learn.chatgpt.com/docs/auth)
- [Codex model-provider configuration](https://learn.chatgpt.com/docs/config-file/config-reference)
- [Claude authentication](https://code.claude.com/docs/en/authentication)
- [Claude LLM gateway](https://code.claude.com/docs/en/llm-gateway)
- [Claude secure deployment](https://code.claude.com/docs/en/agent-sdk/secure-deployment)
- [Claude legal and compliance](https://code.claude.com/docs/en/legal-and-compliance)

## V2.1 lease and independent FAST boundary

Before consuming ModelIntentV2, the broker atomically checks owner, lease generation, authority epoch, permitted state and lease_expires_at > database_current_time together with the root allowance and one-use nonce. A reaper is not the expiry oracle. Per-request external accounting follows EffectIntentV2 and 50; actual late billing settles without reviving result authority. Deterministic FAST contains no required model turn and must run with model services unavailable. The existing linked provider documentation is inherited context, not new confirmation of a supported mode; every actual provider/version/auth cell still needs atomic P0-01 evidence.
