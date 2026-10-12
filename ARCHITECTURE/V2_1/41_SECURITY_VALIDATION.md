# Security Validation

## Validation domains

| Domain | Required evidence |
|---|---|
| Authority separation | Attempt cross-calls and credential/key use among API, Review, Approval, Gate, Publisher, Checkout, Model roles |
| Hostile execution | OS escape, host secret discovery, egress bypass, persistence/reset, process-tree termination, resource exhaustion |
| Git truth | Stale head/base, merge queue, normal/squash/rebase candidates, check conclusion/configuration, provider revocation |
| Model brokerage | Prompt/header/URL injection, tool correlation, credential theft, cancellation, billing ambiguity, subscription/API parity |
| Evidence | Stolen runner key, collusion/failure-domain assumptions, proposed-test causality, artifact substitution |
| Approval/publication | WebAuthn phishing/origin/RP/challenge/replay, retargeting, tree mismatch, expiry, revocation, ambiguous effects |
| Tenancy/privacy | RLS, composite FK, repository transfer, backup/restore, artifact grants/deletion/legal hold, log leakage |
| Distributed correctness | Lease/fence races, duplicate/reordered delivery, split brain, late results/spend, N/N-1 skew |
| Supply chain | Build provenance, signed release/tool images, dependency confusion, compromised bundle impact |

## Method

Threat-led testing uses disposable tenants, provider/model staging accounts, hostile repositories, and instrumented brokers. Each test states the claim, adversary authority, expected fail-closed behavior, observables, and cleanup. Security controls are tested at their actual boundary; process or diagram inspection is insufficient.

## Independence

The implementing A2 manager cannot be the only accepting reviewer. A4 Security and A4 Contract Consistency independently review raw evidence. Provider-dependent claims retain official documentation plus staging proof. Unsupported or inconclusive tests remain blockers.

## Red-team stop conditions

Any ability to mint approval without WebAuthn UV, publish a different tree/target, turn incomplete work into permitting check status, expose reusable credentials to hostile code, accept stale/fenced evidence, cross tenant boundaries, downgrade a critical protocol/capability, or lose actual spend from the ledger is release blocking.

## Requalification

Revalidate on provider permission/semantics, model/harness/auth, kernel/hypervisor, runner, canonical schema, broker permission, key policy, or critical dependency changes. Compatibility rows carry evidence dates and expiry.

## V2.1 additional release blockers

Release-blocking counterexamples include PASS with any mandatory manifest unit absent; an agent-authored arbitrary expectation promoted through repetition alone; same-domain runs mislabeled C3; registry credentials or scripts entering the acquisition broker's hostile parsing boundary; dependency SSRF/cache poisoning; compromised API insertion of qualified receipts; runner/sandbox database access; a stale internal evaluation with untracked provider SUCCESS; and a provider send without prior durable effect intent. P0-07A–D, P0-05E–H, P0-03C–E, P0-09A–H and P0-10H–I isolate these claims. Independent reviewers inspect both positive and injected-failure traces; architecture review is separate from prototype PASS.
