# A4-V2_1-PRODUCT-OPERABILITY

Reviewer: fresh independent product/operability reviewer; not the repair author. Review completed 2026-10-03 after resuming the interrupted review and rereading changed current artifacts.

Disposition: **PASS_FOR_NARROW_EXTERNAL_AUDIT**. Remaining product/operability architecture blockers: **none identified**. This is an architecture review, not prototype PASS, production freeze, or implementation authorization.

## Scope and method

Read the user's bounded V2→V2.1 repair brief. Reviewed the complete normative architecture at package level: invariants, 01–57 architecture, state/API/event contracts, trust/threat/privacy boundaries, sequence diagrams, deployment/compatibility/work DAGs, and ADR-001–028. Examined the schema registry/encoding and the approval recovery, resolution and atomic-claim schemas relevant to this remit. Reviewed all 55 atomic claims' procedures, failure injections, expected observations and metadata in the authoritative JSON and its readable mirror. Historical BASELINE_HISTORY material is excluded. Final status, export metadata and the in-progress schema authority map are the integrating authority's packaging responsibility; their temporary absence was not treated as a defect.

This review concentrates on whether the twenty repairs have operable, truthful user outcomes and executable future acceptance contracts. It does not independently prove every schema constraint or cryptographic vector, which is the contract-consistency review and later independent Rust/TypeScript conformance scope. No application code was written, no P0 experiment ran, and no external service was changed.

## Counterexamples against all twenty repairs

| Finding | Attempted product/operability counterexample | Current disposition and controlling contract |
|---|---|---|
| 1 | A compromised planner asks for an existing source-ref update while the UI calls publication create-only. | Contained by exact gateway vocabulary; actual gateway/token residual authority is disclosed separately. 08, 14, 53; P0-09D/E. |
| 2 | The UI remains green after analyzer revocation while GitHub still accepts old success. | Internal truth, confirmed projection, unsafe interval and freeze progress are separate; lack of tested freeze leaves advisory use. 16, 54; P0-03C–E. |
| 3 | Ordinary UI displays patch A and sends plan B to approval. | AA independently retrieves/renders B or rejects; it does not attest ordinary UI A. Complete binary/large-artifact review is required. 52; P0-09A/C. |
| 4 | Recover login, add an approver, and publish; alternatively lose the sole personal authenticator. | Login recovery grants no approval. Individual offline-recovery/ownership bootstrap is explicit; all-credential loss invalidates old authority before rebootstrap. Team recovery has distinct custodians. 15, 52; P0-09B. |
| 5 | Ordinary API inserts believable evidence rows and asks the signer for PASS. | Protected producer signatures, scoped writers and independent reduction prevent ordinary request authority from becoming evidence. 06–07; P0-07B. |
| 6 | An analyzer never ran but produced no findings, so a completed review is shown. | Exact mandatory set, explicit zero-findings receipts and coverage qualification prevent this. 09, 13, 51; P0-07A. |
| 7 | Three independent reproductions make an arbitrary agent expectation blocking. | Oracle trust is separate from causality/independence; requirement-backed resolution causes fresh verification/evaluation. 13, 56; P0-07C. |
| 8 | A disconnected worker renews an expired but unreaped lease and finalizes a result. | Actual DB-clock expiry after locks rejects authority; late usage may still settle. 07, 19, 29; P0-08A. |
| 9 | Restore a backup from before the provider effect and inspect only surviving DB rows. | Independent complete journal enumeration and fresh security bootstrap are prerequisites; missing witness/roots keeps authority frozen. 35, 50; P0-10H/I. |
| 10 | Operator treats one missing ref/PR read as absence and retries. | Exact terminal nonexecution or qualified idempotency proof is required; unresolved case closure keeps tombstones and conflicting work blocked. 50, 56; P0-09H. |
| 11 | Parent publication progress authorizes ref/PR after approval revocation. | Every individual effect rechecks authority; orphaned immutable objects and provider completion beyond client timeout are disclosed. 19, 27, 50, 53; P0-09G. |
| 12 | Different clients invent option, timestamp, version or signature representations. | Closed schemas/encoding specify representations and independent conformance is still required. Prototype fields are architecture JSON, explicitly separate from signed runtime CBOR. SCHEMAS; P0-06A–D. |
| 13 | Local ephemeral patch disappears after approval, so publication reconstructs it from a working tree. | Exact custody/pins, residency disclosure and invalidation prevent substituted bytes; uncertain effects preserve custody/metadata obligations. 14, 24; P0-09F/10C. |
| 14 | Two same-owner runners advertise C3; personal setup consequently cannot use any review. | Individual/team single-domain profiles remain C0–C2 with useful deterministic/advisory work; unavailable required C3 is explicit non-pass. Independent domains are an optional proven capability. 13, 31–32, 47; P0-07D. |
| 15 | Private registry installation needs a guest credential or unrestricted curl. | Exact finite broker grants, private endpoint policy, tenant cache, Git/toolchain pins and offline scripts provide the bounded path; unsupported dynamic downloads remain visible. 36, 55; P0-05E–H. |
| 16 | A maintainer clicks “resolve” to turn inadequate evidence or UNKNOWN into deterministic PASS. | Scoped trusted resolution display/UV, exact evidence, expiry, fresh evaluation and non-pass accepted-risk/uncertain closure prevent promotion. 19, 56; P0-10J. |
| 17 | Model outage or an unqualified subscription integration blocks FAST delivery. | 05/38/39 explicitly remove model prerequisites; 09/17/31/47 preserve model-free FAST and applicable core-only qualification. |
| 18 | One umbrella gate or a run lacking exact component versions enables every deployment. | Fifty-five independent claims cover ten families; exact local manifests must be sealed before RUNNING, undisclosed provider internals are identified honestly, and only exact current accepted PASS cells qualify. 42, 57 and claim JSON. |
| 19 | Similar protocol names silently mean different signed contracts or states. | Canonical registry/glossary and mechanical documentation checks are required; semantic state/result separation is now explicit for prototype evidence. 49, SCHEMAS, 19/57 addendum. |
| 20 | Runner/agent diagrams are implemented as direct PostgreSQL access. | 25–27 route through authenticated trusted ingress and label database-bearing roles; 04/10/22/32 prohibit guest credentials. |

## Issues raised and reconciliation

**PO-01 — Medium, CLOSED:** The interrupted snapshot used PASS/FAIL/INCONCLUSIVE as an evidence lifecycle while 19 used SUBMITTED/ACCEPTED/REJECTED. Counterexample: an executor could present a raw PASS as accepted qualification, or a consumer could treat accepted failure evidence as enablement. Minimal repair: distinguish lifecycle from measured outcome and require independently ACCEPTED plus PASS plus currentness. Current 19 and 57 both contain this exact mapping; reread confirmed closure.

**PO-02 — Medium, WITHDRAWN AFTER CURRENT-SNAPSHOT CHECK:** The interrupted P0-10I text instructed reapplication of revocation records, inconsistent with 35's chosen full fresh-bootstrap recovery without a lossless security-mutation log. The current authoritative claim already deliberately omits all post-backup revocations, rejects bulk re-signing restored keys/authenticators/policy/membership, and tests new offline-root/current-owner bootstrap and missing-root denial. Current JSON and readable 57 were reread; no remaining defect is asserted. This records the review observation without misrepresenting a superseded snapshot as a final finding.

## Atomic claim and operability assessment

Inventory check found exactly 55 claims: P0-01=4, P0-02=4, P0-03=5, P0-04=3, P0-05=8, P0-06=4, P0-07=5, P0-08=4, P0-09=8 and P0-10=10. Every record contains all required metadata fields and remains NOT_RUN. All name evidence reviewers and require independent acceptance; no repaired document is experimental evidence.

The claim contracts are sufficiently bounded for a later authorized executor to seal concrete fixtures, exact versions/images/lockfiles/configuration and observer information before starting. They intentionally do not guess versions of unbuilt software. Such unresolved execution manifests prohibit execution and PASS; they do not by themselves prevent this architecture package from reaching narrow external audit. Each unsupported environment/registry/auth/merge-mode variant remains disabled, and a family label cannot substitute for its applicable atomic claims.

The material operational limits are visible and intentional: emergency external revocation is not instantaneous; uncertain provider calls can remain unresolved indefinitely; journal loss stops sends; recovery requires new security bootstrap and can exceed the infrastructure RTO; private dependencies require qualified adapter cells; loss of all personal recovery credentials disables approval pending ownership rebootstrap; unavailable C3 yields non-pass for policy requiring it. None is hidden behind a success label, ordinary account recovery, or manual evidence override.

## Final judgment

No remaining Critical, High or Medium product/operability architecture defect was identified in the current reviewed package. The twenty repairs preserve the frozen stack, local/customer-owned modes, bounded Graph Engineering, deterministic FAST independent of models, human authorization and explicit unsupported outcomes. Proceed to narrow external audit; execute atomic P0 claims only after that audit passes and separate authorization is given. Production enablement remains prohibited pending applicable accepted, current prototype evidence.
