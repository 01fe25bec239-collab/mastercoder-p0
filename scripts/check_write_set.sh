#!/usr/bin/env bash
# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).
# Audit F-1/F-2: CI checks out protected main; scopes come from that HEAD, never the candidate.
# Hook: newline-separated names | bash scripts/check_write_set.sh BRANCH [REV]
# CI: bash scripts/check_write_set.sh --pr BASE_SHA HEAD_SHA BRANCH TITLE
# Tests: bash scripts/check_write_set.sh --self-test
# Safe setup probe: bash scripts/check_write_set.sh --isolation-test
set -eu
script_root=$(cd "$(dirname "$0")/.." && pwd)
python3 - "$script_root" "$@" 3<&0 <<'PY'
import ast
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import tempfile

trusted = Path(sys.argv[1])
args = sys.argv[2:]


def git(*args, cwd=None, data=None):
    # Resolve the fixture directory at call time, not at function definition.
    return subprocess.run(["git", "--no-optional-locks", *args], cwd=trusted if cwd is None else cwd,
                          input=data, capture_output=True, check=True).stdout


def repository_snapshot(root):
    index = Path(os.fsdecode(git("rev-parse", "--git-path", "index", cwd=root)).strip())
    if not index.is_absolute():
        index = root / index
    return (git("rev-parse", "HEAD", cwd=root), git("ls-files", "--stage", "-z", cwd=root),
            git("status", "--porcelain", "-z", cwd=root), index.read_bytes())


def isolation_test():
    # Compile the actual helper with a synthetic source: even a stale-default
    # regression can only damage this disposable sentinel, never the task repo.
    body = (trusted / "scripts/check_write_set.sh").read_text().split("<<'PY'\n", 1)[1].rsplit("\nPY", 1)[0]
    function = next(node for node in ast.parse(body).body
                    if isinstance(node, ast.FunctionDef) and node.name == "git")
    with tempfile.TemporaryDirectory(prefix="git-isolation-") as tmp:
        sentinel, scratch = Path(tmp) / "sentinel", Path(tmp) / "scratch"
        sentinel.mkdir()
        scratch.mkdir()
        git("init", "-q", "-b", "main", cwd=sentinel)
        (sentinel / "sentinel.txt").write_bytes(b"sentinel\n")
        git("add", ".", cwd=sentinel)
        tree = git("write-tree", cwd=sentinel).decode().strip()
        base = git("-c", "user.name=Synthetic", "-c", "user.email=fixture@example.test",
                   "-c", "commit.gpgsign=false", "commit-tree", tree, data=b"sentinel\n", cwd=sentinel).decode().strip()
        git("update-ref", "refs/heads/main", base, cwd=sentinel)
        before = repository_snapshot(sentinel)
        namespace = {"trusted": sentinel, "subprocess": subprocess}
        exec(compile(ast.Module(body=[function], type_ignores=[]), "<actual-git-helper>", "exec"), namespace)
        namespace["trusted"] = scratch
        fixture_git = namespace["git"]
        fixture_git("init", "-q", "-b", "main")
        assert Path(os.fsdecode(fixture_git("rev-parse", "--show-toplevel")).strip()) == scratch.resolve(), "scratch Git escaped its repository"
        (scratch / "fixture.txt").write_bytes(b"fixture\n")
        fixture_git("add", ".")
        tree = fixture_git("write-tree").decode().strip()
        head = fixture_git("-c", "user.name=Synthetic", "-c", "user.email=fixture@example.test",
                           "-c", "commit.gpgsign=false", "commit-tree", tree, data=b"fixture\n").decode().strip()
        fixture_git("update-ref", "refs/heads/main", head)
        assert fixture_git("rev-parse", "HEAD").decode().strip() == head
        assert repository_snapshot(sentinel) == before, "fixture changed sentinel HEAD/index/status"
        assert (sentinel / "sentinel.txt").read_bytes() == b"sentinel\n"
    print("PASS: scratch Git isolation sentinel (source HEAD/index/status/bytes unchanged)")


def reject(message):
    raise ValueError(message)


def pattern(glob):
    if not isinstance(glob, str) or not glob or glob.startswith("/") or ".." in glob.split("/"):
        reject("invalid trusted scope glob")
    return re.compile(re.escape(glob).replace(r"\*\*", ".*").replace(r"\*", "[^/]*") + r"\Z")


DOC_IGNORE = """# Documentation and agent state stay local
*.md
*.mdc
**/coordination/*
!**/coordination/WRITE_SCOPES.json
**/.claude/
**/.cursor/
/.agents/
/.claude/
/.cursor/
/ARCHITECTURE/V2_1/TRACEABILITY/V2_TO_V2_1_FILE_CHANGES.json
/ARCHITECTURE/V2_1/TRACEABILITY/finding-resolution.json
"""


def documentation_path(path):
    parts = PurePosixPath(path).parts
    if re.fullmatch(r"ws0[1-6]-[a-z0-9-]+/coordination/WRITE_SCOPES\.json", path):
        return False
    return (path.lower().endswith((".md", ".mdc"))
            or parts[0] in (".agents", ".claude", ".cursor")
            or any(part in ("coordination", ".claude", ".cursor") for part in parts)
            or path in ("ARCHITECTURE/V2_1/TRACEABILITY/V2_TO_V2_1_FILE_CHANGES.json",
                        "ARCHITECTURE/V2_1/TRACEABILITY/finding-resolution.json"))


def check(branch, revision, files, title=None):
    match = re.fullmatch(r"ws(0[1-6])/([a-z0-9][a-z0-9-]*)", branch)
    if not match:
        reject("branch must be ws0N/<task-id-lowercase>[-rN] or ws0N/coordination")
    lane, slug = match.groups()
    roots = [p.decode() + "/" for p in git("ls-tree", "--name-only", "HEAD").splitlines()
             if re.fullmatch(rb"ws" + lane.encode() + rb"-[a-z0-9-]+", p)]
    if len(roots) != 1:
        reject("cannot find exactly one trusted workstream directory")
    root = roots[0]
    task = re.sub(r"-r[0-9]+$", "", slug).upper()
    architecture_import = lane == "06" and slug.startswith("arch-import-")
    label = "coordination" if slug == "coordination" else "arch-import" if architecture_import else task
    if title is not None:
        tokens = re.findall(r"\[([^\[\]]+)\]", title)
        if label.casefold() not in [t.casefold() for t in tokens]:
            reject("PR title must contain [" + label + "]")
        if any(re.match(r"W0[1-6]-", t, re.I) and t.casefold() != label.casefold() for t in tokens):
            reject("PR title names another task")
    migration = branch == "ws06/w06-docs-local-only-r0"
    if migration:
        # Owner-authorized deletion only, enumerated from protected HEAD, never the PR.
        documents = {p.decode() for p in git("ls-tree", "-rz", "--name-only", "HEAD").split(b"\0")
                     if p and documentation_path(p.decode())}
        allow, deny = [pattern(p) for p in sorted(documents | {".gitignore"})], []
    elif slug == "coordination":
        allow, deny = [pattern(root + "coordination/**")], []
    elif architecture_import:
        allow, deny = [pattern("ARCHITECTURE/**")], []
    else:
        scopes = json.loads(git("show", "HEAD:" + root + "coordination/WRITE_SCOPES.json"))
        record = scopes["tasks"].get(task)
        if not record:
            reject("no dispatch scope for task " + task)
        allow = [pattern(p) for p in record["allow"]]
        deny = [pattern(p) for p in record.get("deny", [])]
        if not allow:
            reject("empty trusted task scope")
    # Query the complete tree/index once; deleted paths remain scope-checked.
    entries = {}
    raw = git("ls-tree", "-rz", revision) if revision else git("ls-files", "--stage", "-z")
    for entry in raw.split(b"\0"):
        if not entry:
            continue
        meta, name = entry.split(b"\t", 1)
        fields = meta.split()
        mode, oid = fields[0], fields[2] if revision else fields[1]
        if not revision and fields[2] != b"0":
            reject("unmerged index")
        entries[name.decode("utf-8", "surrogateescape")] = (mode, oid)
    for path in files:
        if not path or any(ord(c) < 32 or ord(c) == 127 for c in path):
            reject("control character in changed path")
        if str(PurePosixPath(path)) != path or path.startswith("/") or ".." in path.split("/"):
            reject("noncanonical changed path")
        if migration:
            owned = path in documents or path == ".gitignore"
            if path in documents and path in entries:
                reject("documentation migration is deletion-only: " + repr(path))
        elif path.startswith("ARCHITECTURE/"):
            owned = architecture_import
        elif path.startswith(root + "coordination/"):
            owned = slug == "coordination"
        elif path.startswith(root) or path.startswith(".github/workflows/ws" + lane + "-"):
            owned = True
        else:
            owned = lane == "06" and not re.match(r"ws0[1-5]-|\.github/workflows/ws0[1-5]-", path)
        if not owned or not any(p.fullmatch(path) for p in allow) or any(p.fullmatch(path) for p in deny):
            reject("files outside the write set: " + repr(path))
        if path in entries and documentation_path(path):
            reject("documentation and agent state must remain local: " + repr(path))
        if re.search(r"(^|/)(\.env(?:\..*)?|[^/]*\.(pem|key|p12|pfx)|id_rsa[^/]*|id_ed25519[^/]*)$", path) and not path.endswith("/.env.example"):
            reject("secret-like path: " + repr(path))
        if path in entries:
            mode, oid = entries[path]
            if mode not in (b"100644", b"100755"):
                reject("nonregular changed file: " + repr(path))
            # Retain bootstrap guard's 5 MiB committed-blob ceiling (WRITE_OWNERSHIP_MATRIX).
            if int(git("cat-file", "-s", oid.decode())) > 5242880:
                reject("changed blob exceeds 5 MiB: " + repr(path))

    if migration:
        if any(documentation_path(path) for path in entries):
            reject("documentation migration must remove all tracked documents")
        if ".gitignore" not in entries or git("cat-file", "blob", entries[".gitignore"][1].decode()) != git("show", "HEAD:.gitignore") + DOC_IGNORE.encode():
            reject("unexpected documentation ignore rules")
        for path in git("ls-tree", "-rz", "--name-only", "HEAD").split(b"\0"):
            if path.endswith(b"/coordination/WRITE_SCOPES.json"):
                name = path.decode()
                if name not in entries or git("cat-file", "blob", entries[name][1].decode()) != git("show", "HEAD:" + name):
                    reject("trusted scope registries must remain unchanged")


def pr(base, head, branch, title):
    for revision in (base, head):
        if not re.fullmatch(r"[0-9a-f]{40}", revision):
            reject("PR revisions must be full commit SHAs")
        git("cat-file", "-e", revision + "^{commit}")
    if git("rev-parse", "HEAD").decode().strip() != base:
        reject("guard must run from checked-out protected base")
    merge_base = git("merge-base", base, head).decode().strip()
    names = git("diff", "--name-only", "-z", "--no-renames", merge_base, head).split(b"\0")
    check(branch, head, [n.decode("utf-8", "surrogateescape") for n in names if n], title)


def scratch_self_test():
    global trusted
    source = trusted
    with tempfile.TemporaryDirectory(prefix="write-scope-test-") as tmp:
        trusted = Path(tmp)
        git("init", "-q", "-b", "main")
        for path in source.glob("ws0[1-6]-*/coordination/WRITE_SCOPES.json"):
            dest = trusted / path.relative_to(source)
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, dest)
        for directory in ("scripts", ".github/workflows", "ARCHITECTURE"):
            shutil.copytree(source / directory, trusted / directory)
        (trusted / ".gitignore").write_bytes(git("show", "HEAD:.gitignore", cwd=source))
        git("add", ".")
        env = dict(os.environ, GIT_AUTHOR_NAME="Synthetic", GIT_AUTHOR_EMAIL="fixture@example.test",
                   GIT_COMMITTER_NAME="Synthetic", GIT_COMMITTER_EMAIL="fixture@example.test")
        def commit(parent=None):
            tree = git("write-tree").decode().strip()
            # Synthetic disposable objects are test inputs, not product commits.
            command = ["git", "-c", "commit.gpgsign=false", "commit-tree", tree, "-m", "scratch fixture"]
            if parent:
                command += ["-p", parent]
            return subprocess.run(command, cwd=trusted, env=env, capture_output=True, check=True).stdout.decode().strip()
        base = commit()
        git("update-ref", "refs/heads/main", base)
        branch = "ws06/w06-t000-root-hardening-r0"
        title = "[WS06][W06-T000-ROOT-HARDENING] scratch"
        cases = [
            ("valid task scope", branch, title, {".editorconfig": b"fixture\n"}, None),
            ("neighbouring task file", branch, title, {"ws06-platform-oss-qualification/docs/staging-environment.md": b"x"}, "outside the write set"),
            ("coordination/memory.md", branch, title, {"ws06-platform-oss-qualification/coordination/memory.md": b"x"}, "outside the write set"),
            ("excluded harness/Cargo.toml", "ws06/w06-t016-p0-06a-harness-r0", "[W06-T016-P0-06A-HARNESS]", {"ws06-platform-oss-qualification/claims/P0-06A/harness/Cargo.toml": b"x"}, "outside the write set"),
            ("task absent from WRITE_SCOPES.json", "ws06/w06-t999-absent-r0", "[W06-T999-ABSENT]", {".editorconfig": b"x"}, "no dispatch scope"),
            ("PR title names another task", branch, "[W06-T001-WS06-WORKSPACE-SCAFFOLD]", {".editorconfig": b"x"}, "PR title must contain"),
            ("PR title names two tasks", branch, title + " [W06-T002-TESTDB-I14]", {".editorconfig": b"x"}, "names another task"),
            ("foreign workstream", branch, title, {"ws01-core-security/foreign.rs": b"x"}, "outside the write set"),
            ("rename source escapes scope", branch, title, {"scripts/check_write_set.sh": None, "ws01-core-security/renamed.sh": b"x"}, "outside the write set"),
            ("newline filename", branch, title, {".editorconfig\nforeign": b"x"}, "control character"),
            ("oversized blob", branch, title, {".editorconfig": b"x" * 5242881}, "exceeds 5 MiB"),
            ("candidate scope forgery", branch, title, {"ws06-platform-oss-qualification/coordination/WRITE_SCOPES.json": b'{"tasks":{}}'}, "outside the write set"),
        ]
        for name, ref, subject, changes, reason in cases:
            git("read-tree", base)
            for path, content in changes.items():
                if content is None:
                    git("update-index", "--force-remove", "--", path)
                else:
                    oid = git("hash-object", "-w", "--stdin", data=content).decode().strip()
                    git("update-index", "--add", "--cacheinfo", "100644," + oid + "," + path)
            head = commit(base)
            try:
                pr(base, head, ref, subject)
            except ValueError as error:
                assert reason and reason in str(error), (name, str(error))
            else:
                assert reason is None, name
            print("PASS: " + name)
        # One candidate replaces every guard, then alters architecture and foreign paths.
        git("read-tree", base)
        changes = {"scripts/check_write_set.sh": b"exit 0\n", "scripts/verify_architecture.py": b"raise SystemExit(0)\n",
                   ".github/workflows/common-arch-check.yml": b"name: weakened\n",
                   ".github/workflows/common-write-set-guard.yml": b"name: weakened\n",
                   "ARCHITECTURE/V2_1/SCHEMAS/REGISTRY.json": b"tampered\n",
                   "ws01-core-security/foreign.rs": b"x"}
        for path, content in changes.items():
            oid = git("hash-object", "-w", "--stdin", data=content).decode().strip()
            git("update-index", "--add", "--cacheinfo", "100644," + oid + "," + path)
        head = commit(base)
        try:
            pr(base, head, branch, title)
        except ValueError as error:
            assert "outside the write set" in str(error)
        else:
            raise AssertionError("self-weakened candidate passed write scope")
        archive = subprocess.Popen(["git", "archive", "--worktree-attributes", head, "ARCHITECTURE"], cwd=trusted, stdout=subprocess.PIPE)
        import tarfile
        candidate = trusted / "candidate"
        candidate.mkdir()
        with tarfile.open(fileobj=archive.stdout, mode="r|") as tar:
            tar.extractall(candidate, filter="data")
        assert archive.wait() == 0
        result = subprocess.run([sys.executable, str(source / "scripts/verify_architecture.py"), str(candidate / "ARCHITECTURE/V2_1")], capture_output=True, text=True)
        assert result.returncode == 1 and "SCHEMAS/REGISTRY.json" in result.stdout, result.stdout + result.stderr
        print("PASS: self-weakened candidate plus architecture/foreign path => both checks red")
        documents = [p.decode() for p in git("ls-tree", "-rz", "--name-only", base).split(b"\0")
                     if p and documentation_path(p.decode())]
        ignore = git("show", base + ":.gitignore") + DOC_IGNORE.encode()
        cases = [
            ("documentation deletion", {}, None),
            ("modified document", {documents[0]: b"modified"}, "deletion-only"),
            ("new document", {"NEW.md": b"new"}, "outside the write set"),
            ("source modification", {"scripts/verify_architecture.py": b"changed"}, "outside the write set"),
            ("source deletion", {"scripts/verify_architecture.py": None}, "outside the write set"),
            ("registry modification", {"ws01-core-security/coordination/WRITE_SCOPES.json": b"{}"}, "outside the write set"),
            ("registry deletion", {"ws01-core-security/coordination/WRITE_SCOPES.json": None}, "outside the write set"),
            ("unexpected ignore rules", {".gitignore": ignore + b"*.rs\n"}, "unexpected documentation ignore rules"),
        ]
        for name, mutation, reason in cases:
            git("read-tree", base)
            changes = dict.fromkeys(documents)
            changes[".gitignore"] = ignore
            changes.update(mutation)
            for path, content in changes.items():
                if content is None:
                    git("update-index", "--force-remove", "--", path)
                else:
                    oid = git("hash-object", "-w", "--stdin", data=content).decode().strip()
                    git("update-index", "--add", "--cacheinfo", "100644," + oid + "," + path)
            head = commit(base)
            try:
                pr(base, head, "ws06/w06-docs-local-only-r0", "[W06-DOCS-LOCAL-ONLY] Keep documents local")
            except ValueError as error:
                assert reason and reason in str(error), (name, str(error))
            else:
                assert reason is None, name
            print("PASS: " + name)
        print("PASS: temporary scratch repository removed on exit")


def self_test():
    global trusted
    source = trusted
    isolation_test()
    before = repository_snapshot(source)
    # Include untracked drafts as well as tracked files without reading ignored env.
    paths = git("ls-files", "-z", "--cached", "--others", "--exclude-standard", cwd=source).split(b"\0")
    contents = {path: hashlib.sha256((source / os.fsdecode(path)).read_bytes()).digest()
                for path in paths if path and (source / os.fsdecode(path)).is_file()}
    try:
        scratch_self_test()
    finally:
        trusted = source
        assert repository_snapshot(source) == before, "self-test changed original HEAD/index/status"
        assert all(hashlib.sha256((source / os.fsdecode(path)).read_bytes()).digest() == digest
                   for path, digest in contents.items()), "self-test changed original file bytes"
    print("PASS: original task HEAD/index/status/file-byte sentinel unchanged")


try:
    if args == ["--self-test"]:
        self_test()
    elif args == ["--isolation-test"]:
        isolation_test()
    elif len(args) == 5 and args[0] == "--pr":
        pr(*args[1:])
        print("write-set guard: PASS")
    elif len(args) in (1, 2) and not args[0].startswith("--"):
        paths = os.fdopen(3, "rb").read().decode("utf-8", "surrogateescape").splitlines()
        check(args[0], args[1] if len(args) == 2 else "", paths)
    else:
        reject("usage: check_write_set.sh BRANCH [REV] < names | --pr BASE HEAD BRANCH TITLE | --self-test | --isolation-test")
except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
    print("BLOCKED (write-set guard): " + str(error), file=sys.stderr)
    if isinstance(error, subprocess.CalledProcessError) and error.stderr:
        # Escape control characters and bound diagnostics; never emit raw blobs.
        print("git stderr: " + repr(error.stderr.decode("utf-8", "replace")[:4096]), file=sys.stderr)
    sys.exit(1)
PY
