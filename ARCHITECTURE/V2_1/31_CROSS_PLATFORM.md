# Cross-Platform Architecture

## Separate capability dimensions

Platform support is not one Boolean. A runner advertises:

- host OS/version and CPU architecture
- sandbox backend and demonstrated S tier
- filesystem semantics and case sensitivity
- network-control and reset capabilities
- toolchain/image digests
- native project/language cells
- checkout cells including LFS/submodules
- model-harness cells
- verifier-independence class

The capability digest is bound into AttemptBindingV2.

## Initial matrix

| Host | Runner protocol | Hostile sandbox target | Native verification |
|---|---|---|---|
| Linux x86_64/arm64 | Required | S2 microVM/VM | Only declared container/VM toolchain cells |
| macOS arm64/x86_64 | Required | Proven VM/dedicated-host backend | macOS-native cells only after P0-02 |
| Windows x86_64/arm64 | Required | Proven VM/Hyper-V or dedicated backend | Windows-native cells only after P0-02 |

Container availability does not make a shared kernel S2. Running the Rust binary does not prove project builds, filesystem behavior, process-tree termination, or egress enforcement.

## Scheduling

The scheduler matches exact required capabilities; it never sends work and asks the runner to degrade. EXPERIMENTAL_NON_GATING may produce advisory findings. UNSUPPORTED mandatory work yields non-pass.

## Protocol portability

Canonical signed records avoid native endianness, path strings as identity, locale time, filesystem inode assumptions, and shell-specific quoting. Paths in findings use normalized repository byte/text rules defined by the tool adapter, with the original provider/object identity retained.

## Upgrades

Runner auto-update is not assumed. A signed release and compatibility window may be added after supply-chain and rollback gates. Until then, administrators install versions explicitly and the control plane advertises N/N-1 compatibility.

## V2.1 administrative independence

Host diversity is not C3. Two operating systems, two VMs or two runners controlled by the same administrator remain at most C2. 13 defines INDIVIDUAL_SINGLE_DOMAIN, TEAM_SINGLE_DOMAIN, INDEPENDENT_DOMAINS and EXTERNAL_TRUST_ROOT profiles. Scheduler matching includes proven administrative-domain separation and qualified dependency-manager/registry/toolchain cells. A model cell is not part of the minimum deterministic FAST capability digest.
