# DEPENDENCIES — WORKSTREAM-01-CORE-SECURITY-AUTHORITY

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-01
- I-02
- I-03
- I-04
- I-05
- I-07
- I-08
- I-09
- I-16

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-06 (W02 reducer)
- I-10 (none; W01 must NOT read wire-ts source)
- I-12 (W02 fake GitHub)
- I-13 (W06 evidence kit)
- I-14 (W06 test DB)

## Integration milestones your claim RUNs need
IM-2, IM-4, IM-5, IM-6, IM-7 — definitions in the integration contract.

## Supporting components per claim
- P0-06C: —
- P0-06D: W03 runner negotiation client
- P0-07B: W02 reducer/planner/verification tables, W03 runner key
- P0-08A: W03 runner, W04 model broker + dependency endpoint
- P0-09B: W05 webauthn-harness / aa-origin
- P0-09C: W05 webauthn-harness
- P0-09D: W02 staging GitHub + fake GitHub
- P0-09F: W06 artifact-store
- P0-09G: W02 staging/fake GitHub
- P0-09H: W02 staging/fake GitHub, W05 resolution UI
- P0-10A: W06 testdb
- P0-10I: W06 restore drill
- P0-10J: W05 resolution UI, W02 oracle case

## External / human dependencies
- master is the program owner: sign `COMMON/AUTHORIZATION/G1_PROTOTYPE_BUILD_AUTHORIZATION.md` (fill budget caps) before any A2 starts in any workstream.
- Day 0: follow `COMMON/DAY0_RUNBOOK.md` (program.config → bootstrap_repo.sh → create_github_repo.sh --dry-run → live → verify → setup_workspace.py in every lane).
- Buy/obtain a staging domain pair (e.g. `mc-staging.example` for AA and a different registrable domain for the ordinary web) — AA must be a separate registrable domain (52).
- Approval-key profile: as the single operator you qualify INDIVIDUAL_OFFLINE (create an offline recovery key, store it offline). DUAL_ADMIN (G-053) needs a second independent custodian; without one that cell stays DISABLED.
- Install mkcert (local TLS for AA origin), Keycloak via Docker (test IdP).
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
