FROM: Root A1 (acting W01-A1)
TO: W01-A3[W01-T004-CONTRACT-TYPES-CODEGEN-r2-a]
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r2
DISPOSITION: FRESH_PRELIMINARY_LOCAL_A3_PREPARED; NOT_LAUNCHED
DATE_UTC: 2026-10-13T22:55:03.338466+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  REVIEW_WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2-review-a
  BRANCH: NONE (detached)
  IMPLEMENTATION_BRANCH: ws01/w01-t004-contract-types-codegen-r2
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: 9f40fae0db8fc6f6b9fc20e4b7f56cc8a56e1174
ORIGINAL_PROTECTED_BASE: a217d94ff1ac1262e21cf9b2e9f713666a830a18
IMPLEMENTATION_SHA / REVIEWED_SHA: ae25048f83bc66d0e7d31f3d6d01fd3756bf92ec
TREE: 84feea18fafbe2da317b9ea602b8d3f4364fdc05
SOLE_PARENT: 9f40fae0db8fc6f6b9fc20e4b7f56cc8a56e1174
ACCEPTED_SHA / MERGED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: A3 no product edits
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

PHASE: Fresh independent preliminary local review of changed r2. F-2 is A2-reported locally addressed, not independently closed. Prior F-1 CLOSED_LOCALLY at r1 must remain preserved. Actual remote CI NOT_RUN/unauthorized; if all local criteria satisfactory and only real CI remains, report PRELIMINARY_LOCAL_STATUS SATISFACTORY / VERDICT BLOCKED:FINAL_PR_CI_PENDING. A local defect requires REPAIR_REQUIRED. Never record final PASS/acceptance/publication/consumer unlock with missing evidence. Owner no-extra-post-CI-A3 rule remains: changed-candidate local review required, duplicate review just after unchanged CI is not required.

INDEPENDENCE: You and every helper must be fresh and uninvolved in all earlier A2/A3 implementation/review/helper work, including r2 A2 reason_mapping/evidence_requirements (failed/unusable launches still excluded), r1 A2 enum_regressions/header_regressions, r0 A2 primitives_v2/generator_v2/primitive_tests_v2, r0/r1 A3 and their standards/spec teams, root A1 and all preflight/dispatch-audit helpers including w01_r2_dispatch_audit. Declare actual identities/model/effort/independence; do not reuse earlier executable outputs as your own proof. Exact token telemetry unavailable unless host supplies it; no silent substitutions or repeated failed spawns.

LOCATION: Detached review exists/clean at exact candidate; CREATE not needed. Verify HEAD/tree/status before source reading and after every review phase/end. Unexpected changes => INVALID_REVIEW_TARGET; preserve, never reset/stash/delete/repair. Only the named report may be written in coordination.

READ FIRST:
1. Print actual WORK LOCATION. Live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, SKILLS.md (all applicable §13.3/14/15/17), memory.md, latest handoff, full task/criteria and this request twice. Live coordination governs stale checkout copies.
2. Exact r2 dispatch /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r2-DISPATCH.md, RETURN /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-A2_RETURN.md SHA256 cef9a1e0a8dceb57e28e3c8c435a8c3237cd31c61e484ce6dbf8184c94a29a0d; previous r1/r0 reports/RETURNs/dispatches without edits. r1-a report hash2241c21ed4df2483622632aaff75dca096209e736d64e2f05a7bca06305f29a7 preserves F-1 closure and F-2 finding. r0-a report hash54aca4abc6e609430d18aa5d4912a94181e2f7bb668d0734fef9604223c70e2c.
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-processing/verification.json and /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-repair-evidence/evidence-manifest.sha256 (194files/hashc012d8b4f65ad204f997bada5015c73f1e5a1f64815bbb3555fe57bd8c7007cc).393 earlier artifacts independently rehashed by A1; original sealed manifests/reports remain unchanged. Inspect retained scripts before use, perform your own fresh checks; A1's one-test spotcheck is context only.
4. Frozen SCHEMAS/REGISTRY.json, all85 schema definitions, ENCODING.md,49 glossary, INV-18 and wire-vectors.json. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md, BUILD_REVIEW_CI_SEQUENCE_DECISION.md, original task applicability and SUBAGENT_POLICY. Never read W05 wire-ts codec as a Rust oracle.

AUTHORIZED IMPLEMENTATION SCOPE (A3 writes no product):
Original task: ws01-core-security/crates/mc-contracts/src/**, crates/mc-contracts/tests/** and tools/codegen/src/**.
r2 repair delta: ONLY ws01-core-security/crates/mc-contracts/tests/reject_vectors.rs.
Verify one-file delta 9f40fae0db8fc6f6b9fc20e4b7f56cc8a56e1174..ae25048f83bc66d0e7d31f3d6d01fd3756bf92ec and original ten-file delta a217d94ff1ac1262e21cf9b2e9f713666a830a18..ae25048f83bc66d0e7d31f3d6d01fd3756bf92ec. Other9 source files, dependencies/manifests/toolchain/generated bytes must remain identical to repair base. Verify signatures/sole parent/unchanged original worktrees/reports.

F-2 ORIGINAL FINDING VERBATIM:
F-2 | BLOCKING | LOW (test quality; no observed product acceptance bypass) | crates/mc-contracts/tests/reject_vectors.rs:304–309, :370–374, :418–422 | Required persistent rejection-reason assertions are missing. projection_level_rejects_all_reject checks is_err while printing vector reason; null sweep and revision-kind checks do likewise. SKILLS.md §6.2 says “each asserting the specific rejection reason, not just 'error'”; §15 requires meaningful reason assertions for task negatives. Concrete scenario: ScopeV1-extra-field rejected by an erroneous revision error, or wire_revision="1" rejected by an unrelated missing-field error, would still satisfy these assertions, even though it does not exercise the intended boundary. Fresh independent_reasons.rs verifies the correct current reasons for all261fixtures/967nulls/bothrevision paths, but it exists only in reviewer scratch and is absent from the persistent product suite. Required minimal repair: add explicit expected reason matching per vector reason category, field/header-specific null reason matching, and invalid-type/u16 reason checks for malformed revision kinds. Preserve positive controls/counts/both direct paths; no guard/vector/schema changes needed.


SHARP REVIEW FOCUS:
- Are all261 projection REJECT cases persistently checked for their intended category and correct changed field/header? Unknown/missing fields must assert the exact field; wrong revision, decimal/UUID/set-identity/publication-subset reasons must correspond to frozen inputs. Rejecting for another reason must fail the test. Review correct AttemptBindingV2 lease_generation mapping and restored valid controls, not a fabricated schema field.
- Nulls: verify all967 required fields, complete positive control and exact sole-field substitution; expected diagnostic derived from the frozen descriptor. Serde errors may lack path: do not require invented path metadata, but an unrelated missing/unknown/revision/type diagnostic must not satisfy the intended case.
- Malformed revisions: both from_value/from_str independently exercised, numeric float/string/bool/null kind and u16 diagnostics, positive full-record controls. No short circuit conceals an untested path; errors must be specific, not broad substring matches admitting unrelated failures.
- Independently prove wrong-reason substitutions that still return Err are detected, including wrong field under correct category, wrong null kind and both revision methods. Original weak test passing such substitution is context; new tests must fail intended assertions, not compiler errors.
- Verify F-1 string-only boundaries remain: all85 markers/197 literals/41 tables, direct/nested/dispatcher alternate representations with positive serialization controls and meaningful derive-reversal scratch mutation. Preserve all original canonical ordering/NFC/options/limits/object-only/duplicate/public API rules.
- Full original checks/vectors/generator deterministic byte identity and read-only stale/missing/registry/schema freshness. A2 reports56 contract/60 workspace tests,12 mutation commands/14 intended assertion failures; independently reproduce meaningful coverage. Count empty suites as compilation only.94 CBOR-only cases NOT_EXECUTED/outside scope, no authority/runtime/P0 proof.

FRESH REQUIRED COMMANDS through wrapper:
cargo metadata --locked --format-version 1
cargo fmt --all --check
cargo test -p mc-contracts --locked
cargo run -p codegen --locked -- --check
cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
Fresh intended-reason/null/revision controls, all original named negatives/F-1 probes/meaningful wrong-reason and guard mutations, independent full regeneration/read-only freshness; scope/diff/signature/scans/hardcoding/pins/identity/provenance.
Root python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py => p0_executed:false (spec consistency, never codec oracle).
Candidate/staged/new-ranges clean; actual full-history separately classified/nonzero (A2 observed118 inherited matches), no suppressions or blanket clean claim. Preserve earlier68/86/96/102 observations; do not invent ignored-target cleanliness. No credential env or new DB/runtime host setup for pure JSON checks.

EXECUTION:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2-review-a/ws01-core-security -- COMMAND ARGUMENTS
Root checks --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r2-review-a. Every executable validation/install/build/test/scan/scratch check queues through wrapper; one actual executing build acrossA2/A3, max3A2/1lane/jobs2/cache5G/floor20GiB. Own review target/no shared CARGO_TARGET_DIR. No source edit/guard bypass/hook change/pin/toolchain/spend/account/provider/host/claim action. Scratch mutations external or owned ignored paths only, never original worktrees. No cleanup of retained work/artifacts.

VERBATIM ORIGINAL FULL TASK SPECIFICATION:
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

OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r2-a-A3_REVIEW.md
Only this report may be written in coordination. Use complete A3_REVIEW_TEMPLATE with exact binding/independence/start-end clean status, each criterion/named negative/check exit/evidence, concrete findings/minimal repairs/severity, actual limitations/hashed evidence/MEMORY_UPDATES. Preserve originals; no task/memory/RETURN/request edits or outward approval.
Entire final response exactly one fenced text code box: result/checks/blockers/absolute REPORT/WORK LOCATION/complete Root A1 resume prompt, nothing outside/no extra or nested fences. Stop after report; Root A1 processes it.
