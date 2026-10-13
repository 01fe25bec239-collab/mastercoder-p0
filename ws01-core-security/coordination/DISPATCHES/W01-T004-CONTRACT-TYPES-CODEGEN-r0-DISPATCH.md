FROM: Root A1 (acting W01-A1)
TO: W01-A2[W01-T004-CONTRACT-TYPES-CODEGEN-r0]
MESSAGE_TYPE: DISPATCH
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r0
DISPOSITION: PREPARED_FOR_FRESH_A2; NOT_LAUNCHED
DATE_UTC: 2026-10-13T16:30:10.256015+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0
  BRANCH: ws01/w01-t004-contract-types-codegen-r0
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: a217d94ff1ac1262e21cf9b2e9f713666a830a18
IMPLEMENTATION_SHA / REVIEWED_SHA: NONE
TREE: NONE
START_TREE: 1c88f7f7d5d58d48c6d685e2c94ee3ea67d3a307
SOLE_PARENT: NONE
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

OBJECTIVE: **In plain words:** Generate Rust types for all 85 data records defined in the architecture, straight from the registry file, so every lane uses exactly the same names, fields and rules. This is the first thing the other workstreams wait for: do it as early as possible. **Precisely:** Generate the `mc-contracts` crate (I-01@v1): Rust types for all 85 registered contracts (serde JSON projection, registry field order, enum code tables), generated mechanically from `REGISTRY.json` by `tools/codegen`. **This is the program's critical path**: W02, W03 and W04 start their first real task after it. Dispatch it the moment T000 is ACCEPTED.

ENTRY/READ FIRST (stop on mismatch/missing): print actual WORK LOCATION, verify clean exact BASE_SHA/START_TREE/branch. Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md then SKILLS.md (all applicable procedures/§13.2/14/15), memory.md, fulltask W01-T004 and this dispatch twice. Read SCHEMAS/REGISTRY.json, ENCODING.md, all applicable JSON schemas and wire-vectors.json, ARCHITECTURE/V2_1/49_CANONICAL_PROTOCOL_GLOSSARY.md. Live coordinator absolute files govern; task checkout copies may be stale. Read NOTICES/W01-T004-PREFLIGHT-DEPENDENCY-REQUEST.md and dependency integration disposition; original task fields below remain unchanged. Root remainsA1, you are freshA2.

AUTHORIZED WRITE SET ONLY:
- ws01-core-security/crates/mc-contracts/src/**
- ws01-core-security/crates/mc-contracts/tests/**
- ws01-core-security/tools/codegen/src/**
Only coordination exception: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md. Do not edit manifests, lockfiles, build.rs, docs, workflows, architecture, other lanes or othercoordination. Choose the task's explicitly allowed tests/registry_hash.rs alternative. No newdependency; existingdirectunicode-normalization=0.1.25 is now supplied for NFC rejection. Stop with DEPENDENCY_REQUEST if any other direct crate/package access is genuinely necessary; do not use an undeclared transitive import or silently defer a required rule.

PRECONDITIONS/RESOURCES: W01-T000 INTEGRATED merge9a7922bf29c0a6689903b96896dab8851d1eeeeb must be ancestor of BASE. Standing W01-DEPENDENCY-UPDATE is verified INTEGRATED at merge a217d94ff1ac1262e21cf9b2e9f713666a830a18; its immutablecandidate72410409afd609bd98eb9d8742592287d8b1cb6a is ancestor and normalUnicodepin available. PhasedG0/G1 signed, BUILDonly. One reservedW01slot; no othernewA2dispatched. Globalmax3A2/max1lane/max1activebuild/compilerjobs2/cache5G/floor20GiB; recheckliveboards/diskbefore execution. ModelAPI1Mtokens/$5total/cloud$0/keys$0 unchanged, no newpaidcall authorized. G2NOT_PASSED/G3none/allP0claimsNOT_RUN. No credential-shaped env or DB runtime needed for pureJSON/type/codegen tests; don't fake resource-dependent checks. No CARGO_TARGET_DIR override; worktree-ownedtarget. Ifneeded use existing nonsensitive sccacheconfiguration only.

DISPATCH NOTES (bounded implementation choices; original task and frozen contracts unchanged):
- Generate all 85 registry names, field orders and schema subsets. Verify the exact raw registry hash. Enum codes follow registered order. Choose deterministic Rust names for unnamed inline enums while preserving every serialized value and field-specific subset. GitPublicationCapabilityV2's narrow effect_kind enum handles projection rejection structurally; semantic authority implementation remains out of scope.
- Reject non-NFC text rather than normalizing it or restricting valid Unicode refs to ASCII. Enforce schema lexical and byte limits, closed records, required fields, exact type/wire_revision, and no Default. Nested values and generic wrappers must retain their required validation.
- SetOf rejects unsorted or duplicate input; never sort it. ENCODING specifies lexical comparison of complete canonical encoded element bytes. Rust Ord, JSON text order and decimal-string order are not substitutes. Private comparison logic may derive this order without adding a CBOR encoder/decoder, public wire codec or mc-cbor dependency. Consider variable lengths, headers and nested types; include distinguishing cases such as canonical string order "b" before "aa". Return a concrete scope/architecture blocker if this cannot be implemented within the original projection-only scope.
- Generate generated.rs mechanically. Do not hand-edit generated types. Committed output must be formatted and byte-identical on regeneration; --check must be read-only and fail on stale output. Keep explicit --registry/--out arguments and sensible checkout-derived defaults so the prescribed cargo run -p codegen -- --check works. Do not embed machine paths in product code.
- Preflight counted 85 ACCEPT projections, 261 projection-level REJECTs and 94 CBOR-only REJECTs. Recount the authoritative fixtures and report each class. All projection rejects are mandatory. Report byte-only cases separately as outside this task; do not claim they passed. Use authoritative vectors and meaningful independent negative/mutation controls, not an oracle derived solely from your implementation.

EXACT REQUIRED COMMANDS (all through sharedwrapper, actualexitcodes/evidence returned):
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0/ws01-core-security -- COMMAND ARGUMENTS
- cargo metadata --locked --format-version 1
- cargo fmt --all --check
- cargo test -p mc-contracts --locked (everyoriginalnamedtest must exist/pass)
- cargo run -p codegen --locked -- --check
- cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
- cargo check --workspace --locked
- cargo test --workspace --locked
For root specificationcheck use --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r0 -- python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py; p0_executed must remainfalse. Runmeaningful tasknegatives/mutations/regeneration checks andscope/newrangesecrets/hardcodingscans throughwrapper too. Preserve allchecks; no unverifiedtoolversion/dependency upgrades, sharedtarget, hook/wrapperedit or skipemptytests. Activeprecommitscope/gitleakshooks/signing remainmandatory.

SECRETS: Candidate, staged and new-range scans must be clean. Full history is known to contain inherited W01 contract-list matches; actual counts vary with shared refs. Classify every current occurrence and retain redacted provenance. Do not suppress findings or call history clean. Any supplemental ignored-target metadata findings must be independently classified and reported separately. No secrets, customer data or production action.

VERBATIM FULL TASK SPECIFICATION:
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


COMPLETION: Every original criterion, named test and applicable local check must pass. Deliver only scoped changes, signed local commits, a clean worktree, exact SHA/tree/parents and hashed evidence. If remote CI is the only remaining gate, report local readiness with remote CI NOT_RUN. A1 arranges fresh independent local A3, then separately authorized genuine CI/disposition/integration. No self-approval, extra post-CI A3 or outward authorization is inferred. A1 publishes the I-01 notice as files only after verified integration; A2 does not message other chats or unlock tasks.

STOP: Preserve work and return NOT_READY for foreign dirt, changed base, missing prerequisites, an out-of-scope need, ambiguous architecture, repeated failure, context/disk limits, unavailable tools, or a required secret/provider/claim action. List completed and remaining work precisely. Never reset, stash, delete, force, amend, disable guards/tests or commit foreign changes. A1 does not implement. No push, PR or merge is authorized for T004. This is one bounded attempt.
RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md
TEMPLATE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A2_RETURN_TEMPLATE.md
Write the RETURN before the final chat response. Include durable facts under MEMORY_UPDATES; A1 alone edits memory.md. Put the entire final response in exactly one fenced text code block, with result, checks, blockers, absolute RETURN path, WORK LOCATION and complete A1 resume prompt inside it. No prose, links, extra or nested fences outside it. Stop after writing the RETURN; root A1 processes it.
