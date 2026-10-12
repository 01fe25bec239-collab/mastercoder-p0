# SKILLS.md — WORKSTREAM-01-CORE-SECURITY-AUTHORITY (Core Security Authority (HARD AUTHORITY))

Purpose: make any capable coding agent (including mid-level models) produce senior, high-reasoning-quality work in this workstream by following explicit, checkable procedures. Mandatory for every A1, A2, A3 and evidence-A3 session, read right after `AGENTS.md`.
Architecture: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1 · REGISTRY_SHA256 b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787.

Bundled upstream skill catalog: package ../SKILL.md or repository /SKILL.md. Select relevant Matt Pocock SKILL.md files; they supplement these mandatory procedures and never replace scope, gates or review independence.

## 0. How to use this file

- **Every agent session** (A1, A2, A3, evidence A3) reads this file right after `AGENTS.md` and before acting.
- Sections 1–12 are universal. Section 13 is your role playbook; read only your role. Section 14 is this workstream's technical pack.
- These are **procedures**, not advice. When a step says "write X", actually write it (in your scratch notes or the output file). A mid-level model reaches senior-level quality by making its reasoning explicit and checkable, not by "thinking harder".
- If this file conflicts with `AGENTS.md` or the architecture, AGENTS.md and the architecture win. Report the conflict.

## 1. The core reasoning loop (use for every non-trivial step)

Run these seven steps in order. Never skip from Understand to Act.

| Step | Do | Output you must produce |
|---|---|---|
| 1. UNDERSTAND | Restate the task in your own words. List the acceptance criteria verbatim. Name what "done" looks like. | 3–6 line restatement + criteria list |
| 2. LOCATE | Find every file, contract, schema field, invariant and existing code the task touches. Open and read them; don't guess from names. Grep for all callers/usages of anything you will change. | A list of `path:line` facts, each with a one-line summary |
| 3. PLAN | Write the smallest sequence of changes that satisfies every criterion. Map each criterion → change → test. | A numbered plan + a criteria→test table |
| 4. PREDICT | Before running anything, write what you expect (test passes/fails, output shape, error). | One expected outcome per action |
| 5. ACT SMALL | Make one coherent change at a time. Compile/test after each. | Small diffs, frequent test runs |
| 6. VERIFY | Compare actual vs predicted. Re-run the full required command set. Re-read the acceptance criteria and tick each with evidence. | Criteria table with MET/NOT_MET + evidence |
| 7. REFLECT | Ask: what did I assume? What would a hostile reviewer attack first? What did I not test? Fix or record it. | Assumptions + limitations list for the RETURN |

A prediction mismatch is information. Stop and explain it before continuing; never "try something else" without a hypothesis.

## 2. Context and memory discipline

1. **Read before you write.** Never edit a file you have not opened in this session. Re-open a file after any tool or other agent may have changed it.
2. **Facts carry sources.** Every claim you rely on gets a `path:line` or command-output citation in your notes. "I recall that…" is not a source.
3. **Work from files, not memory of the chat.** Coordination truth lives in `task.md`, `memory.md`, handoffs, returns and reviews. If the chat and a file disagree, the file wins; report the discrepancy.
4. **Keep a scratch ledger** while working: decisions, open questions, failed attempts. Its durable facts go into `memory.md` (A1 writes it directly; A2/A3 list them under MEMORY_UPDATES in their RETURN/REVIEW); the rest stays out.
5. **Context budget.** Don't paste huge files into your reasoning; read the relevant ranges. When your context is past about 60%, finish the current atomic step and hand off (A1) or return NOT_READY with progress (A2).
6. **Exact identifiers only.** Copy SHAs, type names, field names, enum values and paths with copy/paste or a command. Never retype them from memory.

## 3. Reading the architecture like a senior engineer

1. Start from **invariants** (`00D`): list the INV-IDs your task touches and what each forbids.
2. Go to the **schema** (`SCHEMAS/REGISTRY.json` → your types): field order, primitive widths, options/sets, enum arrays, and the `semantic_rules` text. Semantic rules are where the real constraints are; read them in full.
3. Read **`CONTRACT_AUTHORITY.md`** for who may produce/write each record and what every `*_digest` field references.
4. Read the **numbered doc section** for behavior (e.g. 50 for effects, 51 for coverage, 52 for approval).
5. Check **vectors** (`TEST_VECTORS`) for the exact bytes and the rejection cases your code must reproduce.
6. Build a **requirement checklist** before coding: `REQ-n | source (file §) | exact rule | how I will test it`.
7. If two sources disagree, or a rule is missing, **do not choose**. Draft a CCR and stop (AGENTS.md §10).
8. Words that mean something specific here: *STALE / REVOKED / INVALID* (currentness), *PASS vs ACCEPTED* (outcome vs evidence lifecycle), *UNKNOWN* (never "failed" or "absent"), *C0–C4 vs E0–E4 vs oracle class* (three independent dimensions).

## 4. Planning and decomposition

- Turn every acceptance criterion and every listed NEGATIVE TEST into a named test **before** writing implementation code. Write the test list first.
- Order work: types/contracts → pure logic → persistence → I/O boundary → harness. Pure functions first, because they are easiest to prove correct.
- Each step must leave the tree compiling and tests runnable.
- Estimate: if the plan has more than ~12 steps, or touches files outside the write set, stop and tell the A1 the task needs splitting (BLOCKED with a proposed split).
- Write down what you will NOT do (out of scope), so you don't drift.

## 5. Implementation craft

- **Smallest correct diff.** No speculative abstractions, no "while I'm here" refactors, no new dependency when the standard library or an already-pinned crate works.
- **Fail closed.** Unknown, missing, unsupported, ambiguous or expired input → explicit rejection or non-permitting state, never a default. No `unwrap()`/`expect()` on any authority, parsing or network path in Rust; use typed errors (`enum` + `thiserror` if already used).
- **No hardcoding (AGENTS.md §0.3).** Every environment-, person- or account-specific value comes from `.env` through one validated config module; normative constants are defined once with a citation.
- **No silent fallback.** Never fall back to another provider, credential, sandbox tier, schema revision or code path when the intended one fails.
- **Make illegal states unrepresentable** where cheap: newtypes for digests, epochs, generations; separate types for UUID epochs vs counters (ADR-005).
- **Time and randomness are inputs.** Inject the clock and RNG so tests are deterministic. The only acceptance clock is the DB `clock_timestamp()` after locks.
- **Every file** starts with `PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)`.
- **Comments explain why**, especially for security decisions; cite the architecture (`// 50 §dispatch step 4: one claim per effect`).
- Keep functions short enough to review. Prefer explicit `match` over clever chains in security code.

## 6. Test design (where mid-level agents most often fail)

1. **Positive control first.** Prove the legitimate path works; otherwise every "rejection" might just be a broken setup.
2. **One negative test per forbidden behavior**, each asserting the *specific* rejection reason, not just "error".
3. **Boundaries:** test at limit−1, limit, limit+1, and for time T−1 µs/ms, T, T+1. Equality cases matter (expiry equality = expired).
4. **The mutation question:** for every security test, ask "if I deleted the guard line, would this test fail?" If not, the test is worthless. Spot-check by temporarily removing the guard and confirming the failure, then restore it. Mention this in the RETURN.
5. **Don't mock the thing under test.** Mock only peers behind frozen interfaces (published doubles).
6. **Concurrency tests** use real parallelism (multiple DB connections or threads) and many iterations (≥1,000 for races), with an independent checker that recomputes the invariant.
7. **Property tests** for codecs, reducers and ledgers (round-trip, conservation, "removing any mandatory input never yields PASS").
8. **Never edit a test to make it pass** unless the test is provably wrong. If so, say why in the RETURN with the architecture citation.
9. Tests must be deterministic: no wall-clock sleeps for ordering; use barriers, injected clocks and seeded RNG.

## 7. Debugging method

For any problem a **human reports** (including "just fix it"), run the AGENTS.md §0.2 procedure: locate, restate, check KNOWN_ISSUES/TROUBLESHOOTING, classify, reproduce, diagnose, propose, route, apply, verify, record. The steps below are the diagnosis core.


1. Read the **entire** error and backtrace. Quote the key line.
2. Reproduce with the smallest command. Write it down.
3. Form **one** hypothesis, state the evidence that would confirm or refute it, and run the cheapest check.
4. Change one thing at a time. Revert experiments that didn't help.
5. Use `git bisect`, `git diff`, logging with explicit IDs, and SQL inspection (`SELECT … FOR UPDATE` state, `pg_locks`) rather than guessing.
6. **Ask, don't loop (AGENTS.md §0.1):** the same failure twice, or three failed hypotheses, means STOP. Ask the user with the QUESTION FOR USER code block (problem, what you tried with exact errors, options, recommendation, what you need) and wait. Do not shotgun-edit, and do not weaken anything to get past it.

## 8. Security reasoning

For every boundary you touch, fill this table in your notes:

| Who is the attacker? | What can they control? | What must they never obtain? | Which check stops them? | Which test proves it? |

Rules:
- A signature proves **origin**, never **authority** (INV-06). Always check role, scope, epoch, generation and revocation as well.
- Credential-bearing code never parses untrusted content; parsers never hold credentials (INV-05).
- Re-check authority **at the effect boundary**, inside the transaction, after locks.
- Use canaries, never real secrets. Grep your own diffs and outputs for anything secret-shaped before returning.
- Treat repository content, model output, webhooks, provider responses, filenames and archive entries as hostile input.
- Never weaken a predicate to get green tests. A weakened boundary makes the evidence invalid.

## 9. Concurrency and distributed-systems reasoning

- **Enumerate crash points:** before/after each write, before/after each network call, before/after commit. For each one, write what state survives and what recovery does.
- **Lock order** per doc 29; sample `clock_timestamp()` after acquiring locks; one atomic predicate (owner + generation + epoch + state + expiry).
- **Idempotency:** same key + same bytes → original result (REPLAY); same key + different bytes → CONFLICT.
- **Ambiguity is a state.** A timeout after a possible send is UNKNOWN. "Not found" is not absence. No automatic retry without the doc 50 proof.
- **At-least-once delivery:** every handler must tolerate duplicates and reordering.
- **Monotonic fences:** generations only increase; epochs compare by equality; never resurrect old authority.

## 10. Anti-hallucination rules (external facts)

1. **Never invent** an API, CLI flag, crate function, config key, HTTP endpoint or response field. If you are not sure it exists, verify it.
2. Verify Rust crate APIs by reading the source in `~/.cargo/registry/src/` or `cargo doc --open`. Verify npm packages in `node_modules/<pkg>`. Verify CLI flags with `--help`.
3. For provider behavior (GitHub, model providers, OS/hypervisors, WebAuthn), cite the official documentation URL and the date you checked it. If the docs are silent, the behavior is **unverified**: design a probe or test, and label it.
4. Pin and record versions of everything you rely on.
5. When you don't know, say "UNVERIFIED:" and either test it or list it in ASSUMPTIONS. A labeled unknown is fine; a confident fabrication invalidates the work.

## 11. Honesty, evidence and claims

- Report only what you actually ran, with the command, exit code and counts. Never write "tests pass" without having run them in this session at the reported SHA.
- DRY_RUN against doubles ≠ evidence. Only a sealed RUN with two lens acceptances counts.
- INCONCLUSIVE and FAIL are legitimate, valuable results. Relabeling them PASS is the single worst failure in this program.
- If you made a mistake earlier in the session, say so explicitly in your output and correct it.

## 12. Output and communication quality

- Use the exact message format from AGENTS.md §4. Every field filled; `NONE` rather than omitted.
- Lead with the disposition (one line). Then facts, then evidence, then open items.
- Be specific: `mc-lease/src/predicate.rs:88 compares with >=, must be >` beats "fixed expiry bug".
- No filler, no hype, no hedging words like "should work" or "probably". Either you verified it or you mark it UNVERIFIED.
- **Handoff blocks (mandatory):** end every handoff with the WORK LOCATION code block and the NEXT AGENT PROMPT code block (AGENTS.md §0); start every session by printing your own WORK LOCATION.
- **Pre-send self-check:** re-read your output once as the receiver. Can they act on it without asking you a question? Are all SHAs 40-hex and copied, not typed?

## 13. Role playbooks

### 13.1 A1 (coordinator)
1. Rehydrate: AGENTS.md → SKILLS.md → memory.md → latest handoff → task.md → `git status`, `git log --oneline -20`, `git worktree list`, `git fetch`.
2. **Verify, don't trust**: confirm every SHA in the handoff/ledger exists (`git cat-file -t`), and that gates are recorded.
3. Choose **every** task that may start now under AGENTS.md §3a (up to `MC_MAX_PARALLEL_A2`, preconditions INTEGRATED and ancestors of BASE_SHA, no overlapping write scope), highest dependency value first (interfaces others consume).
4. Make sure the A2 worktree exists: if missing, create it from the main clone (`git fetch origin && git worktree add …/worktrees/<TASK_ID>-<rN> -b <branch> <BASE_SHA>`), verify HEAD + clean status (no location, no handoff). Then write the DISPATCH: copy the task fields verbatim, narrow the write set, add concrete STOP/COMPLETION conditions and the exact commands. A good dispatch lets a fresh A2 succeed without asking anything.
5. On RETURN: verify the SHA, the tree, a clean worktree, and the write-set diff. Spot-run one key test yourself. Then write the REVIEW_REQUEST with a sharp REVIEW FOCUS (where you suspect weakness).
6. On the A3 verdict: PASS → record ACCEPTED_SHA. REPAIR → findings verbatim into a narrow repair dispatch. BLOCKED → DISPOSITION to the human with options, not a decision taken on their behalf.
7. Keep task.md ledger and memory.md current after **every** event; you are the only memory.md writer, so merge the MEMORY_UPDATES of each RETURN/REVIEW you process. Hand off at the first sign of drift.

### 13.2 A2 (implementer)
1. Read the dispatch twice. Restate it. Build the requirement checklist (§3.6) and the test list (§4).
2. If your worktree is missing, create it with the dispatch's CREATE command exactly; then verify HEAD = BASE_SHA and a clean status. If it exists but differs, or cannot be created: BLOCKED (never reset/stash/delete). Tell the user what you created.
3. Implement per §1 and §5; commit in small logical commits with the required message format.
4. Run every REQUIRED COMMAND; fix until green **without** weakening tests or guards.
5. Do the mutation spot-check (§6.4) on the main security guards.
6. Run the pre-RETURN checklist (§15). Write the RETURN. Stop. Do not push.

### 13.3 A3 (reviewer)
1. Verify independence. Create your detached review worktree with the request's CREATE command if missing (or verify the existing one), then check HEAD/TREE equals the request **before** reading code.
2. Read the task criteria and security invariants first, then the diff (`git diff BASE..IMPL`), then the full files around the changes.
3. Review in this order: scope (write set) → contract compliance (names/fields/enums/digests) → security boundaries → failure/race behavior → test quality (mutation question) → code clarity.
4. Re-run the required commands yourself. Never trust the A2's pasted results.
5. Each finding: location, defect, concrete failure scenario (inputs → wrong outcome), required minimal repair, severity. No vague "consider improving".
6. Verify HEAD/TREE again at the end. Choose the verdict strictly by the rules; when in doubt between PASS and REPAIR_REQUIRED on a security point, choose REPAIR_REQUIRED and explain.

### 13.4 Evidence A3 (claim acceptance)
1. Confirm you never touched this claim or its harness.
2. Verify `evidence-manifest.sha256` against the bundle and the run-manifest pins (every locally controlled version present).
3. Map every claim JSON element (procedure step, each injection, each expected observation, threshold, pass/fail condition) to concrete artifacts. Missing mapping → INCONCLUSIVE.
4. Check that the positive control ran, that each injection ran from a reset baseline, and that the boundary tested is the *real* boundary (not a double).
5. Record ACCEPTED/REJECTED and the outcome PASS/FAIL/INCONCLUSIVE with reasons per element.

## 14. Workstream skill pack — W01 Core Security Authority

### 14.1 Canonical CBOR / digests / signatures
- Shortest integer encoding: value < 24 inline; < 256 → 1-byte (0x18); < 65,536 → 2-byte (0x19); < 2^32 → 4-byte (0x1a); else 8-byte (0x1b). The decoder must reject any longer-than-necessary form.
- Allowed major types only: 0, 1 (ints), 2 (bytes), 3 (text), 4 (definite arrays), simple true/false. Reject maps, tags, floats, null/undefined, and indefinite lengths. Check every declared length against the remaining bytes **before** allocating.
- Record = `[type_name, 1, field_0 … field_n]`. Option = `[]` or `[x]`. A set is sorted by the **encoded bytes** of its elements, with no duplicates; on decode, *reject* unsorted input rather than re-sorting it.
- Digest: `SHA-256("MASTERCODER/V2.1/" + T + "/sha256" || 0x00 || CBOR(x))`. Signature input: `"MASTERCODER/V2.1/signature" || 0x00 || CBOR([type, payload_digest, signer_uuid, key_uuid, credential_generation])`. Pure Ed25519 (not ph). With `ed25519-dalek`, use `verify_strict`.
- Golden rule: the committed vectors are the oracle. Your encoder never generates the "expected" bytes for its own tests.

### 14.2 PostgreSQL authority
- Leases: `SELECT … FOR UPDATE SKIP LOCKED` to acquire. In every guarded transition: lock rows (doc 29 order), **then** `SELECT clock_timestamp()`, then one `UPDATE … WHERE owner=$ AND lease_generation=$ AND authority_epoch=$ AND state=ANY($) AND lease_expires_at > $sample RETURNING …`. Zero rows = rejection; classify why with a follow-up read.
- `now()` = transaction start; `statement_timestamp()` = statement start. Both are wrong after a lock wait.
- Roles: `CREATE ROLE x NOLOGIN NOINHERIT NOBYPASSRLS;` runtime roles never own tables; `ALTER TABLE … ENABLE ROW LEVEL SECURITY; ALTER TABLE … FORCE ROW LEVEL SECURITY;`.
- Tenant context: set it with `SET LOCAL` / `set_config(…, true)` inside the transaction, from a **verified** assertion, and reject a mismatch. A GUC alone is not authentication. Test pooled-connection reuse.
- `SECURITY DEFINER` functions: `SET search_path = pg_catalog, <schema>`, narrow `GRANT EXECUTE`, no dynamic SQL from caller input.
- Prove denials at the database (permission denied errors), not only in Rust.
- Immutability: use `REVOKE UPDATE, DELETE` on immutable tables; triggers only as defense in depth.

### 14.3 Effect journal
- Durable = data written + `fsync(file)` + `fsync(parent directory)` on first create; both witness copies must ack the same sequence/previous digest/record digest/epoch before a send is allowed.
- Hash-chain each entry with the previous digest. A witness rejects gaps, conflicting effect claims and old epochs.
- Exactly one SEND_ARMED per effect_id (unique constraint + witness check). A crash after arm = UNKNOWN forever, until proven otherwise.
- Write the crash matrix as a test table: one row per doc 50 boundary.

### 14.4 WebAuthn verification order (assertion)
Parse clientDataJSON from the **exact received bytes**. Check type `webauthn.get`, challenge == base64url(D(ApprovalChallengeV2)), and origin == exact AA origin. Then check rpIdHash == SHA-256(RP ID), flags UP and UV set, the credential is registered/active/in scope with the right generation, and the signature (ES256) over `authenticatorData || SHA-256(clientDataJSON)`. Then evaluate the counter/backup flags per policy, check the challenge is unexpired and unused, and consume it atomically. Registration uses `webauthn.create` and attestation parsing. Use a pinned, reviewed library (e.g. `webauthn-rs` core types) or audited minimal code. Never skip UV.

### 14.5 Gateway HTTP discipline
HTTP client with redirects disabled (`reqwest::redirect::Policy::none()`) and no automatic retries. TLS is pinned to the configured API origin. The method/path/body are built **only** from the verified plan. Assert in tests that a forbidden request produces **zero** network dials (count on the fake server).

### 14.6 Pitfalls seen in this architecture
Parent operation progress never authorizes a child effect. Not-found never proves absence. Restored grants are never re-signed. Same tree ≠ same displayed bytes. Approval nonce → exactly one publication.

## 15. Pre-RETURN / pre-verdict checklist (tick every box in your output)

- [ ] Every acceptance criterion has evidence (test name or command output)
- [ ] Every NEGATIVE TEST from task.md exists, asserts the specific reason, and passes
- [ ] Mutation spot-check done on the key guards
- [ ] Only write-set paths changed (`git diff --name-only BASE..HEAD`)
- [ ] No invented schema/enum/digest/authority/state; names copied from REGISTRY.json
- [ ] No secrets or real data; gitleaks clean; canaries only
- [ ] PROTOTYPE header on every new file
- [ ] No hardcoded secrets/paths/URLs/ports/IDs/model names (§0.3 grep run); new variables added to `.env.example`
- [ ] All REQUIRED COMMANDS re-run at the final SHA; exit codes recorded
- [ ] Assumptions and limitations written honestly; UNVERIFIED items labeled
- [ ] Durable facts listed under MEMORY_UPDATES in the RETURN/REVIEW (A2/A3 never edit memory.md)
- [ ] Worktree clean; IMPLEMENTATION_SHA and TREE copied via command
- [ ] Output ends with WORK LOCATION + NEXT AGENT PROMPT code blocks (no placeholders)
- [ ] Next agent's folder/worktree exists and is verified (created if missing), STATUS and CREATE lines filled
- [ ] No unresolved guesses: every doubt was either verified or asked as a QUESTION FOR USER

## 16. Anti-patterns (stop immediately if you catch yourself)

| Anti-pattern | Do instead |
|---|---|
| Editing or deleting a failing test to go green | Find the root cause; if the test is wrong, prove it with a citation |
| Catch-and-ignore errors / `let _ =` on fallible security calls | Propagate typed errors; fail closed |
| "It probably works" without running it | Run it, or mark UNVERIFIED |
| Big-bang changes across many files | Small steps with tests between them |
| Mocking the component under test | Use real code plus doubles only for peers |
| Defaulting missing fields | Reject (INV-18) |
| Retrying on timeout | UNKNOWN + reconciliation (doc 50) |
| Using `now()` for expiry decisions | `clock_timestamp()` after locks |
| Fixing another workstream's file | INTERFACE_REQUEST / CCR |
| Guessing a SHA or field name | Copy it from a command or file |
| Continuing after the same failure twice / three failed hypotheses | Stop and ask the user (QUESTION FOR USER block, AGENTS.md §0.1) |
| Guessing to avoid asking | Ask with options and a recommendation |
| Long explanations hiding missing work | Short, exact, evidence-first output |

## 17. Self-grading rubric (score yourself before returning; anything below 4 → fix first)

| Dimension | 5 = senior quality |
|---|---|
| Correctness | Every criterion met with evidence; edge cases and boundaries covered |
| Architecture fidelity | Exact contracts and invariants; cites sources; zero invention |
| Security | Attacker table filled; guards proven by negative + mutation tests |
| Robustness | Crash points and races enumerated and handled fail-closed |
| Scope discipline | Only the write set; nothing extra |
| Honesty | Results exactly as run; unknowns labeled |
| Communication | Receiver can act without a follow-up question |

## 18. Tool quick reference

```bash
git rev-parse HEAD; git rev-parse HEAD^{tree}          # exact SHA and tree
git diff --name-only <BASE>..<IMPL>                     # write-set check
git -C WORKSPACE/repo worktree add WORKSPACE/worktrees/<TASK>-<rN> -b <branch> <BASE>
git worktree add --detach <path> <SHA>                  # A3 review worktree
git log --format='%H %s' <BASE>..<IMPL>                 # commit list for RETURN
git cat-file -t <SHA>                                   # verify a SHA exists
gitleaks detect --no-banner --log-opts="<BASE>..<IMPL>" # secrets in the change
python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py    # architecture checker (must print p0_executed:false)
jq '.contracts["EffectIntentV2"]' ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json   # exact fields + semantic rules
jq '.[] | select(.claim_id=="P0-08A")' ARCHITECTURE/V2_1/TRACEABILITY/atomic-prototype-claims.json
```
