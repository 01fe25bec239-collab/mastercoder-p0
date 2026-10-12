# PRESTART CHECKLIST — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION (lane 06, operator: master)

Complete every box before the first A1 session. Anything you cannot complete: write it under "Open items" — the lane's A1 will treat it as a recorded blocker instead of stalling silently. Machine-wide items (tools, GitHub, accounts) are done once; tick them in every lane.

## A0. If your laptop runs Windows (do this first)
Everything for this program runs **inside WSL2 Ubuntu**, not in native Windows (the hooks are bash, the tooling is Linux, symlinks must work).
- [ ] PowerShell (admin): `wsl --install -d Ubuntu`, reboot, create the Ubuntu user. Check with `wsl -l -v` (VERSION 2).
- [ ] Docker Desktop → Settings → General: "Use the WSL 2 based engine"; Resources → WSL integration: enable Ubuntu. (Or install Docker Engine inside Ubuntu.)
- [ ] Put **all** project files in the Linux home, e.g. `~/mastercoder/`, never under `/mnt/c/...` (slow, breaks permissions and symlinks). Unzip your package there: `unzip <zip> -d ~/mastercoder`.
- [ ] Install git, gh, python3, rustup/cargo (and node/pnpm if listed) **inside Ubuntu**. Run Claude Code / Codex **inside Ubuntu**; in Cursor use "Connect to WSL" and open the folder from `~/mastercoder`.
- [ ] Browsers and Windows Hello stay on Windows. Dev servers in WSL are reachable from Windows at `localhost`.
- [ ] Windows edition: Hyper-V experiments (W03, mandatory Windows cell) need Windows 10/11 **Pro, Enterprise or Education** on the lab laptop (Home has no Hyper-V).

## A. Accounts and access
- [ ] GitHub account with 2FA; SSH key added; **SSH commit signing** configured (`git config --global gpg.format ssh; git config --global user.signingkey ~/.ssh/id_ed25519.pub; git config --global commit.gpgsign true`).
- [x] You own the private repo `mastercoder-p0` (your personal account with GitHub Pro); you are the sole CODEOWNER.
- [x] `gh auth login` done; `gh repo view <org>/mastercoder-p0` works.
- [ ] Password manager entry "mastercoder-staging-lane06" for this lane's staging secrets.
- [ ] Access to the private evidence store (created by the W06 lane, W06-T010).

## B. Tools (record versions in memory.md "Environment")
- [x] git ≥ 2.40, gh CLI
- [x] rustup with the toolchain pinned in `/rust-toolchain.toml`; `cargo fmt`, `clippy`; `cargo install cargo-deny gitleaks sccache` (or brew gitleaks sccache). `sccache` lets 15 parallel worktrees share compiled crates (`RUSTC_WRAPPER=sccache` in the lane `.env`).
- [x] Docker / Docker Compose (local PostgreSQL)
- [x] Python ≥ 3.10 (architecture checker), `jq`, `psql` client
- [ ] Node LTS + pnpm (`corepack enable`), Playwright browsers
- [ ] A coding-agent tool you will use for A1/A2/A3 sessions (e.g. Claude Code or Codex CLI), able to read/write files and run shell commands in the worktree, and a plan/rate limit that allows ~15 A2 + 6 A1 + A3 sessions at once
- [ ] Machine: a MacBook Pro M4 Pro with 24 GB RAM runs 5 parallel A2s comfortably. A full wave of 15 works with `CARGO_BUILD_JOBS=2` (in every lane `.env`), `sccache` and Docker Desktop limited to about 4 GB, because the A2s rarely all compile at the same moment. If macOS starts swapping heavily (Activity Monitor → Memory Pressure red), lower the lane caps. Keep ≥100 GB free disk; the A1 removes finished worktrees and their `target/` (AGENTS.md §3b)

## C. Workstream-specific
- [ ] Admin access to the `mastercoder-p0` repo settings (you are the repo owner) to apply branch protection and secret scanning.
- [ ] Two cloud accounts with different root owners: account A (you) and account B (administered by a second, independent administrator — the one role master cannot fill alone; see COMMON/PROGRAM_README.md, section What still needs a second person) for P0-07D; no shared billing root admin, CI credential or recovery contact.
- [ ] A private object-lock bucket for the evidence store (not public, not in the repo).
- [x] Install: cargo-deny, cargo-audit, cargo-cyclonedx, gitleaks, syft, tshark, k6 (or a Rust load tool), MinIO client `mc`.
- [ ] Approval-key profile: as the single operator you qualify INDIVIDUAL_OFFLINE (create an offline recovery key, store it offline). DUAL_ADMIN (G-053) needs a second independent custodian; without one that cell stays DISABLED.

## D. Workspace setup (automated)

Run `python3 setup_workspace.py` in this folder. It is safe to run any time: it never deletes anything, fills the machine-specific paths into `.env`, checks your tools, verifies the architecture, and after G0 clones the repo, enables the hooks, creates the COORDINATION worktree, symlinks `.env` and records ARCH_IMPORT_COMMIT. It ends with `[TODO]` items for anything only you can do. `python3 setup_workspace.py --check` only reports.

Manual equivalent (for understanding or if the script reports a problem):
```bash
cd LANE_06_PLATFORM_OSS_QUALIFICATION
git clone <MC_REPO_URL> repo
mkdir -p worktrees
git -C repo config core.hooksPath .githooks          # guardrail hooks (write set, secrets, no push to main)
git -C ../WORKSPACE/repo worktree add ../WORKSPACE/worktrees/COORD-ws06 -b ws06/coordination origin/main   # this lane's A1 works here
ln -s "$PWD/.env" "../WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/.env.local"   # every worktree of this lane gets this symlink (CREATE commands include it)
python3 ../COMMON/verify_architecture.py repo/ARCHITECTURE/V2_1          # expect mismatches: []
python3 ../COMMON/verify_architecture.py ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1  # expect mismatches: []
(cd repo && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)        # expect p0_executed:false
(cd repo && git log --oneline -1)                                         # note ARCH_IMPORT_COMMIT
```
- [ ] All commands succeeded; ARCH_IMPORT_COMMIT written into `../WORKSPACE/worktrees/COORD-ws06/ws06-platform-oss-qualification/coordination/memory.md` §Environment.
- [x] The COORDINATION worktree exists on branch `ws06/coordination`. From now on its `ws06-platform-oss-qualification/coordination/` copy is authoritative.
- [x] Hooks active: `git -C repo config core.hooksPath` prints `.githooks`.
- [ ] Agent tool guardrails set (AGENT_TOOLING/README.md → Guardrails) and tested: starting a session in the COORDINATION worktree makes the agent cite the WORK LOCATION rule.

## E. Knowledge (read before the first session)
- [ ] `COMMON/PROGRAM_README.md`, `MASTER_SCOPE.md`, `WORKSTREAM_MATRIX.md`, `DEFINITION_OF_DONE.md`
- [ ] `AGENTS.md` (this folder) end to end
- [ ] `SKILLS.md` (this folder), at least §0, §1, §6, §13 and §14
- [ ] `HUMAN_OPERATOR_GUIDE.md` (this folder) end to end
- [ ] `TROUBLESHOOTING.md` (this folder), at least the SECURITY INCIDENT section
- [ ] `00_READ_ME_FIRST.md`, `00A`, `00D`, `00E`, `00F` in the architecture
- [ ] The "MUST READ" rows of `ARCHITECTURE_MAP.md`

## F. Gates
- [ ] G0 recorded (repo exists, protection on) — date: ____
- [x] G1 signed by master — G1 file SHA-256: 2b5253f7739ed7b4a434053af256887f8a84b2e6d0def4cc56744eec10692712 (owner typed chat signature; recorded in memory.md)
- [ ] I understand G2/G3 are needed before any claim RUN.

## Open items
- Startup check 2026-10-12: see `NOTICES/STARTUP_2026-10-12.md`. G0 and PRESTART sign-off remain open.
- Signing key registered by owner and local SSH signing verified; account 2FA status not independently checked.
- Only ~41 GiB free, below the 100 GB checklist; Docker memory tuning to about 4 GB remains open.
- Future-task resources remain unprovisioned; setup reports placeholders. No fabricated accounts, credentials, host access or evidence-store readiness.
- Complete the lane-specific knowledge/read and manual-access checks before signing PRESTART.
-

Signed: master ______  Date (UTC): ______
