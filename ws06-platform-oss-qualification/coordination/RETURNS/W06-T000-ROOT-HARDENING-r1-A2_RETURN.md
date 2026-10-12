# W06-T000-ROOT-HARDENING r1 — A2 RETURN

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

```text
FROM: W06-A2[W06-T000-ROOT-HARDENING-r1]
TO: STARTUP-A1 (acting W06-A1)
MESSAGE_TYPE: RETURN
TASK_ID: W06-T000-ROOT-HARDENING
ATTEMPT: r1
DISPOSITION: NOT_READY: LOCAL_COMPLETE_PENDING_PR_CI
DATE_UTC: 2026-10-12T19:20:14Z
WORKSTREAM: WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION
LOCATION:
  REPO: mastercoder-p0 (git@github.com:01fe25bec239-collab/mastercoder-p0.git)
  WORKTREE: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r1
  BRANCH: ws06/w06-t000-root-hardening-r1
  WORKSTREAM_DIR: ws06-platform-oss-qualification/
  COORDINATION_DIR: /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination
FROZEN_ARCHITECTURE:
  ARCH_ID: MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1
  REGISTRY_SHA256: b516004c690dd86860c263334d9f32f79a6c935ac82f2b801d3117223ed8d787
  ARCH_IMPORT_COMMIT: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e

TASK_ID: W06-T000-ROOT-HARDENING
BASE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
IMPLEMENTATION_SHA: b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
TREE: af193b0df3b88e64f0616b8d04d61799ba0d5045
SOLE_PARENT: 5a48f18aa52532d2b4e8d055911e6c067da2054d
ACCEPTED_SHA: NONE
COMMITS:
  - b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
  - 5a48f18aa52532d2b4e8d055911e6c067da2054d
CHANGED_FILES:
  .editorconfig                                      |   1 +
  .gitattributes                                     |   1 +
  .github/workflows/common-arch-check.yml            |  63 +++-
  .github/workflows/common-write-set-guard.yml       |  28 +-
  rust-toolchain.toml                                |   4 +-
  scripts/check_action_pins.py                       | 298 +++++++++++++++++
  scripts/check_write_set.sh                         | 355 +++++++++++++++++----
  scripts/verify_architecture.py                     | 272 ++++++++++++++--
  ws06-platform-oss-qualification/docs/branch-protection.md | 133 ++++++++
  9 files changed, 1036 insertions(+), 119 deletions(-)
WRITE_SET_CHECK: PASS: exactly the nine authorized paths; protected-base scope check accepts actual implementation objects; architecture unchanged; clean product worktree.
TESTS_ADDED:
  - scripts/check_write_set.sh::isolation_test — actual Git helper compiled against synthetic source, switched to scratch; sentinel HEAD/index/status/file bytes remain unchanged — POSITIVE/NEGATIVE
  - scripts/check_write_set.sh::self_test — original task HEAD/raw index/staged entries/status/tracked and untracked file-byte sentinel around full fixture — POSITIVE/NEGATIVE
  - scripts/check_write_set.sh::scratch_self_test — 12 scoped positive/negative table cases plus combined self-weakening; base guard and verifier reject the same hostile candidate — POSITIVE/NEGATIVE
  - scripts/verify_architecture.py::self_test — 16 named positive/missing/hash/pin/extra/symlink/FIFO/P0/path groups — POSITIVE/NEGATIVE
  - scripts/check_action_pins.py::self_test — 43 syntax controls including literal/folded hidden target events, two permitted guard controls and real Git positive/negative/worktree-isolation/symlink controls — POSITIVE/NEGATIVE
  - RETURN Appendix A::offline workflow replay — seven candidate scenarios using exact implemented workflow run blocks, including trusted specification output and hostile export-ignore — POSITIVE/NEGATIVE
  - RETURN Appendix B::mutation controls — five predicate/default regressions must fail in disposable copies — NEGATIVE
  - RETURN Appendix C::actual candidate scope and safe diagnostics — actual SHA against imported-base scopes, plus bounded escaped Git stderr — POSITIVE/NEGATIVE
TESTS_RUN:
  - All test/build/scan commands used shared wrapper /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py; actual wrapper invocations and cwd below.
TEST_RESULTS:
  - Final-SHA command table below; all applicable local assertions exit 0.
  - python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py -> exit 0; 85 ACCEPT_ENCODING, 355 REJECT_ENCODING, 40 semantic cases; p0_executed:false.
  - Full-history gitleaks -> exit 1; 26 baseline findings, exactly two unchanged W01 INPUT CONTRACTS lines repeated across 13 commits; not a clean full scan.
  - Staged and implementation-range scans -> exit 0; zero findings.
  - Remote scratch PR / real CI -> NONE, pending authorization and A1 orchestration.
ARCHITECTURE_CONTRACTS_USED:
  - 00D INV-02/03/05/07/11/18/20: bind exact bytes, fail closed, no hostile code with credentials, honest evidence, no production grant.
  - 00F_ATOMIC_PROTOTYPE_STATUS.md: all claims NOT_RUN; local fixture checks are not P0 execution.
  - 00F_TECHNOLOGY_STACK.md: standard library first; frozen stack unchanged.
  - 36_SUPPLY_CHAIN.md: immutable build/action pins and isolated trust boundaries.
  - 49_CANONICAL_PROTOCOL_GLOSSARY.md: run documentation checker before export; distinguish product Mastercoder Review from development CI.
  - W06-T000 task / audit F-1/F-2 and live AGENTS.md: protected-base scopes, ownership and candidate-as-data checks.
SCHEMAS_USED:
  - No product record types implemented or redefined. Pinned SCHEMAS/REGISTRY.json and TRACEABILITY/PACKAGE_FILES_SHA256.json verified as immutable data.
  - Canonical trusted check_spec.py checks wire-vectors.json, semantic-vectors.json and registered schema projections (85 contracts).
DOUBLES_USED: NONE
ASSUMPTIONS:
  - NONE that weaken invariants. Fixture base snapshots contain implementation drafts only to exercise new trusted logic locally; actual import-base scope is independently checked against actual implementation objects.
  - New guard/lint are not claimed to be deployed on remote main; bootstrap CI cannot prove this implementation.
KNOWN_LIMITATIONS:
  - PROTOTYPE_ONLY: disposable root hardening; G2 not passed, G3 absent, claims NOT_RUN, production unauthorized.
  - Pin lint deliberately permits a documented conservative YAML subset and rejects local/Docker uses under the task's every-uses-SHA policy; future syntax expansion requires scoped review.
  - Offline workflow replay omits the authenticated remote fetch/checkout step; its trust layout is constructed locally. Ubuntu runner/provider behavior and real PR checks remain pending.
  - Cargo/database checks are inapplicable before W06-T001/T002: no root/lane Cargo.toml or lane compose.yaml; no crate name invented, toolchain install or unrelated DB borrowed.
  - Existing six lane CI placeholders are not compilation/qualification evidence.
BLOCKERS:
  - Owner/A1: actual remote green/red scratch PR/CI links for every task-required case, including main/valid candidate, architecture byte/extra tampering, unpinned action and combined self-weakening; remote write-scope/title cases remain pending as applicable.
  - Only mandatory remote CI evidence remains for acceptance. Eligible for preliminary independent review under BUILD_REVIEW_CI_SEQUENCE_DECISION.md; not final READY/PASS/ACCEPTED.
CCR_DRAFTED: NONE
SECRETS_CHECK:
  - gitleaks protect --staged --no-banner --redact -> exit 0; zero findings.
  - gitleaks detect --no-banner --redact --log-opts 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 -> exit 0; two commits scanned, zero findings.
  - gitleaks detect --no-banner --redact --report-format json --report-path /tmp/w06-r1-history-redacted.json -> exit 1; 50 commits scanned, 26 repeated known baseline findings; no true new secret.
MEMORY_UPDATES:
  - Known pitfalls — KI-001 repaired: cwd=trusted captured the original repository at definition; dynamic cwd and safe preflight isolation prevent fixture escape. Full self-test verifies original HEAD/index/status/file bytes unchanged.
  - Known pitfalls — trusted check_spec.py emits a JSON report followed by a semantic summary; json.load on all output fails Extra data. Workflow now parses the leading trusted JSON and explicitly requires p0_executed:false.
  - Known pitfalls — non-guard block scalar on values hid target events from the draft lint; literal/folded controls now reject them. Provider acceptance of such a form was not remotely tested; local lint acceptance was reproduced.
  - Environment — actual python3 is 3.13.5, differing from historical bootstrap memory 3.14.6. My import probe generated one bytecode file; it was removed, and subsequent inline imports use -B. No foreign dirt remains.
  - Environment — full-history gitleaks scans shared refs: 26 findings = two known lines repeated in 13 commits, not 26 unique secrets. Never suppress them or call a nonzero scan clean.
  - Environment — final free disk 29.41 GiB; every test/scan and commit hook ran under the single-build wrapper.
  - Decisions — CODEOWNERS is required as ownership/routing documentation; bootstrap code-owner approval is false with zero owner-PR approvals. Independent fresh exact-head A3 remains mandatory. Runbook does not change settings.
  - Interface pins — Rust 1.97.1 bootstrap pin retained, official release verified; no new dependency or environment variable.
PUSH / PR / MERGE: NONE
READY_FOR_A3_REVIEW: NO
```

## Recovery and problem handling

Entry branch/base/tree matched the dispatch; the index was at the import base.
All eight recorded recovered hashes, modes and sizes matched before any edit.
No operations or commits targeted preserved r0.

| Recovered path | Verified SHA-256 |
| --- | --- |
| .editorconfig | 13897a0ac0a682b357c71757d16166ee62e8caec0a739ba20e545b8feca179c4 |
| .gitattributes | 35e3e511b531b69c39bf3051d39884e132884f3c1b19590e3be7420f48c82ff0 |
| .github/workflows/common-arch-check.yml | b24bc42c42fe516e1861358b0e2a89b0a3a3a3ab80e0cb416b96407f1dfb38ce |
| .github/workflows/common-write-set-guard.yml | 030b3130e5f35f6fd6124deb9f07a171be4d25e169dbc2a721151f8076eb8c48 |
| rust-toolchain.toml | 96455c1c2b49754d89bec0951dc3b6fb3e5660d9b1da150d398c079ede5696ff |
| scripts/check_action_pins.py | 3f440bef0268011ee2fdd8d94d18a323a3157ddafb4f870e81b58b8aa3ab484b |
| scripts/check_write_set.sh | 3b575899e4a9dbb61ed085281dde27e4f20f1a2885348b99c8e9f485da0e059d |
| scripts/verify_architecture.py | 2fe1d2aef39fc21efaff02dec6701320d894f809508634a817d0bf7fb480f471 |

Class TEST, existing KI-001: extracted actual helper reproduced the stale default
in two disposable repositories; scratch commit-tree exited 128 with
`fatal: not a git repository`. No live task init/add operation was used for
the red reproduction. Repair was proposed before edits. The isolated actual-helper
test passed before the full self-test; the mutant stale default now fails in
disposable copies. Safe Git stderr is escaped and limited to 4096 decoded characters.
Product signing remained enabled; unsigned commit-tree overrides occur only
inside disposable fixtures.

Additional bounded repairs came from the draft audit: hidden block scalar events
and the valid checker's trailing semantic summary. Both were reproduced locally
before editing, corrected in scoped files and verified through the wrapper.
No guard was weakened, no unrelated dependency added, no claims run.

Two read-only helpers were requested through host-exposed model IDs:
`gpt-6.1-sol/high` (security draft audit) and `gpt-6-luna/xhigh` (runbook facts).
The host accepted both requests, but helper sessions cannot expose independent
actual-runtime model/effort or precise context telemetry; these are **UNVERIFIED**,
not silently claimed verified. Helpers made no edits or tests and did not act
as A3. Parent personally checked their findings and reran every assertion.
Skills used: bundled diagnosing-bugs and ponytail; project scope/gates remained
authoritative. No memory.md/task.md or other coordination file was edited.

## Final-SHA commands and results

All commands in the following table ran with cwd `/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r1` through:

```bash
python3 /Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/run_single_build.py --cwd "/Users/omkar/Documents/A1_Mastercoder/MASTERCODER_6_WORKSTREAM_PROGRAM/WORKSPACE/worktrees/W06-T000-ROOT-HARDENING-r1" -- COMMAND ARGUMENTS
```

No test used another task's build directory or database. Appendix heredocs were
passed directly as `python3 -B -` after the wrapper's `--`; they are recorded
below so the independent reviewer can reproduce the exact probes.

| Command after wrapper `--` | Exit | Actual result |
| --- | --- | --- |
| bash scripts/check_write_set.sh --isolation-test | 0 | Synthetic source HEAD/index/status/bytes unchanged; scratch owns Git directory and HEAD |
| bash scripts/check_write_set.sh --self-test | 0 | 12 table cases, combined rejection, isolation and original-task sentinels pass; fixtures removed |
| python3 scripts/verify_architecture.py ARCHITECTURE/V2_1 | 0 | 235 manifest files, 3 pins, mismatches: [], p0_executed:false |
| python3 scripts/verify_architecture.py ARCHITECTURE/V2_1 --self-test | 0 | 16 named groups pass, none ignored |
| python3 scripts/check_action_pins.py --self-test | 0 | 43 syntax controls, 2 permitted guards, Git positive/negative/isolation/symlink checks pass |
| python3 scripts/check_action_pins.py --revision b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 | 0 | 8 metadata files, 0 uses (workflows use Git directly), 0 violations |
| python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py | 0 | 85 accepted / 355 rejected encoding vectors; 40 semantic cases; p0_executed:false |
| git diff --check 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 | 0 | No whitespace errors |
| git verify-commit b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 5a48f18aa52532d2b4e8d055911e6c067da2054d | 0 | Both SSH signatures good; log status G for both commits |
| gitleaks detect --no-banner --redact --log-opts 58b6ab38b0a514d94d8851679ad66ef9bf696c9e..b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03 | 0 | 2 implementation commits; zero findings |
| gitleaks detect --no-banner --redact --report-format json --report-path /tmp/w06-r1-history-redacted.json | 1 | 50 shared-history commits; 26 known repeated baseline findings; not clean |
| Appendix A, python3 -B - heredoc | 0 | Seven exact workflow scenarios pass locally; no candidate code execution |
| Appendix B, python3 -B - heredoc | 0 | Five mutants rejected; baseline multiplicity confirmed; absent prerequisites documented |
| Appendix C, python3 -B - heredoc | 0 | Actual candidate scope pass; safe stderr test pass; final clean SHA/tree/syntax/scope verified |

Precommit checks also ran under the wrapper: staged ownership check via
`sh -c 'git diff --cached --name-only | bash scripts/check_write_set.sh ws06/w06-t000-root-hardening-r1'`
exit 0; `gitleaks protect --staged --no-banner --redact` exit 0;
`git diff --check` exit 0. Both `git commit -S` invocations used the wrapper
so mandatory hooks inherited the shared slot. Official links/hardcoding scan
found only fixed provider/documentation URLs, no machine paths/account IDs,
secrets or new configuration. Shell, embedded Python, Python and TOML parsing
passed. `git diff --name-only 58b6ab38b0a514d94d8851679ad66ef9bf696c9e`
returned exactly the nine paths above.

Earlier full-history scan before commits: exit 1, 48 commits, same 26 findings;
each is generic-api-key in `ws01-core-security/coordination/task.md` at line 822
or 932. Appendix B compares every affected historical line to the imported
baseline and confirms it is an identical INPUT CONTRACTS type-name list.
There were no implementation-range or staged findings; no suppression added.
Do not report history as clean.

`cargo fmt --check`, lane `cargo fmt --all --check`,
`cargo clippy -p <crate> --all-targets -- -D warnings`, workspace locked
clippy/test and `docker compose ... ps --status running` were **not run**:
root/lane Cargo.toml and lane compose.yaml are absent, as anticipated by the
dispatch. Appendix B verified absence; no out-of-scope manifest, crate, DB
or dependency was invented. This is inapplicable scaffold work, not a pass.

## Criterion-to-evidence checklist

| Exact task criterion/test | Local evidence at implementation SHA | Remote required evidence |
| --- | --- | --- |
| scratch PR changing one byte under ARCHITECTURE/ -> arch-check red | Appendix A byte case; verifier one_byte specific HASH rejection | PENDING |
| scratch PR adding an extra file under ARCHITECTURE/ -> arch-check red | Appendix A extra case; verifier EXTRA rejection | PENDING |
| main -> arch-check green | Appendix A trusted fixture positive, canonical verifier/spec positive | PENDING for implementation; existing bootstrap runs are not substituted |
| scratch unpinned uses -> pin lint red | Appendix A floating candidate; 43 syntax controls | PENDING |
| python3 scripts/verify_architecture.py ARCHITECTURE/V2_1 -> mismatches: [] | Exit 0; 235 files, 3 pins, [] | Local command MET |
| scratch task branch touching a neighbouring task's file -> write-set-guard red | guard neighbouring task file: outside the write set | PENDING where remote scratch evidence required |
| scratch task branch touching coordination/memory.md -> write-set-guard red | guard coordination/memory.md: outside the write set | PENDING where remote scratch evidence required |
| scratch harness branch touching excluded harness/Cargo.toml -> write-set-guard red | guard excluded harness/Cargo.toml: outside the write set | PENDING where remote scratch evidence required |
| scratch branch for task absent from WRITE_SCOPES.json -> write-set-guard red | guard absent task: no dispatch scope | PENDING where remote scratch evidence required |
| scratch PR whose title names another task -> write-set-guard red | guard title mismatch and two-task title rejection | PENDING |
| scratch PR edits guard/verifier/workflow to pass plus ARCHITECTURE/ and foreign path -> both checks red | guard combined case and Appendix A combined candidate: both specific rejections | PENDING |
| pin lint active | implemented arch workflow calls trusted lint on candidate revision; Appendix A exercises exact command | Remote deployment/CI PENDING |
| runbook written | docs/branch-protection.md committed; policy and official sources checked | MET locally; no settings mutation |
| exact stable toolchain / notice | retained Rust 1.97.1; official release checked | A1 sends all-lane notice before merge |
| signed scoped clean implementation | two verified signed commits, exact 9 paths, clean task tree | MET locally |
| all mandatory green/red CI links | NONE created or claimed | NOT_MET; sole acceptance blocker |

CI_LINKS: NONE. REVIEW_VERDICT: NONE. CLAIM_RUNS: NONE.
No final completion, A3 PASS, self-approval, acceptance or integration is claimed.

## Pre-RETURN checklist

- [ ] Every acceptance criterion has final mandatory evidence: real PR/CI pending.
- [x] Every local negative task case exists, asserts the relevant rejection and passes.
- [x] Mutation spot checks catch predicate deletion and stale cwd.
- [x] Exact nine-path diff; no architecture or coordination task-branch changes.
- [x] No invented schema/digest/authority/enum/state; canonical pins retained.
- [x] No new secrets; staged/range clean. Full history is honestly nonzero baseline.
- [x] Prototype headers; no new configuration or dependencies.
- [x] All applicable local commands rerun at final SHA; absent workspace/DB explicit.
- [x] Assumptions, limitations, helpers' missing telemetry recorded.
- [x] MEMORY_UPDATES provided; memory.md/task.md untouched.
- [x] Product worktree clean; implementation SHA/tree and signatures verified.
- [x] Coordinator worktree existed clean before RETURN, branch ws06/coordination,
      HEAD 76c9f7f8eb196f9830fd0161405b3fde1bc525d6,
      tree c64e8950fd24186231929a52540db189e0e1ddfa.
      This RETURN is the only authorized new coordinator dirt.
- [x] No unresolved implementation guesses or provider mutations.
- [x] Handoff prompt directs A1 to process this RETURN; no further task work.

## Reproduction appendices

For each appendix, run at the task root, through the exact shared wrapper above,
using `-- python3 -B - <<'PY'`, the Python body below, then `PY`.
These are offline disposable fixtures, not real provider PRs.

### Appendix A — exact workflow replay

```python
import os,re,shutil,subprocess,tempfile,textwrap,json
from pathlib import Path

source=Path.cwd()
workflow=(source/'.github/workflows/common-arch-check.yml').read_text().splitlines()
steps=[]
for i,line in enumerate(workflow):
 if line.startswith('        run:'):
  command=line.split('run:',1)[1].strip()
  if command=='|':
   body=[]
   for nextline in workflow[i+1:]:
    if nextline.strip() and not nextline.startswith('          '):break
    body.append(nextline[10:] if nextline.startswith('          ') else '')
   command='\n'.join(body)
  steps.append(command)
assert len(steps)==5,steps
assert 'git checkout -q --detach "$TRUSTED_SHA"' in steps[0]
assert 'git checkout' not in '\n'.join(steps[1:])

def git(repo,*args,data=None):
 return subprocess.run(['git','--no-optional-locks','-C',str(repo),*args],input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True).stdout
def commit(repo,parent=None):
 tree=git(repo,'write-tree').decode().strip()
 args=['-c','user.name=Synthetic','-c','user.email=fixture@example.test','-c','commit.gpgsign=false','commit-tree',tree]
 if parent:args+=['-p',parent]
 return git(repo,*args,data=b'local workflow fixture\n').decode().strip()

with tempfile.TemporaryDirectory(prefix='w06-workflow-replay-') as tmp:
 repo=Path(tmp)/'repo';repo.mkdir()
 for folder in ('scripts','.github/workflows','ARCHITECTURE'):
  shutil.copytree(source/folder,repo/folder)
 for scope in source.glob('ws0[1-6]-*/coordination/WRITE_SCOPES.json'):
  dest=repo/scope.relative_to(source);dest.parent.mkdir(parents=True);shutil.copyfile(scope,dest)
 git(repo,'init','-q','-b','main')
 git(repo,'add','.')
 base=commit(repo);git(repo,'update-ref','refs/heads/main',base)
 branch='ws06/w06-t000-root-hardening-r1';title='[W06-T000-ROOT-HARDENING] workflow replay'
 cases=[
  ('main -> arch-check green',{},None,None),
  ('valid task -> both checks green',{'.editorconfig':b'fixture\n'},None,None),
  ('scratch PR changing one byte under ARCHITECTURE/ -> arch-check red',
   {'ARCHITECTURE/V2_1/00F_ATOMIC_PROTOTYPE_STATUS.md':b'!' +(source/'ARCHITECTURE/V2_1/00F_ATOMIC_PROTOTYPE_STATUS.md').read_bytes()[1:]},'HASH:00F_ATOMIC_PROTOTYPE_STATUS.md','outside the write set'),
  ('scratch PR adding an extra file under ARCHITECTURE/ -> arch-check red',{'ARCHITECTURE/V2_1/extra.txt':b'extra'},'EXTRA:extra.txt','outside the write set'),
  ('scratch unpinned uses -> pin lint red',{'.github/workflows/common-arch-check.yml':b'name: unpinned\njobs:\n  test:\n    steps:\n      - uses: example/action@main # v1.2\n'},'40-hex',None),
  ('combined self-weakening -> both checks red',{
   'scripts/check_write_set.sh':b'exit 0\n','scripts/verify_architecture.py':b'raise SystemExit(0)\n',
   'scripts/check_action_pins.py':b'raise SystemExit(0)\n',
   '.github/workflows/common-arch-check.yml':b'name: weakened\n',
   '.github/workflows/common-write-set-guard.yml':b'name: weakened\n',
   'ARCHITECTURE/V2_1/00F_TECHNOLOGY_STACK.md':b'tampered\n','ws01-core-security/foreign.rs':b'x'
  },'HASH:00F_TECHNOLOGY_STACK.md','outside the write set'),
  ('candidate export-ignore cannot hide tamper',{
   '.gitattributes':b'ARCHITECTURE/V2_1/00F_TECHNOLOGY_STACK.md export-ignore\n',
   'ARCHITECTURE/V2_1/00F_TECHNOLOGY_STACK.md':b'tampered\n'
  },'HASH:00F_TECHNOLOGY_STACK.md','outside the write set')
 ]
 for number,(name,changes,arch_reason,guard_reason) in enumerate(cases):
  git(repo,'read-tree',base)
  for path,data in changes.items():
   oid=git(repo,'hash-object','-w','--stdin',data=data).decode().strip()
   git(repo,'update-index','--add','--cacheinfo','100644,'+oid+','+path)
  head=commit(repo,base) if changes else base
  runner=Path(tmp)/f'runner-{number}';runner.mkdir()
  env=dict(os.environ,CANDIDATE_SHA=head,RUNNER_TEMP=str(runner))
  outcome=None
  for step in steps[1:]:
   result=subprocess.run(['bash','-euo','pipefail','-c',step],cwd=repo,env=env,capture_output=True,text=True)
   if result.returncode:
    outcome=result
    break
  if arch_reason:
   assert outcome and arch_reason in outcome.stdout+outcome.stderr,(name,outcome)
  else:
   assert outcome is None,(name,outcome.stdout if outcome else '')
   report,_=json.JSONDecoder().raw_decode((runner/'spec.json').read_text())
   assert report['p0_executed'] is False
   assert 'Semantic specification cases: 40' in (runner/'spec.json').read_text()
  guard=subprocess.run(['bash','scripts/check_write_set.sh','--pr',base,head,branch,title],cwd=repo,capture_output=True,text=True)
  if guard_reason:assert guard.returncode==1 and guard_reason in guard.stderr,(name,guard.stderr)
  else:assert guard.returncode==0,(name,guard.stderr)
  assert git(repo,'rev-parse','HEAD').decode().strip()==base
  print('PASS:',name)
print('PASS: exact workflow run blocks replayed from trusted fixture; no candidate code executed; temporary fixtures deleted')

```

### Appendix B — mutation controls and baseline classification

The redacted history report is generated by the full-history scan command above
before this appendix; it contains no unredacted secret material.

```python
import ast,json,os,shutil,subprocess,tempfile
from pathlib import Path
source=Path.cwd()

def git(repo,*args,data=None):
 return subprocess.run(['git','-C',str(repo),*args],input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True).stdout
def fixture(root):
 for folder in ('scripts','.github/workflows','ARCHITECTURE'):
  shutil.copytree(source/folder,root/folder)
 for scope in source.glob('ws0[1-6]-*/coordination/WRITE_SCOPES.json'):
  dest=root/scope.relative_to(source);dest.parent.mkdir(parents=True);shutil.copyfile(scope,dest)
 git(root,'init','-q','-b','main');git(root,'add','.')
 tree=git(root,'write-tree').decode().strip()
 head=git(root,'-c','user.name=Synthetic','-c','user.email=fixture@example.test','-c','commit.gpgsign=false','commit-tree',tree,data=b'mutation fixture\n').decode().strip()
 git(root,'update-ref','refs/heads/main',head)

cases=[
 ('stale cwd sentinel catches regression','scripts/check_write_set.sh','cwd=None, data=None','cwd=trusted, data=None',['bash','scripts/check_write_set.sh','--isolation-test'],'scratch Git escaped'),
 ('scope predicate removal is caught','scripts/check_write_set.sh','reject("files outside the write set: " + repr(path))','pass',['bash','scripts/check_write_set.sh','--self-test'],'neighbouring task file'),
 ('architecture digest predicate removal is caught','scripts/verify_architecture.py','digests[path] != entry["sha256"] or len(contents[path]) != entry["bytes"]','False',['python3','scripts/verify_architecture.py','ARCHITECTURE/V2_1','--self-test'],'one_byte'),
 ('action pin predicate removal is caught','scripts/check_action_pins.py','if not PIN.fullmatch(value):','if False:',['python3','scripts/check_action_pins.py','--self-test'],'floating tag'),
 ('block event predicate removal is caught','scripts/check_action_pins.py','if key == "on" and BLOCK.fullmatch(value):','if False:',['python3','scripts/check_action_pins.py','--self-test'],'target literal block event')
]
for name,path,before,after,command,reason in cases:
 with tempfile.TemporaryDirectory(prefix='w06-mutation-') as tmp:
  root=Path(tmp);fixture(root)
  p=root/path;content=p.read_text();assert content.count(before)==1,(name,before)
  p.write_text(content.replace(before,after))
  result=subprocess.run(command,cwd=root,capture_output=True,text=True)
  assert result.returncode!=0 and reason in result.stdout+result.stderr,(name,result.returncode,result.stdout,result.stderr)
  print('PASS:',name,'-> mutated test exit',result.returncode,'; expected assertion:',reason)

findings=json.loads(Path('/tmp/w06-r1-history-redacted.json').read_text())
base='58b6ab38b0a514d94d8851679ad66ef9bf696c9e'
path='ws01-core-security/coordination/task.md'
base_lines=git(source,'show',base+':'+path).decode().splitlines()
for f in findings:
 assert f['File']==path and f['StartLine']==f['EndLine'] and f['StartLine'] in (822,932)
 lines=git(source,'show',f['Commit']+':'+path).decode().splitlines()
 assert lines[f['StartLine']-1]==base_lines[f['StartLine']-1]
 assert '**INPUT CONTRACTS:**' in lines[f['StartLine']-1]
assert len(findings)==26 and len({f['Commit'] for f in findings})==13
print('PASS: 26 history findings = two unchanged baseline INPUT CONTRACTS lines repeated in 13 commits; no new finding')
assert not (source/'Cargo.toml').exists()
lane=source/'ws06-platform-oss-qualification'
assert not (lane/'Cargo.toml').exists() and not (lane/'compose.yaml').exists()
print('INAPPLICABLE: cargo fmt/clippy/test and docker compose ps: task/root Cargo.toml and lane compose.yaml absent; no workspace/database invented')

```

### Appendix C — actual candidate scopes, diagnostics and final integrity

```python
import ast,json,os,shutil,subprocess,tempfile,tomllib
from pathlib import Path
source=Path.cwd();base='58b6ab38b0a514d94d8851679ad66ef9bf696c9e';head='b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03'
def git(repo,*args):return subprocess.check_output(['git','--no-optional-locks','-C',str(repo),*args])
expected={'.editorconfig','.gitattributes','.github/workflows/common-arch-check.yml','.github/workflows/common-write-set-guard.yml','rust-toolchain.toml','scripts/check_action_pins.py','scripts/check_write_set.sh','scripts/verify_architecture.py','ws06-platform-oss-qualification/docs/branch-protection.md'}
assert set(git(source,'diff','--name-only',base+'..'+head).decode().splitlines())==expected
assert git(source,'rev-parse','HEAD').decode().strip()==head
assert git(source,'status','--porcelain')==b''
subprocess.run(['git','merge-base','--is-ancestor',base,head],check=True)
with tempfile.TemporaryDirectory(prefix='real-candidate-guard-') as tmp:
 repo=Path(tmp)
 git(repo,'init','-q')
 git(repo,'fetch','-q',str(source),base,head)
 git(repo,'checkout','-q','--detach',base)
 # Exercise implementation guard against actual candidate objects and BASE scopes.
 # This local control does not claim the new guard already exists on remote main.
 shutil.copyfile(source/'scripts/check_write_set.sh',repo/'scripts/check_write_set.sh')
 result=subprocess.run(['bash','scripts/check_write_set.sh','--pr',base,head,'ws06/w06-t000-root-hardening-r1','[W06-T000-ROOT-HARDENING] local candidate'],cwd=repo,capture_output=True,text=True)
 assert result.returncode==0 and 'PASS' in result.stdout,(result.stdout,result.stderr)
 print('PASS: actual implementation SHA fits protected-base scopes')
 error=subprocess.run(['bash','scripts/check_write_set.sh','--pr','a'*40,head,'ws06/w06-t000-root-hardening-r1','[W06-T000-ROOT-HARDENING]'],cwd=repo,capture_output=True,text=True)
 assert error.returncode==1 and 'git stderr: ' in error.stderr and 'fatal:' in error.stderr
 diagnostic=error.stderr.split('git stderr: ',1)[1].strip()
 assert isinstance(ast.literal_eval(diagnostic),str) and len(diagnostic)<4200
 print('PASS: Git failure retains bounded escaped stderr')
for p in expected:
 text=(source/p).read_text();assert 'PROTOTYPE — DISPOSABLE — NOT PRODUCTION' in text
 if p.endswith('.py'):ast.parse(text)
 if p.endswith('.sh'):
  subprocess.run(['bash','-n',p],check=True);ast.parse(text.split("<<'PY'\n",1)[1].rsplit('\nPY',1)[0])
assert tomllib.loads((source/'rust-toolchain.toml').read_text())['toolchain']['channel']=='1.97.1'
print('PASS: final scope/headers/Python/shell/TOML; clean original worktree')
print('Final HEAD:',head)
print('Final TREE:',git(source,'rev-parse',head+'^{tree}').decode().strip())
print('Final SOLE_PARENT:',git(source,'rev-parse',head+'^').decode().strip())
print('Free disk GiB:',round(shutil.disk_usage(source).free/2**30,2))

```

