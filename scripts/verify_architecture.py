#!/usr/bin/env python3
"""Verify an architecture snapshot against its own TRACEABILITY/PACKAGE_FILES_SHA256.json and the pinned identity.
Usage: python3 verify_architecture.py <path-to/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1>"""
import hashlib, json, sys
from pathlib import Path

PINNED = {  # COMMON/PROGRAM_README.md identity table
    "TRACEABILITY/PACKAGE_FILES_SHA256.json": "97c604ecf6aefeec481b1bc8770a8a4c62d7643104d64bfdca4d8b5be5eb9c9d",
    "SCHEMAS/REGISTRY.json": "b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787",
    "TRACEABILITY/atomic-prototype-claims.json": "d2eda91044f761603e127f1946c742f82bf94d80b8930caba47ec17325094db7",
}
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()

root = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
bad = [k for k, v in PINNED.items() if sha(root / k) != v]
manifest = json.loads((root / "TRACEABILITY/PACKAGE_FILES_SHA256.json").read_text())
for e in manifest["files"]:
    p = root / e["path"]
    if not p.is_file() or sha(p) != e["sha256"]:
        bad.append(e["path"])
# Audit F-2: a candidate must not be able to add files the manifest does not list (only the manifest itself is exempt).
listed = {e["path"] for e in manifest["files"]} | {"TRACEABILITY/PACKAGE_FILES_SHA256.json"}
bad += sorted("EXTRA:" + str(q.relative_to(root)) for q in root.rglob("*") if q.is_file() and q.name != ".DS_Store" and str(q.relative_to(root)) not in listed)
print(json.dumps({"root": str(root), "files_checked": len(manifest["files"]), "pinned_checked": len(PINNED), "mismatches": bad}, indent=1))
sys.exit(1 if bad else 0)
