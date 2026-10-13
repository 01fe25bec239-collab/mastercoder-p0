#!/usr/bin/env python3
# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Verify package bytes as data; never execute candidate architecture code."""

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import tempfile


# Pinned identity: canonical COMMON/verify_architecture.py and W06 AGENTS.md §1.
MANIFEST = "TRACEABILITY/PACKAGE_FILES_SHA256.json"
PINNED = {
    MANIFEST: "97c604ecf6aefeec481b1bc8770a8a4c62d7643104d64bfdca4d8b5be5eb9c9d",
    "SCHEMAS/REGISTRY.json": "b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787",
    "TRACEABILITY/atomic-prototype-claims.json": "d2eda91044f761603e127f1946c742f82bf94d80b8930caba47ec17325094db7",
}


def safe_relative_path(value):
    """Manifest paths cannot escape, normalize ambiguously, or use OS aliases."""
    return (
        isinstance(value, str)
        and bool(value)
        and not PurePosixPath(value).is_absolute()
        and not any(part in ("", ".", "..") for part in value.split("/"))
        and "\\" not in value
        and ":" not in value
        and "\x00" not in value
    )


def p0_executed(value):
    if isinstance(value, dict):
        return value.get("p0_executed") is True or any(
            p0_executed(item) for item in value.values()
        )
    if isinstance(value, list):
        return any(p0_executed(item) for item in value)
    return False


def verify(root):
    root = Path(root)
    bad = []
    contents = {}
    directories = set()
    result = {
        "root": str(root),
        "files_checked": 0,
        "pinned_checked": len(PINNED),
        "p0_executed": False,
        "mismatches": bad,
    }

    def walk(directory):
        with os.scandir(directory) as entries:
            for entry in sorted(entries, key=lambda item: item.name):
                path = Path(entry.path)
                relative = path.relative_to(root).as_posix()
                mode = entry.stat(follow_symlinks=False).st_mode
                if stat.S_ISLNK(mode):
                    bad.append("SYMLINK:" + relative)
                elif stat.S_ISDIR(mode):
                    directories.add(relative)
                    walk(path)
                elif not stat.S_ISREG(mode):
                    bad.append("NONREGULAR:" + relative)
                elif not path.resolve().is_relative_to(root.resolve()):
                    bad.append("PATH_ESCAPE:" + relative)
                else:
                    # Reject final-component links and nonregular replacements at open.
                    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
                    with os.fdopen(fd, "rb") as stream:
                        if not stat.S_ISREG(os.fstat(stream.fileno()).st_mode):
                            bad.append("NONREGULAR:" + relative)
                        else:
                            contents[relative] = stream.read()

    try:
        mode = root.lstat().st_mode
        if stat.S_ISLNK(mode):
            bad.append("SYMLINK:ROOT")
            return result
        if not stat.S_ISDIR(mode):
            bad.append("NONDIRECTORY:ROOT")
            return result
        walk(root)
    except OSError as error:
        bad.append("READ_ERROR:" + str(error))
        return result

    digests = {
        path: hashlib.sha256(data).hexdigest() for path, data in contents.items()
    }
    for path, expected in PINNED.items():
        if path not in contents:
            bad.append("MISSING:" + path)
        elif digests[path] != expected:
            bad.append("PINNED:" + path)

    # 00F: local specification validation is never evidence of executed P0 claims.
    for path, data in contents.items():
        if path.endswith(".json"):
            try:
                executed = p0_executed(json.loads(data))
            except (ValueError, UnicodeError, RecursionError):
                bad.append("INVALID_JSON:" + path)
                continue
        else:
            # The frozen DOCUMENT_VALIDATION_REPORT.md embeds this JSON field.
            executed = bool(re.search(rb'"p0_executed"\s*:\s*true\b', data))
        if executed:
            result["p0_executed"] = True
            bad.append("P0_EXECUTED:" + path)

    # Never trust a changed manifest to choose files or filesystem paths.
    if digests.get(MANIFEST) != PINNED[MANIFEST]:
        return result
    try:
        manifest = json.loads(contents[MANIFEST])
        listed = {}
        for entry in manifest["files"]:
            path = entry["path"]
            if not safe_relative_path(path):
                bad.append("PATH_ESCAPE:" + str(path))
                continue
            if path in listed or path == MANIFEST:
                bad.append("DUPLICATE:" + path)
                continue
            listed[path] = entry
        result["files_checked"] = len(listed)
        for path, entry in listed.items():
            if path not in contents:
                bad.append("MISSING:" + path)
            elif digests[path] != entry["sha256"] or len(contents[path]) != entry["bytes"]:
                bad.append("HASH:" + path)
        expected_paths = set(listed) | {MANIFEST}
        bad.extend("EXTRA:" + path for path in sorted(set(contents) - expected_paths))
        expected_directories = {
            parent.as_posix()
            for path in expected_paths
            for parent in PurePosixPath(path).parents
            if parent.as_posix() != "."
        }
        bad.extend(
            "EXTRA_DIRECTORY:" + path
            for path in sorted(directories - expected_directories)
        )
    except (KeyError, TypeError, ValueError, UnicodeError, RecursionError) as error:
        bad.append("INVALID_MANIFEST:" + str(error))
    return result


def self_test(source):
    """Alter isolated package copies, asserting each specific boundary reason."""
    with tempfile.TemporaryDirectory(prefix="architecture-verifier-") as temporary:
        copy = Path(temporary) / "package"
        shutil.copytree(source, copy, symlinks=True)
        positive = verify(copy)
    if positive["mismatches"]:
        raise AssertionError("positive control: " + repr(positive["mismatches"]))
    names = ["positive"]

    def run(name, mutation, expected):
        with tempfile.TemporaryDirectory(prefix="architecture-verifier-") as temporary:
            copy = Path(temporary) / "package"
            shutil.copytree(source, copy, symlinks=True)
            mutation(copy)
            outcome = verify(copy)
            if expected not in outcome["mismatches"]:
                raise AssertionError(name + ": " + repr(outcome["mismatches"]))
        names.append(name)

    document = "00F_ATOMIC_PROTOTYPE_STATUS.md"

    def change_byte(copy):
        path = copy / document
        original = path.read_bytes()
        path.write_bytes(bytes([original[0] ^ 1]) + original[1:])

    run("one_byte", change_byte, "HASH:" + document)
    for extra in ("extra.txt", ".DS_Store"):
        run("extra_" + extra, lambda copy, path=extra: (copy / path).write_bytes(b"extra"), "EXTRA:" + extra)
    run("missing", lambda copy: (copy / document).unlink(), "MISSING:" + document)
    for pinned in PINNED:
        run(
            "tamper_" + pinned,
            lambda copy, path=pinned: (copy / path).write_bytes((copy / path).read_bytes() + b"\n"),
            "PINNED:" + pinned,
        )

    def symlink(copy):
        path = copy / document
        path.unlink()
        path.symlink_to((Path(source) / document).resolve())

    run("file_symlink", symlink, "SYMLINK:" + document)

    def directory_symlink(copy):
        path = copy / "SCHEMAS"
        shutil.rmtree(path)
        path.symlink_to((Path(source) / "SCHEMAS").resolve(), target_is_directory=True)

    run("directory_symlink", directory_symlink, "SYMLINK:SCHEMAS")
    run("nonregular_fifo", lambda copy: os.mkfifo(copy / "extra.fifo"), "NONREGULAR:extra.fifo")
    run("extra_directory", lambda copy: (copy / "extra-directory").mkdir(), "EXTRA_DIRECTORY:extra-directory")

    def mark_executed(copy):
        path = copy / "TRACEABILITY/DOCUMENT_VALIDATION_REPORT.md"
        path.write_bytes(path.read_bytes().replace(b'"p0_executed": false', b'"p0_executed": true'))

    run("p0_executed_true", mark_executed, "P0_EXECUTED:TRACEABILITY/DOCUMENT_VALIDATION_REPORT.md")
    run(
        "p0_executed_json_escaped_key",
        lambda copy: (copy / "extra.json").write_bytes(b'{"p0_\\u0065xecuted": true}'),
        "P0_EXECUTED:extra.json",
    )
    with tempfile.TemporaryDirectory(prefix="architecture-verifier-") as temporary:
        link = Path(temporary) / "package"
        link.symlink_to(Path(source).resolve(), target_is_directory=True)
        if "SYMLINK:ROOT" not in verify(link)["mismatches"]:
            raise AssertionError("root_symlink")
    names.append("root_symlink")
    for path in ("../outside", "/outside", "a/../outside", "a//b", "a\\b", "C:/outside", "a/./b", "", "a\x00b"):
        if safe_relative_path(path):
            raise AssertionError("path_escape: " + repr(path))
    if not safe_relative_path(document):
        raise AssertionError("safe manifest path")
    names.append("path_escape")
    return {"self_test": "PASS", "tests": names, "tests_passed": len(names), "p0_executed": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path, help="architecture package directory")
    parser.add_argument("--self-test", action="store_true", help="test temporary copies of this package")
    args = parser.parse_args()
    if args.self_test:
        try:
            result = self_test(args.root)
        except (AssertionError, OSError) as error:
            print(json.dumps({"self_test": "FAIL", "error": str(error)}, indent=1))
            return 1
    else:
        result = verify(args.root)
    print(json.dumps(result, indent=1))
    return 1 if result.get("mismatches") else 0


if __name__ == "__main__":
    raise SystemExit(main())
