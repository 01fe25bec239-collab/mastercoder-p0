# task.md — WORKSTREAM-05-WEB-APPROVAL-UX (Web & Approval UX)

Operator: master (one human, all six lanes). Coordinator: this lane's current A1 (see latest `HANDOFFS/`). Every task below is execution-ready with all 20 fields. A1 runs up to `MC_MAX_PARALLEL_A2` A2 tasks at once (AGENTS.md §3a), each in its own worktree with a fresh A2.

**Mission.** Build the truthful human surfaces and the independent TypeScript codec: the ordinary Next.js UI (untrusted preview only), the AA-origin trusted renderer on a separate registrable domain with strict CSP and no framing, the trusted resolution UI, WebAuthn browser ceremony harness and authenticator matrix, accessible state language (no color-only security states, no 'complete' for partial work), and accessibility validation evidence.

## In flight (A1 keeps this current; copy it into every handoff)

| TASK_ID | ATTEMPT | WORKTREE | BASE_SHA | WRITE SCOPE | STARTED (UTC) | WAITING FOR |
|---|---|---|---|---|---|---|
| — | — | — | — | — | — | — |

## Parallel waves (BUILD tasks, from PRECONDITIONS)

A wave can start when every task in the earlier waves that it depends on is INTEGRATED (merged, and an ancestor of the new BASE_SHA). Within a wave, tasks run in parallel if their write scopes don't overlap (AGENTS.md §3a). 37 BUILD tasks, 7 waves, widest wave 15 tasks. Cross-workstream preconditions are listed; for BUILD, use the published double if the real interface is not merged yet.

| WAVE | TASKS | TASK IDS |
|---|---|---|
| 1 | 1 | W05-T000 |
| 2 | 15 | W05-T001, W05-T002, W05-T003, W05-T004, W05-T005, W05-T006, W05-T007, W05-T008, W05-T009, W05-T010, W05-T011, W05-T012, W05-T013 (also needs W01-T001), W05-T014 (also needs W01-T002), W05-T036 |
| 3 | 5 | W05-T015 (also needs W06-T003), W05-T016, W05-T017, W05-T018, W05-T019 |
| 4 | 11 | W05-T020, W05-T021, W05-T022, W05-T023, W05-T024, W05-T025, W05-T026, W05-T027, W05-T028, W05-T029, W05-T030 |
| 5 | 1 | W05-T031 |
| 6 | 3 | W05-T032, W05-T033, W05-T034 |
| 7 | 1 | W05-T035 |

## Task state ledger (A1 updates; never delete rows)

| TASK_ID | KIND | STATE | ATTEMPT | IMPLEMENTATION_SHA | ACCEPTED_SHA | MERGED_SHA | NOTES |
|---|---|---|---|---|---|---|---|
| W05-T000-WORKSPACE-SCAFFOLD | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T001-CONTRACT-TYPES-TS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T002-STATE-LANGUAGE | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T003-WIRE-TS-CBOR-CORE | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T004-TRUSTED-ORIGIN-KIT | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T005-EXTERNAL-AUDIT-PACKET | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T006-G047-RESOLUTION-ORIGIN-TRIAGE | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T007-TEST-IDP | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T008-WEBAUTHN-CROSSCHECK-FIXTURES | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T009-SYNTHETIC-VULNERABLE-REPOS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T010-PATH-FIXTURES | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T011-PACK-FIXTURES | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T012-REGISTRY-AND-ARCHIVE-FIXTURES | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T013-AA-API-MOCK-SERVER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T014-RESOLUTION-API-MOCK-SERVER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T015-WIRE-TS-CANONICAL | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T016-ORDINARY-API-MOCKS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T017-AA-ORIGIN-SHELL | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T018-RESOLUTION-ORIGIN-SHELL | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T019-WEBAUTHN-HARNESS-CI | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T020-REVIEW-COVERAGE-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T021-REVIEW-FINDINGS-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T022-REVIEW-PROJECTION-PANEL | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T023-ATTEMPT-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T024-GRAPH-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T025-SETUP-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T026-PATCH-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T027-ADMIN-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T028-DISCLOSURE-BUDGET-VIEW | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T029-AA-DIFF-RENDERER | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T030-RESOLUTION-FLOWS-UI | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T031-AA-CEREMONY | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T032-AUTHENTICATOR-MATRIX | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T033-P0-09A-HARNESS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T034-A11Y-AUTOMATED | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T035-A11Y-MANUAL | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-T036-ESCAPE-CORPUS | BUILD | NOT_STARTED | — | NONE | NONE | NONE |  |
| W05-RUN-P0-09A | RUN | NOT_STARTED | — | NONE | NONE | NONE | needs G2+G3 |

States: NOT_STARTED · DISPATCHED · RETURNED · IN_REVIEW · REPAIR · ACCEPTED · INTEGRATED · BLOCKED. RUN tasks additionally carry, per cell and per immutable run, the measured outcome (PASS/FAIL/INCONCLUSIVE, `claims/<id>/cells/<cell>/runs/<RUN_ID>/outcome.json`, written once) and the evidence lifecycle (NOT_RUN/RUNNING/SUBMITTED/ACCEPTED/REJECTED, `lifecycle.jsonl`); `cells/<cell>/current.json` names the current run.

## Ordering rules

- BUILD tasks may start after G0 + G1 when their PRECONDITIONS hold; use published doubles for anything another workstream has not INTEGRATED yet. Never let a lane idle waiting for another lane — use the double and record the pin.
- RUN tasks (`W05-RUN-*`) start only after G2 + G3 + their INTEGRATED build task(s) + integration milestone. Until then the harness is exercised as DRY_RUN inside BUILD tasks.
- Interfaces you provide (I-10, UX fixtures/screenshots for the dossier, escape corpus for W03 (W05-T036)) should be published early (doubles first) — consumers depend on them.
- Post-freeze production work is NOT in this file; see `POST_FREEZE/POST_FREEZE_BACKLOG.md` (not dispatched).

---

## BUILD tasks

### W05-T000-WORKSPACE-SCAFFOLD
- **TASK_ID:** W05-T000-WORKSPACE-SCAFFOLD
- **OBJECTIVE:** **In plain words:** Set up the empty web project: the three apps (normal app, approval website, resolution website), all shared packages, test tools and CI, with every dependency pinned and no screens yet. Later tasks only add their own screen or package code, so many A2s can work at once. **Precisely:** Create the complete `ws05-web-approval-ux/` pnpm workspace so every later W05 task only edits its own route folder, package source, tests or fixtures and never touches any `package.json`, `pnpm-lock.yaml`, root layout or CI: three Next.js apps (`web`, `aa-origin`, `resolution-origin`) with root layouts and a navigation listing every planned route; packages `contracts-ts`, `state-language`, `wire-ts`, `webauthn-harness`, `webauthn-fixtures`, `trusted-origin`; mock packages `mocks/control-api`, `mocks/aa-api`, `mocks/resolution-api`; test packages `claims/P0-09A/harness` and `a11y`; TypeScript strict config, lint, unit test runner, Playwright, axe, and CI.
- **WHY IT EXISTS:** Parallel A2s conflict only on shared files (package manifests, lockfile, layouts, CI); the scaffold owns them once (AGENTS.md §3a).
- **ARCHITECTURE REQUIREMENTS:** 00F frozen web stack (Next.js, React, TypeScript); 16; 36 pinned lockfiles.
- **P0 CLAIMS:** none directly (enables P0-09A).
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** Workspace layout; `docs/MODULES.md` (package/route → owning task).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/{package.json,pnpm-workspace.yaml,pnpm-lock.yaml,.npmrc,.nvmrc,tsconfig.base.json,eslint.config.mjs,vitest.workspace.ts,playwright.config.ts,.env.example,README.md,.gitignore}`, `ws05-web-approval-ux/docs/{MODULES.md,README.md}`, `ws05-web-approval-ux/apps/*/{package.json,tsconfig.json,next.config.mjs,next-env.d.ts}`, `ws05-web-approval-ux/apps/*/app/{layout.tsx,page.tsx,not-found.tsx}`, `ws05-web-approval-ux/packages/*/{package.json,tsconfig.json,src/index.ts}`, `ws05-web-approval-ux/mocks/*/{package.json,tsconfig.json,src/index.ts}`, `ws05-web-approval-ux/claims/P0-09A/harness/{package.json,tsconfig.json}`, `ws05-web-approval-ux/claims/README.md`, `ws05-web-approval-ux/a11y/{package.json,tsconfig.json}`, `.github/workflows/ws05-ci.yml`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** G0, G1.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `package.json`; `pnpm-workspace.yaml`; `pnpm-lock.yaml`; `.npmrc`; `.nvmrc`; `tsconfig.base.json`; `eslint.config.mjs`; `vitest.workspace.ts`; `playwright.config.ts`; `.env.example`; `README.md`; `.gitignore`; `docs/{MODULES.md,README.md}`; `apps/{web,aa-origin,resolution-origin}/{package.json,tsconfig.json,next.config.mjs,next-env.d.ts,app/layout.tsx,app/page.tsx,app/not-found.tsx}`; `packages/{contracts-ts,state-language,wire-ts,webauthn-harness,webauthn-fixtures,trusted-origin}/{package.json,tsconfig.json,src/index.ts}`; `mocks/{control-api,aa-api,resolution-api}/{package.json,tsconfig.json,src/index.ts}`; `claims/P0-09A/harness/{package.json,tsconfig.json}`; `claims/README.md`; `a11y/{package.json,tsconfig.json}`; `../.github/workflows/ws05-ci.yml`. **Steps:** 1. pnpm workspace; Node version pinned in `.nvmrc`; every dependency pinned to an exact version (latest stable on the scaffold day, recorded in `docs/MODULES.md`): next, react, react-dom, typescript, eslint (+ next/react/jsx-a11y plugins), vitest, @testing-library/react, @playwright/test, @axe-core/playwright, msw, openapi-typescript, json-schema-to-typescript, @noble/ed25519, @noble/hashes, fast-check, zod. 2. `tsconfig.base.json`: `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`. 3. Each app: root `layout.tsx` with the navigation list of every planned route (links to routes that later tasks create; a not-yet-built route shows `not-found.tsx` with "not built yet"), and no other pages. Planned routes: web — `/review/[id]/coverage`, `/review/[id]/findings`, `/review/[id]/projection`, `/attempt/[id]`, `/attempt/[id]/graph`, `/setup`, `/patch/[id]`, `/admin`, `/disclosure`, `/budget`; aa-origin — `/plan/[planId]`; resolution-origin — `/case/[caseId]`. 4. Packages and mocks: `package.json` with exactly the dependencies their tasks need, `src/index.ts` exporting nothing yet. 5. ESLint rule banning `<iframe` and `dangerouslySetInnerHTML` in all three apps. 6. CI `ws05-ci.yml` (job `ws05-ci`; change gate over the lane directory + itself); `permissions: contents: read`; `pnpm install --frozen-lockfile`, typecheck, lint, vitest, Playwright (Chromium, Firefox, WebKit), axe; `python3 -m pytest fixtures` (the W05-T009…T012 fixture builders); `check_spec.py`; no `pull_request_target`. 7. `docs/MODULES.md` with the owner table and the dependency rule (no task adds a dependency; it returns DEPENDENCY_REQUEST). **Required-check rule (one operator, parallel auto-merge):** the workflow is named and has one job named `ws05-ci` (a required check on main), triggers on **every** pull request to main and every push to main (no workflow-level `paths:` filter, so it always reports), and its first step decides with `git diff --name-only <merge-base> HEAD` whether this lane's paths or the upstream paths listed here changed; if not, the remaining steps are skipped by `if:` and the job still reports success. Without this, auto-merge could merge a PR whose lane tests never ran.
- **SECURITY INVARIANTS:** No secrets; CI read-only; iframes and raw HTML banned by lint from day one.
- **FAILURE CASES:** A package missing a dependency a later task needs; layout without the planned routes.
- **TESTS REQUIRED:** `pnpm install --frozen-lockfile && pnpm -r typecheck && pnpm -r lint && pnpm -r test` green; `pnpm --filter web build`, `--filter aa-origin build`, `--filter resolution-origin build` succeed; CI green. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `pnpm install --frozen-lockfile` exits 0; `pnpm -r typecheck && pnpm -r lint && pnpm -r test` exits 0; `pnpm --filter web build`, `--filter aa-origin build`, `--filter resolution-origin build` exit 0; `a scratch `<iframe>` in apps/web fails lint`; `a scratch `dangerouslySetInnerHTML` fails lint`; `every planned route appears in its app's layout navigation`; `CI green on the PR`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** A scratch file containing `<iframe>` in apps/web fails lint (shown in the return, then deleted).
- **ACCEPTANCE CRITERIA:** All 3 apps, 6 packages, 3 mock packages, harness and a11y packages install and build; navigation lists every planned route.
- **EVIDENCE TO RETURN:** CI run URL; pinned version table.
- **OUT-OF-SCOPE:** Any view, component, mock data or package logic.
- **BLOCKERS:** G0/G1 not done → BLOCKED.
- **A3 REVIEW FOCUS:** Dependency list complete; lint bans in place; nothing hand-written beyond skeletons.

### W05-T001-CONTRACT-TYPES-TS
- **TASK_ID:** W05-T001-CONTRACT-TYPES-TS
- **OBJECTIVE:** **In plain words:** Generate TypeScript types for all 85 data records straight from the architecture's schema files, so no screen or mock ever types a record by hand. **Precisely:** Generate TypeScript types for all 85 contracts' JSON projections into `packages/contracts-ts` directly from `ARCHITECTURE/V2_1/SCHEMAS/*.schema.json` with `json-schema-to-typescript`, plus runtime validators (zod schemas generated from the same files), so no view or mock ever hand-types a contract.
- **WHY IT EXISTS:** Shared Contract Rule: no local variants of contract names or fields.
- **ARCHITECTURE REQUIREMENTS:** SCHEMAS/*.schema.json; ENCODING.md JSON projection (u64/i64 as decimal strings, lowercase hex, options as 0/1-element arrays).
- **P0 CLAIMS:** none (enabler).
- **INPUT CONTRACTS:** all 85 schema files.
- **OUTPUT CONTRACTS:** `packages/contracts-ts/src/generated/*.ts`, `src/index.ts`, `scripts/generate.mjs`; CI freshness check. **Exact public API (names and signatures are binding; do not rename):** `export type <ContractName> = …` for each of the 85; `export const validators: Record<ContractName, ZodType>`; `export function validate<T>(name, json): T` (throws on unknown field); CLI `node scripts/generate.mjs [--check]`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/contracts-ts/src/**`, `ws05-web-approval-ux/packages/contracts-ts/scripts/**`, `ws05-web-approval-ux/packages/contracts-ts/test/**`
- **READ-ONLY DEPENDENCIES:** `ARCHITECTURE/V2_1/SCHEMAS/**`. Independence: never open `ws01-core-security/crates/mc-contracts/**`.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/contracts-ts/scripts/generate.mjs`; `packages/contracts-ts/src/generated/<Contract>.ts (85 files, generated)`; `packages/contracts-ts/src/validators.ts (generated)`; `packages/contracts-ts/src/index.ts`; `packages/contracts-ts/test/contracts.test.ts`. **Steps:** 1. Generator script reads every schema and writes one file per contract. 2. 64-bit integers typed as `string` (decimal), never `number`. 3. Validators reject unknown fields. 4. `--check` mode fails when output differs.
- **SECURITY INVARIANTS:** No hand edits in generated files.
- **FAILURE CASES:** u64 typed as number.
- **TESTS REQUIRED:** Every ACCEPT projection in `wire-vectors.json` validates; freshness check. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `contracts.test.ts::all_accept_projections_validate`; `contracts.test.ts::unknown_field_rejected`; `contracts.test.ts::numeric_u64_rejected`; `contracts.test.ts::uppercase_hex_rejected`; `contracts.test.ts::u64_typed_as_string (type-level test)`; `node scripts/generate.mjs --check` exits 0. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Unknown field rejected; numeric u64 rejected; uppercase hex rejected.
- **ACCEPTANCE CRITERIA:** 85 types; all ACCEPT projections validate; regeneration identical.
- **EVIDENCE TO RETURN:** Test counts; generator command.
- **OUT-OF-SCOPE:** CBOR (T003/T015).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No hand-typed contracts; 64-bit as strings.

### W05-T002-STATE-LANGUAGE
- **TASK_ID:** W05-T002-STATE-LANGUAGE
- **OBJECTIVE:** **In plain words:** Build the status badges and banners (PASS, FAIL, BLOCKED, UNKNOWN and so on) that every screen uses. Each has an icon, words and screen-reader text, never colour alone, and nothing partial or unknown is ever worded as complete or safe. **Precisely:** Build `packages/state-language`: one React component and one text label per security state — every GateProofV2 conclusion in the registry (PASS, FAIL, BLOCKED, HUMAN_REVIEW_REQUIRED, PARTIALLY_AVAILABLE, STALE, REVOKED, INVALID) plus UNSUPPORTED, UNKNOWN and the derived EXTERNAL_GATE_UNSAFE — each with icon + text + ARIA, never color alone, and with copy that never says "complete" or "safe" for partial or unknown work.
- **WHY IT EXISTS:** 16 state language; INV-03: nothing may look complete when it is partial.
- **ARCHITECTURE REQUIREMENTS:** 16 in full; INV-03.
- **P0 CLAIMS:** none (enabler).
- **INPUT CONTRACTS:** state enum values from `contracts-ts` where they exist; EXTERNAL_GATE_UNSAFE as defined in 16/54.
- **OUTPUT CONTRACTS:** `<StateBadge state=…>`, `<StateBanner state=…>`, `stateLabel(state)`, `announce(state)` (ARIA live region). **Exact public API (names and signatures are binding; do not rename):** `export type SecurityState = 'PASS'|'FAIL'|'BLOCKED'|'HUMAN_REVIEW_REQUIRED'|'PARTIALLY_AVAILABLE'|'STALE'|'UNSUPPORTED'|'UNKNOWN'|'EXTERNAL_GATE_UNSAFE'` (registry values where defined); `stateLabel(s): string`; `<StateBadge state />`; `<StateBanner state reason? />`; `announce(s): void`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/state-language/src/**`, `ws05-web-approval-ux/packages/state-language/test/**`
- **READ-ONLY DEPENDENCIES:** `contracts-ts` (types only; may be the skeleton until T001 lands).
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/state-language/src/{labels.ts,StateBadge.tsx,StateBanner.tsx,announce.ts,icons.tsx,index.ts}`; `packages/state-language/test/{labels.test.ts,badge.test.tsx,a11y.test.tsx}`. **Steps:** 1. Label table in one file. 2. Icons as inline SVG with `aria-hidden` plus visible text. 3. Banner variant for page-level states. 4. Unit + axe tests per state.
- **SECURITY INVARIANTS:** Partial/unknown never shown as complete or safe.
- **FAILURE CASES:** Green check for STALE provider success.
- **TESTS REQUIRED:** Render + axe test per state; snapshot of the label table. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `labels.test.ts::every_state_has_label`; `labels.test.ts::partial_unknown_stale_never_say_complete_safe_or_passed`; `badge.test.tsx::every_state_has_text_icon_and_aria`; `badge.test.tsx::states_distinguishable_without_colour`; `a11y.test.tsx::axe_clean_for_every_state`; `a11y.test.tsx::removing_text_fails_axe`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Every registry conclusion has a label (a test enumerates the GateProofV2 conclusion enum); no label for PARTIALLY_AVAILABLE/UNKNOWN/STALE/REVOKED/INVALID contains "complete", "safe" or "passed"; a test removes the text and expects the a11y test to fail.
- **ACCEPTANCE CRITERIA:** 9 states distinct by text + icon + ARIA.
- **EVIDENCE TO RETURN:** Screenshot sheet of all states; axe report.
- **OUT-OF-SCOPE:** Views.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Truthful wording.

### W05-T003-WIRE-TS-CBOR-CORE
- **TASK_ID:** W05-T003-WIRE-TS-CBOR-CORE
- **OBJECTIVE:** **In plain words:** Build the low-level part of our own TypeScript version of the binary record format (CBOR). It must reject every malformed input, and is written independently from the W01 lane's Rust version so the two can be compared. **Precisely:** Implement the byte layer of `@mc/wire-ts` (independent of W01): a strict deterministic CBOR encoder/decoder for exactly the ENCODING.md subset (unsigned/negative integers as BigInt, byte strings, NFC-checked text, definite arrays, booleans false/true), rejecting every forbidden form before allocating memory.
- **WHY IT EXISTS:** ENCODING/TEST_VECTORS require an independent TypeScript implementation (G-039); splitting lets the byte layer start before the contract layer.
- **ARCHITECTURE REQUIREMENTS:** ENCODING.md (shortest integers; no maps/tags/floats/null/undefined/indefinite; record ≤16 MiB, nesting ≤32, arrays ≤65,536; NFC rejected not normalized); TEST_VECTORS README; INV-18.
- **P0 CLAIMS:** supports P0-06A, P0-06B (W06).
- **INPUT CONTRACTS:** none (bytes).
- **OUTPUT CONTRACTS:** `encode(value)`, `decode(bytes)` in `packages/wire-ts/src/cbor/`. **Exact public API (names and signatures are binding; do not rename):** `export type CborValue = {kind:'uint',v:bigint}|{kind:'nint',v:bigint}|{kind:'bytes',v:Uint8Array}|{kind:'text',v:string}|{kind:'array',v:CborValue[]}|{kind:'bool',v:boolean}` (bool = exactly CBOR false 0xf4 / true 0xf5 per ENCODING.md; null, undefined, floats and every other simple value stay FORBIDDEN_MAJOR; audit F-4); `encode(v): Uint8Array`; `decode(b, limits?): CborValue` throwing `CborError` with `code: 'TRUNCATED'|'TRAILING'|'NON_MINIMAL'|'FORBIDDEN_MAJOR'|'INDEFINITE'|'BOUND'|'NESTING'|'NFC'|'UTF8'`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/wire-ts/src/cbor/**`, `ws05-web-approval-ux/packages/wire-ts/test/cbor/**`
- **READ-ONLY DEPENDENCIES:** ENCODING.md. **Forbidden reads:** `ws01-core-security/crates/{mc-cbor,mc-wire}/**`, `check_spec.py` source.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/wire-ts/src/cbor/{value.ts,encode.ts,decode.ts,errors.ts,index.ts}`; `packages/wire-ts/test/cbor/{roundtrip.test.ts,reject.test.ts,fuzz.test.ts}`. **Steps:** 1. Values: `{kind:'uint',v:bigint}`, `{kind:'nint',v:bigint}`, bytes, text, array, `{kind:'bool',v:boolean}` (exactly 0xf4/0xf5; ENCODING.md "Boolean is exactly CBOR false/true"). 2. Length checked against remaining bytes before any allocation. 3. Text must be valid UTF-8 and already NFC (`s.normalize('NFC') === s`), else reject. 4. fast-check property tests.
- **SECURITY INVARIANTS:** INV-18; never `Number` for integers; no normalization.
- **FAILURE CASES:** Precision loss; NFC normalized instead of rejected.
- **TESTS REQUIRED:** Round-trip property tests; one test per rejection class; long fast-check decode run with arbitrary bytes and no exception other than a typed rejection. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `roundtrip.test.ts::decode_encode_identity_property`; `roundtrip.test.ts::encoding_unique`; `roundtrip.test.ts::bool_false_f4_true_f5`; `reject.test.ts::other_simple_values`; `reject.test.ts::truncated`; `reject.test.ts::trailing_byte`; `reject.test.ts::non_minimal_int`; `reject.test.ts::float`; `reject.test.ts::tag`; `reject.test.ts::map`; `reject.test.ts::null_undefined`; `reject.test.ts::indefinite`; `reject.test.ts::nesting_33`; `reject.test.ts::length_exceeds_remaining_no_alloc`; `reject.test.ts::non_nfc_text`; `reject.test.ts::invalid_utf8`; `fuzz.test.ts::arbitrary_bytes_only_typed_errors`; `lint: no `Number(` or `parseInt` on integer paths`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Non-minimal integer, float, tag, map, null, indefinite length, nesting 33, length > remaining, non-NFC text, invalid UTF-8, trailing byte.
- **ACCEPTANCE CRITERIA:** Every rejection class covered; no `Number` on integer paths (lint rule test).
- **EVIDENCE TO RETURN:** Test counts.
- **OUT-OF-SCOPE:** Contracts, digests, signatures (T015).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** BigInt everywhere; independence.

### W05-T004-TRUSTED-ORIGIN-KIT
- **TASK_ID:** W05-T004-TRUSTED-ORIGIN-KIT
- **OBJECTIVE:** **In plain words:** Build the shared safety kit for the two trusted websites: strict browser security headers, secure session cookie, CSRF protection, refusal to run inside a frame, and the helper that asks the user's fingerprint/security key to sign. **Precisely:** Build `packages/trusted-origin`, shared by the AA origin and the resolution origin: server-side security headers (strict CSP with scripts from self only, `frame-ancestors 'none'`, no inline scripts, HSTS, `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Resource-Policy: same-origin`), host-only Secure HttpOnly session cookie, CSRF token check on mutations, a guard that refuses to run when framed or opened cross-origin, service-worker prohibition, and a browser helper that performs `navigator.credentials.get` with a server-issued challenge (base64url, no padding) and returns the raw assertion fields.
- **WHY IT EXISTS:** 52: both trusted origins need identical, reviewed protections; one kit avoids two slightly different copies.
- **ARCHITECTURE REQUIREMENTS:** 52 trusted display and ceremony rules; 16; ENCODING WebAuthn encoding; INV-10, INV-11.
- **P0 CLAIMS:** supports P0-09A.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `securityHeaders(config)`, `withSession`, `requireCsrf`, `assertTopLevelSameOrigin()`, `getAssertion(challenge, rpId, allowCredentials)`. **Exact public API (names and signatures are binding; do not rename):** `securityHeaders(cfg: TrustedOriginConfig): Record<string,string>`; `withSession(handler)`; `requireCsrf(req)`; `assertTopLevelSameOrigin(): void` (throws when framed or opened cross-origin); `getAssertion({challenge, rpId, allowCredentials}): Promise<RawAssertion>` (userVerification always 'required'); `TrustedOriginConfig` from env (origin, rpId), no widening defaults.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/trusted-origin/src/**`, `ws05-web-approval-ux/packages/trusted-origin/test/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/trusted-origin/src/{headers.ts,session.ts,csrf.ts,frameGuard.ts,ceremony.ts,config.ts,index.ts}`; `packages/trusted-origin/test/{headers.test.ts,session.test.ts,csrf.test.ts,ceremony.test.ts}`; `packages/trusted-origin/test/e2e/frame.spec.ts`. **Steps:** 1. Header builder with config from env (origin, RP ID), no defaults that widen anything. 2. Session cookie `__Host-` prefix. 3. Framing/opener guard runs before any render. 4. Ceremony helper: `userVerification: 'required'`, no fallback.
- **SECURITY INVARIANTS:** RP ID never widened to the ordinary app; no inline scripts.
- **FAILURE CASES:** CSP with `unsafe-inline`; cookie without `__Host-`.
- **TESTS REQUIRED:** Header unit tests; framing guard test in Playwright with a test page that frames it. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `headers.test.ts::csp_scripts_self_only`; `headers.test.ts::never_unsafe_inline`; `headers.test.ts::frame_ancestors_none`; `headers.test.ts::coop_corp_hsts_present`; `session.test.ts::host_prefixed_secure_httponly_cookie`; `csrf.test.ts::mutation_without_token_rejected`; `ceremony.test.ts::user_verification_always_required`; `ceremony.test.ts::challenge_base64url_no_padding`; `frame.spec.ts::framed_page_refuses_to_render`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** `unsafe-inline` never produced; framed page refuses; mutation without CSRF rejected; `userVerification` always `required`.
- **ACCEPTANCE CRITERIA:** All protections covered by tests.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Pages (T017, T018).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** CSP strictness; RP ID scoping.

### W05-T005-EXTERNAL-AUDIT-PACKET
- **TASK_ID:** W05-T005-EXTERNAL-AUDIT-PACKET
- **OBJECTIVE:** **In plain words:** Write the request pack for the external security audit: what exactly the auditor must check, concrete questions, who counts as independent, and a tracker for their findings. **Precisely:** Produce the narrow external audit request packet and finding tracker so master can commission the audit that gates claim execution (G2). Moved from W01 to balance load: it is documentation only.
- **WHY IT EXISTS:** 00A/58 make the narrow external audit of the repaired contracts the next step; no claim may RUN until it passes (gap G-002).
- **ARCHITECTURE REQUIREMENTS:** 58 "External audit focus" (gateway vs stolen token; stale-success exposure and freeze; display/enrollment/recovery/eligibility; protected evidence writes, expected analysis/oracles; per-call journal, uncertain send, post-restore bootstrap; schema/reference/custody lifetimes); REVIEWS/INTEGRATED_REVIEW_LOG.md; TRACEABILITY/ASTRA_FINDING_RESOLUTION_MATRIX.md.
- **P0 CLAIMS:** none (gate G2 for all 55).
- **INPUT CONTRACTS:** Entire V2.1 package (read-only); identity hashes in COMMON/PROGRAM_README.md.
- **OUTPUT CONTRACTS:** `docs/external-audit/AUDIT_REQUEST.md`, `AUDIT_SCOPE_CHECKLIST.md`, `FINDINGS_TRACKER.md` (finding id, severity, affected contract, CCR id, status), `AUDITOR_INDEPENDENCE.md`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/docs/external-audit/**`
- **READ-ONLY DEPENDENCIES:** `ARCHITECTURE/V2_1/**`, `COMMON/CONTRACT_CHANGE_PROTOCOL.md`.
- **PRECONDITIONS:** W05-T000 INTEGRATED (it owns `docs/**` until then).
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `docs/external-audit/{AUDIT_REQUEST.md,AUDIT_SCOPE_CHECKLIST.md,FINDINGS_TRACKER.md,AUDITOR_INDEPENDENCE.md}`. **Steps:** 1. AUDIT_REQUEST: package identity (ZIP/manifest/registry SHA-256), status lines from 00A/00E, the six focus areas of 58 verbatim, non-goals (no P0 evidence, no implementation), ≥3 concrete counterexample-style questions per area. 2. Scope checklist mapping each area → documents/schemas/vectors/ADRs. 3. Independence criteria: not one of the six workstreams, not an author of V2.1 or its reviews; conflict declaration. 4. Tracker with the rule "every finding → CCR or explicit no-change rationale". 5. Result rule: PASS, PASS_WITH_FINDINGS (all → CCR adjudicated before G2), FAIL.
- **SECURITY INVARIANTS:** No capability claimed as proven; no secrets or staging account identifiers.
- **FAILURE CASES:** Overclaiming readiness; a focus area missing.
- **TESTS REQUIRED:** A3 trace: each of the 6 areas maps to ≥1 doc and ≥1 schema/ADR. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `A3 trace: each of the 6 focus areas maps to ≥1 doc and ≥1 schema/ADR`; `grep -niE 'proven|production-ready|enabled'` finds no capability claim; `identity hashes match PROGRAM_README.md exactly`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Search for words implying PASS/enabled/production-ready about any capability finds none.
- **ACCEPTANCE CRITERIA:** Six areas with questions; identity hashes exact; independence explicit; tracker present.
- **EVIDENCE TO RETURN:** Paths + SHA-256 per file. A1 tells master the packet is ready.
- **OUT-OF-SCOPE:** Contacting or choosing the auditor (master).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Completeness against 58; honesty of status.

### W05-T006-G047-RESOLUTION-ORIGIN-TRIAGE
- **TASK_ID:** W05-T006-G047-RESOLUTION-ORIGIN-TRIAGE
- **OBJECTIVE:** **In plain words:** Decide from the architecture text which website address the resolution screen should run on, with exact quotes; if the architecture doesn't say, write a change request instead of guessing. **Precisely:** Triage gap G-047 (is the Resolution Authority UI hosted on the AA origin or its own origin?) from the architecture text, write the decision note, and draft a CCR if the architecture is genuinely unspecified. Moved from W01 because W05 builds that UI.
- **WHY IT EXISTS:** W05-T030 must know which origin it is building for.
- **ARCHITECTURE REQUIREMENTS:** 56 trusted resolution UI; 52 AA origin rules; 16.
- **P0 CLAIMS:** supports P0-10J.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `docs/G-047-resolution-origin-triage.md`; if needed `coordination/CONTRACT_CHANGE_REQUESTS/CCR-W05-<NNN>.md` (coordination branch, by A1).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/docs/G-047*`
- **READ-ONLY DEPENDENCIES:** `ARCHITECTURE/V2_1/**`.
- **PRECONDITIONS:** W05-T000 INTEGRATED (it owns `docs/**` until then).
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `docs/G-047-resolution-origin-triage.md`. **Steps:** 1. Quote every architecture passage about the resolution UI origin. 2. If they determine the answer, record it with citations. 3. If not, list the options and their security consequences and hand a CCR draft to A1. 4. Never pick the weaker reading.
- **SECURITY INVARIANTS:** No invented requirement.
- **FAILURE CASES:** Deciding without citations.
- **TESTS REQUIRED:** A3 checks every citation. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `A3 opens every cited passage and confirms the quote`; `decision or CCR draft present`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** none.
- **ACCEPTANCE CRITERIA:** Decision with citations, or CCR draft.
- **EVIDENCE TO RETURN:** Note SHA-256.
- **OUT-OF-SCOPE:** Implementing the UI (W05-T030).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Citations exact; weaker reading not chosen.

### W05-T007-TEST-IDP
- **TASK_ID:** W05-T007-TEST-IDP
- **OBJECTIVE:** **In plain words:** Set up a test login server (Keycloak) with fake users and a password-reset flow, which Master uses to prove that a normal account reset never gives approval power. **Precisely:** Provide the test OIDC identity provider (Keycloak container, pinned by digest) with a realm export that supports ordinary account login and ordinary account recovery, for W01's P0-09B harness (W01-T045) and any W05 login flow. Moved from W01 (G-017).
- **WHY IT EXISTS:** P0-09B must prove that ordinary account recovery changes nothing in the approval registry.
- **ARCHITECTURE REQUIREMENTS:** U-006 (prototype test IdP only; production POST_FREEZE); 15; 52.
- **P0 CLAIMS:** supports P0-09B.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `test-idp/compose.yaml`, `test-idp/realm.json`, `test-idp/README.md` (how to start, test users, how to trigger recovery, all values from `.env.local`).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/test-idp/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `test-idp/{compose.yaml,realm.json,README.md,smoke.sh}`. **Steps:** 1. Compose service with a port and admin password from env (no defaults in git). 2. Realm: two synthetic users, password reset/recovery flow enabled with a local mail catcher. Each user carries a stable numeric `provider_actor_id` claim (GitHub-style user ID) that is different from the username and email, so the W01 harness proves identity is keyed on the stable subject, never on a mutable email/login name (15; planned production login is "Sign in with GitHub", PF-A-023). 3. README with exact commands.
- **SECURITY INVARIANTS:** Synthetic users only; labeled TEST_ONLY.
- **FAILURE CASES:** Admin password committed.
- **TESTS REQUIRED:** Script that starts the IdP, logs in user 1, runs recovery, logs in again. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `bash test-idp/smoke.sh` logs in user 1, runs recovery, logs in again; `gitleaks detect` clean; `admin password only read from env`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** gitleaks clean.
- **ACCEPTANCE CRITERIA:** Login and recovery work from a fresh start using only `.env.local`; renaming a user or changing their email leaves `provider_actor_id` and the `sub` claim unchanged.
- **EVIDENCE TO RETURN:** Script output.
- **OUT-OF-SCOPE:** Production IdP.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No secrets; reproducible start.

### W05-T008-WEBAUTHN-CROSSCHECK-FIXTURES
- **TASK_ID:** W05-T008-WEBAUTHN-CROSSCHECK-FIXTURES
- **OBJECTIVE:** **In plain words:** Generate fingerprint/security-key sign-in test data with a TypeScript fake authenticator (valid ones and one-field-broken ones), so the W01 lane's Rust checker is cross-checked by an independent implementation. **Precisely:** Generate WebAuthn registration and assertion fixtures (valid + one-field mutations) from a TypeScript virtual authenticator, so W01's Rust verifier (W01-T009) is cross-checked against an independent implementation. Moved from W01.
- **WHY IT EXISTS:** Two independent implementations catch errors a single author's self-made test vectors miss (36 focused review of crypto parsers).
- **ARCHITECTURE REQUIREMENTS:** ENCODING WebAuthn rules; 52 ceremony checks.
- **P0 CLAIMS:** supports P0-09B, P0-09C.
- **INPUT CONTRACTS:** none (generates bytes).
- **OUTPUT CONTRACTS:** `packages/webauthn-fixtures/fixtures/*.json` (authenticatorData, clientDataJSON, signature, public key, expected result) + generator script + README.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/webauthn-fixtures/**`
- **READ-ONLY DEPENDENCIES:** none. **Independence rule:** do not open `ws01-core-security/crates/mc-webauthn-verify/**`.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/webauthn-fixtures/src/{authenticator.ts,generate.ts,check.ts}`; `packages/webauthn-fixtures/fixtures/*.json`; `packages/webauthn-fixtures/test/fixtures.test.ts`; `packages/webauthn-fixtures/README.md`. **Steps:** 1. Virtual authenticator with a P-256 key (WebCrypto). 2. Valid get/create ceremonies for a fixed test RP ID and origin. 3. One mutation per check (type, challenge, origin, crossOrigin, rpIdHash, UP, UV, signature, counter, attestation format). 4. Expected result per fixture.
- **SECURITY INVARIANTS:** Test keys only; labeled TEST_ONLY.
- **FAILURE CASES:** A mutation that changes two fields.
- **TESTS REQUIRED:** Generator is deterministic given a seed. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `fixtures.test.ts::deterministic_with_seed`; `fixtures.test.ts::valid_fixtures_verify`; `fixtures.test.ts::each_mutation_breaks_exactly_one_check`; `fixtures.test.ts::at_least_ten_mutations`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Each mutated fixture fails verification in the TS checker of this package.
- **ACCEPTANCE CRITERIA:** Valid + ≥10 mutation fixtures; deterministic.
- **EVIDENCE TO RETURN:** Fixture SHA-256s. A1 sends INTERFACE_NOTICE to W01.
- **OUT-OF-SCOPE:** Real authenticators (W05-T019).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Single-field mutations; independence.

### W05-T009-SYNTHETIC-VULNERABLE-REPOS
- **TASK_ID:** W05-T009-SYNTHETIC-VULNERABLE-REPOS
- **OBJECTIVE:** **In plain words:** Write a script that builds small demo Git repos with planted vulnerabilities (SQL injection, leaked key, bad dependency, path traversal) plus a clean one. The W02, W01 and W03 lanes use them. **Precisely:** Build the synthetic demo/test repositories (G-067) with a script: at least five small repositories — clean; SQL injection; hard-coded secret canary; vulnerable dependency; path traversal — each with a base branch and a PR head branch, exported as git bundles plus a JSON index. Used by W02 (analyzers, FAST smoke), W01 (gateway tests), W03 (checkout) . Moved from W02: it is fixture writing, not authority code.
- **WHY IT EXISTS:** Every workstream needs the same realistic, legal, synthetic PRs.
- **ARCHITECTURE REQUIREMENTS:** 26/09 synthetic hostile/vulnerable PRs; 42 synthetic data only.
- **P0 CLAIMS:** supports W02 P0-07A and the FAST smoke.
- **INPUT CONTRACTS:** W06 canary format for the secret canary (placeholder string documented until W06 publishes it).
- **OUTPUT CONTRACTS:** `fixtures/repos/build_repos.py`, `fixtures/repos/bundles/*.bundle`, `fixtures/repos/index.json` (name, base/head OIDs, planted issue, expected finding per analyzer class), `fixtures/repos/README.md`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/fixtures/repos/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `fixtures/repos/{build_repos.py,index.json,README.md}`; `fixtures/repos/bundles/*.bundle`; `fixtures/repos/tests/test_build.py`. **Steps:** 1. Script writes source files from text templates (no third-party code copied), commits base and head with fixed author/committer/date so OIDs are reproducible. 2. Each vulnerable repo plants exactly one issue in the head branch. 3. The vulnerable dependency uses a real advisory ID from OSV in a manifest file only (no download). 4. Export bundles and index with SHA-256 of each bundle.
- **SECURITY INVARIANTS:** Synthetic code only; the secret is a non-functional canary in the W06 format; no real credentials.
- **FAILURE CASES:** Non-reproducible OIDs; a real-looking working secret.
- **TESTS REQUIRED:** Running the script twice yields byte-identical bundles and index. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_build.py::test_reproducible_twice`; `test_build.py::test_five_repos_present`; `test_build.py::test_one_issue_per_vulnerable_repo`; `gitleaks flags only the canary`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** gitleaks flags the canary (detectable) and nothing else.
- **ACCEPTANCE CRITERIA:** ≥5 repos; reproducible; index complete.
- **EVIDENCE TO RETURN:** Index SHA-256; reproducibility output. A1 sends INTERFACE_NOTICE to W01, W02, W03.
- **OUT-OF-SCOPE:** Analyzer runs (W02).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Reproducibility; synthetic only.

### W05-T010-PATH-FIXTURES
- **TASK_ID:** W05-T010-PATH-FIXTURES
- **OBJECTIVE:** **In plain words:** Write a script that creates Git repos with tricky file names (case clashes, symlinks, very long and non-UTF-8 paths) for the W03 lane's per-OS file tests. **Precisely:** generate git bundles containing a case-fold collision (`README` vs `readme`), a symlink and a Windows junction-like entry, a path longer than 260 characters, and a non-UTF-8 byte path, plus `index.json` with each path's raw bytes (hex) and the expected outcome per OS (byte-identical or reject per ENCODING Git path rules). Moved from W03: fixture writing, not sandbox code.
- **WHY IT EXISTS:** P0-02D needs identical tricky-path inputs on every native cell.
- **ARCHITECTURE REQUIREMENTS:** 31; ENCODING Git path rules.
- **P0 CLAIMS:** supports W03 P0-02D.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `fixtures/paths/build_paths.py`, `fixtures/paths/bundles/*.bundle`, `fixtures/paths/index.json`, `fixtures/paths/README.md`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/fixtures/paths/**`
- **READ-ONLY DEPENDENCIES:** ENCODING.md.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `fixtures/paths/{build_paths.py,index.json,README.md}`; `fixtures/paths/bundles/*.bundle`; `fixtures/paths/tests/test_paths.py`. **Steps:** 1. Build repos with `git` plumbing (`hash-object`, `mktree`) so paths are exact bytes, independent of the local filesystem. 2. Fixed author/date for reproducible OIDs. 3. Index with expected per-OS outcomes.
- **SECURITY INVARIANTS:** Synthetic content only.
- **FAILURE CASES:** Paths altered by the generating machine's filesystem.
- **TESTS REQUIRED:** Script run twice → byte-identical bundles and index. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_paths.py::test_reproducible_twice`; `test_paths.py::test_four_path_kinds_present`; `test_paths.py::test_git_fsck_passes`; `test_paths.py::test_paths_exact_bytes`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** `git fsck` on each bundle passes (fixtures are valid Git, only the paths are tricky).
- **ACCEPTANCE CRITERIA:** All four path kinds present; reproducible.
- **EVIDENCE TO RETURN:** Index SHA-256. A1 sends INTERFACE_NOTICE to W03.
- **OUT-OF-SCOPE:** Materialization (W03).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Exact bytes.

### W05-T011-PACK-FIXTURES
- **TASK_ID:** W05-T011-PACK-FIXTURES
- **OBJECTIVE:** **In plain words:** Write a script that creates normal and deliberately broken Git download files (packs) for the W03 lane's checkout safety tests. Big ones are generated on demand, never committed. **Precisely:** generate (a) a 1 MiB repository and (b) a 64 MiB repository (generated on demand, never committed, because the repository guard rejects files over 5 MB), and broken packs: corrupt object, inflated object count, wrong OID, malformed delta chain, hostile filename (`../`, NUL, control characters); `index.json` lists each fixture, its SHA-256 and the expected parser verdict. Moved from W03.
- **WHY IT EXISTS:** P0-05A/B need reproducible good and hostile packs.
- **ARCHITECTURE REQUIREMENTS:** 08 checkout; 22; 42 synthetic data.
- **P0 CLAIMS:** supports W03 P0-05A, P0-05B.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `fixtures/packs/build_packs.py` (`--large` generates the 64 MiB repo into a git-ignored directory), small packs committed, `fixtures/packs/index.json`, `fixtures/packs/README.md`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/fixtures/packs/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `fixtures/packs/{build_packs.py,index.json,README.md}`; `fixtures/packs/small/*`; `fixtures/packs/tests/test_packs.py`. **Steps:** 1. Good repos with deterministic content. 2. Broken packs produced by editing bytes of a good pack at documented offsets. 3. Large repo generated from a fixed seed. 4. Index with SHA-256 of every file (including the generated large one).
- **SECURITY INVARIANTS:** Synthetic content; no committed file over 5 MB.
- **FAILURE CASES:** Large file committed; non-reproducible output.
- **TESTS REQUIRED:** Script run twice → identical SHA-256 values (including `--large`). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_packs.py::test_reproducible_including_large`; `test_packs.py::test_index_pack_rejects_broken`; `test_packs.py::test_index_pack_accepts_good`; `test_packs.py::test_nothing_over_5mb_committed`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** `git index-pack` rejects every broken pack and accepts every good one.
- **ACCEPTANCE CRITERIA:** All fixture kinds present; reproducible; nothing over 5 MB committed.
- **EVIDENCE TO RETURN:** Index SHA-256. A1 sends INTERFACE_NOTICE to W03.
- **OUT-OF-SCOPE:** Parsing (W03).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Each broken pack breaks exactly one thing.

### W05-T012-REGISTRY-AND-ARCHIVE-FIXTURES
- **TASK_ID:** W05-T012-REGISTRY-AND-ARCHIVE-FIXTURES
- **OBJECTIVE:** **In plain words:** Write a script that creates fake npm packages (normal, look-alike, malicious-but-harmless) and lockfiles for the W03 lane's dependency download and install tests. **Precisely:** generate npm tarballs and metadata for a public scope and two private tenant scopes: package `q@1` with a pinned integrity, a same-name/same-version package with different bytes, a dependency-confusion package (private name published publicly), an archive bomb, a path-traversal archive, a package whose install script tries `curl`, one whose install script searches for credentials, plus lockfiles (npm v3) referencing them; `index.json` with SHA-256 and expected outcome per fixture. Moved from W03.
- **WHY IT EXISTS:** P0-05E/F/H need reproducible registry and archive fixtures.
- **ARCHITECTURE REQUIREMENTS:** 55; 42 synthetic data.
- **P0 CLAIMS:** supports W03 P0-05E, P0-05F, P0-05H.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `fixtures/registries/build_registries.py`, `fixtures/registries/{public,tenant-a,tenant-b}/**`, `fixtures/registries/lockfiles/*.json`, `fixtures/registries/index.json`, `fixtures/registries/README.md` (how W03 loads them into the local Verdaccio).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/fixtures/registries/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `fixtures/registries/{build_registries.py,index.json,README.md}`; `fixtures/registries/{public,tenant-a,tenant-b}/**`; `fixtures/registries/lockfiles/*.json`; `fixtures/registries/tests/test_registries.py`. **Steps:** 1. Tarballs built from text templates with fixed mtimes. 2. Malicious scripts are harmless markers (they only try to connect to a local test address or read a canary path and print what they found). 3. Archive bomb kept under 5 MB compressed. 4. Lockfiles with exact integrity values.
- **SECURITY INVARIANTS:** No working malware; scripts only touch local test addresses and canary paths.
- **FAILURE CASES:** A script that contacts the internet.
- **TESTS REQUIRED:** Script run twice → identical SHA-256 values. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `test_registries.py::test_reproducible_twice`; `test_registries.py::test_all_fixture_kinds_present`; `test_registries.py::test_no_public_hostname_in_scripts`; `test_registries.py::test_bomb_under_5mb`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** `grep` shows no public hostname in any script.
- **ACCEPTANCE CRITERIA:** All fixture kinds present; reproducible; harmless.
- **EVIDENCE TO RETURN:** Index SHA-256. A1 sends INTERFACE_NOTICE to W03.
- **OUT-OF-SCOPE:** Registry server (W06-T034).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Harmlessness; reproducibility.

### W05-T013-AA-API-MOCK-SERVER
- **TASK_ID:** W05-T013-AA-API-MOCK-SERVER
- **OBJECTIVE:** **In plain words:** Build a mock server for the approval API from the W01 lane's spec, so the approval website can be built and tested before the W01 lane's real backend exists. **Precisely:** Build the I-07 mock server from W01's OpenAPI (`ws01-core-security/doubles/aa-api/v1/openapi.yaml`, W01-T001) with plan A/B fixtures and a WebAuthn test RP, so the AA origin (W05-T017) is developed end to end before W01's backend exists. Moved from W01.
- **WHY IT EXISTS:** Contract-first: removes W05's wait on W01's AA backend.
- **ARCHITECTURE REQUIREMENTS:** 52; 16 AA origin; INV-11.
- **P0 CLAIMS:** supports P0-09A.
- **INPUT CONTRACTS:** I-07 OpenAPI v1.
- **OUTPUT CONTRACTS:** `mocks/aa-api/` (MSW handlers or a small Node server) + README (TEST_ONLY).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/mocks/aa-api/**`
- **READ-ONLY DEPENDENCIES:** W01 OpenAPI file; `packages/webauthn-harness` (when present).
- **PRECONDITIONS:** W05-T000 INTEGRATED; W01-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `mocks/aa-api/src/{generated.ts,handlers.ts,fixtures/planA.ts,fixtures/planB.ts,server.ts}`; `mocks/aa-api/test/contract.test.ts`; `mocks/aa-api/README.md`. **Steps:** 1. Generate request/response types from the OpenAPI (no hand-written copies). 2. Fixtures plan A and plan B. 3. Error responses for every code in the spec's error enum.
- **SECURITY INVARIANTS:** Mock never accepts display content from the client (same as the spec).
- **FAILURE CASES:** Mock drifts from the spec.
- **TESTS REQUIRED:** Contract test validating every mock response against the OpenAPI. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `contract.test.ts::every_operation_mocked`; `contract.test.ts::every_response_validates_against_openapi`; `contract.test.ts::every_error_code_reachable`; `contract.test.ts::no_display_content_accepted_from_client`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Each error code reachable by a documented fixture.
- **ACCEPTANCE CRITERIA:** 100% of operations mocked; contract test green.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Real backend (W01).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Spec conformance.

### W05-T014-RESOLUTION-API-MOCK-SERVER
- **TASK_ID:** W05-T014-RESOLUTION-API-MOCK-SERVER
- **OBJECTIVE:** **In plain words:** Build a mock server for the resolution API from the W01 lane's spec, so the resolution website can be built before the W01 lane's real backend exists. **Precisely:** Build the I-08 mock server from W01's OpenAPI (`ws01-core-security/doubles/resolution-api/v1/openapi.yaml`, W01-T002) with one case fixture per flow of doc 56. Moved from W01.
- **WHY IT EXISTS:** Contract-first: removes W05-T030's wait on W01's resolution backend.
- **ARCHITECTURE REQUIREMENTS:** 56; INV-22.
- **P0 CLAIMS:** supports P0-10J.
- **INPUT CONTRACTS:** I-08 OpenAPI v1.
- **OUTPUT CONTRACTS:** `mocks/resolution-api/` + README (TEST_ONLY).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/mocks/resolution-api/**`
- **READ-ONLY DEPENDENCIES:** W01 OpenAPI file.
- **PRECONDITIONS:** W05-T000 INTEGRATED; W01-T002 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `mocks/resolution-api/src/{generated.ts,handlers.ts,fixtures/cases.ts,server.ts}`; `mocks/resolution-api/test/contract.test.ts`; `mocks/resolution-api/README.md`. **Steps:** 1. Types generated from the OpenAPI. 2. Seven case fixtures (one per flow). 3. Stale-revision and expired-challenge errors reachable.
- **SECURITY INVARIANTS:** No PASS value anywhere in mock responses.
- **FAILURE CASES:** Mock drifts from the spec.
- **TESTS REQUIRED:** Contract test against the OpenAPI. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `contract.test.ts::every_operation_mocked`; `contract.test.ts::seven_flow_cases`; `contract.test.ts::every_error_code_reachable`; `contract.test.ts::no_pass_value_anywhere`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Every error code reachable.
- **ACCEPTANCE CRITERIA:** All operations and 7 flows mocked; contract test green.
- **EVIDENCE TO RETURN:** Test report.
- **OUT-OF-SCOPE:** Real backend (W01).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Spec conformance; no PASS.

### W05-T015-WIRE-TS-CANONICAL
- **TASK_ID:** W05-T015-WIRE-TS-CANONICAL
- **OBJECTIVE:** **In plain words:** Finish our independent TypeScript record codec: turn each of the 85 record types into exact bytes, hash and check signatures, matching the architecture's test vectors. The W06 lane compares it with the W01 lane's Rust version. **Precisely:** Complete `@mc/wire-ts` (I-10@v1): registry-driven canonical encoding of all 85 contracts on top of the CBOR core (field order and enum codes read from REGISTRY.json, not from Rust), D(T,x) digests, the signature input, Ed25519 verification with a pinned audited library, and the CLI `wire-ts-vectors` that writes per-vector results for W06's parity harness.
- **WHY IT EXISTS:** P0-06A/B compare this independent implementation with W01's.
- **ARCHITECTURE REQUIREMENTS:** ENCODING.md in full; REGISTRY.json; ADR-024; INV-18; TEST_VECTORS README.
- **P0 CLAIMS:** supports P0-06A, P0-06B (W06).
- **INPUT CONTRACTS:** REGISTRY.json, wire-vectors.json, signature-vectors.json.
- **OUTPUT CONTRACTS:** `encodeRecord`, `decodeRecord`, `digest`, `signatureInput`, `verifyDetached`; CLI `wire-ts-vectors --vectors <file> --out <file>` (output = `vector-results.json` exactly per the W06-T003 schema, I-13). **Exact public API (names and signatures are binding; do not rename):** `encodeRecord(typeName, projection): Uint8Array`; `decodeRecord(bytes): {typeName, projection}`; `digest(typeName, bytes): Uint8Array(32)`; `signatureInput(contractType, payloadDigest, signerUuid, keyUuid, generation: bigint): Uint8Array`; `verifyDetached(...)`: Promise<boolean>; CLI `wire-ts-vectors --vectors <file> --out <file>`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/wire-ts/src/{registry,record,digest,signature,cli}/**`, `ws05-web-approval-ux/packages/wire-ts/src/index.ts`, `ws05-web-approval-ux/packages/wire-ts/test/{vectors,signature,properties}/**`
- **READ-ONLY DEPENDENCIES:** REGISTRY.json, vectors, T003 CBOR core. **Forbidden reads:** `ws01-core-security/crates/mc-wire/**`, `check_spec.py` source.
- **PRECONDITIONS:** W05-T003 INTEGRATED; W06-T003 INTEGRATED (vector-results schema).
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/wire-ts/src/{registry,record,digest,signature,cli}/**`; `packages/wire-ts/src/index.ts`; `packages/wire-ts/test/{vectors,signature,properties}/**`. **Steps:** 1. Load REGISTRY.json, verify its SHA-256. 2. Encode `[type_name, 1, fields…]` in registry order; enums as zero-based codes; options; sets sorted by encoded bytes and unique. 3. SHA-256 via WebCrypto/node:crypto; domain strings byte-exact. 4. Ed25519 via `@noble/ed25519`. 5. CLI output stable across runs.
- **SECURITY INVARIANTS:** INV-18; no Number for 64-bit; never normalize malformed input.
- **FAILURE CASES:** Set re-sorted silently on decode; precision loss.
- **TESTS REQUIRED:** 85 ACCEPT exact bytes and digests; 355 REJECT; 6 valid + 6 corrupted signatures; property tests (locale/timezone/key insertion order do not change bytes). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `vectors.test.ts::accept_85_bytes_exact`; `vectors.test.ts::accept_85_sha256_exact`; `vectors.test.ts::reject_355_all_reject`; `signature.test.ts::six_valid_verify`; `signature.test.ts::six_corrupted_reject`; `properties.test.ts::locale_timezone_key_order_invariant`; `properties.test.ts::numeric_u64_json_rejected`; `cli output identical on two runs`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** All REJECT vectors + numeric JSON for u64 + non-NFC strings + uppercase hex.
- **ACCEPTANCE CRITERIA:** 85/85, 355/355, 6/6 + 6/6; CLI output identical on two runs.
- **EVIDENCE TO RETURN:** CLI output SHA-256; test counts. A1 sends INTERFACE_NOTICE I-10@v1 to W06 (IM-1).
- **OUT-OF-SCOPE:** Parity comparison (W06).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Independence; registry-driven order.

### W05-T016-ORDINARY-API-MOCKS
- **TASK_ID:** W05-T016-ORDINARY-API-MOCKS
- **OBJECTIVE:** **In plain words:** Build mock data and a mock API for the normal app's screens (reviews, findings, attempts, graphs, setup, patches, admin, budget) covering normal and failure cases, so every screen can be built in parallel without a backend. **Precisely:** Build `mocks/control-api`: MSW handlers and fixture data for the ordinary Control API read model used by every view in `apps/web` — review subjects, expected manifests with receipts (including zero-finding and missing units), findings, check projections (including EXTERNAL_GATE_UNSAFE and freeze progress), attempts with envelopes and graphs, setup/policy/runner readiness, patch previews, admin, disclosure and budget — all typed with `contracts-ts`, synthetic, and using W02/W04 fixtures where they exist.
- **WHY IT EXISTS:** Every view task builds against the same mock data in parallel, with no backend.
- **ARCHITECTURE REQUIREMENTS:** 16 primary views; 51; 54; 11; 14; 24.
- **P0 CLAIMS:** none (enabler).
- **INPUT CONTRACTS:** `contracts-ts` types; W04 `fixtures/graphs/*.json` (W04-T012) when available.
- **OUTPUT CONTRACTS:** `handlers` export; one fixture module per view area; `README.md` listing every endpoint and fixture scenario.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/mocks/control-api/src/**`, `ws05-web-approval-ux/mocks/control-api/test/**`, `ws05-web-approval-ux/mocks/control-api/README.md`
- **READ-ONLY DEPENDENCIES:** `contracts-ts`; W02/W04 fixtures (read-only).
- **PRECONDITIONS:** W05-T001 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `mocks/control-api/src/{handlers.ts,scenarios.ts,fixtures/**}`; `mocks/control-api/test/fixtures.test.ts`; `mocks/control-api/README.md`. **Steps:** 1. Endpoint list (GET only for read models; mutations only CSRF-protected idempotent stubs). 2. Scenarios: clean pass, missing unit, partial, stale, unknown effect, external gate unsafe, cancelled attempt, budget exhausted, unobservable subagent. 3. Every fixture validated by the generated zod schemas in a test.
- **SECURITY INVARIANTS:** Synthetic data only; hostile strings included on purpose (for escaping tests).
- **FAILURE CASES:** Fixture not matching the contract schema.
- **TESTS REQUIRED:** All fixtures validate; every endpoint returns its scenario. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `fixtures.test.ts::every_fixture_validates_against_contract`; `fixtures.test.ts::every_endpoint_returns_its_scenarios`; `fixtures.test.ts::all_nine_scenarios_present`; `fixtures.test.ts::hostile_strings_present`; `fixtures.test.ts::invalid_fixture_fails_validation`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** A deliberately invalid fixture fails validation.
- **ACCEPTANCE CRITERIA:** Every scenario available to every view.
- **EVIDENCE TO RETURN:** Scenario list.
- **OUT-OF-SCOPE:** AA/resolution APIs (T013/T014).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Schema validity; scenario coverage.

### W05-T017-AA-ORIGIN-SHELL
- **TASK_ID:** W05-T017-AA-ORIGIN-SHELL
- **OBJECTIVE:** **In plain words:** Build the shell of the approval website on its own separate domain: it takes only a plan number from the address, fetches everything else itself, applies the safety kit, and builds identically every time so its fingerprint (hash) can be recorded. **Precisely:** Build the AA-origin app shell (`apps/aa-origin`), served from a **separate registrable domain**: route `/plan/[planId]` that takes only the plan ID from the URL, fetches the display model itself from the AA API (I-07; the W05-T013 mock until W01's backend is ready), applies the trusted-origin kit (headers, session, CSRF, framing guard), loads no third-party script/CDN/embed/service worker, and builds reproducibly so `renderer_bundle_digest` is stable across two clean builds (G-048).
- **WHY IT EXISTS:** ADR-018: the trusted display lives on its own origin; the digest is bound into TrustedDisplayReceiptV1.
- **ARCHITECTURE REQUIREMENTS:** 52 trusted display (origin, CSP, session); TrustedDisplayReceiptV1 (renderer_bundle_digest, display_origin); INV-10, INV-11.
- **P0 CLAIMS:** supports P0-09A.
- **INPUT CONTRACTS:** I-07 (via `mocks/aa-api`).
- **OUTPUT CONTRACTS:** `apps/aa-origin/app/plan/[planId]/{page.tsx,loading.tsx}` (shell only: fetch + state handling), `apps/aa-origin/middleware.ts`, `apps/aa-origin/scripts/bundle-digest.mjs`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/aa-origin/app/plan/**`, `ws05-web-approval-ux/apps/aa-origin/middleware.ts`, `ws05-web-approval-ux/apps/aa-origin/lib/**`, `ws05-web-approval-ux/apps/aa-origin/scripts/**`, `ws05-web-approval-ux/apps/aa-origin/test/shell/**`
- **READ-ONLY DEPENDENCIES:** `trusted-origin`, `state-language`, `mocks/aa-api`.
- **PRECONDITIONS:** W05-T004 INTEGRATED; W05-T013 INTEGRATED; W05-T002 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/aa-origin/app/plan/[planId]/{page.tsx,loading.tsx}`; `apps/aa-origin/middleware.ts`; `apps/aa-origin/lib/{fetchDisplayModel.ts,config.ts}`; `apps/aa-origin/scripts/bundle-digest.mjs`; `apps/aa-origin/test/shell/{headers.spec.ts,url.spec.ts,reproducible.test.ts}`. **Steps:** 1. Middleware applies the kit's headers. 2. Page reads only `planId`; any other query parameter is ignored and logged. 3. Display-model fetch from same origin API; errors map to state-language. 4. Reproducible build: pinned lockfile, `SOURCE_DATE_EPOCH`, deterministic chunk names; script prints SHA-256 of the build output.
- **SECURITY INVARIANTS:** INV-10, INV-11; no content from the URL.
- **FAILURE CASES:** Third-party script; non-reproducible build.
- **TESTS REQUIRED:** Header assertions in Playwright; two clean builds → identical digest. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `headers.spec.ts::all_security_headers_present`; `headers.spec.ts::cross_origin_script_blocked_by_csp`; `headers.spec.ts::framed_load_refused`; `url.spec.ts::only_plan_id_read_from_url`; `url.spec.ts::url_supplied_diff_ignored`; `reproducible.test.ts::two_clean_builds_same_digest`; `url.spec.ts::no_third_party_requests`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** URL-supplied diff/plan digest ignored; framed load refused; cross-origin script blocked by CSP (console violation captured).
- **ACCEPTANCE CRITERIA:** Shell loads the mock plan; protections verified; digest stable.
- **EVIDENCE TO RETURN:** Two-build digest comparison; header report.
- **OUT-OF-SCOPE:** Diff rendering (T029); ceremony (T031).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Origin isolation; reproducibility.

### W05-T018-RESOLUTION-ORIGIN-SHELL
- **TASK_ID:** W05-T018-RESOLUTION-ORIGIN-SHELL
- **OBJECTIVE:** **In plain words:** Build the shell of the resolution website with the same protections as the approval website: it takes only a case number from the address and fetches the case itself. **Precisely:** Build the resolution-origin app shell (`apps/resolution-origin`) on the origin decided in W05-T006 (G-047): route `/case/[caseId]` that takes only the case ID from the URL, fetches the canonical case and evidence itself via I-08 (the W05-T014 mock until W01's backend is ready), with the same trusted-origin protections as the AA origin.
- **WHY IT EXISTS:** 56: resolution decisions need the same trusted display rules as approval.
- **ARCHITECTURE REQUIREMENTS:** 56; 52 origin rules; INV-22.
- **P0 CLAIMS:** supports P0-10J (W01).
- **INPUT CONTRACTS:** I-08 (via `mocks/resolution-api`).
- **OUTPUT CONTRACTS:** `apps/resolution-origin/app/case/[caseId]/{page.tsx,loading.tsx}` (shell), `middleware.ts`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/resolution-origin/app/case/[caseId]/{page.tsx,loading.tsx}`, `ws05-web-approval-ux/apps/resolution-origin/middleware.ts`, `ws05-web-approval-ux/apps/resolution-origin/lib/**`, `ws05-web-approval-ux/apps/resolution-origin/test/shell/**`
- **READ-ONLY DEPENDENCIES:** `trusted-origin`, `state-language`, `mocks/resolution-api`, `docs/G-047-resolution-origin-triage.md`.
- **PRECONDITIONS:** W05-T004 INTEGRATED; W05-T014 INTEGRATED; W05-T006 INTEGRATED; W05-T002 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/resolution-origin/app/case/[caseId]/{page.tsx,loading.tsx}`; `apps/resolution-origin/middleware.ts`; `apps/resolution-origin/lib/{fetchCase.ts,config.ts}`; `apps/resolution-origin/test/shell/{headers.spec.ts,url.spec.ts}`. **Steps:** 1. Middleware with the kit. 2. Case fetch; revision kept for later CAS. 3. Errors → state-language.
- **SECURITY INVARIANTS:** INV-22; no case content from the URL.
- **FAILURE CASES:** Case content accepted from the URL.
- **TESTS REQUIRED:** Header assertions; case loads from the mock. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `headers.spec.ts::all_security_headers_present`; `headers.spec.ts::framed_load_refused`; `url.spec.ts::only_case_id_read_from_url`; `url.spec.ts::every_mock_case_loads`; `url.spec.ts::case_revision_kept`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** URL-supplied case content ignored; framed load refused.
- **ACCEPTANCE CRITERIA:** Shell loads every mock case; protections verified.
- **EVIDENCE TO RETURN:** Header report.
- **OUT-OF-SCOPE:** Flow screens (T030).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Same protections as AA.

### W05-T019-WEBAUTHN-HARNESS-CI
- **TASK_ID:** W05-T019-WEBAUTHN-HARNESS-CI
- **OBJECTIVE:** **In plain words:** Build the automated fingerprint/security-key test tool for CI (a virtual authenticator in the browser), proven on its own test page. **Precisely:** Build `packages/webauthn-harness`: Playwright helpers that attach the Chrome DevTools virtual authenticator (UV true or false, resident key, sign-count control) and drive `webauthn.get` and `webauthn.create` ceremonies on a page, capturing clientDataJSON, authenticatorData flags (UP/UV/BE/BS) and counters as public data; plus a self-test page inside the package so the harness is proven before the origins exist.
- **WHY IT EXISTS:** CI needs repeatable ceremonies for P0-09A and to feed W01's P0-09B/C work; A-004 (G-027).
- **ARCHITECTURE REQUIREMENTS:** 52 (UV required, counter/backup flags, cross-origin denied); ENCODING WebAuthn rules; INV-10.
- **P0 CLAIMS:** supports P0-09A, P0-09B, P0-09C.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `withVirtualAuthenticator(page, opts)`, `runGet(page, …)`, `runCreate(page, …)`, `captureCeremony()`. **Exact public API (names and signatures are binding; do not rename):** `withVirtualAuthenticator(page, {uv, residentKey, signCount})`; `runGet(page, opts): Promise<CeremonyCapture>`; `runCreate(page, opts): Promise<CeremonyCapture>`; `CeremonyCapture { clientDataJSON, authenticatorData, flags:{up,uv,be,bs}, signCount, attestationFmt? }`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/packages/webauthn-harness/src/**`, `ws05-web-approval-ux/packages/webauthn-harness/test/**`, `ws05-web-approval-ux/packages/webauthn-harness/selftest/**`
- **READ-ONLY DEPENDENCIES:** `trusted-origin` (ceremony helper).
- **PRECONDITIONS:** W05-T004 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `packages/webauthn-harness/src/{virtualAuthenticator.ts,ceremonies.ts,capture.ts,index.ts}`; `packages/webauthn-harness/selftest/{index.html,server.ts}`; `packages/webauthn-harness/test/selftest.spec.ts`. **Steps:** 1. CDP WebAuthn domain wrapper. 2. Capture public ceremony data only. 3. Self-test page served by Playwright's test server.
- **SECURITY INVARIANTS:** No private key material leaves the virtual authenticator; captures are public data only.
- **FAILURE CASES:** UV=false silently treated as UV.
- **TESTS REQUIRED:** Self-test get/create with UV true. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `selftest.spec.ts::get_with_uv_true`; `selftest.spec.ts::create_with_uv_true`; `selftest.spec.ts::uv_false_reported_in_capture`; `selftest.spec.ts::wrong_origin_cannot_use_credential`; `selftest.spec.ts::capture_contains_no_private_material`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** UV false surfaces `uv:false` in the capture; wrong-origin page cannot use the credential.
- **ACCEPTANCE CRITERIA:** Ceremonies reproducible in CI.
- **EVIDENCE TO RETURN:** CI log.
- **OUT-OF-SCOPE:** Physical authenticators (T032).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Honest flag capture.

### W05-T020-REVIEW-COVERAGE-VIEW
- **TASK_ID:** W05-T020-REVIEW-COVERAGE-VIEW
- **OBJECTIVE:** **In plain words:** Build the review coverage screen: every required scan appears as a row with its result; a missing scan is shown loudly and the page is never shown as passed. **Precisely:** Build `/review/[id]/coverage` in `apps/web`: exact head/base/candidate identities and currentness, and the expected-manifest coverage table with one row per mandatory unit — status from its receipt, "MISSING — no receipt" when absent, and the finding count shown even when it is 0 — with a non-pass banner whenever any mandatory unit is missing or partial.
- **WHY IT EXISTS:** 51/INV-03 made visible: zero findings is only meaningful with a receipt.
- **ARCHITECTURE REQUIREMENTS:** 16; 51; 30 coordinates; INV-03.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** MergeSubjectV2, ExpectedAnalysisManifestV1, AnalysisCompletionReceiptV1 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/review/coverage/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/review/[id]/coverage/**`, `ws05-web-approval-ux/apps/web/components/review/coverage/**`, `ws05-web-approval-ux/apps/web/test/review-coverage/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/review/[id]/coverage/{page.tsx,loading.tsx}`; `apps/web/components/review/coverage/{IdentityHeader.tsx,CoverageTable.tsx,index.ts}`; `apps/web/test/review-coverage/coverage.spec.ts`. **Steps:** 1. Identity header with full OIDs (copyable, never truncated without a full value available). 2. Table virtualized for large manifests. 3. Banner via state-language.
- **SECURITY INVARIANTS:** Display only; repository strings escaped.
- **FAILURE CASES:** Missing unit hidden.
- **TESTS REQUIRED:** Playwright over mock scenarios; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `coverage.spec.ts::every_mandatory_unit_has_a_row`; `coverage.spec.ts::zero_findings_shown_as_0`; `coverage.spec.ts::missing_unit_shows_non_pass_banner`; `coverage.spec.ts::full_oids_available`; `coverage.spec.ts::hostile_unit_name_rendered_as_text`; `coverage.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Missing-unit scenario shows the non-pass banner; hostile unit name renders as text.
- **ACCEPTANCE CRITERIA:** Every unit visible; truthful states.
- **EVIDENCE TO RETURN:** Screenshots; test report.
- **OUT-OF-SCOPE:** Findings and projection (T021, T022).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Nothing hidden.

### W05-T021-REVIEW-FINDINGS-VIEW
- **TASK_ID:** W05-T021-REVIEW-FINDINGS-VIEW
- **OBJECTIVE:** **In plain words:** Build the findings screen: each finding with its evidence and trust level, new or old, with filters; nothing from the repository is ever run as code in the page. **Precisely:** Build `/review/[id]/findings` in `apps/web`: findings with evidence class (C0–C4 / E0–E4), oracle class, novelty (NEW / PRE_EXISTING / UNKNOWN) and unsupported/partial reasons, with filters, and every repository-controlled string rendered as escaped text.
- **WHY IT EXISTS:** 16 primary views; 13 classes.
- **ARCHITECTURE REQUIREMENTS:** 16; 13; 09 novelty.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** FindingCandidateV2, VerificationSetV2 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/review/findings/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/review/[id]/findings/**`, `ws05-web-approval-ux/apps/web/components/review/findings/**`, `ws05-web-approval-ux/apps/web/test/review-findings/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/review/[id]/findings/{page.tsx,loading.tsx}`; `apps/web/components/review/findings/{FindingsList.tsx,Filters.tsx,ClassBadge.tsx,index.ts}`; `apps/web/test/review-findings/findings.spec.ts`. **Steps:** 1. List with class badges and filters. 2. UNKNOWN novelty shown as its own state, never folded into PRE_EXISTING. 3. Code snippets as escaped preformatted text.
- **SECURITY INVARIANTS:** No HTML/Markdown from findings rendered.
- **FAILURE CASES:** `<script>` in finding text executing.
- **TESTS REQUIRED:** Playwright filters; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `findings.spec.ts::class_and_oracle_badges_shown`; `findings.spec.ts::filters_work`; `findings.spec.ts::unknown_novelty_not_shown_as_pre_existing`; `findings.spec.ts::script_tag_inert`; `findings.spec.ts::img_onerror_inert`; `findings.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Hostile `<script>` and `<img onerror>` in finding text are inert; UNKNOWN novelty not displayed as PRE_EXISTING.
- **ACCEPTANCE CRITERIA:** All classes visible; escaping proven.
- **EVIDENCE TO RETURN:** Screenshots; XSS test output.
- **OUT-OF-SCOPE:** Coverage and projection.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Escaping.

### W05-T022-REVIEW-PROJECTION-PANEL
- **TASK_ID:** W05-T022-REVIEW-PROJECTION-PANEL
- **OBJECTIVE:** **In plain words:** Build the GitHub check screen: what GitHub currently shows versus our real verdict, withdrawal and freeze progress, and a loud warning while GitHub may still show an old success. **Precisely:** Build `/review/[id]/projection` in `apps/web`: the GitHub check projection — EvaluationIdentityV2, projection generation, last confirmed provider state, withdrawal and freeze progress, whether it is current — with EXTERNAL_GATE_UNSAFE shown as a page-level banner while the provider may still show a stale success (G-085).
- **WHY IT EXISTS:** 54: the gap between internal truth and GitHub's displayed state must be visible.
- **ARCHITECTURE REQUIREMENTS:** 16; 54 projection lifecycle and freeze; INV-21.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** CheckProjectionV2, EvaluationIdentityV2 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/review/projection/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/review/[id]/projection/**`, `ws05-web-approval-ux/apps/web/components/review/projection/**`, `ws05-web-approval-ux/apps/web/test/review-projection/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/review/[id]/projection/{page.tsx,loading.tsx}`; `apps/web/components/review/projection/{GenerationTimeline.tsx,FreezeProgress.tsx,index.ts}`; `apps/web/test/review-projection/projection.spec.ts`. **Steps:** 1. Generation timeline. 2. Freeze progress (REQUESTED → CONFIRMED/FAILED). 3. Unsafe banner.
- **SECURITY INVARIANTS:** Never show "safe" before provider-confirmed freeze or withdrawal.
- **FAILURE CASES:** Green state while EXTERNAL_GATE_UNSAFE.
- **TESTS REQUIRED:** Playwright over projection scenarios; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `projection.spec.ts::generation_and_provider_state_shown`; `projection.spec.ts::unsafe_banner_when_external_gate_unsafe`; `projection.spec.ts::freeze_requested_not_shown_safe`; `projection.spec.ts::withdrawal_pending_shown`; `projection.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Unsafe scenario shows the banner; freeze REQUESTED is not shown as safe.
- **ACCEPTANCE CRITERIA:** Every projection state truthful.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** Freeze controls (operator-only, not in UI).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No premature "safe".

### W05-T023-ATTEMPT-VIEW
- **TASK_ID:** W05-T023-ATTEMPT-VIEW
- **OBJECTIVE:** **In plain words:** Build the job (attempt) screen: lease, sandbox level, and budget meters with exact numbers. **Precisely:** Build `/attempt/[id]` in `apps/web`: lease and generation, sandbox tier, tool/model cells, resource-envelope consumption as numeric meters (root counters vs ceilings), receipts, and stale/cancelled state.
- **WHY IT EXISTS:** 16 Attempt view.
- **ARCHITECTURE REQUIREMENTS:** 16; ResourceEnvelopeV2; INV-09.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** AttemptBindingV2, ResourceEnvelopeV2 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/attempt/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/attempt/[id]/page.tsx`, `ws05-web-approval-ux/apps/web/app/attempt/[id]/loading.tsx`, `ws05-web-approval-ux/apps/web/components/attempt/**`, `ws05-web-approval-ux/apps/web/test/attempt/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/attempt/[id]/{page.tsx,loading.tsx}`; `apps/web/components/attempt/{LeasePanel.tsx,EnvelopeMeters.tsx,index.ts}`; `apps/web/test/attempt/attempt.spec.ts`. **Steps:** 1. Meters show used / ceiling as numbers plus a bar. 2. Tier shown as text (TEST_ONLY shown prominently). 3. Cancelled/stale banners.
- **SECURITY INVARIANTS:** Display only.
- **FAILURE CASES:** TEST_ONLY tier shown like S2.
- **TESTS REQUIRED:** Playwright; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `attempt.spec.ts::meters_equal_fixture_numbers`; `attempt.spec.ts::test_only_tier_warning`; `attempt.spec.ts::budget_exhausted_non_pass`; `attempt.spec.ts::cancelled_banner`; `attempt.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** TEST_ONLY scenario shows its warning; exhausted budget shows non-pass state.
- **ACCEPTANCE CRITERIA:** Meters equal fixture numbers.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** Graph (T024).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Numeric accuracy.

### W05-T024-GRAPH-VIEW
- **TASK_ID:** W05-T024-GRAPH-VIEW
- **OBJECTIVE:** **In plain words:** Build the agent graph screen: agents and sub-agents as a keyboard-navigable diagram plus a text table; hidden sub-agents are clearly marked and never shown as verified. **Precisely:** Build `/attempt/[id]/graph` in `apps/web`: the agent graph from W04's export format (nodes, broker-created edges, states, envelope slices) as keyboard-navigable SVG plus an equivalent text table, with opaque native subagents labeled UNOBSERVABLE and never shown as verified or independent (G-084).
- **WHY IT EXISTS:** 11 graph honesty.
- **ARCHITECTURE REQUIREMENTS:** 16; 11 agent graph, UNOBSERVABLE topology; INV-09.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** W04 `fixtures/graphs/FORMAT.md` v1 (W04-T012); mocks in the same format until then.
- **OUTPUT CONTRACTS:** route + `components/graph/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/attempt/[id]/graph/**`, `ws05-web-approval-ux/apps/web/components/graph/**`, `ws05-web-approval-ux/apps/web/test/graph/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`, `ws04-agent-model-graph/fixtures/graphs/**`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/attempt/[id]/graph/{page.tsx,loading.tsx}`; `apps/web/components/graph/{GraphSvg.tsx,layout.ts,GraphTable.tsx,index.ts}`; `apps/web/test/graph/graph.spec.ts`. **Steps:** 1. Layout computed in TS (no external graph service). 2. Arrow-key navigation between nodes; focus outline. 3. Text table with the same data.
- **SECURITY INVARIANTS:** Display only.
- **FAILURE CASES:** Graph only visual (screen readers get nothing).
- **TESTS REQUIRED:** Playwright keyboard navigation; axe; text table equals graph data. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `graph.spec.ts::four_w04_fixture_shapes_render`; `graph.spec.ts::keyboard_navigation_between_nodes`; `graph.spec.ts::text_table_matches_graph`; `graph.spec.ts::unobservable_never_marked_verified`; `graph.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** UNOBSERVABLE node never rendered with a verified/independent mark.
- **ACCEPTANCE CRITERIA:** All four W04 fixture shapes render correctly.
- **EVIDENCE TO RETURN:** Screenshots; a11y report.
- **OUT-OF-SCOPE:** Live updates.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Honest independence display.

### W05-T025-SETUP-VIEW
- **TASK_ID:** W05-T025-SETUP-VIEW
- **OBJECTIVE:** **In plain words:** Build the repository setup screen: identity, policy, and whether a runner is ready for each required capability. **Precisely:** Build `/setup` in `apps/web`: repository installation, stable identity, ownership epoch, policy version, and runner/capability readiness per required cell (READY, UNSUPPORTED, MISSING) from the capability rows.
- **WHY IT EXISTS:** 16 repository setup; 47 capability rows.
- **ARCHITECTURE REQUIREMENTS:** 16; 47; 31.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** PolicyVersionV1, CapabilityProfileV1 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/setup/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/setup/**`, `ws05-web-approval-ux/apps/web/components/setup/**`, `ws05-web-approval-ux/apps/web/test/setup/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/setup/{page.tsx,loading.tsx}`; `apps/web/components/setup/{RepoIdentity.tsx,ReadinessTable.tsx,index.ts}`; `apps/web/test/setup/setup.spec.ts`. **Steps:** 1. Identity by stable ID; display name secondary. 2. Readiness table per cell.
- **SECURITY INVARIANTS:** Display only.
- **FAILURE CASES:** UNSUPPORTED cell shown as ready.
- **TESTS REQUIRED:** Playwright; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `setup.spec.ts::stable_id_primary_display_name_secondary`; `setup.spec.ts::unsupported_and_missing_not_ready`; `setup.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** UNSUPPORTED and MISSING cells show non-ready state.
- **ACCEPTANCE CRITERIA:** Readiness truthful.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** Editing setup.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Truthful readiness.

### W05-T026-PATCH-VIEW
- **TASK_ID:** W05-T026-PATCH-VIEW
- **OBJECTIVE:** **In plain words:** Build the patch preview screen, clearly marked untrusted, with a link (plan number only) that sends the user to the separate approval website. This app can never approve anything itself. **Precisely:** Build `/patch/[id]` in `apps/web`: an immutable diff/manifest preview carrying the banner "Untrusted preview — approval happens on the Approval Authority origin", the verification set, effects, and publication/reconciliation states including UNKNOWN and MANUAL_RECONCILIATION, with a "Review and approve" link that is a top-level navigation to the AA origin carrying the plan ID only.
- **WHY IT EXISTS:** 16 Patch view; ADR-018: the ordinary app never approves.
- **ARCHITECTURE REQUIREMENTS:** 16; 14; 50 outcome states; INV-11.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** PatchCustodyV1, PublicationOperationV1, EffectStateV1 projections (mock).
- **OUTPUT CONTRACTS:** route + `components/patch/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/patch/**`, `ws05-web-approval-ux/apps/web/components/patch/**`, `ws05-web-approval-ux/apps/web/test/patch/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/patch/[id]/{page.tsx,loading.tsx}`; `apps/web/components/patch/{Preview.tsx,EffectsList.tsx,ApproveLink.tsx,index.ts}`; `apps/web/test/patch/patch.spec.ts`. **Steps:** 1. Preview rendered as escaped text. 2. Banner always visible. 3. Approve link built from plan ID only (`target="_top"` navigation).
- **SECURITY INVARIANTS:** No approve action in the ordinary app; no iframe of the AA origin.
- **FAILURE CASES:** An approve button that calls an API.
- **TESTS REQUIRED:** Playwright; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `patch.spec.ts::untrusted_banner_always_visible`; `patch.spec.ts::no_element_performs_approval`; `patch.spec.ts::approve_link_has_only_plan_id`; `patch.spec.ts::approve_link_is_top_level_navigation`; `patch.spec.ts::unknown_effect_non_pass`; `patch.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** No element performs approval; link contains only the plan ID; UNKNOWN effect shows non-pass state.
- **ACCEPTANCE CRITERIA:** Authority boundary visible and enforced.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** AA rendering.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Authority boundary.

### W05-T027-ADMIN-VIEW
- **TASK_ID:** W05-T027-ADMIN-VIEW
- **OBJECTIVE:** **In plain words:** Build the admin screen: identities, revocations, retention, audit export, and P0 claim status with progress and result shown separately. **Precisely:** Build `/admin` in `apps/web`: identities, runner/broker generations, revocations, retention, audit export, and P0 capability status read from W06's claim ledger format (NOT_RUN/RUNNING/SUBMITTED/ACCEPTED/REJECTED with outcome PASS/FAIL/INCONCLUSIVE shown separately).
- **WHY IT EXISTS:** 16 Administration.
- **ARCHITECTURE REQUIREMENTS:** 16; 42 claim states; 48 revocation.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** mock admin projections; W06 ledger format (I-13).
- **OUTPUT CONTRACTS:** route + `components/admin/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/admin/**`, `ws05-web-approval-ux/apps/web/components/admin/**`, `ws05-web-approval-ux/apps/web/test/admin/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/admin/{page.tsx,loading.tsx}`; `apps/web/components/admin/{Identities.tsx,Revocations.tsx,ClaimStatus.tsx,index.ts}`; `apps/web/test/admin/admin.spec.ts`. **Steps:** 1. Tables per area. 2. Claim lifecycle and outcome in two separate columns.
- **SECURITY INVARIANTS:** Display only.
- **FAILURE CASES:** SUBMITTED shown as PASS.
- **TESTS REQUIRED:** Playwright; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `admin.spec.ts::lifecycle_and_outcome_separate_columns`; `admin.spec.ts::not_run_and_submitted_never_pass`; `admin.spec.ts::inconclusive_non_pass`; `admin.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** NOT_RUN/SUBMITTED never rendered as PASS; INCONCLUSIVE shown as non-pass.
- **ACCEPTANCE CRITERIA:** Claim status truthful.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** Admin mutations.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Lifecycle vs outcome separation.

### W05-T028-DISCLOSURE-BUDGET-VIEW
- **TASK_ID:** W05-T028-DISCLOSURE-BUDGET-VIEW
- **OBJECTIVE:** **In plain words:** Build the data-location (disclosure) and budget screens: where each artifact was made and kept, deletion status, and budget with uncertain charges shown separately. **Precisely:** Build `/disclosure` and `/budget` in `apps/web`: local-first disclosure (where each artifact was produced, whether source-derived content left the runner, retention, deletion/legal hold, DELETION_PENDING) and budget screens (reservations, settled, held-ambiguous, debt freeze).
- **WHY IT EXISTS:** 16 local-first disclosure and budget; 24; 14.
- **ARCHITECTURE REQUIREMENTS:** 16; 24; 14 custody/residency; 07 ledger.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** mock disclosure and ledger projections.
- **OUTPUT CONTRACTS:** two routes + `components/disclosure/**`, `components/budget/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/web/app/{disclosure,budget}/**`, `ws05-web-approval-ux/apps/web/components/{disclosure,budget}/**`, `ws05-web-approval-ux/apps/web/test/{disclosure,budget}/**`
- **READ-ONLY DEPENDENCIES:** `state-language`, `mocks/control-api`.
- **PRECONDITIONS:** W05-T002 INTEGRATED; W05-T016 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/web/app/{disclosure,budget}/{page.tsx,loading.tsx}`; `apps/web/components/{disclosure,budget}/**`; `apps/web/test/{disclosure,budget}/*.spec.ts`. **Steps:** 1. Disclosure table per artifact. 2. Budget summary with held-ambiguous shown separately from free budget.
- **SECURITY INVARIANTS:** Display only.
- **FAILURE CASES:** Held-ambiguous counted as available.
- **TESTS REQUIRED:** Playwright; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `disclosure.spec.ts::deletion_pending_not_shown_deleted`; `disclosure.spec.ts::source_left_runner_flag_shown`; `budget.spec.ts::held_ambiguous_not_free`; `budget.spec.ts::debt_freeze_shown`; `disclosure.spec.ts::axe_clean`; `budget.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** DELETION_PENDING not shown as deleted; held-ambiguous not shown as free.
- **ACCEPTANCE CRITERIA:** Both screens truthful.
- **EVIDENCE TO RETURN:** Screenshots.
- **OUT-OF-SCOPE:** Deletion actions.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Truthful residency.

### W05-T029-AA-DIFF-RENDERER
- **TASK_ID:** W05-T029-AA-DIFF-RENDERER
- **OBJECTIVE:** **In plain words:** Build the approval website's display: every changed file, mode, symlink, binary download, branch and every effect, shown exactly. If anything can't be shown faithfully, approval is switched off. **Precisely:** Build the trusted display renderer on the AA origin: from the display model fetched by the shell, render every changed file with mode, deletion, executable bit and symlink target; binary objects via a complete artifact download path; destination ref/base; all effects including PR metadata; verification result; oracle limitations; policy; expiry; approver scope — as escaped text only, with no truncated "approve anyway": when faithful rendering is not possible, approval is disabled with the reason.
- **WHY IT EXISTS:** ADR-018/INV-11: the human must see exactly what will be published.
- **ARCHITECTURE REQUIREMENTS:** 52 trusted display; 14; INV-11.
- **P0 CLAIMS:** supports P0-09A.
- **INPUT CONTRACTS:** display model (I-07).
- **OUTPUT CONTRACTS:** `apps/aa-origin/components/display/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/aa-origin/components/display/**`, `ws05-web-approval-ux/apps/aa-origin/test/display/**`, `ws05-web-approval-ux/apps/aa-origin/app/plan/[planId]/page.tsx` (only to mount the renderer; W05-T017 is finished first)
- **READ-ONLY DEPENDENCIES:** shell (T017), `state-language`, `mocks/aa-api`.
- **PRECONDITIONS:** W05-T017 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/aa-origin/components/display/{FileChange.tsx,ModeChange.tsx,Symlink.tsx,BinaryDownload.tsx,Effects.tsx,PlanMeta.tsx,faithful.ts,index.ts}`; `apps/aa-origin/app/plan/[planId]/page.tsx (mount only)`; `apps/aa-origin/test/display/display.spec.ts`. **Steps:** 1. One component per change kind. 2. Large/binary → download link with SHA-256 shown. 3. "Faithful rendering" check returns a reason when it fails; the page then disables approval.
- **SECURITY INVARIANTS:** Never render repository HTML/Markdown; no partial display with an approve option.
- **FAILURE CASES:** Truncated diff with an enabled approve button.
- **TESTS REQUIRED:** Playwright over plans A and B; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `display.spec.ts::plan_a_every_field_rendered`; `display.spec.ts::plan_b_every_field_rendered`; `display.spec.ts::hostile_filename_and_content_inert`; `display.spec.ts::missing_custody_bytes_disables_approval`; `display.spec.ts::oversized_text_offers_download_not_truncation`; `display.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Hostile filenames and contents inert; missing custody bytes → approval disabled; oversized text → download path, not truncation.
- **ACCEPTANCE CRITERIA:** Every display-model field rendered; disable rule proven.
- **EVIDENCE TO RETURN:** Screenshots of plans A/B.
- **OUT-OF-SCOPE:** Ceremony (T031).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Completeness; escaping.

### W05-T030-RESOLUTION-FLOWS-UI
- **TASK_ID:** W05-T030-RESOLUTION-FLOWS-UI
- **OBJECTIVE:** **In plain words:** Build the seven resolution screens (for example "false positive" or "unknown if the call ran"), each with a fresh fingerprint/key check and a 5-minute timer. There is never a "mark as passed" button. **Precisely:** Build the seven resolution screens on the resolution origin (oracle approval, disputed finding, false-positive exclusion, unsupported C3, UNKNOWN effect positive result, non-execution claim, unresolved closure): exact scope and evidence links, the proposed action submitted with the case revision, a fresh UV ceremony through the trusted-origin helper, a 5-minute challenge countdown, accepted risk shown as non-pass, and no control anywhere that produces a PASS.
- **WHY IT EXISTS:** 56; ADR-027 / INV-22.
- **ARCHITECTURE REQUIREMENTS:** 56; 13 maintainer-oracle protocol; 16; INV-22.
- **P0 CLAIMS:** supports P0-10J (W01).
- **INPUT CONTRACTS:** I-08 (mock); ManualCaseStateV1, ManualResolutionActionV1, ResolutionChallengeV1 projections.
- **OUTPUT CONTRACTS:** `apps/resolution-origin/components/flows/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/resolution-origin/components/flows/**`, `ws05-web-approval-ux/apps/resolution-origin/test/flows/**`, `ws05-web-approval-ux/apps/resolution-origin/app/case/[caseId]/page.tsx` (only to mount the flows; W05-T018 is finished first)
- **READ-ONLY DEPENDENCIES:** shell (T018), `trusted-origin`, `state-language`, `mocks/resolution-api`, `webauthn-harness`.
- **PRECONDITIONS:** W05-T018 INTEGRATED; W05-T019 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/resolution-origin/components/flows/{ProvideOracle.tsx,DisputedFinding.tsx,FalsePositive.tsx,UnsupportedC3.tsx,UnknownPositive.tsx,NonExecution.tsx,Unresolvable.tsx,Countdown.tsx,index.ts}`; `apps/resolution-origin/app/case/[caseId]/page.tsx (mount only)`; `apps/resolution-origin/test/flows/flows.spec.ts`. **Steps:** 1. One component per flow. 2. Submit includes the case revision; stale revision shows an error. 3. Ceremony + countdown.
- **SECURITY INVARIANTS:** INV-22; ordinary app cannot create a resolution.
- **FAILURE CASES:** UI copy implying PASS after accepted risk.
- **TESTS REQUIRED:** Playwright per flow with the virtual authenticator; axe. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `flows.spec.ts::provide_oracle`; `flows.spec.ts::disputed_finding`; `flows.spec.ts::false_positive`; `flows.spec.ts::unsupported_c3`; `flows.spec.ts::unknown_positive`; `flows.spec.ts::non_execution`; `flows.spec.ts::unresolvable`; `flows.spec.ts::no_pass_control_in_dom`; `flows.spec.ts::stale_revision_error`; `flows.spec.ts::expired_challenge_error`; `flows.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** No PASS-equivalent control (DOM search); stale revision error; expired challenge error.
- **ACCEPTANCE CRITERIA:** All 7 flows render exact scope; no PASS path.
- **EVIDENCE TO RETURN:** Screenshots per flow.
- **OUT-OF-SCOPE:** Backend.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** No PASS laundering.

### W05-T031-AA-CEREMONY
- **TASK_ID:** W05-T031-AA-CEREMONY
- **OBJECTIVE:** **In plain words:** Build the approval step on the approval website: ask for a challenge, have the user approve with fingerprint/security key, send it, and show the exact result or error. **Precisely:** Complete the AA origin approval flow: request the challenge for the displayed plan, perform `navigator.credentials.get` with UV required through the trusted-origin helper, send the raw assertion to `finish`, and show the result (approved / every I-07 error code) via state-language; refuse cross-origin and embedded contexts; approval button enabled only when the renderer reported faithful rendering.
- **WHY IT EXISTS:** ADR-018: only the AA origin collects the approval ceremony after its own display.
- **ARCHITECTURE REQUIREMENTS:** 52 ceremony; ENCODING WebAuthn; INV-10, INV-11.
- **P0 CLAIMS:** supports P0-09A.
- **INPUT CONTRACTS:** I-07 challenge/finish (mock).
- **OUTPUT CONTRACTS:** `apps/aa-origin/components/ceremony/**`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/apps/aa-origin/components/ceremony/**`, `ws05-web-approval-ux/apps/aa-origin/test/ceremony/**`, `ws05-web-approval-ux/apps/aa-origin/app/plan/[planId]/page.tsx` (only to mount the ceremony; W05-T029 is finished first)
- **READ-ONLY DEPENDENCIES:** shell, renderer, `trusted-origin`, `webauthn-harness`, `mocks/aa-api`.
- **PRECONDITIONS:** W05-T029 INTEGRATED; W05-T019 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `apps/aa-origin/components/ceremony/{ApproveButton.tsx,useApproval.ts,ResultView.tsx,index.ts}`; `apps/aa-origin/app/plan/[planId]/page.tsx (mount only)`; `apps/aa-origin/test/ceremony/ceremony.spec.ts`. **Steps:** 1. Challenge request bound to the plan ID. 2. Ceremony via helper. 3. Result mapping for every error code of the I-07 spec.
- **SECURITY INVARIANTS:** UV required; no fallback ceremony.
- **FAILURE CASES:** Approve enabled while rendering unfaithful.
- **TESTS REQUIRED:** Playwright with the virtual authenticator: approve flow end to end on the mock. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `ceremony.spec.ts::approve_end_to_end_on_mock`; `ceremony.spec.ts::uv_false_shows_error`; `ceremony.spec.ts::framed_page_refuses`; `ceremony.spec.ts::every_i07_error_code_displayed`; `ceremony.spec.ts::approve_disabled_when_unfaithful`; `ceremony.spec.ts::axe_clean`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** UV false → error shown; framed page refuses; each I-07 error code displayed; approve disabled for unfaithful plan.
- **ACCEPTANCE CRITERIA:** End-to-end approval works on the mock; all errors visible.
- **EVIDENCE TO RETURN:** Test report. A1 sends INTERFACE_NOTICE (aa-origin ready, IM-4 W05 side) to W01.
- **OUT-OF-SCOPE:** Backend verification (W01).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Ceremony gating.

### W05-T032-AUTHENTICATOR-MATRIX
- **TASK_ID:** W05-T032-AUTHENTICATOR-MATRIX
- **OBJECTIVE:** **In plain words:** Test the approval on real devices (Windows Hello on any Windows PC or the Azure Windows VM, Mac Touch ID, any security keys) in each browser, and write down exactly which combinations work. **Precisely:** Run the approval and enrollment ceremonies on real devices — Windows Hello on any Windows PC (Windows Home is fine for Windows Hello; without one, the Azure Windows VM with a virtual TPM for PIN-only, recorded as such), Touch ID on the Mac, and any physical FIDO2 keys you own — across Chrome, Edge, Firefox and Safari where available, capture the public ceremony data (flags UP/UV/BE/BS, counters, attestation format), and write `docs/authenticator-matrix.md`: exactly which browser × authenticator cells are supported (tested) and which are not (G-027, G-066).
- **WHY IT EXISTS:** A-004: the supported authenticator list is exactly what was tested.
- **ARCHITECTURE REQUIREMENTS:** 52 (UV, counters, backup flags); INV-10.
- **P0 CLAIMS:** supports P0-09A, P0-09B, P0-09C.
- **INPUT CONTRACTS:** AA origin on the mock.
- **OUTPUT CONTRACTS:** `docs/authenticator-matrix.md`; `packages/webauthn-harness/manual/RUNBOOK.md`; captures `packages/webauthn-harness/manual/captures/*.json`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/docs/authenticator-matrix.md`, `ws05-web-approval-ux/packages/webauthn-harness/manual/**`
- **READ-ONLY DEPENDENCIES:** AA origin, harness.
- **PRECONDITIONS:** W05-T031 INTEGRATED; W05-T019 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `docs/authenticator-matrix.md`; `packages/webauthn-harness/manual/RUNBOOK.md`; `packages/webauthn-harness/manual/captures/*.json`. **Steps:** 1. Runbook with exact steps per device. 2. Human runs; A2 records captures. 3. Matrix with SUPPORTED/UNSUPPORTED and reason per cell.
- **SECURITY INVARIANTS:** Captures contain public ceremony data only.
- **FAILURE CASES:** A cell marked supported without a capture.
- **TESTS REQUIRED:** Each SUPPORTED cell has a capture with UV=true. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `every SUPPORTED cell has a capture with uv=true`; `a capture with uv=false makes its cell UNSUPPORTED`; `captures contain public data only (A3 check)`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** A capture with UV=false makes its cell UNSUPPORTED.
- **ACCEPTANCE CRITERIA:** Matrix covers every available device/browser.
- **EVIDENCE TO RETURN:** Matrix SHA-256; captures.
- **OUT-OF-SCOPE:** Buying hardware.
- **BLOCKERS:** Needs the Windows laptop and the Mac (human-run by master).
- **A3 REVIEW FOCUS:** Every SUPPORTED cell evidenced.

### W05-T033-P0-09A-HARNESS
- **TASK_ID:** W05-T033-P0-09A-HARNESS
- **OBJECTIVE:** **In plain words:** Build the test program for claim P0-09A: even if the normal app is hacked, the approval website shows the real plan or refuses, in Chrome, Firefox and Safari engines. **Precisely:** Build the P0-09A harness: plans A and B with distinct diff/repository/ref/tree; the ordinary UI is assumed compromised; inject the UI supplying diff A with plan B's digest, an iframe overlay, a cross-origin script, and replaced object bytes; capture the authority render, canonical diff digest and browser security results across Chromium, Firefox and WebKit.
- **WHY IT EXISTS:** P0-09A: the authority displays canonical B or rejects; it never asserts A for B.
- **ARCHITECTURE REQUIREMENTS:** 52; 16; ADR-018; INV-10, INV-11.
- **P0 CLAIMS:** P0-09A (primary)
- **INPUT CONTRACTS:** AA origin; `mocks/aa-api` (DRY_RUN); W01 AA backend at RUN (IM-4).
- **OUTPUT CONTRACTS:** harness; `run-manifest.draft.json`. **Evidence files the harness writes:** `authority-render-capture.pdf`, `canonical-diff.sha256`, `browser-security-results.json`, `P0-09A/run-manifest.json`, `P0-09A/fixture-digests.json`, `P0-09A/evidence-manifest.sha256`, `P0-09A/independent-review.json` (run-manifest/evidence-manifest/independent-review are finalized only in the RUN task).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/claims/P0-09A/**` (except `harness/package.json` and `harness/tsconfig.json`)
- **READ-ONLY DEPENDENCIES:** AA origin, `webauthn-harness`, mock.
- **PRECONDITIONS:** W05-T031 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `claims/P0-09A/harness/{tests/*.spec.ts,hostile-ui/**,report.ts}`; `claims/P0-09A/{README.md,run-manifest.draft.json,fixture-digests.json}`. **Steps:** 1. Hostile ordinary-page fixture. 2. Injections from reset baselines. 3. Captures per browser. **Claim procedure the harness must automate (verbatim from atomic-prototype-claims.json P0-09A):** (1) Provision exactly the sealed cell and empty evidence directory named P0-09A. (2) Start review A; Authority independently fetches canonical plan/bytes and renders exact digest/effects before fresh UV. (3) Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. (4) Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE.
- **SECURITY INVARIANTS:** INV-10, INV-11.
- **FAILURE CASES:** Harness that only tests Chromium.
- **TESTS REQUIRED:** DRY_RUN on three browser engines. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `scenarios::positive_control`; `scenarios::injection_a_ui_supplies_diff_a_with_plan_b_digest`; `scenarios::injection_b_iframe_overlay`; `scenarios::injection_c_cross_origin_script`; `scenarios::injection_d_replaced_object_bytes`; `run on chromium, firefox and webkit projects`; `report::rerun_is_byte_identical`; `report::every_case_has_expected_and_observed`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** All P0-09A injections. **Claim injections — each one a separate scenario from a reset baseline, keeping control and failure traces:** (a) UI supplies diff A with plan B digest; (b) iframe overlay; (c) cross-origin script; (d) replaced object bytes.
- **ACCEPTANCE CRITERIA:** Authority displays canonical B or rejects in every engine. **Claim expected observations:** Authority displays canonical B or rejects, never asserts approved A for B; framing/origin/content restrictions verified. **Threshold:** Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. **DRY_RUN must show:** positive control succeeds and every injection produces the expected observation.
- **EVIDENCE TO RETURN:** DRY_RUN SHA-256s.
- **OUT-OF-SCOPE:** Claim RUN (IM-4).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Injection realism.

### W05-T034-A11Y-AUTOMATED
- **TASK_ID:** W05-T034-A11Y-AUTOMATED
- **OBJECTIVE:** **In plain words:** Run automated accessibility checks on every screen and scripted keyboard-only walk-throughs of approval and resolution; every problem becomes a fix task. **Precisely:** Run automated accessibility checks (axe via Playwright) across every route of the three apps and every mock scenario, plus keyboard-only scripted walkthroughs of the approval and resolution flows, and produce `a11y/REPORT.md` with zero critical/serious violations or a filed defect per violation (G-036).
- **WHY IT EXISTS:** 02 "Accessible and truthful UX" needs evidence; no claim covers it.
- **ARCHITECTURE REQUIREMENTS:** 16 accessibility and safety; 02.
- **P0 CLAIMS:** none (freeze-dossier evidence).
- **INPUT CONTRACTS:** all routes.
- **OUTPUT CONTRACTS:** `a11y/tests/**`, `a11y/axe/*.json`, `a11y/REPORT.md`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/a11y/**` (except `package.json` and `tsconfig.json`)
- **READ-ONLY DEPENDENCIES:** the three apps.
- **PRECONDITIONS:** W05-T020 INTEGRATED; W05-T021 INTEGRATED; W05-T022 INTEGRATED; W05-T023 INTEGRATED; W05-T024 INTEGRATED; W05-T025 INTEGRATED; W05-T026 INTEGRATED; W05-T027 INTEGRATED; W05-T028 INTEGRATED; W05-T030 INTEGRATED; W05-T031 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `a11y/tests/{routes.spec.ts,keyboard-approval.spec.ts,keyboard-resolution.spec.ts}`; `a11y/axe/*.json`; `a11y/REPORT.md`. **Steps:** 1. Route × scenario matrix. 2. Keyboard-only scripts for approval and resolution. 3. Violations → repair tasks listed in the report for A1.
- **SECURITY INVARIANTS:** Security state always has text, not color only.
- **FAILURE CASES:** Ceremony not keyboard-operable.
- **TESTS REQUIRED:** axe over every route/scenario; keyboard walkthroughs. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `routes.spec.ts::axe_every_route_every_scenario`; `keyboard-approval.spec.ts::approval_keyboard_only`; `keyboard-resolution.spec.ts::resolution_keyboard_only`; `a scratch label removal makes routes.spec.ts fail`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Removing a label in a scratch branch makes the run fail.
- **ACCEPTANCE CRITERIA:** No critical/serious violations, or each one listed as a repair task.
- **EVIDENCE TO RETURN:** Report SHA-256.
- **OUT-OF-SCOPE:** Manual screen-reader runs (T035).
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Route coverage.

### W05-T035-A11Y-MANUAL
- **TASK_ID:** W05-T035-A11Y-MANUAL
- **OBJECTIVE:** **In plain words:** Do manual screen-reader checks (VoiceOver on Mac, NVDA on Windows) for the important flows and record the results. **Precisely:** Run and record manual screen-reader checks — VoiceOver on the Mac and NVDA on any Windows PC or the Azure Windows VM — for the review, patch, approval and resolution flows against a WCAG 2.2 AA checklist (focus order, labels, announcements, reduced motion, contrast), adding the results to `a11y/REPORT.md`.
- **WHY IT EXISTS:** Automated checks miss screen-reader experience; 02 needs it.
- **ARCHITECTURE REQUIREMENTS:** 16; 02.
- **P0 CLAIMS:** none.
- **INPUT CONTRACTS:** the three apps on mocks.
- **OUTPUT CONTRACTS:** `a11y/manual/{voiceover.md,nvda.md}`, report section.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/a11y/manual/**`, `ws05-web-approval-ux/a11y/REPORT.md`
- **READ-ONLY DEPENDENCIES:** the three apps.
- **PRECONDITIONS:** W05-T034 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `a11y/manual/{voiceover.md,nvda.md}`; `a11y/REPORT.md`. **Steps:** 1. Checklist per flow. 2. Human runs it; A2 records results and turns each failure into a listed repair item.
- **SECURITY INVARIANTS:** Security decisions announced in words.
- **FAILURE CASES:** Approval result not announced.
- **TESTS REQUIRED:** Checklist complete for both screen readers. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `voiceover checklist complete`; `nvda checklist complete`; `every failure listed as a repair item`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** none.
- **ACCEPTANCE CRITERIA:** Both checklists complete; failures listed.
- **EVIDENCE TO RETURN:** Checklist SHA-256s.
- **OUT-OF-SCOPE:** Certification.
- **BLOCKERS:** Needs a Mac and a Windows laptop (human-run).
- **A3 REVIEW FOCUS:** Coverage of security-critical flows.

### W05-T036-ESCAPE-CORPUS
- **TASK_ID:** W05-T036-ESCAPE-CORPUS
- **OBJECTIVE:** **In plain words:** Write the escape-test kit: small probe programs that run inside a sandbox and each try one way to break out (reach the host, find a secret, use the network, mount the disk). Every probe also proves it would detect a real escape. **Precisely:** Build the escape regression corpus (G-068): small guest-side probe programs/scripts that each try one way out — mount/device/socket/procfs inspection, nested-virtualization request, cloud metadata IP routes, host-socket bait, host canary credential search, writable shared mount write — and print a JSON verdict (`blocked`/`escaped` + evidence).
- **WHY IT EXISTS:** P0-02A needs the same hostile probes on every backend and OS.
- **ARCHITECTURE REQUIREMENTS:** 10 host protections; 23 threat model; INV-05, INV-08.
- **P0 CLAIMS:** supports P0-02A, P0-02C.
- **INPUT CONTRACTS:** none.
- **OUTPUT CONTRACTS:** `corpus/escape/<probe>/` (source + README + expected verdict), `corpus/escape/index.json`, `corpus/escape/run_all.sh` (inside a guest).
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/corpus/escape/**`
- **READ-ONLY DEPENDENCIES:** none.
- **PRECONDITIONS:** W05-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws05-web-approval-ux/`):** `corpus/escape/<probe>/{probe.sh|probe.rs,README.md,expected.json} for: mount_inspect, device_inspect, host_socket, procfs_inspect, nested_virt, metadata_route, canary_search, shared_mount_write`; `corpus/escape/index.json`; `corpus/escape/run_all.sh`; `corpus/escape/selftest.sh`. **Steps:** 1. One directory per probe, each a single static binary or POSIX shell script (Linux guest first; PowerShell variants for Windows guests listed separately). 2. Each probe has a positive self-test mode (`--selftest`) that proves it would detect an escape when run on an unprotected host. 3. JSON output schema documented.
- **SECURITY INVARIANTS:** Probes never exfiltrate; they only report. Hostile probes are never run on a daily machine: their self-test runs only inside a throwaway container with no mounts and no network (`docker run --rm --network none`); real runs happen on the W03 lane's lab hosts.
- **FAILURE CASES:** A probe that reports "blocked" because it is broken.
- **TESTS REQUIRED:** `--selftest` of every probe on a throwaway container reports `escaped` (proves the probe works). **Exact test functions (each must exist and pass; the RETURN lists them with results):** `bash corpus/escape/selftest.sh` in a throwaway unprotected container reports every probe `escaped` (proves detection works); `each probe prints valid JSON`; `each probe reports `error` (not `blocked`) when its environment is malformed`. Always also: `pnpm -r typecheck`, `pnpm -r lint` (and `python3 -m pytest` for Python fixture scripts).
- **NEGATIVE TESTS:** Malformed environment → probe exits with `error`, never `blocked`.
- **ACCEPTANCE CRITERIA:** ≥8 probes; each self-test proven.
- **EVIDENCE TO RETURN:** Self-test outputs.
- **OUT-OF-SCOPE:** Running against backends (harness tasks).
- **BLOCKERS:** Never run on a daily-use machine.
- **A3 REVIEW FOCUS:** Probes cannot false-negative silently.

---

## RUN tasks (atomic P0 claims; generated from the authoritative claim JSON)

### W05-RUN-P0-09A
- **TASK_ID:** W05-RUN-P0-09A
- **OBJECTIVE:** **In plain words:** Run the finished test program for claim P0-09A ("Trusted display binds approval bytes") on the real staging setup, collect every evidence file, and hand the sealed bundle to two fresh reviewer sessions (each applying another lane's review playbook), who decide if it passed. Allowed only after G2 (external audit passed) and G3 (you authorized this claim family). **Precisely:** Execute atomic claim P0-09A — "Trusted display binds approval bytes" — for its exact sealed capability cell and submit evidence for independent two-lens acceptance (lifecycle NOT_RUN→RUNNING→SUBMITTED→ACCEPTED|REJECTED; outcome PASS|FAIL|INCONCLUSIVE kept separate).
- **WHY IT EXISTS:** Trusted display binds approval bytes; failure cannot be hidden by an umbrella gate result. Family gate P0-09. BLOCKS_FREEZE: CELL (publication).
- **ARCHITECTURE REQUIREMENTS:** 57 §P0-09A; TRACEABILITY/atomic-prototype-claims.json record P0-09A; 19 Prototype Evidence; 42; 52, 16, ADR-018, INV-10, INV-11.
- **P0 CLAIMS:** P0-09A (primary owner W05; status NOT_RUN).
- **INPUT CONTRACTS:** Harness from W05-T033. Topology: Two plans A/B with distinct diff, repository/ref and candidate tree; ordinary UI/API compromised, Authority origin trusted. Fixture: P0-09A fixture as described in the procedure; fixtures committed and content-digested before execution. Configuration: One exact capability cell per execution; sealed run-manifest binds all settings, endpoint policies, limits, identity/permission/ruleset data and fixture digests. No inherited unspecified default is permitted.
- **OUTPUT CONTRACTS:** Evidence artifacts (I-13 formats): `authority-render-capture.pdf`, `canonical-diff.sha256`, `browser-security-results.json`, `P0-09A/run-manifest.json`, `P0-09A/fixture-digests.json`, `P0-09A/evidence-manifest.sha256`, `P0-09A/independent-review.json`; `ws05-web-approval-ux/claims/P0-09A/cells/default/runs/<RUN_ID>/{run-manifest.json,outcome.json,lifecycle.jsonl,fixture-digests.json,reviews/}`; `ws05-web-approval-ux/claims/P0-09A/cells/default/current.json`.
- **EXACT WRITE SCOPE:** `ws05-web-approval-ux/claims/P0-09A/cells/default/**` (one new `runs/<RUN_ID>/` directory per execution plus `current.json`; earlier run directories are never modified); raw evidence only in the private evidence store under `evidence://P0-09A/default/<RUN_ID>/`.
- **READ-ONLY DEPENDENCIES:** W01 AA backend + custody — each pinned at an INTEGRATED SHA recorded in the run-manifest; W06 evidence-kit; ARCHITECTURE/V2_1.
- **PRECONDITIONS:** G2 PASS; G3 signed for P0-09; W05-T033 INTEGRATED; integration milestone(s): IM-4; Later explicit prototype authorization; disposable tenants/accounts and synthetic source only. No prototype execution is authorized by V2.1 architecture repair. Two plans A/B with distinct diff, repository/ref and candidate tree; ordinary UI/API compromised, Authority origin trusted. Lock the exact run-manifest and fixture digests, record DB/provider observation clocks, name independent evidence reviewers and prove the positive control is executable before failure injection.
- **IMPLEMENTATION STEPS:** 1. Confirm G2 PASS and the signed G3 file for family P0-09 (put both SHA-256 in the run-manifest and in the RETURN MEMORY_UPDATES). 2. Allocate a new RUN_ID (`mc-evidence new-run`, UTC timestamp + random suffix) and create `ws05-web-approval-ux/claims/P0-09A/cells/default/runs/<RUN_ID>/`; never reuse or edit an earlier run directory. Seal `ws05-web-approval-ux/claims/P0-09A/cells/default/runs/<RUN_ID>/run-manifest.json` with `mc-evidence seal` (W06 I-13) per the pinning rule; record INTEGRATED SHAs of every participating component; refuse if any locally controlled version is missing. 3. Commit fixtures and `fixture-digests.json`; name both lens reviewers; prove the positive control is executable. 4. Provision exactly the sealed cell and empty evidence directory named P0-09A. 5. Start review A; Authority independently fetches canonical plan/bytes and renders exact digest/effects before fresh UV. 6. Run each semicolon-separated injected case separately from reset baseline; retain both control and failure traces. 7. Evaluate the declared observations and thresholds; reviewer signs evidence manifest and declares PASS, FAIL or INCONCLUSIVE. 8. Immediately after execution write the measured outcome PASS/FAIL/INCONCLUSIVE once to `ws05-web-approval-ux/claims/P0-09A/cells/default/runs/<RUN_ID>/outcome.json` (`mc-evidence outcome`; never edited afterwards, kept through rejection, expiry and invalidation — audit F-15). 9. Upload the complete bundle to the private evidence store under `evidence://P0-09A/default/<RUN_ID>/`; write `evidence-manifest.sha256`; append lifecycle SUBMITTED to `ws05-web-approval-ux/claims/P0-09A/cells/default/runs/<RUN_ID>/lifecycle.jsonl` and point `ws05-web-approval-ux/claims/P0-09A/cells/default/current.json` at this RUN_ID (`mc-evidence set-current`, recording the run it supersedes). 10. Send EVIDENCE_REVIEW_REQUEST to each lens reviewer; on two results append ACCEPTED or REJECTED to `lifecycle.jsonl`. Only ACCEPTED + PASS + current (not expired, not invalidated) enables this exact cell; the ledger derives that, nobody sets it by hand. 11. If FAIL/INCONCLUSIVE: keep the cell DISABLED, file CCR/ADR-reopen note (38 stop rule), never relabel. A rerun is a new RUN_ID; the earlier run stays as sealed.
- **SECURITY INVARIANTS:** Trusted display binds approval bytes; failure cannot be hidden by an umbrella gate result. Disposable accounts and synthetic data only; canaries instead of real secrets; executor never reviews; no NOT_RUN/INCONCLUSIVE relabeled PASS.
- **FAILURE CASES:** UI supplies diff A with plan B digest; iframe overlay; cross-origin script; replaced object bytes.
- **TESTS REQUIRED:** Positive control first; then each injected case separately from a reset baseline, retaining both control and failure traces.
- **NEGATIVE TESTS:** UI supplies diff A with plan B digest; iframe overlay; cross-origin script; replaced object bytes.
- **ACCEPTANCE CRITERIA:** All positive controls succeed and every injected case produces the specified observation within the declared threshold; complete immutable raw evidence independently reviewed for this exact cell. No untested variant inherits PASS. Threshold: Zero unauthorized acceptances, effects or disclosure across all listed cases; every legitimate control succeeds. Expected observations: Authority displays canonical B or rejects, never asserts approved A for B; framing/origin/content restrictions verified.. ACCEPTED only with both lens reviews (SEC, CC).
- **EVIDENCE TO RETURN:** authority-render-capture.pdf, canonical-diff.sha256, browser-security-results.json, P0-09A/run-manifest.json, P0-09A/fixture-digests.json, P0-09A/evidence-manifest.sha256, P0-09A/independent-review.json; RUN_ID; evidence-manifest SHA-256; run-manifest SHA-256; outcome.json; lifecycle.jsonl; current.json. Expiry: max 30 days; invalidated on: Any participating schema/vector revision, local binary/image digest, kernel/hypervisor or policy/trust/permission/auth/ruleset change. Provider semantics or entitlement change, newly relevant security advisory, or failing recurrence. Environment/fixture modification or evidence integrity/reviewer independence failure.
- **OUT-OF-SCOPE:** Any untested variant or other cell (no PASS inheritance); other claims; production implementation; capability enablement in production.
- **BLOCKERS:** G2 (external audit), G3 (execution authorization), IM-4; external: YES — real browsers + authenticators.
- **A3 REVIEW FOCUS:** Evidence lenses: SEC lens → fresh evidence-A3 session using the W01 lane playbook, CC lens → fresh evidence-A3 session using the W06 lane playbook. Fail condition: Any injected case violates the stated observation/threshold, or positive control fails. Missing/inaccessible evidence, unpinned participating component, unobservable required outcome or non-independent review yields INCONCLUSIVE, which disables the cell like FAIL.
