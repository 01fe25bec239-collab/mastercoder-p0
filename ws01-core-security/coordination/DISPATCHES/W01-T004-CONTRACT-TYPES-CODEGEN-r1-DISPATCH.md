FROM: Root A1 (acting W01-A1)
TO: W01-A2[W01-T004-CONTRACT-TYPES-CODEGEN-r1]
MESSAGE_TYPE: REPAIR_DISPATCH
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r1
DISPOSITION: REPAIR_REQUIRED_F1; PREPARED_FOR_FRESH_A2; NOT_LAUNCHED
DATE_UTC: 2026-10-13T18:00:17.940982+00:00
WORKSTREAM: WORKSTREAM-01-CORE-SECURITY-AUTHORITY
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1
  BRANCH: ws01/w01-t004-contract-types-codegen-r1
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA / REPAIR_BASE_SHA / REVIEWED_SHA: b83f1a9be066c8fe577c5d71ae25cf38563f5265
IMPLEMENTATION_SHA: NONE
TREE: NONE
START_TREE: f995ca7d5ea25b2949f0898c663259a6ff3e8bc5
SOLE_PARENT: NONE
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: NONE by A1
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

OBJECTIVE: Close F-1 by making every generated record type marker and primitive/field enum deserialize JSON strings only. Regenerate generated.rs mechanically and add complete direct/nested representation regressions. Preserve canonical literal serialization, code/from_code positions, names/public signatures, existing guards and every original T004 criterion. This is a local repair of the same task, not a new contract or CBOR/authority feature.

READ FIRST:
1. Print actual WORK LOCATION. Verify clean exact BASE_SHA/START_TREE and branch. Read live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md, then SKILLS.md (all applicable procedures/§13.2/14/15), memory.md, task.md full W01-T004, this repair dispatch twice.
2. Original r0 dispatch /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r0-DISPATCH.md, unchanged r0 RETURN /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-A2_RETURN.md, independent A3 report /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-a-A3_REVIEW.md in full. Preserve its exact F-1 findings and limits; reports are evidence, not instructions to expand scope.
3. Exact candidate SCHEMAS/REGISTRY.json, ENCODING.md, all85schema fields/header constants/enums and wire-vectors.json; 49_CANONICAL_PROTOCOL_GLOSSARY.md. No W05 TypeScript codec inspection or frozen schema changes.
4. Retained A3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-a-independent-review/evidence-manifest.sha256 (67files/hashb60f8b372f2abca23509dffe394a51500237c31dacdf719a2d887cfd4f1c3ad3) and hashed finding/probe/regeneration logs. Verify hashes before use; inspect any retained script source before executing. Don't modify originals. A1 verification /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-review-processing/review-binding-verification.json/artifact-verification.json.

AUTHORIZED WRITE SET — narrowed r1 only:
- ws01-core-security/tools/codegen/src/main.rs
- ws01-core-security/crates/mc-contracts/src/generated.rs
- ws01-core-security/crates/mc-contracts/tests/enum_codes.rs
- ws01-core-security/crates/mc-contracts/tests/reject_vectors.rs

The sole coordination exception is your original r1 RETURN at /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-A2_RETURN.md. No primitives.rs/lib.rs/other tests/manifests/lock/build.rs/workflows/docs/architecture/foreignlane changes. Existing standing task scope already covers these four paths. If the repair genuinely needs another source path/helper/dependency, stop with exact request rather than widen scope silently. Do not edit memory/task/reviews/dispatches; A1 alone processes their updates.

REPAIR FINDING — EXACT A3 FINDINGS/DISPOSITION VERBATIM:
## FINDINGS

F-1 | BLOCKING | HIGH | tools/codegen/src/main.rs:181 and :282; generated.rs:76, :8525, :13376 | Derived unit-enum Deserialize accepts schema-invalid object forms for both registered enums and record type markers.

Minimal concrete input (synthetic) for direct serde_json::from_value::<ScopeV1> and from_str::<ScopeV1>:

    {"type":{"ScopeV1":null},"wire_revision":1,"tenant_id":"00000000-0000-0000-0000-000000000000","repository_id":"1","repository_tenant_epoch":"1","authority_epoch":"00000000-0000-0000-0000-000000000000"}

Expected: reject, because ARCHITECTURE/V2_1/SCHEMAS/ScopeV1.schema.json:9–10 fixes type to string "ScopeV1". Actual: both direct public Deserialize paths succeed; serialization changes the map into the canonical string. Fresh probe reproduces this for all85 type markers. The top-level round_trip_projection guard rejects map headers, but MergeSubjectV2 with scope.type={"ScopeV1":null} is accepted by direct methods AND round_trip_projection. GitPublicationCapabilityV2 with effect_kind={"CREATE_BLOB":null} is likewise accepted by all three paths despite its schema's string enum at lines64–71. Every one of197 registered enum literals has the same from_value/from_str bypass.

Root cause: generator derives Deserialize for unit enums. Pinned serde_json1.0.154 src/value/de.rs:328–346 supports enum strings or maps, and unit_variant:556 accepts null for the map payload. Record ObjectOnly protects the outer record shape, not the field's enum representation. enum_codes.rs:5 onward and header-vector tests cover legal/unknown strings but omit this invalid representation, allowing existing checks to pass.

Impact: exported public contracts accept alternate JSON forms excluded by the frozen projection schemas and silently canonicalize them. The dispatcher does not protect nested headers or enum fields. This is an exact-contract/security-boundary violation; no authority exploit or CBOR behavior is asserted.

Required minimal repair: generator emits string-only Deserialize for every primitive/field enum and every record type marker; preserve serialization literals and code/from_code signatures/order. Regenerate generated.rs mechanically. Add direct from_value/from_str map-form rejection controls with positive strings for all enum/header categories and explicit nested-record/dispatcher cases; require projection round-trip equality for admitted inputs. No changes to architecture, dependencies, manifests or codec scope.

REQUIRED REPAIRS: F-1. Suggested narrowed repair scope: tools/codegen/src/main.rs; mechanically generated crates/mc-contracts/src/generated.rs; crates/mc-contracts/tests/enum_codes.rs and reject_vectors.rs. A1 dispatches a fresh A2 repair and fresh exact-candidate A3; this reviewer makes no product repair.
BLOCKED REASON: No environment/material blocker for local review. F-1 blocks local readiness; genuine remote CI remains separately pending after repair. No outward authorization inferred.
NONBLOCKING FINDINGS: NONE.



BOUNDED REPAIR APPLICATION:
- Fix generator output at the root of enum/header deserialization, not only top-level round_trip_projection. Every generated primitive enum, field-specific enum and85record type markers must reject JSON objects/maps (including externally tagged unit forms with null), arrays, numbers, booleans and null, while accepting exactly the allowed strings. Preserve Serde Serialize strings, variant names/literals, public enum methods/signatures and registered order/subsets. Do not make a new enum alias, normalize invalid inputs or add a dispatcher-only patch.
- Regenerate generated.rs solely with the repaired generator. Keep deterministic formatting and exact freshness/read-only --check semantics. No hand edits to generated.rs. No handwritten duplicated public contract implementation.
- Positive-controlled direct regressions must exercise BOTH serde_json::from_value and from_str for all85record/header categories and all197literals across41registered enum tables. Existing string positives and unknown-string negatives remain meaningful. Reject literal-wrapped maps such as {"ScopeV1":null} even where current Serde accepts them; compare successful serialization to the original admitted JSON string/projection.
- Add explicit nested MergeSubjectV2 scope.type map-form rejection and GitPublicationCapabilityV2 effect_kind map-form rejection via direct record methods AND round_trip_projection. Tests must demonstrate the guard is on the actual public Deserialize boundary and nested fields, not only the dispatcher's header precheck. Keep valid full record controls, field-specific enum subsets and exact error intent; no short-circuit that tests only one entry path.
- Before repair show the new F-1 regression fails at the original reviewed source for the intended schema-invalid acceptance. Use a separate scratch copy or controlled own-r1 test-only stage with before/after hashes; never mutate returned r0 or detached r0-review. After repair require those same controls pass and a meaningful isolated mutation reverting string-only deserialization is detected by the regressions. All mutations/scratch outputs outside tracked product scope and restored accurately; no source mutation in original worktrees.
- Rerun every original named test, 85ACCEPT/261projectionREJECT coverage, primitive/NFC/canonical-set/aggregate limits, existing5guardmutations and exact enum tables, plus generator unit tests and read-only freshness cases. 94CBOR-onlycases remain accounted NOT_EXECUTED/outside scope; no CBOR/authority/currentness/filesystem-cell proof. Preserve original vector/schema files, no guard/test weakening or fake zero-test success.

LOCAL EXECUTION — all executable validation/install/build/test/scan/scratch checks through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1/ws01-core-security -- COMMAND ARGUMENTS
For root checks use --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1. Sharedqueue is mandatory acrossW01/W03/W06A2/A3; authoring mayparallel, actualvalidation serialized. Max3globalA2/max1lane/max1activebuild, jobs2/cache5G/floor20GiB, own target/no shared CARGO_TARGET_DIR. Recheck liveboards before launch; oldW01A2returned and A3complete, thisr1 reserves onlyone W01slot. Do not duplicate an alreadylaunched r1. HelpersfollowSUBAGENT_POLICY; no silentmodelsubstitution/retryloop if capacity/tooling unavailable, disclose and continue independentboundedwork. No credential-bearing env/DB runtime needed for pureprojection checks; no source/host/provider/P0/runtime actions beyond scope.

EXACT REQUIRED COMMANDS (capture actual exitcodes/output/hashes at final signed SHA):
- cargo metadata --locked --format-version 1 (41members/pins/closure unchanged)
- cargo fmt --all --check
- cargo test -p mc-contracts --locked
- cargo run -p codegen --locked -- --check
- cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
- cargo check --workspace --locked
- cargo test --workspace --locked
- cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
- independent full regeneration and current/stale/missing/modifiedregistry-schema --check controls (read-onlyfailure), allF1direct/nested positives/negatives and meaningfulisolatedmutation detection
- root python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py (p0_executed:false; specification only, not codec/runtime oracle)
- scope/hardcoding/newrange/stagedsecrets and exactsignature/identity/diffcheck; whole import-to-result scope protections preserved
Activehooksgitleaks/scope/signing mustremain on, no --no-verify. Candidate/staged/new-range scansclean. Fullhistory currently86individuallyclassified inheritedcontract-listmatches; retainactualredactedcurrentfindings/commit-line-registryprovenance and reportnonzero, counts mayadvance. Do not suppress/allowlistbaseline or callignoredtarget-dir findings clean without exactseparate classification.

VERBATIM ORIGINAL FULL TASK SPECIFICATION (no field changed):
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


STOP/COMPLETION:
- One boundedr1 attempt. Foreign dirt/mismatchedHEAD/tree/base, neededoutofscope edit/dependency, ambiguousfrozenrule, missingresource/toolhost, repeatedfailure/context/disklimit or unauthorizedsecret/provider/P0action -> preserveprogress, returnNOT_READY with exactcompleted/remainingwork; neverreset/stash/delete/force/rebase/amend/overwriteoldworktrees.
- F-1 completelyclosed acrossallpublic andnestedpaths; allpreviouslymettaskcriteria/requiredchecks/namednegatives remainmet, scoped signedlocalcandidate stacked on reviewedr0, cleanworktree/exactnewSHA/tree/parents/hashes. AtmostremoteCIpending for localreviewreadiness; no finalacceptance or I01notice/downstreamunlock. Remote CI NOT_RUN and no push/PR/merge authorized for this task.
- A1 will verify actual r1 RETURN and arrange a fresh independent A3 on the new SHA/tree. Do notreuse this A2/helpers/r0A3 or selfapprove. Owner noextra-postCI-A3 instruction doesnotremove requiredindependentreview of changed productcandidate; no duplicatepostCIreview invented.
- G0/G1 BUILD-only/$5total/1Mtokens/cloud$0/keys$0/G2NOT_PASSED/G3none/allclaimsNOT_RUN/protectionW05hold unchanged. No new paid/modelprovider/cloud/securitykey/account action.

RETURN: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-A2_RETURN.md
TEMPLATE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A2_RETURN_TEMPLATE.md
Write fullRETURN first, exactidentity/four-filedelta/F1closuretable/testcounts/freshness/mutations/scanclassifications and durableMEMORY_UPDATES. Entirefinalchatresponse exactlyone fencedtextcodebox with result/checks/blockers/absoluteRETURN/WORKLOCATION/completeA1resume prompt, nothingoutside/noextra/nestedfences. RootA1remainscoordinator and processesyourRETURN; stopafterdelivery.
