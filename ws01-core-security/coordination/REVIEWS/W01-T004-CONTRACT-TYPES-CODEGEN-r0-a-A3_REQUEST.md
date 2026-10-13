FROM: Root A1 (acting W01-A1)
TO: W01-A3[W01-T004-CONTRACT-TYPES-CODEGEN-r0-a]
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r0
DISPOSITION: PRELIMINARY_LOCAL_REVIEW_PREPARED; NOT_LAUNCHED
DATE_UTC: 2026-10-13T17:28:10.571211+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  REVIEW_WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0-review-a
  BRANCH: NONE (detached exact candidate)
  IMPLEMENTATION_BRANCH: ws01/w01-t004-contract-types-codegen-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: a217d94ff1ac1262e21cf9b2e9f713666a830a18
IMPLEMENTATION_SHA / REVIEWED_SHA: b83f1a9be066c8fe577c5d71ae25cf38563f5265
TREE: f995ca7d5ea25b2949f0898c663259a6ff3e8bc5
SOLE_PARENT: a217d94ff1ac1262e21cf9b2e9f713666a830a18
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: A2 ten scoped source/test files; A1/A3 product edits NONE
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

PHASE: Fresh independent preliminary LOCAL review. Remote CI NOT_RUN and not authorized for this task. Owner no-extra-post-CI-A3 instruction remains in force: do not interpret this necessary first product review as a duplicate later review. If all local criteria are satisfactory and only actual PR CI/integration remains, report PRELIMINARY_LOCAL_STATUS: SATISFACTORY with VERDICT BLOCKED:FINAL_PR_CI_PENDING; no final acceptance/PASS/downstream unlock while required evidence is absent. A1 will obtain concrete task-specific outward authorization and verify genuine remaining CI separately. No self-approval or provider actions.

INDEPENDENCE: You and every helper must not have authored, repaired, dispatched, assisted or previously reviewed this candidate. Exclude the entire A2 team, including primitives_v2, generator_v2, primitive_tests_v2 and all earlier/provisional implementation helpers, root A1 and its next_t004_preflight/next_parallel_preflight. Use a fresh A3 session/helpers; declare independence. Do not reuse the earlier dependency-update A3 as this review. Your detached worktree exists at exact SHA/tree, clean; CREATE not needed. Verify identities/status at start and end. Unexpected dirt or moving SHA/tree -> INVALID_REVIEW_TARGET, preserve state; never reset/stash/delete/amend/repair files.

READ FIRST:
1. Print actual WORK LOCATION. Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md then SKILLS.md (all applicable procedures, §13.3/14/15/17), memory.md, full task specification W01-T004, this request twice.
2. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r0-DISPATCH.md; /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md in full; /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/NOTICES/W01-T004-PREFLIGHT-DEPENDENCY-REQUEST.md; integrated dependency disposition and owner-approved source/check clarification boundaries if relevant. Do not import unrelated W03/W06 task-specific exceptions into Rust codegen.
3. Exact candidate SCHEMAS/REGISTRY.json, ENCODING.md, all85 relevant JSON schemas, TEST_VECTORS/wire-vectors.json, 49_CANONICAL_PROTOCOL_GLOSSARY.md and applicable INV-18/00D/contract authority boundaries. Treat architecture as read-only; no W05 TypeScript codec inspection or reused codec implementation.
4. Full diff and complete connected handwritten source, generator, generated output, every test; inspect mechanically generated structure across all85 contracts/41enum tables rather than sampling a few names. Check registry/schema coverage is actually complete and freshness proof independent. Do not trust test counts alone.
5. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-processing/verification.json and retained-artifact-verification.json; original A2 evidence /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-implementation-b83f1a9be066/evidence-manifest.sha256. Verify all74 artifact hashes and original RETURN hash before using retained scratch executables; inspect their source first. Retained output is context, not a substitute for fresh independent validation. Original manifest SHA256597dc5b96a071456a8c4671eef88ae45b0a947b80e2eca102860e0d08b51d126, RETURN SHA2560e318ff346e829bace6167e2ac574e536e8cee33ea50ce0e1ca2a922e55cd3dc.
6. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md, governing BUILD_REVIEW_CI_SEQUENCE_DECISION.md and current owner no-extra-postCI-review direction. Root SUBAGENT_POLICY governs independent helpers, never grants outward/acceptance authority.

AUTHORIZED IMPLEMENTATION SCOPE (A3 no product writes):
- ws01-core-security/crates/mc-contracts/src/**
- ws01-core-security/crates/mc-contracts/tests/**
- ws01-core-security/tools/codegen/src/**

SHARP REVIEW FOCUS:
- Exact public primitive/contract/enum signatures and every one of85 registered types; required headers equal exact registered name/wire_revision1, closed/nested records, missing option containers rejected, no Default or silently accepted positional arrays/duplicate textual keys. Check all serde entry paths (from_value/from_str and serialization), not just dispatcher round_trip_projection.
- Decimal boundaries/canonical spelling/no JSON numbers for u64/i64; hex/UUID lowercase/canonical shape; NFC rejection without normalization; legitimate Unicode refs and byte-vs-character/schema lexical bounds; origin/hostname/GitPath rules source-bound without invented stricter grammar. Semantic authority/currentness/GitOID algorithm correlation/filesystem cell collision checks remain outside this task; identify true structural misses without widening scope.
- SetOf accepts only input already sorted by complete canonical encoded element bytes and unique; constructor/deserialization/serialization preserve the rule. Inspect ordering across signed integers/header-width boundaries, variable text/byte lengths, options/arrays/nested records, registry field order and field-specific enum positions; neither RustOrd nor JSON/decimal lexical ordering is acceptable. Named-identity uniqueness is independent of whole-element encoding. Evaluate private comparison/size accounting scope without claiming a CBOR codec or executing94byte-onlycases.
- Aggregate record size/depth accounting: correctness in both JSON directions, saturating arithmetic/limits/inclusive boundaries and no inconsistent paths accepting mutated public fields. A2 reports85golden canonical-length matches and16MiB/depth32 guards; verify architecture source and actual behavior independently.
- Mechanical generator covers complete registry/schema data, no handwritten generated repairs or lost constraints; formatting deterministic, explicit/default paths portable, stale/missing/changed-registry --check fails without writing; check source/generated bytes agree after independent regeneration into isolated scratch paths. Pin hash raw bytes correctly. Field-specific enums preserve all41tables/197literals/codes and unknown rejection, including five-value publication subset.
- All85ACCEPTprojections round-trip to equal JSON values and all261projection-levelREJECTs fail for intended reasons;94CBOR-onlycases accounted NOT_EXECUTED/outside scope, not called PASS. Original17namedtests genuinely exist and every required negative is meaningful. A2 reports52contract/56workspace tests,74suites/67empty peers: don't claim empty peer suites prove behavior.
- Independently inspect/reproduce meaningful guard mutation controls; A2 has5 mutations (decimal, setorder, NFC, object-only, aggregate size), each specific test fails, source restored byte-for-byte. Perform any fresh mutation in isolated scratch copy only; immutable candidate/review source unchanged. Distinguish test assertion fixes from weakened guards.
- Exact10paths, no manifests/lock/build.rs/workflow/architecture/docs/foreignlane/coord source change; validSSHsignature/parent/base/cleanworktree. Candidate/staged/newrange0; fullhistory currentlyreported68individuallyclassified baseline contractlists, not cleanhistory. Review your actual current scan count honestly (sharedrefs may advance), no suppressions or blanketallowlist. Added hardcoding matches must be exact synthetic/normative classifications, no secret/machine/provider values.

EXECUTION/RESOURCES:
Every executable validation/build/install/test/scan/scratch check through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0-review-a/ws01-core-security -- COMMAND ARGUMENTS
For root spec use --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0-review-a. One actual build slot across A2/A3; W03/W06 may resume concurrently, queue normally. Do not bypass lock or use shared CARGO_TARGET_DIR. Jobs2/cache5G/floor20GiB and per-worktree target. Only nonsensitive existing tool preferences; do not source credential-shaped env or start DB. This pure JSON/generator task's required checks need no DB service; if a genuine required check does need it, report blocker rather than fake PASS. No root toolchain/dependency upgrade, host probe, paid/modelAPI/cloud/security keys/provider/P0/production action.

REQUIRED FRESH COMMANDS through wrapper:
- cargo metadata --locked --format-version 1 (41 members, dependency/pin drift check)
- cargo fmt --all --check
- cargo test -p mc-contracts --locked
- cargo run -p codegen --locked -- --check
- cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
- cargo check --workspace --locked
- cargo test --workspace --locked
- cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
- independent regen/read-only stale-output/changed-registry controls, meaningful isolated guard mutations, exact coverage and structural boundary checks
- root python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (p0_executed:false, spec consistency only, never codec oracle)
- scoped/range/staged/fullhistory redacted scans with individual classifications, hardcoding/scope/diffcheck/signature/identity at start/end
Retain actual outputs/exitcodes/hashes in isolated scratch evidence outside product scope. Preserve original evidence/source; no mutation or deletion of original snapshots. Do not rerun unrelated qualification or formatting-control remotePR experiments.

VERBATIM FULL ORIGINAL TASK SPECIFICATION:
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


OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-a-A3_REVIEW.md
Only this coordination report may be written by A3. Use COMMON/A3_REVIEW_TEMPLATE.md; exactSHA/tree start/end/independence, everycriterion/namedcheck and meaningful negative/mutation result, freshfindings with exactlocation/input/failure/minimalrepair/severity, actuallimitations, hashedverifiable evidence and MEMORY_UPDATES. Do not edit memory/task/dispatch/request/RETURN or accept/integrate/push/PR/merge. Entire final chat response exactlyone fencedtextcodebox with result/checks/blockers/reportpath/WORKLOCATION/completeA1resume prompt, nothingoutside/noextra/nestedfences. Stopafterreport; rootA1processes it.
