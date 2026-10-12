# Owner subagent policy — all A1/A2/A3 sessions

Normally use **2–3 independent subagents**, maximum 3 concurrently per main-agent session, when the task has parallel work. A trivial indivisible task may use fewer; never invent work just to fill slots. These helpers are additional to the six lane A2 caps, not additional A2 dispatches.

| Main-agent family | Normal helpers | Complex/security-critical helpers |
|---|---|---|
| OpenAI | GPT-6 Luna, xhigh effort (`gpt-6-luna`) | GPT-6.1 Sol, high effort (`gpt-6.1-sol`) |
| Claude | Haiku 5.5, max effort (requested model `haiku-5.5`) | Sonnet 5.5, high effort (requested model `sonnet-5.5`) |

These are the owner's requested model preferences, not a claim that an account/tool offers them. Before spawning, resolve the configured model/effort against the host's actual model list. Never silently use a different version or downgrade effort. If unavailable, state the unavailable preference and get the owner's alternative; continue independent work that does not require that helper. Model IDs/aliases vary by host; record the resolved exact ID and effort in the return/review.

Hand off and replace a Haiku helper **before it reaches 100,000 context tokens**; do not continue the same helper after that threshold. Apply the same ceiling to other helpers as a default. The outgoing helper writes a short handoff: goal, completed work, remaining work, authorized paths, observed test results and unresolved doubts, with no secrets. Launch a fresh helper from that handoff. If the host cannot expose token/context usage, proactively rotate at conservative bounded milestones and disclose that exact token usage is not observable; do not claim a measured 100k limit. Rotation is not automatic unless the host supports it.

Give helpers disjoint paths and bounded objectives; no shared-file races. They never widen scope, commit/push, mutate providers, edit coordination records, approve themselves or modify frozen contracts. The main agent checks every result and owns its return/verdict. A3 uses fresh helpers that did not participate in A2 implementation. Independent Rust/TypeScript codec separation still applies.

Source defaults live in `_build/gen.py`; every lane's .env.example and local model preference fields carry the same values. Local credentials must never be packaged or logged. Lane instructions and tool entry points repeat this policy so it is visible in Claude, Codex and Cursor.
