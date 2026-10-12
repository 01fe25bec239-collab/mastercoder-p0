# Runner and Sandbox

## Runner protocol

The customer-controlled Rust runner enrolls a workload identity, advertises OS/architecture/sandbox/tool capabilities, and long-polls over outbound mTLS. A lease offer names job, authority_epoch, lease_generation, AttemptBindingV2 digest, required capability digest, resource envelope, artifact policy, and deadline. Acceptance is conditional on an exact capability match.

## Sandbox tiers

| Tier | Meaning | Gate use |
|---|---|---|
| S0 | No repository code execution; safe metadata/object parsing only | Allowed for non-executing analysis |
| S1 | Process/container isolation for trusted or explicitly low-risk code | Never the default for hostile PRs |
| S2 | VM/microVM-grade boundary with ephemeral disk, bounded network, no host credentials | Default for hostile repository execution |
| S3 | Dedicated tenant/host or stronger isolation for native/high-risk workloads | Required by policy/risk cell |

The tier is a capability statement, not a product name. Linux shared-kernel containers do not satisfy S2. macOS and Windows must advertise the actual hypervisor or dedicated-host backend proved by P0-02.

## Host protections

- No reusable provider, model, control-plane, or artifact-store credential in the guest.
- Default-deny egress; model and checkout access only through typed host brokers.
- Ephemeral workspace and VM identity; verified reset between attempts.
- CPU, memory, process, disk, network, wall-clock, output, and event limits.
- Read-only toolchain images by digest; no host socket, package-manager credential, or runner key exposure.
- Untrusted stdout/stderr are bounded artifacts and pass secret/source filters before export.

## Receipt semantics

The runner signs session, binding, environment, command/tool digest, timing, exit, and artifact digests. This proves which enrolled runner reported the receipt. Evidence class additionally depends on sandbox tier, independent workspace/runner/admin relationship, verifier reproduction, and policy.

## Cancellation and fencing

Cancellation revokes future broker intents and asks the sandbox to terminate. The runner may report late completion, but the control plane accepts it only as a fenced audit fact. Broker resource consumption is still settled. A runner that loses authority cannot publish checks, approvals, or repository effects.

## Cross-platform truth

Supporting a Rust runner on an OS does not mean every project can be verified natively there. Each language/tool/project cell is SUPPORTED, EXPERIMENTAL_NON_GATING, or UNSUPPORTED. Required unsupported cells produce a non-pass result.

## V2.1 coverage, dependencies and receipt ingress

Runner Control is the authenticated ingress for lease acquisition, heartbeat, completion and analysis/verification receipt submission. Runner, sandbox and agent hold no PostgreSQL credentials or database network route. Trusted server handlers perform atomic owner/generation/epoch/state/actual-database-expiry checks, independently of the reaper. Expired completion is audit-only.

The lease includes ExpectedAnalysisManifestV1 digest; required work cannot disappear locally. Each unit emits its explicit completion status and coverage, including zero findings. Dependencies enter only through DependencyAcquisitionGrantV1 or approved pre-resolved artifacts under 55; package managers and scripts run offline inside the declared tier. No registry credential or arbitrary package-network fallback enters the guest.
