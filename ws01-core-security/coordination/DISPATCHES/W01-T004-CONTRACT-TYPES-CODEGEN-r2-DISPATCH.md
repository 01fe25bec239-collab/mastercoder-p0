FROM: Root A1 (acting W01-A1)
TO: W01-A2[W01-T004-CONTRACT-TYPES-CODEGEN-r2]
MESSAGE_TYPE: REPAIR_DISPATCH
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r2
DISPOSITION: REPAIR_REQUIRED_F2; PREPARED_FOR_FRESH_A2; NOT_LAUNCHED
DATE_UTC: 2026-10-13T22:20:46.166454+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2
  BRANCH: ws01/w01-t004-contract-types-codegen-r2
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA / REPAIR_BASE_SHA / REVIEWED_SHA: 9f40fae0db8fc6f6b9fc20e4b7f56cc8a56e1174
ORIGINAL_PROTECTED_BASE: a217d94ff1ac1262e21cf9b2e9f713666a830a18
START_TREE: df2f6a1223631c729ca0470a74478295557e9b4e
IMPLEMENTATION_SHA: NONE
TREE: NONE
SOLE_PARENT: NONE
ACCEPTED_SHA / MERGED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE by A1
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

ORIGINAL TASK OBJECTIVE (verbatim): **In plain words:** Generate Rust types for all 85 data records defined in the architecture, straight from the registry file, so every lane uses exactly the same names, fields and rules. This is the first thing the other workstreams wait for: do it as early as possible. **Precisely:** Generate the `mc-contracts` crate (I-01@v1): Rust types for all 85 registered contracts (serde JSON projection, registry field order, enum code tables), generated mechanically from `REGISTRY.json` by `tools/codegen`. **This is the program's critical path**: W02, W03 and W04 start their first real task after it. Dispatch it the moment T000 is ACCEPTED.

READ FIRST:
1. Print actual WORK LOCATION; verify the clean existing worktree, branch, exact BASE_SHA and START_TREE before editing. Live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md (all applicable sections, §13.2/14/15), memory.md, task.md full task and this dispatch twice.
2. Read /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-a-A3_REVIEW.md in full; report SHA256 2241c21ed4df2483622632aaff75dca096209e736d64e2f05a7bca06305f29a7. Findings below are copied verbatim, not permission to widen scope. Preserve all earlier attempts/returns/reviews and their ignored evidence.
3. Read original dispatch/RETURN, current review request, task architecture and frozen inputs. Consult /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/REVIEW_REPAIR_PROCESSING_20261014/review-verification.json for A1's binding/scope/signature/hash verification. Full-history scans remain individually classified and nonzero (this review102 matches); candidate/new-range/staged scans must remain clean. Counts can advance; retain earlier counts honestly.
4. Fresh A2 must not be an earlier implementation or review session/helper. A later A3 must be new, exclude all A2 teams and earlier A3s/helpers. Do not use W05 wire-ts sources for W01 codec work. Root remains A1.

AUTHORIZED PRODUCT WRITE SET — repair delta only:
- ws01-core-security/crates/mc-contracts/tests/reject_vectors.rs
Only coordination exception: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-A2_RETURN.md. A1 alone edits task/memory/dispatch/review records.

EXACT FINDING TEXT (verbatim):
F-2 | BLOCKING | LOW (test quality; no observed product acceptance bypass) | crates/mc-contracts/tests/reject_vectors.rs:304–309, :370–374, :418–422 | Required persistent rejection-reason assertions are missing. projection_level_rejects_all_reject checks is_err while printing vector reason; null sweep and revision-kind checks do likewise. SKILLS.md §6.2 says “each asserting the specific rejection reason, not just 'error'”; §15 requires meaningful reason assertions for task negatives. Concrete scenario: ScopeV1-extra-field rejected by an erroneous revision error, or wire_revision="1" rejected by an unrelated missing-field error, would still satisfy these assertions, even though it does not exercise the intended boundary. Fresh independent_reasons.rs verifies the correct current reasons for all261fixtures/967nulls/bothrevision paths, but it exists only in reviewer scratch and is absent from the persistent product suite. Required minimal repair: add explicit expected reason matching per vector reason category, field/header-specific null reason matching, and invalid-type/u16 reason checks for malformed revision kinds. Preserve positive controls/counts/both direct paths; no guard/vector/schema changes needed.


REPAIR OBJECTIVE: Add persistent specific-reason assertions to the projection-rejection fixtures, required-field null sweep and malformed revision-kind checks. F-1 is CLOSED_LOCALLY, not reopened: preserve all85 headers/197 literals/41 tables and the direct/nested/dispatcher string-only guards.

REPAIR REQUIREMENTS:
- Check each projection REJECT's actual error against its intended reason category and the affected field/header when relevant; printing the vector reason next to is_err is insufficient. Use the frozen input categories without changing/normalizing vector data or defining new contract behavior.
- Add field/header-specific null reason assertions and invalid-type/u16 checks for malformed revision kinds. Preserve both from_value/from_str paths, all positive controls and coverage/counts. An unrelated earlier rejection must not satisfy the test.
- Demonstrate the assertions detect an isolated wrong-reason substitution/control that still rejects, so the new test distinguishes intended validation from generic failure. Keep scratch mutations outside original worktrees; no production error/guard changes, new dependencies or frozen input edits.
- Retain all original F-1 tests and meaningful original guard mutations; reproduce85 ACCEPT/261 projection REJECT coverage.94 CBOR-only cases remain NOT_EXECUTED/outside scope.

EXCLUDED WRITES: generator, generated.rs, primitives.rs/lib.rs, enum_codes.rs/other tests, schemas/vectors, manifests/lockfiles/build.rs, docs/workflows/root/foreign lanes. This repair edits only the single allowed test file; report any proven need outside it.

EXACT LOCAL REQUIRED COMMANDS (through wrapper; at final signed candidate):
- cargo metadata --locked --format-version 1 (41 members and pins unchanged)
- cargo fmt --all --check
- cargo test -p mc-contracts --locked
- cargo run -p codegen --locked -- --check
- cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
- cargo check --workspace --locked
- cargo test --workspace --locked
- cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
- Full byte-identical regeneration, current/stale/missing/modified-registry/schema read-only freshness controls; F-1 direct/nested controls, existing five guard mutations and new wrong-reason detection.
- Root python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (p0_executed:false, not codec/runtime proof)
- Scope both repair-base and original-protected-base ranges, diff/hardcoding/new-range/staged scans and exact signature/parent/tree/clean status. Classify every actual inherited history finding; do not suppress or claim history clean.
Pure JSON/Cargo work needs no credential-bearing env or new DB/runtime host setup. Preserve the original pure-task applicability and required commands; no unrelated service evidence is invented.

VERBATIM ORIGINAL TASK SPECIFICATION (all fields unchanged):
### W01-T004-CONTRACT-TYPES-CODEGEN
- **TASK_ID:** W01-T004-CONTRACT-TYPES-CODEGEN
- **OBJECTIVE:** **In plain words:** Generate Rust types for all 85 data records defined in the architecture, straight from the registry file, so every lane uses exactly the same names, fields and rules. This is the first thing the other workstreams wait for: do it as early as possible. **Precisely:** Generate the `mc-contracts` crate (I-01@v1): Rust types for all 85 registered contracts (serde JSON projection, registry field order, enum code tables), generated mechanically from `REGISTRY.json` by `tools/codegen`. **This is the program's critical path**: W02, W03 and W04 start their first real task after it. Dispatch it the moment T000 is ACCEPTED.
- **WHY IT EXISTS:** Every workstream needs the same canonical names, order and enums without local variants (ADR-024; Shared Contract Rule).
- **ARCHITECTURE REQUIREMENTS:** SCHEMAS/REGISTRY.json (exact names, order, primitives, enums, options/sets); ENCODING.md (u64/i64 as decimal strings in JSON, UUID lowercase hyphenated, bytes lowercase hex, option = 0/1-element array, sets sorted); 49 glossary (no aliases).
- **P0 CLAIMS:** supports P0-06A/B/C/D.
- **INPUT CONTRACTS:** REGISTRY.json (REGISTRY_SHA256 b516004c…); `*.schema.json`.
- **OUTPUT CONTRACTS:** crate `mc-contracts`: `src/primitives.rs` (hand-written), `src/generated.rs` (generated, committed), one struct per contract, one enum per registered enum, `const TYPE_NAME`, `const FIELD_ORDER: &[&str]`, enum `code()`/`from_code()` = zero-based registry position, `REGISTRY_SHA256` constant checked at build time. **Exact public API (names and signatures are binding; do not rename):** `pub struct U64Str(pub u64)`, `pub struct I64Str(pub i64)`, `pub struct Hex<const N: usize>(pub [u8; N])`, `pub struct Bytes(pub Vec<u8>)`, `pub struct GitPath(pub Vec<u8>)`, `pub struct Opt<T>(pub Option<T>)`, `pub struct SetOf<T>(Vec<T>)` (constructor `SetOf::new(Vec<T>) -> Result<Self, SetError>` sorts? NO: rejects unsorted/duplicates), each with Serialize/Deserialize per ENCODING.md; per contract `pub struct <Name> { … }` + `impl <Name> { pub const TYPE_NAME: &'static str; pub const FIELD_ORDER: &'static [&'static str]; }`; per enum `pub enum <Enum> { … }` + `fn code(&self) -> u64` + `fn from_code(u64) -> Option<Self>`; `pub const REGISTRY_SHA256: &str`; codegen CLI `codegen --registry <path> --out <path> [--check]`.
- **EXACT WRITE SCOPE:** `ws01-core-security/crates/mc-contracts/src/**`, `ws01-core-security/crates/mc-contracts/tests/**`, `ws01-core-security/tools/codegen/src/**`
- **READ-ONLY DEPENDENCIES:** `ARCHITECTURE/V2_1/SCHEMAS/**`, `ARCHITECTURE/V2_1/TEST_VECTORS/wire-vectors.json`.
- **PRECONDITIONS:** W01-T000 INTEGRATED.
- **IMPLEMENTATION STEPS:** **Files to create/edit (paths under `ws01-core-security/`):** `crates/mc-contracts/src/lib.rs (owned by this task: `pub mod primitives; pub mod generated; pub use generated::*;`)`; `crates/mc-contracts/src/primitives.rs`; `crates/mc-contracts/src/generated.rs (generated)`; `crates/mc-contracts/build.rs or tests/registry_hash.rs`; `crates/mc-contracts/tests/{primitives.rs,accept_vectors.rs,reject_vectors.rs,registry_hash.rs}`; `tools/codegen/src/main.rs`. **Steps:** 1. `primitives.rs`: `U64Str`/`I64Str` (string (de)serialization; reject leading zeros, plus, exponent, overflow, JSON numbers), `Uuid` lowercase-hyphenated only, `Hex<N>` exact-length lowercase hex (digest/bytes32/bytes64), `Bytes` variable hex, `GitPath`, `Opt<T>` as `[]`/`[x]`, `SetOf<T>` validated sorted+unique on deserialize. 2. `tools/codegen`: read REGISTRY.json, emit `generated.rs` with `#[serde(deny_unknown_fields)]` on every struct; `type` and `wire_revision` validated equal to the registered name and 1; no `Default` derives. 3. `build.rs` or a test asserts SHA-256(REGISTRY.json) == REGISTRY_SHA256. 4. CI freshness test: running codegen produces byte-identical `generated.rs`. 5. Vector tests: every ACCEPT projection deserializes and re-serializes to an equal JSON value; every projection-level REJECT vector rejects.
- **SECURITY INVARIANTS:** INV-18; no aliases (49); no defaults for missing fields; no JSON Number for u64/i64.
- **FAILURE CASES:** Silent default on a missing field; numeric JSON accepted for u64; enum order drift; alias type names.
- **TESTS REQUIRED:** `cargo test -p mc-contracts`; `cargo run -p codegen -- --check`; clippy/fmt clean. **Exact test functions (each must exist and pass; the RETURN lists them with results):** `primitives::u64_rejects_json_number`; `primitives::u64_rejects_leading_zero`; `primitives::u64_rejects_plus_and_exponent`; `primitives::u64_rejects_overflow`; `primitives::hex_rejects_uppercase`; `primitives::hex_rejects_wrong_length`; `primitives::uuid_rejects_uppercase`; `primitives::opt_rejects_two_elements`; `primitives::set_rejects_unsorted`; `primitives::set_rejects_duplicate`; `accept_vectors::all_85_projections_round_trip`; `reject_vectors::projection_level_rejects_all_reject`; `reject_vectors::unknown_field_rejected`; `reject_vectors::null_rejected`; `reject_vectors::wrong_type_rejected`; `reject_vectors::wrong_wire_revision_rejected`; `registry_hash::registry_sha256_matches`; `cargo run -p codegen -- --check` exits 0. Always also: `cargo fmt --check`, `cargo clippy -p <crate> --all-targets -- -D warnings`.
- **NEGATIVE TESTS:** Unknown field, `null`, numeric u64, leading-zero decimal, wrong `type`/`wire_revision`, unsorted/duplicate set, 2-element option, uppercase hex — each rejected by a named test.
- **ACCEPTANCE CRITERIA:** 85 types; 85/85 ACCEPT projections round-trip; all projection-level REJECT vectors reject; regeneration byte-identical; no hand edits in `generated.rs`.
- **EVIDENCE TO RETURN:** Test counts per vector class; codegen command; `generated.rs` SHA-256. A1 then sends INTERFACE_NOTICE I-01@v1 to W02–W06.
- **OUT-OF-SCOPE:** CBOR (T005/T012); semantic authority checks.
- **BLOCKERS:** none.
- **A3 REVIEW FOCUS:** Primitive mapping vs ENCODING.md; deny_unknown_fields everywhere; enum positions; no aliases.

EXECUTION AND CAPS:
Every executable validation/install/build/test/scan/scratch command queues through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2/ws01-core-security -- COMMAND ARGUMENTS
Root checks use --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2. Maximum3A2 globally/1per lane, one actual executing build across all A2/A3, jobs2/cache5G/diskfloor20GiB, owned target/no shared CARGO_TARGET_DIR. W03r1 plus W01r2 plus W06r1 is the maximum three reservations, not proof of three running agents. Check the live board before launch; do not duplicate owner-launched agents. Helper model/effort follows SUBAGENT_POLICY; disclose unavailability/telemetry, no silent substitution or repeated failed spawn.

STOP/COMPLETION:
- Foreign dirt/head mismatch, ambiguous frozen rule, actual out-of-scope need/dependency, repeated failure/context/disk limit, missing required interface or unauthorized provider/secret action: preserve progress and return NOT_READY; never reset, stash, delete or modify returned/reviewed work.
- Close only the listed finding(s), preserve every previously met original criterion, obtain all required local evidence and a signed scoped local commit with sole parent BASE_SHA, new SHA/tree and clean worktree. Remote CI stays NOT_RUN, so no final PASS/acceptance, interface publication or consumer unlock is inferred.
- A1 processes this RETURN and arranges a fresh independent A3 on the changed candidate; no earlier A2/helper/A3 session is reused. Owner no-extra-post-CI-A3 rule remains: fresh local review of a changed candidate is mandatory, duplicate review merely after unchanged CI is not imposed.
- Preserve G0/G1 BUILD-only, modelAPI1Mtokens/USD5total, cloudUSD0/keysUSD0, G2NOT_PASSED/G3none/all55claimsNOT_RUN, W05 integration hold and all protections. No product push/PR/merge/provider/account/budget mutation, paid review, force/rebase/amend-after-return or self-approval.

RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-A2_RETURN.md
TEMPLATE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A2_RETURN_TEMPLATE.md
Write the full RETURN first with exact identities, scoped delta, each finding/closure proof, actual commands/exits/test counts, meaningful mutations, hashed evidence, nonzero history classification and MEMORY_UPDATES for A1 alone to merge. Entire final chat response exactly one fenced text code box containing result/checks/blockers/absolute RETURN/WORK LOCATION/complete Root A1 resume prompt; nothing outside and no extra/nested fences. Stop after RETURN.
