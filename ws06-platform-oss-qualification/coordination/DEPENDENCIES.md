# DEPENDENCIES — WORKSTREAM-06-PLATFORM-OSS-QUALIFICATION

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-13
- I-14
- canary-scanner
- observability config
- local-services (Git server + registries) for W03/W04 (W06-T034)
- host-lab inventory for W03 (W06-T033)

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-02 CLI (W01)
- I-10 CLI (W05)
- I-03/I-04/I-05/I-07/I-16 (W01)
- I-12 (W02)
- I-15 (W04)
- runner (W03)

## Integration milestones your claim RUNs need
IM-1, IM-7, IM-8 — definitions in the integration contract.

## Supporting components per claim
- P0-06A: W01 mc-wire CLI, W05 wire-ts CLI
- P0-06B: W01 mc-wire CLI, W05 wire-ts CLI
- P0-07D: W03 runner pools, W02 verification, W01 trust authority
- P0-10B: W01 lease/AA/custody, W02 eligibility/projection, W04 model intents
- P0-10C: W01 custody
- P0-10D: W01 audit, W04 broker, all component logging
- P0-10E: W01 lease/outbox, W02 planner/reducer, W04 graph
- P0-10F: W01 negotiation, W03 runner
- P0-10G: W01 lease/journal/effects
- P0-10H: W01 journal/effects, W02 reconciliation reads

## External / human dependencies
- Admin access to the `mastercoder-p0` repo settings (you are the repo owner) to apply branch protection and secret scanning.
- Two cloud accounts with different root owners: account A (you) and account B (administered by a second, independent administrator — the one role master cannot fill alone; see COMMON/PROGRAM_README.md, section What still needs a second person) for P0-07D; no shared billing root admin, CI credential or recovery contact.
- A private object-lock bucket for the evidence store (not public, not in the repo).
- Install: cargo-deny, cargo-audit, cargo-cyclonedx, gitleaks, syft, tshark, k6 (or a Rust load tool), MinIO client `mc`.
- Approval-key profile: as the single operator you qualify INDIVIDUAL_OFFLINE (create an offline recovery key, store it offline). DUAL_ADMIN (G-053) needs a second independent custodian; without one that cell stays DISABLED.
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
