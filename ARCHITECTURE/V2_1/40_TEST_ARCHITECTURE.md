# Test Architecture

## Layers

1. Canonical contract tests: independent Rust/TypeScript golden vectors, rejection vectors, property/fuzz tests, N/N-1 skew.
2. Database tests: constraints, RLS cross-tenant denial, state compare-and-set, job fencing, ledger conservation, inbox/outbox replay, migration/restore.
3. Broker contract tests: positive vocabulary, audience/identity/generation, endpoint/permission denial, idempotency and ambiguity.
4. Runner/sandbox tests: capability match, resource limits, egress, reset, process trees, escape/exfiltration, cross-OS behavior.
5. Evidence tests: malicious runner/key, causal proposed tests, independence, stale subject, unavailable capability.
6. Provider staging tests: Git check/merge queue/candidate, model tools/cancel/usage, checkout object shapes.
7. Approval/publication tests: WebAuthn origin/challenge/UV/replay, exact tuple, tree equality, revocation, provider ambiguity.
8. Operational tests: load, chaos, backup/restore, regional fencing, audit/seal, telemetry leakage, incident rotation.

## Determinism

Fixtures identify architecture, schema, protocol, object algorithm, policy, tool/provider version, OS, sandbox tier, and expected digest. Time and randomness are injected. Provider staging results are retained as evidence with expiry.

## Adversarial cases

- Caller-selected check conclusion, stale evaluation overwrite, neutral/skipped incomplete work.
- API approval forgery, replayed authenticator assertion, changed destination/effects.
- Returned-tree mismatch and source/protected-ref mutation request.
- Guest credential exfiltration and arbitrary URL/header model proxy.
- Agent-proposed malicious test and stolen runner key.
- Lease expiry with late billing, duplicate settlement, split-brain epoch.
- Repository transfer and RLS/backup/artifact cross-tenant access.
- Unknown critical protocol field and downgrade attempt.

## Release evidence

Each gate has reproducible commands/configuration, environment identity, raw results, signed digest, reviewer, date, and expiry. A passing unit test does not replace provider or hostile-environment proof.

## V2.1 atomic acceptance

57_ATOMIC_PROTOTYPE_CLAIMS.md and TRACEABILITY/atomic-prototype-claims.json replace umbrella PASS/FAIL with independently reviewable claims. All are NOT_RUN. Before execution each needs an immutable exact version/configuration/environment manifest and reviewer assignment; this package does not claim experimental evidence. A missing zero-findings receipt, invalid agent oracle, stale provider success, lease expiry before reaper, lost intent during restore, dependency credential leak and direct sandbox database access are explicit negative cases. A family PASS is derived only from every applicable atomic claim's current PASS; NOT_RUN/INCONCLUSIVE/expired evidence disables the corresponding cell.
