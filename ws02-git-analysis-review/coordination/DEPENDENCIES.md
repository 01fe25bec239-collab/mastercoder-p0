# DEPENDENCIES — WORKSTREAM-02-GIT-ANALYSIS-REVIEW

Independence rule: a lane never idles waiting for another lane. Every consumed interface has a published double (`COMMON/CROSS_WORKSTREAM_INTEGRATION_CONTRACT.md`). Only claim RUNs need real peer components, scheduled as integration milestones (IM).

## Interfaces you PROVIDE (publish @v1 + double early)
- I-06 (double W02-T003, real W02-T009)
- I-12 (W02-T002 + W02-T007)
- checkout-grant double (W02-T006) and issuer (W02-T029) for W03
- verify_current() eligibility check (W02-T018) for W01 and W04

## Interfaces you CONSUME (pin vN; use the double until the provider's task is INTEGRATED)
- I-01/I-02 (W01)
- I-03 lease (W01)
- I-04 effects/journal (W01)
- I-05 gate authority (W01)
- I-08 resolution (W01) for oracle approval
- I-13 (W06)
- I-14 (W06)
- I-17 unit execution (W03)

## Integration milestones your claim RUNs need
IM-3, IM-3, IM-6, IM-6 — definitions in the integration contract.

## Supporting components per claim
- P0-03A: —
- P0-03B: —
- P0-03C: W01 journal + gate authority
- P0-03D: W01 revocation authority
- P0-03E: W01 journal
- P0-04A: W01 AA credential bindings (double)
- P0-04B: W01 gateway/AA consumer (double)
- P0-04C: W01 AA credential bindings
- P0-07A: W01 gate authority/ingestion, W03 executor
- P0-07C: W01 resolution (oracle approval), W03 clean sessions
- P0-07E: W03 clean-workspace executor, W01 ingestion
- P0-09E: —

## External / human dependencies
- Create TWO disposable GitHub staging organizations (tenant A, tenant B) — names prefixed `mc-staging-`; never use a real org.
- Create the Review App and the Publication App in org A from the manifests you will write in W02-T000 (create them by hand first if needed; record App IDs in `.env`). Install the Publication App on `disposable-*` repos only.
- Create a separate GitHub account to act as the emergency freeze operator (2FA on).
- Prepare an isolated machine/VM for the P0-09E red-team token holder (not your daily laptop).
- Confirm you can call `gh api` against the staging org with your own account.
- G1 (master) before any A2; G2 (external audit) and G3 (master) before any RUN.
- Evidence reviews: fresh evidence-A3 sessions per lens, using the reviewer lane's playbook (rotation table), never the session that built or ran the claim.
