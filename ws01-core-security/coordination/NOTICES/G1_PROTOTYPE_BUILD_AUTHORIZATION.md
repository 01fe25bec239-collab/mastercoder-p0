# G1 — Prototype Build Authorization

STATUS: **SIGNED_BY_OWNER_IN_CHAT** (G1 build scope only; G0 remains independently required)

Architecture: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1 · REGISTRY_SHA256 b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787

## What this authorizes (when signed)

1. Construction of **disposable** prototype code, harnesses, fixtures, test doubles and CI in the private repository `mastercoder-p0`. Every file is labeled `PROTOTYPE — DISPOSABLE — NOT PRODUCTION`.
2. Disposable staging accounts and resources only: a GitHub staging organization with synthetic repositories, customer-owned model API keys with hard spend caps, local/cloud VMs, a local MinIO/S3 staging bucket, and test authenticators.
3. Synthetic source code and synthetic identities only.
4. Dry runs of claim harnesses against test doubles (labeled DRY_RUN, never evidence).

## What this does NOT authorize

- Moving any claim NOT_RUN → RUNNING. That requires G2 (external audit PASS) **and** a signed G3 for the claim family.
- Production implementation, customer data, production repositories, production keys/tokens, production deployments.
- Weakening any invariant in 00D, or changing the frozen stack.

## Budget caps (fill before signing)

Maximum money master may spend on paid resources during the freeze-closure block. Agents must stop and ask when a cap is near. Your AI coding-tool subscriptions (Claude Code, Codex, Cursor) are **not** included; they are personal.

| Item | Why it may cost money | Cap | Owner |
|---|---|---|---|
| Model provider API (one cheap tool-calling model, e.g. DeepSeek; more providers optional later) | Only for W04's experiments where **Mastercoder's own Model Broker** calls a model programmatically (claims P0-01*, P0-08B–D). These test plumbing, not intelligence, so the cheapest tool-calling model is enough. Coding-tool subscriptions can't be used for this: it is the product calling the model. Everything else uses the free fake model provider. | 1,000,000 tokens / 5 USD | master |
| Cloud machines (optional) | Only if your machine can't run the Linux KVM sandbox cell (WSL2 nested virtualization) or for W06's two-separate-admin-domain / restore drills; student cloud credits may cover it | 0 USD | master |
| Physical security keys (optional) | Only if you want to test roaming FIDO2 keys in addition to Windows Hello / Touch ID | 0 USD | master |
| GitHub | Pro via GitHub Education | 0 | master |

Owner record (2026-10-12): master supplied “about $5 cap” and then supplied the typed signature “Omkar” in direct response to the signature request stating these caps. The agent transcribes that owner-supplied signature; it does not generate a signature on the owner’s behalf. Total paid-resource ceiling: **5 USD**, allocated to model APIs with a **1,000,000-token** ceiling; cloud and physical-key caps: **0 USD**. No paid resources have been used.

## Signature

Signed by: Omkar (master, program owner; owner-supplied typed chat signature)  Recorded at (UTC): 2026-10-12T14:19:54Z
Recorded in each workstream `memory.md` by its A1 as `G1_SIGNED_AT: <date>; G1_FILE_SHA256: <sha256 of this file>`.
