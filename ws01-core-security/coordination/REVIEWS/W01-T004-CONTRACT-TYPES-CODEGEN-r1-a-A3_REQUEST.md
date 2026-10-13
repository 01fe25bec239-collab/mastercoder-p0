FROM: Root A1 (acting W01-A1)
TO: W01-A3[W01-T004-CONTRACT-TYPES-CODEGEN-r1-a]
MESSAGE_TYPE: REVIEW_REQUEST
TASK_ID: W01-T004-CONTRACT-TYPES-CODEGEN
ATTEMPT: r1
DISPOSITION: PRELIMINARY_LOCAL_REVIEW_PREPARED; NOT_LAUNCHED
DATE_UTC: 2026-10-13T18:30:09.295459+00:00
WORKSTREAM: W01
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  REVIEW_WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1-review-a
  BRANCH: NONE (detached exact candidate)
  IMPLEMENTATION_BRANCH: ws01/w01-t004-contract-types-codegen-r1
  WORKSTREAM_DIR: ws01-core-security/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
BASE_SHA: b83f1a9be066c8fe577c5d71ae25cf38563f5265
ORIGINAL_PROTECTED_BASE: a217d94ff1ac1262e21cf9b2e9f713666a830a18
IMPLEMENTATION_SHA / REVIEWED_SHA: 9f40fae0db8fc6f6b9fc20e4b7f56cc8a56e1174
TREE: df2f6a1223631c729ca0470a74478295557e9b4e
SOLE_PARENT: b83f1a9be066c8fe577c5d71ae25cf38563f5265
ACCEPTED_SHA: NONE
PRODUCT WRITES / NEW TESTS / SCANS: A3 no product edits; A2 scoped candidate only
PUSH / PR / MERGE: NONE

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

PHASE: Fresh independent preliminary LOCAL review. Actual remote CI NOT_RUN and not separately authorized for this new task. If local criteria satisfactory and only genuine remoteCI/integration remains, record PRELIMINARY_LOCAL_STATUS: SATISFACTORY / VERDICT BLOCKED:FINAL_PR_CI_PENDING; do not invent finalPASS/acceptance/I01orI13publication/consumerunlock. Findings still require REPAIR_REQUIRED rather than hidebehindCIpending. Owner no-extra-postCI-A3 instruction remains; this required fresh candidate review is not an extra duplicate afterCI. No push/PR/merge/account/provider/provisioning/P0/production/budget action.

INDEPENDENCE: Fresh session and fresh independent helpers; you must not author, repair, dispatch, assist or previously review this candidate or participate in any of its A2 helper work. Root A1 is excluded. Declare independence of every helper. Detachedreview worktree exists/clean at exactSHA/tree; CREATE notneeded. Verify SHA/tree/status before source reading and after allchecks. Unexpected dirt/movingtarget -> INVALID_REVIEW_TARGET/preserve; never reset/stash/delete/repair/amend/sourceedit. Only finalreport may be written under coordination; memory/task/RETURN/request/dispatch read-only.

READ FIRST:
1. Print actual WORKLOCATION. Live /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/AGENTS.md then SKILLS.md(all applicable/§13.3/14/15/17), memory.md, full task and thisrequest twice. Currentabsolute coordinator instructions override stale checkout copies.
2. Exact dispatch /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/DISPATCHES/W01-T004-CONTRACT-TYPES-CODEGEN-r1-DISPATCH.md and unchanged/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/RETURNS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-A2_RETURN.md. Parent reports/decisions/architecture below; read allcriteria/invariants before interpreting passingtests.
3. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-processing/verification.json, retained-artifact-verification.json and original A2 evidence /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-repair-evidence. Verify original RETURNhash656ffeb2c708fb9f6882d77b147a05a4261c4a82cd3a62975fe2aaa1d3b5dce7 and declaredartifacthashes before using retainedscripts; inspect executable source first. A1spotcheck is context, not independentreviewproof. Retained outputs never substitute for freshchecks.
4. /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/COMMON/A3_REVIEW_TEMPLATE.md, owner approved BUILD_REVIEW_CI_SEQUENCE_DECISION.md, applicable ownerdecision(s) and root SUBAGENT_POLICY; no automatic outwardgrant/paidupgrade/modelsubstitution.

AUTHORIZED ORIGINAL TASK SCOPE (A3 no product writes):
- ws01-core-security/crates/mc-contracts/src/**
- ws01-core-security/crates/mc-contracts/tests/**
- ws01-core-security/tools/codegen/src/**

REPAIR REVIEW:
- Do not reuse r1A2 enum_regressions/header_regressions, any r0implementationhelpers/primitives_v2/generator_v2/primitive_tests_v2, r0A3 or its standards/spec helpers, or rootA1 preflight/helpers. Fresh independent A3 required for changedimmutablecandidate.
- Read original r0A3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r0-a-A3_REVIEW.md (hash54aca4abc6e609430d18aa5d4912a94181e2f7bb668d0734fef9604223c70e2c) and r0 RETURN/request/dispatch without editing. Verify r0 and review preserved atb83f1a9be066c8fe577c5d71ae25cf38563f5265/f995ca7d5ea25b2949f0898c663259a6ff3e8bc5. Full r1 taskrangeoriginalbase remains10allowedpaths; repairdeltaexactfourliteralpaths:
- ws01-core-security/tools/codegen/src/main.rs
- ws01-core-security/crates/mc-contracts/src/generated.rs
- ws01-core-security/crates/mc-contracts/tests/enum_codes.rs
- ws01-core-security/crates/mc-contracts/tests/reject_vectors.rs

F-1 ORIGINAL FINDINGS VERBATIM:
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



SHARP FOCUS:
- Independently determine F1closed for all85recordmarkers/197literals/41tables using BOTH directserde_json::from_value/from_str plus nestedrecordfields and actualdispatcher. String-onlyDeserialize must be generated at every enum boundary, not just patcheddispatcher; nonstringmap/taggednull/arrays/numbers/bools/null reject with positivevalidstring/unknownstring controls. Preserve serialization/equality, exactpublicsignatures/codepositions/variantnames/subsets, including nestedMergeSubjectV2.scope.type and GitPublicationCapabilityV2.effect_kind.
- Inspect all126generated customDeserialize implementations and generator templates; removingonly those plusrestoringderives mustleaveoriginalstructure unchanged. Don't trust regex-normalizedaudit withoutchecking normalizationcannotmasksemanticchanges. Restore/mechanicalgenerator output matchescompletecommittedgeneratedfile, neverhandpatchgeneratedtypes.
- Fresh keyF1regressions and independent malformedrepresentation probes; demonstrate repairedsourcepasses and revertingstringonlydeserialization in isolatedscratch is killedbyexpectedtestassertion, notcompilerfailure. A2reportsfiveoriginal+F1mutations/sevenfailing observations; verify eachspecificreason and exactrestoration. Originalr0failproof remainscontext, canreproduceifnecessary in isolatedcopy; don'tedit r0/review.
- Maintain fulloriginal85types/85ACCEPT/261projectionREJECT, original17namedtests+4newregressions+4codegentests; A2reports56contract/60workspace tests, peer/harness emptiesarecompilationonly. All94CBOR-onlycases NOT_EXECUTED/outscope. Allprimitive/NFC/canonicalset/identity/object-only/duplicate-key/aggregate-size/depth/read-onlyfreshness guards stillvalid; no CBORcodec/semantic authority/GitOIDcorrelation/filesystemcell/P0/provider qualification. Readregistry/schemas/glossary/ENCODING inexactcandidate, noW05wire-tsreuse.
- Signature/parent/pins/41workspace members/364packages/no manifest-lock-toolchain drift; rootRust1.97.1/existingUnicode0.1.25. Currenthistory96classifiednonzero vs candidate/staged/ranges0; freshcounts mayadvance, classifyactualoccurrences individually. Original68A2/86A3counts preserved, no suppression/hardcodedcredentials or confignewvalues.

FRESH REQUIRED COMMANDS throughsharedwrapper:
cargo metadata --locked --format-version 1
cargo fmt --all --check
cargo test -p mc-contracts --locked
cargo run -p codegen --locked -- --check
cargo clippy -p mc-contracts -p codegen --all-targets --locked -- -D warnings
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
IndependentF1probe/mutations/read-onlystale-missing-registry-schema freshness controls plusrequirednegatives/namedtests; scope/signature/diffcheck/scans/classifications/identitystartend. Root check_spec.py mustreportp0_executed:false, specificationonlynotcodecproof. Retained evidence /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/STARTUP_CHECKS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-repair-evidence/evidence-manifest.sha256 80files/hash61afed4f34726d65b1d597f263dde7b279ff2bc106d4941af8c4125991236d44 independentlyverified; inspect source before use but performown checks.

LOCAL EXECUTION/RESOURCE RULE:
All executable validation/install/build/test/scan/scratch checks through:
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1-review-a/ws01-core-security -- COMMAND ARGUMENTS
Root check_spec uses --cwd /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W01-T004-CONTRACT-TYPES-CODEGEN-r1-review-a. Localoneactualbuildqueue acrossA2/A3, max3A2global/1lane/jobs2/cache5G/floor20GiB. Ownreviewtarget/cache, no sharedCARGO_TARGET_DIR orcredential-shaped.env sourcing/host orDB startup. Pureformat/projection checks requirenoDB; reportactualmissingresourceifneeded. Do notchangehook/pin/wrapper/toolchain orspend/addproviders/accounts/production/P0. Scratchmutations/evidence outsideproductrepoorignoredlocalownedpaths, neverwriteoriginalcandidate/reviewsource orcleanupreturnedartifacts. No paidreview/silentmodelsubstitution/spawnretryloops.

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


OUTPUT: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws01/ws01-core-security/coordination/REVIEWS/W01-T004-CONTRACT-TYPES-CODEGEN-r1-a-A3_REVIEW.md
Only thiscoordinationreportmaybewritten. UseCOMMON/A3_REVIEW_TEMPLATE.md: exactSHA/tree/statusstart/end/independence, eachcriterion/negative/actualrequiredcheck+exit/evidence, findings withlocation/input->failure/minimalrepair/severity, actuallimitations/hashedevidence/MEMORY_UPDATES. PreservealloriginalRETURNs/reviews/inputs, no memory/task/dispatch/source edit orselfapproval/outwardaction/downstreamunlock. Entire finalchatresponse exactlyonefencedtextcodebox includingresult/checks/blockers/absoluteREPORT/WORKLOCATION/completeA1resume prompt, nothingoutside/noextra/nestedfences. Stopafterreport; rootA1continuescoordinator and processesit.
