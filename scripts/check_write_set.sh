#!/usr/bin/env bash
# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure). Owner: W06.
# Single source of the write-set rule, used by:
#   - .githooks/pre-commit                          (local convenience, staged files)
#   - .github/workflows/common-write-set-guard.yml  (server side; runs THIS FILE FROM THE PROTECTED BASE REVISION,
#                                                    never from the PR candidate — audit F-2)
# Two rules must both hold (audit F-1):
#   1. Broad ownership: the branch's workstream may write only its own paths (COMMON/WRITE_OWNERSHIP_MATRIX.md).
#   2. Exact dispatch scope: a task branch ws0N/<task-id-lowercase>[-rN] may write only the EXACT WRITE SCOPE of that
#      task, read from <ws dir>/coordination/WRITE_SCOPES.json of the revision this script was loaded from
#      (generated from task.md; ad-hoc FIX/LOCKFILE_SYNC tasks are added to it by a reviewed coordination PR first).
# Usage: <newline-separated file list on stdin> | check_write_set.sh <branch-name> [<candidate-rev>]
#   <candidate-rev> = commit whose blob sizes are checked (CI: the PR head). Omitted = the git index (pre-commit).
set -eu
branch=${1:?usage: check_write_set.sh <branch> [<candidate-rev>] < files}
rev=${2:-}
TRUSTED=$(cd "$(dirname "$0")/.." && pwd)   # tree that supplied this script (= scopes source)
python3 -c '
import json, re, subprocess, sys
from pathlib import Path
branch, rev, trusted = sys.argv[1], sys.argv[2], Path(sys.argv[3])
def fail(msg):
    print("BLOCKED (write-set guard): " + msg, file=sys.stderr)
    print("Fix the change, not the guard. See TROUBLESHOOTING.md (Git). Never use --no-verify.", file=sys.stderr)
    sys.exit(1)
m = re.fullmatch(r"ws(0[1-6])/([a-z0-9][a-z0-9-]*)", branch)
if branch == "main": fail("no direct commits/PRs from main; use a ws0N/ branch")
if branch == "DETACHED": fail("detached HEAD (A3 review worktrees never commit)")
if not m: fail("branch %r must be ws0N/<task-id-lowercase>[-rN] or ws0N/coordination" % branch)
n, rest = m.groups()
wsdirs = [p.name + "/" for p in trusted.glob("ws%s-*" % n) if p.is_dir()]
if len(wsdirs) != 1: fail("cannot find exactly one ws%s-*/ directory in the trusted tree" % n)
wsdir = wsdirs[0]
def rx(glob):  # "*" = one path segment, "**" = any depth; braces are pre-expanded by the generator
    out, i = "", 0
    while i < len(glob):
        if glob.startswith("**", i): out += ".*"; i += 2
        elif glob[i] == "*": out += "[^/]*"; i += 1
        else: out += re.escape(glob[i]); i += 1
    return re.compile(out + r"\Z")
if rest == "coordination":
    allow, deny, label = [rx(wsdir + "coordination/**")], [], "coordination records"
elif n == "06" and rest.startswith("arch-import-"):
    allow, deny, label = [rx("ARCHITECTURE/**")], [], "architecture import (adjudicated CCR release only)"
else:
    task = re.sub(r"-r[0-9]+$", "", rest).upper()
    sf = trusted / wsdir / "coordination" / "WRITE_SCOPES.json"
    if not sf.is_file(): fail("missing trusted %s" % sf.relative_to(trusted))
    rec = json.loads(sf.read_text()).get("tasks", {}).get(task)
    if not rec: fail("no dispatch scope for task %s in %s (A1 adds FIX/LOCKFILE_SYNC scopes via a reviewed coordination PR)" % (task, sf.relative_to(trusted)))
    allow, deny, label = [rx(g) for g in rec["allow"]], [rx(g) for g in rec.get("deny", [])], "task " + task
def owned(f):  # rule 1, broad ownership
    if f.startswith("ARCHITECTURE/"): return n == "06" and rest.startswith("arch-import-")
    if f.startswith(wsdir + "coordination/"): return rest == "coordination"
    if f.startswith(wsdir) or f.startswith(".github/workflows/ws%s-" % n): return True
    if n == "06": return not re.match(r"ws0[1-5]-|\.github/workflows/ws0[1-5]-", f)
    return False
def size(f):
    try: return int(subprocess.run(["git", "cat-file", "-s", (rev + ":" if rev else ":") + f], capture_output=True, text=True, check=True).stdout)
    except Exception: return 0   # deleted file
bad = []
for f in (l.strip() for l in sys.stdin):
    if not f: continue
    ok = owned(f) and any(a.match(f) for a in allow) and not any(d.match(f) for d in deny)
    if re.search(r"(^|/)(\.env|\.env\.local|[^/]*\.pem|[^/]*\.key|[^/]*\.p12|[^/]*\.pfx|id_rsa[^/]*|id_ed25519[^/]*)$", f): ok = False
    if ok and size(f) > 5242880: ok = False
    if not ok: bad.append(f)
if bad: fail("files outside the write set of %s on %r (or secret-like / >5 MB): %s" % (label, branch, " ".join(bad)))
' "$branch" "$rev" "$TRUSTED"
