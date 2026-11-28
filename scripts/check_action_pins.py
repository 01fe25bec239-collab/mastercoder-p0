#!/usr/bin/env python3
# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Lint candidate workflows/action metadata as git blobs, never executable code.

Run this trusted base script with --revision FULL_COMMIT_SHA. No YAML dependency
is needed: the permitted subset is block mappings/sequences, same-line scalars,
simple scalar arrays and literal/folded block scalars. Other representations
(flow mappings, anchors/aliases/tags, complex or escaped keys, multiline quoted
scalars, directives/documents) fail closed. This is a pin-policy checker, not a
general workflow schema validator. Local and Docker uses are disallowed by the
task's requirement that *every* uses have a commit SHA and version comment.
"""

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path, PurePosixPath


# W06-T000 IMPLEMENTATION STEPS §3: full SHA and # vX.Y (patch permitted).
PIN = re.compile(r"[A-Za-z0-9_][A-Za-z0-9_.-]*/[A-Za-z0-9_][A-Za-z0-9_.-]*"
                 r"(?:/[A-Za-z0-9_.-]+)*@[0-9a-fA-F]{40}")
VERSION = re.compile(r"# v[0-9]+\.[0-9]+(?:\.[0-9]+)?(?:\s+.*)?")
KEY = re.compile(r"(?P<key>[A-Za-z0-9_.-]+|'(?:[^']|'')*'|\"[^\"\\]*\")\s*:(?:\s+|$)")
BLOCK = re.compile(r"[|>](?:[+-])?")
ARRAY = re.compile(r"\[[A-Za-z0-9_., /-]*\]")
# W06-T000 SECURITY INVARIANTS: only these guard workflows may use this event.
TARGET_GUARDS = {".github/workflows/common-arch-check.yml",
                 ".github/workflows/common-write-set-guard.yml"}


class Rejected(ValueError):
    pass


def scalar(source):
    """Return decoded same-line value and its actual YAML comment."""
    source = source.strip()
    if not source or source.startswith("#"):
        return "", source
    if source.startswith("'"):
        match = re.match(r"'((?:[^']|'')*)'", source)
        if not match:
            raise Rejected("unsupported multiline/unclosed quoted scalar")
        value = match[1].replace("''", "'")
        tail = source[match.end():]
    elif source.startswith('"'):
        try:
            value, end = json.JSONDecoder().raw_decode(source)
        except ValueError as error:
            raise Rejected("unsupported multiline/escaped quoted scalar") from error
        tail = source[end:]
    else:
        parts = re.split(r"\s+#", source, maxsplit=1)
        value = parts[0].rstrip()
        comment = "#" + parts[1] if len(parts) == 2 else ""
        if value.startswith(("&", "*", "!", "{", "?", "%", "@", "`")):
            raise Rejected("unsupported YAML anchor/alias/tag/flow mapping/indicator")
        if value.startswith("[") and not ARRAY.fullmatch(value):
            raise Rejected("unsupported YAML flow collection")
        return value, comment
    if tail and (not tail[0].isspace() or (tail.strip() and not tail.strip().startswith("#"))):
        raise Rejected("unsupported trailing content after quoted scalar")
    return value, tail.strip()


def lint(path, content):
    errors = []
    count = 0
    block_indent = None
    content = content.replace("\r\n", "\n")
    # YAML treats these as line breaks, even inside otherwise skipped run data.
    if any(char in content for char in ("\r", "\u0085", "\u2028", "\u2029")):
        return [f"{path!r}: unsupported control character/line separator"], 0
    restricted_target = path.startswith(".github/workflows/") and path not in TARGET_GUARDS
    for number, line in enumerate(content.split("\n"), 1):
        indent = len(line) - len(line.lstrip(" "))
        stripped = line.strip()
        if block_indent is not None:
            if not stripped or indent > block_indent:
                continue
            block_indent = None
        if not stripped or stripped.startswith("#"):
            continue
        try:
            if any(ord(char) < 32 for char in line):
                raise Rejected("unsupported control character/line separator")
            text = line[indent:]
            sequence = text.startswith("- ")
            if sequence:
                text = text[2:].lstrip(" ")
            match = KEY.match(text)
            if not match:
                if sequence and text and not text.startswith(("?", "-", "|", ">")):
                    value, _ = scalar(text)
                    if restricted_target and re.search(r"\bpull_request_target\b", value):
                        raise Rejected("pull_request_target forbidden outside the two common guards")
                    continue
                raise Rejected("unsupported YAML key/continuation/document")
            key = match["key"]
            if key.startswith(("'", '"')):
                key = key[1:-1].replace("''", "'")
            value, comment = scalar(text[match.end():])
            if restricted_target:
                if key == "pull_request_target" or re.search(r"\bpull_request_target\b", value):
                    raise Rejected("pull_request_target forbidden outside the two common guards")
                if key == "on" and BLOCK.fullmatch(value):
                    raise Rejected("unsupported block scalar event; pull_request_target could be hidden")
            if key == "uses":
                if not PIN.fullmatch(value):
                    raise Rejected("uses must be a remote action/workflow pinned to a full 40-hex commit SHA")
                if not VERSION.fullmatch(comment):
                    raise Rejected("uses must have a same-line # vX.Y or # vX.Y.Z comment")
                count += 1
            elif BLOCK.fullmatch(value):
                # Content indentation is measured from the sequence/mapping line,
                # so a following step/key is never mistaken for shell-script data.
                block_indent = indent + (len(line[indent:]) - len(text))
            elif value.startswith(("|", ">")):
                raise Rejected("unsupported explicit block indentation indicator")
        except Rejected as error:
            errors.append(f"{path!r}:{number}: {error}")
    return errors, count


def git(repo, *args, data=None):
    result = subprocess.run(["git", "--no-replace-objects", "-C", str(repo), *args],
                            input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        # Do not echo candidate-controlled git output or command arguments.
        raise Rejected(f"git {args[0]} failed (exit {result.returncode})")
    return result.stdout


def candidate_files(repo, revision):
    if not re.fullmatch(r"[0-9a-fA-F]{40}", revision):
        raise Rejected("--revision must be a full 40-hex commit SHA")
    if git(repo, "cat-file", "-t", revision).strip() != b"commit":
        raise Rejected("--revision must name a commit")
    entries = git(repo, "ls-tree", "-rz", "--full-tree", revision).split(b"\0")
    for entry in entries:
        if not entry:
            continue
        metadata, raw_path = entry.split(b"\t", 1)
        path = raw_path.decode("utf-8", errors="strict")
        name = PurePosixPath(path).name
        if not ((path.startswith(".github/workflows/") and path.endswith((".yml", ".yaml")))
                or name in ("action.yml", "action.yaml")):
            continue
        mode, kind, object_id = metadata.split()
        if mode not in (b"100644", b"100755") or kind != b"blob":
            raise Rejected(f"{path!r}: workflow/action metadata must be a regular git blob")
        yield path, git(repo, "cat-file", "blob", object_id.decode("ascii")).decode("utf-8", errors="strict")


def check_revision(repo, revision):
    errors = []
    files = pins = 0
    for path, content in candidate_files(repo, revision):
        found, count = lint(path, content)
        errors.extend(found)
        files += 1
        pins += count
    if not files:
        raise Rejected("candidate contains no workflow/action metadata files")
    return errors, files, pins


def self_test():
    sha = "a" * 40  # Synthetic fixture; no provider fact or action tag mapping.
    pin = f"example/action@{sha}"
    cases = [
        ("plain pin", f"- uses: {pin} # v1.2", None),
        ("patch version", f"uses: {pin} # v1.2.3 explanatory note", None),
        ("single quotes", f"'uses': '{pin}' # v1.2", None),
        ("double quotes", f'"uses": "{pin}" # v1.2', None),
        ("run content is data", f"run: |\n  echo 'uses: example/action@main'\nuses: {pin} # v1.2", None),
        ("folded content is data", "run: >-\n  uses: ignored@main", None),
        ("array values", f"on: [push, pull_request]\nuses: {pin} # v1.2", None),
        ("floating tag", "uses: example/action@v1 # v1.2", "40-hex"),
        ("short sha", f"uses: example/action@{sha[:-1]} # v1.2", "40-hex"),
        ("long sha", f"uses: example/action@{sha}a # v1.2", "40-hex"),
        ("nonhex sha", f"uses: example/action@{'g' * 40} # v1.2", "40-hex"),
        ("missing version", f"uses: {pin}", "same-line"),
        ("quoted fake comment", f"uses: '{pin} # v1.2'", "40-hex"),
        ("docker tag", "uses: docker://example/action:v1 # v1.2", "40-hex"),
        ("docker digest", f"uses: docker://example/action@sha256:{sha * 2} # v1.2", "40-hex"),
        ("local uses", "uses: ./example/action # v1.2", "40-hex"),
        ("expression", "uses: ${{ inputs.action }} # v1.2", "40-hex"),
        ("empty uses", "uses:", "40-hex"),
        ("multiline uses", f"uses: >\n  {pin} # v1.2", "40-hex"),
        ("multiline quoted", f'uses: "{pin}\n" # v1.2', "multiline"),
        ("escaped key", f'"u\\u0073es": {pin} # v1.2', "unsupported YAML key"),
        ("flow step", "- {uses: example/action@main}", "flow mapping"),
        ("flow nested", "steps: [{uses: example/action@main}]", "flow collection"),
        ("anchored mapping", "step: &x\n  uses: example/action@main", "anchor"),
        ("anchored scalar", f"uses: &x {pin} # v1.2", "anchor"),
        ("alias", "steps: *x", "alias"),
        ("merge key", "<<: *x", "unsupported YAML key"),
        ("tag", f"uses: !!str {pin} # v1.2", "tag"),
        ("complex key", f"? uses\n: {pin} # v1.2", "unsupported YAML key"),
        ("document", "---\nuses: example/action@main", "document"),
        ("tab", "\tuses: example/action@main", "control character"),
        ("target event", "on: [pull_request_target]", "pull_request_target forbidden"),
        ("target block event", "on:\n  pull_request_target:", "pull_request_target forbidden"),
        ("target sequence event", "on:\n  - 'pull_request_target'", "pull_request_target forbidden"),
        ("target literal block event", "on: |\n  pull_request_target", "block scalar event"),
        ("target folded block event", "on: >-\n  pull_request_target", "block scalar event"),
        ("alternate YAML linebreak", "name: test\u0085uses: example/action@main", "line separator"),
        ("linebreak in run data", "run: |\n  echo ok\u0085uses: example/action@main", "line separator"),
        ("CRLF block", f"run: |\r\n  echo ok\r\nuses: {pin} # v1.2\r\n", None),
        ("next step after block", "- run: |\n    echo ok\n- uses: example/action@main", "40-hex"),
        ("next key after block", "- run: |\n    echo ok\n  uses: example/action@main", "40-hex"),
        ("explicit indentation", "run: |2\n  uses: example/action@main", "explicit block indentation"),
        ("quoted suffix", f'uses: "{pin}", uses: example/action@main # v1.2', "trailing content"),
    ]
    for name, text, reason in cases:
        errors, _ = lint(".github/workflows/test.yml", text)
        if (reason is None and errors) or (reason is not None and not any(reason in e for e in errors)):
            raise AssertionError(f"{name}: expected {reason!r}; received {errors!r}")
        print(f"PASS: {name}")
    for path in TARGET_GUARDS:
        assert not lint(path, "on:\n  pull_request_target:")[0]
    # Real git object I/O: lint committed candidates, ignore later worktree edits,
    # include composite metadata, and reject metadata symlinks without following.
    with tempfile.TemporaryDirectory(prefix="action-pins-") as directory:
        repo = Path(directory)
        git(repo, "init", "-q")
        workflow = repo / ".github/workflows/test.yaml"
        workflow.parent.mkdir(parents=True)
        workflow.write_text(f"uses: {pin} # v1.2\n", encoding="utf-8")
        action = repo / "nested/action.yml"
        action.parent.mkdir()
        action.write_text(f"runs:\n  steps:\n    - uses: {pin} # v1.2\n", encoding="utf-8")
        git(repo, "add", ".")
        tree = git(repo, "write-tree").strip().decode("ascii")
        commit = subprocess.run(
            ["git", "-C", directory, "-c", "user.name=Synthetic Test", "-c", "user.email=test@example.invalid",
             "-c", "commit.gpgsign=false", "commit-tree", tree], input=b"synthetic pin test\n",
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True).stdout.strip().decode("ascii")
        assert check_revision(repo, commit) == ([], 2, 2)
        workflow.write_text("uses: example/action@main\n", encoding="utf-8")
        assert check_revision(repo, commit) == ([], 2, 2)
        action.write_text("uses: example/action@main # v1.2\n", encoding="utf-8")
        git(repo, "add", ".")
        bad_tree = git(repo, "write-tree").strip().decode("ascii")
        bad_commit = subprocess.run(
            ["git", "-C", directory, "-c", "user.name=Synthetic Test", "-c", "user.email=test@example.invalid",
             "-c", "commit.gpgsign=false", "commit-tree", bad_tree], input=b"synthetic bad pin test\n", stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, check=True).stdout.strip().decode("ascii")
        errors, files, pins = check_revision(repo, bad_commit)
        assert files == 2 and pins == 0 and len(errors) == 2 and all("40-hex" in e for e in errors)
        action.unlink()
        action.symlink_to("../.github/workflows/test.yaml")
        git(repo, "add", ".")
        symlink_tree = git(repo, "write-tree").strip().decode("ascii")
        symlink_commit = subprocess.run(
            ["git", "-C", directory, "-c", "user.name=Synthetic Test", "-c", "user.email=test@example.invalid",
             "-c", "commit.gpgsign=false", "commit-tree", symlink_tree], input=b"synthetic symlink test\n", stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, check=True).stdout.strip().decode("ascii")
        try:
            check_revision(repo, symlink_commit)
        except Rejected as error:
            assert "regular git blob" in str(error)
        else:
            raise AssertionError("metadata symlink was accepted")
    print(f"PASS: {len(cases)} syntax controls; 2 allowed guards; git positive, candidate negative, worktree isolation, symlink rejection")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--revision", help="full candidate commit SHA, read as git data")
    parser.add_argument("--repo", default=".", help="repository containing candidate git objects")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        if args.revision:
            parser.error("--self-test cannot be combined with --revision")
        self_test()
        return 0
    if not args.revision:
        parser.error("--revision is required (or use --self-test)")
    try:
        errors, files, pins = check_revision(args.repo, args.revision)
    except (Rejected, UnicodeError, OSError) as error:
        print(f"action-pins: rejected candidate: {error}", file=sys.stderr)
        return 1
    for error in errors:
        print(error, file=sys.stderr)
    print(f"action-pins: {files} metadata files; {pins} pinned uses; {len(errors)} violations")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
