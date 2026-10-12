# PRESTART CHECKLIST — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION (lane 03, operator: master)

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
- [ ] You own the private repo `mastercoder-p0` (your personal account with GitHub Pro); you are the sole CODEOWNER.
- [ ] `gh auth login` done; `gh repo view <org>/mastercoder-p0` works.
- [ ] Password manager entry "mastercoder-staging-lane03" for this lane's staging secrets.
- [ ] Access to the private evidence store (created by the W06 lane, W06-T010).

## B. Tools (record versions in memory.md "Environment")
- [ ] git ≥ 2.40, gh CLI
- [ ] rustup with the toolchain pinned in `/rust-toolchain.toml`; `cargo fmt`, `clippy`; `cargo install cargo-deny gitleaks sccache` (or brew gitleaks sccache). `sccache` lets 15 parallel worktrees share compiled crates (`RUSTC_WRAPPER=sccache` in the lane `.env`).
- [ ] Docker / Docker Compose (local PostgreSQL)
- [ ] Python ≥ 3.10 (architecture checker), `jq`, `psql` client
- [ ] A coding-agent tool you will use for A1/A2/A3 sessions (e.g. Claude Code or Codex CLI), able to read/write files and run shell commands in the worktree, and a plan/rate limit that allows ~15 A2 + 6 A1 + A3 sessions at once
- [ ] Machine: a MacBook Pro M4 Pro with 24 GB RAM runs 5 parallel A2s comfortably. A full wave of 15 works with `CARGO_BUILD_JOBS=2` (in every lane `.env`), `sccache` and Docker Desktop limited to about 4 GB, because the A2s rarely all compile at the same moment. If macOS starts swapping heavily (Activity Monitor → Memory Pressure red), lower the lane caps. Keep ≥100 GB free disk; the A1 removes finished worktrees and their `target/` (AGENTS.md §3b)

## C. Workstream-specific
- [ ] Hardware lab (record exact OS/kernel/hypervisor versions): **All three cells are mandatory.** **Windows cell:** Hyper-V on x86_64 is needed (Windows Home has none). Choose in this order: (1) an Azure VM with nested virtualization (Dv5/Ev5-series, Windows Server or Windows 11 image with the Hyper-V role), paid from the Azure for Students credit, started only while W03 Windows tasks or RUNs need it and deallocated afterwards; (2) a Windows PC upgraded to Windows 11 Education/Pro (check Azure for Students → Education → Software for a free Windows 11 Education licence). Not on the Mac: Windows-on-ARM VMs on Apple silicon cannot host the x86_64 Hyper-V cell. AWS offers nested virtualization only on expensive bare-metal instance types (verify current offerings before choosing it). **macOS cell:** your Mac, but only inside a dedicated non-admin macOS user account (for example `mclab`) used for nothing else — no personal Apple ID, browser logins, SSH keys, cloud credentials or repo clones of your own work — with FileVault on; the run-manifest records `LAB_HOST: shared hardware, dedicated user`. **Linux KVM cell:** a Linux VM on the Mac with nested virtualization (Apple M3/M4 on macOS 15 or later; check `ls /dev/kvm` inside the VM; this gives an arm64 cell), run from the same `mclab` account, or an Azure Dv5 Linux VM for an x86_64 cell. Linux KVM cell = WSL2 with nested virtualization (`nestedVirtualization=true` in `%UserProfile%\.wslconfig`, Windows 11, check `ls /dev/kvm` inside WSL) or, if that fails, a rented Linux VM with nested virtualization / bare metal (cloud credits from GitHub Education may cover it).
- [ ] Install Firecracker and Cloud Hypervisor binaries on the Linux host (for the U-001 spike), Apple Virtualization.framework tooling on the Mac, the Hyper-V role and PowerShell module on the Azure Windows lab VM, and a self-hosted GitHub Actions runner on the Mac and the Windows laptop for the macOS/Windows CI jobs.
- [ ] Install tshark/Wireshark for pcap evidence; `jq`; `mkcert` for the staging CA.
- [ ] Confirm you can create network namespaces/iptables rules on the Linux host (root or CAP_NET_ADMIN).
- [ ] Never run escape-corpus tests in your own macOS user account — only in the dedicated `mclab` lab account or on the Azure lab VMs.

## D. Workspace setup (automated)

Run `python3 setup_workspace.py` in this folder. It is safe to run any time: it never deletes anything, fills the machine-specific paths into `.env`, checks your tools, verifies the architecture, and after G0 clones the repo, enables the hooks, creates the COORDINATION worktree, symlinks `.env` and records ARCH_IMPORT_COMMIT. It ends with `[TODO]` items for anything only you can do. `python3 setup_workspace.py --check` only reports.

Manual equivalent (for understanding or if the script reports a problem):
```bash
cd LANE_03_RUNNER_SANDBOX_ACQUISITION
git clone <MC_REPO_URL> repo
mkdir -p worktrees
git -C repo config core.hooksPath .githooks          # guardrail hooks (write set, secrets, no push to main)
git -C ../WORKSPACE/repo worktree add ../WORKSPACE/worktrees/COORD-ws03 -b ws03/coordination origin/main   # this lane's A1 works here
ln -s "$PWD/.env" "../WORKSPACE/worktrees/COORD-ws03/ws03-runner-sandbox-acquisition/.env.local"   # every worktree of this lane gets this symlink (CREATE commands include it)
python3 ../COMMON/verify_architecture.py repo/ARCHITECTURE/V2_1          # expect mismatches: []
python3 ../COMMON/verify_architecture.py ARCHITECTURE_READONLY/MASTERCODER_CONVERGENCE_ARCHITECTURE_SOL_V2_1  # expect mismatches: []
(cd repo && python3 ARCHITECTURE/V2_1/TEST_VECTORS/check_spec.py)        # expect p0_executed:false
(cd repo && git log --oneline -1)                                         # note ARCH_IMPORT_COMMIT
```
- [ ] All commands succeeded; ARCH_IMPORT_COMMIT written into `../WORKSPACE/worktrees/COORD-ws03/ws03-runner-sandbox-acquisition/coordination/memory.md` §Environment.
- [ ] The COORDINATION worktree exists on branch `ws03/coordination`. From now on its `ws03-runner-sandbox-acquisition/coordination/` copy is authoritative.
- [ ] Hooks active: `git -C repo config core.hooksPath` prints `.githooks`.
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
- [ ] G1 signed by master — G1 file SHA-256: ____ (A1 records it in memory.md)
- [ ] I understand G2/G3 are needed before any claim RUN.

## Open items
-

Signed: master ______  Date (UTC): ______
