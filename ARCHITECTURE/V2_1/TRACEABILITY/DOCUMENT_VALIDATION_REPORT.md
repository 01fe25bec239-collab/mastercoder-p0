# Document and fixture validation report

Final validation date: 2026-10-03. Scope: architecture artifacts only.

The coordinating author ran `_tools/validate_package.py` from the export workspace.
It checked the mandatory document inventory; canonical type names and prohibited
aliases; local Markdown links; hostile-execution database arrows in sequence
diagrams; all 55 prototype claim document shapes and identities; all 20 finding
references; the input ZIP hash and every extracted baseline file against that ZIP.
It invoked the included `TEST_VECTORS/check_spec.py` for structural wire fixtures
and declarative semantic examples. Exit status: 0.

The independent Contract Consistency report additionally records full comparison
of all 85 registry/schema projections and independent reconstruction/verification
of six valid and six corrupted Ed25519 signature fixtures. See the original
review report for its methods and scope. Those checks are not relabeled as
runtime or independent Rust/TypeScript interoperability evidence.

Reviewed registry SHA-256: `b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787`.

Captured final local validator output:

```text
{
  "scope": "LOCAL_ARCHITECTURE_FIXTURE_CHECK_ONLY",
  "contracts": 85,
  "vectors": {
    "ACCEPT_ENCODING": 85,
    "REJECT_ENCODING": 355
  },
  "p0_executed": false
}
Semantic specification cases: 40 (not runtime/P0 evidence)
{
  "mandatory_files": 66,
  "canonical_runtime_schemas": 85,
  "atomic_claims": 55,
  "all_claims_not_run": true,
  "baseline_sha256": "e418ff17e0f35ea695159448a94fd803781830f70a8d5b4ae508b43fe8d3209d",
  "errors": []
}
```

The export script separately accounts for every original V2 file in
`V2_TO_V2_1_FILE_CHANGES.json`, builds `PACKAGE_FILES_SHA256.json`, and verifies
each archived byte against the payload manifest and source tree. The manifest
excludes itself from hashing and includes itself in the total file count.
Python caches, bytecode, and macOS metadata are excluded from the deliverable.
The adjacent export receipt records the final archive hash and actual file count.

Limits: these checks cannot establish implemented security, real provider
semantics, operational recovery, sandbox isolation, or hardware behavior.
All 55 atomic prototype claims remain NOT_RUN. No product was implemented,
no P0 experiment was executed, and production freeze remains NOT_GRANTED.
