# DEPENDENCIES — WORKSTREAM-03-RUNNER-SANDBOX-ACQUISITION

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-17 types (W03-T003)
- fake-sandbox (W03-T006) and fake-runner (W03-T022) doubles
- runner client for IM-2 (W03-T014)

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-01/I-02 (W01)
- I-03/I-09 lease + Runner Control (W01)
- I-04 effects (W01) for CHECKOUT_FETCH/DEPENDENCY_FETCH
- checkout-grant double (W02)
- I-11 model endpoint audience (W04) for sandbox allowlist
- endpoint policy + dependency grants (W04-T027/T029)
- escape corpus (W05-T036)
- host-lab inventory + local Git server (W06-T033/T034)
- I-13/I-14 (W06)
- canary-scanner (W06)

## Integration milestones your claim RUNs need
none — definitions in the integration contract.

## Supporting components per claim
- P0-02A: W06 canary-scanner
- P0-02B: —
- P0-02C: W04 broker audience (allowlist), W06 canary-scanner
- P0-02D: —
- P0-05A: W02 checkout-grant issuer, W01 effects
- P0-05B: W02 checkout-grant issuer, W06 canary-scanner
- P0-05C: W02 checkout-grant issuer
- P0-05D: W02 checkout-grant issuer
- P0-05H: —

## External / human dependencies
- Hardware lab (record exact OS/kernel/hypervisor versions): **All three cells are mandatory.** **Windows cell:** Hyper-V on x86_64 is needed (Windows Home has none). Choose in this order: (1) an Azure VM with nested virtualization (Dv5/Ev5-series, Windows Server or Windows 11 image with the Hyper-V role), paid from the Azure for Students credit, started only while W03 Windows tasks or RUNs need it and deallocated afterwards; (2) a Windows PC upgraded to Windows 11 Education/Pro (check Azure for Students → Education → Software for a free Windows 11 Education licence). Not on the Mac: Windows-on-ARM VMs on Apple silicon cannot host the x86_64 Hyper-V cell. AWS offers nested virtualization only on expensive bare-metal instance types (verify current offerings before choosing it). **macOS cell:** your Mac, but only inside a dedicated non-admin macOS user account (for example `mclab`) used for nothing else — no personal Apple ID, browser logins, SSH keys, cloud credentials or repo clones of your own work — with FileVault on; the run-manifest records `LAB_HOST: shared hardware, dedicated user`. **Linux KVM cell:** a Linux VM on the Mac with nested virtualization (Apple M3/M4 on macOS 15 or later; check `ls /dev/kvm` inside the VM; this gives an arm64 cell), run from the same `mclab` account, or an Azure Dv5 Linux VM for an x86_64 cell. Linux KVM cell = WSL2 with nested virtualization (`nestedVirtualization=true` in `%UserProfile%\.wslconfig`, Windows 11, check `ls /dev/kvm` inside WSL) or, if that fails, a rented Linux VM with nested virtualization / bare metal (cloud credits from GitHub Education may cover it).
- Install Firecracker and Cloud Hypervisor binaries on the Linux host (for the U-001 spike), Apple Virtualization.framework tooling on the Mac, the Hyper-V role and PowerShell module on the Azure Windows lab VM, and a self-hosted GitHub Actions runner on the Mac and the Windows laptop for the macOS/Windows CI jobs.
- Install tshark/Wireshark for pcap evidence; `jq`; `mkcert` for the staging CA.
- Confirm you can create network namespaces/iptables rules on the Linux host (root or CAP_NET_ADMIN).
- Never run escape-corpus tests in your own macOS user account — only in the dedicated `mclab` lab account or on the Azure lab VMs.
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
