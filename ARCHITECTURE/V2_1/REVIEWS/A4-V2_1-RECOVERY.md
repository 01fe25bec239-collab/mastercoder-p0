# A4-V2_1-RECOVERY — independent architecture review

Reviewer: fresh Recovery reviewer, independent of the repair authors. Review date: 2026-10-03. Result: **NO_NEW_ARCHITECTURE_BLOCKER_FOUND** within the recovery review scope. This report also independently closes the remaining DS-03 propagation item below; it does not alter the original Distributed Systems review or its authorship.

## Scope and method

Read the supplied correction brief; the available numbered architecture through 56, including the state, API/event, trust, sequence, reliability, deployment and recovery contracts; relevant ADR amendments 016–028 and preserved ADR context; wire encoding rules and recovery-relevant schema shapes; and the atomic claim procedures, injected faults and expected observations across the 55 claims in document 57's authoritative JSON counterpart. Read the complete P0-10I claim and targeted recovery/expiry/custody/projection claims in detail. Historical baseline copies were excluded from normative authority. Package status, final export metadata and the full independent Rust/TypeScript vector-conformance audit are outside this review; the integrator was still completing package metadata.

Counterexamples were traced across intent creation, individual dispatch, lost acknowledgments, rollback, late observations, fresh authority bootstrap, byte custody and provider convergence. No product implementation, database/provider experiment or P0 execution occurred. No architecture file was edited by this reviewer.

## Recovery findings

No new unresolved architectural recovery finding was established. The conclusion depends on the stated trusted-authority and independent-journal failure envelope; it does not certify those mechanisms empirically.

1. **Lost intent and no re-arming.** Documents 18, 29, 35 and 50 require complete intent metadata to be attached durably in the witness domain, not merely referenced by a digest into the restorable database. PREPARED and a unique SEND_ARMED claim precede each possible send. Missing acknowledgment forbids dispatch; replay/recovery cannot obtain another send claim. Recovery authenticates full high-water coverage and earlier unresolved records, including effects absent from the restored database. A surviving-row scan is explicitly insufficient. Original epochs and logical-key tombstones survive bootstrap.
2. **Uncertainty cannot become absence.** Documents 19, 50 and 56 preserve UNKNOWN after timeout, missing row, expired token or not-found response. Terminal nonexecution requires complete never-dispatched history plus sender fencing, or a provider-correlated guarantee. A qualified provider-idempotency retry is a new recorded call under fresh authority and does not mark the original UNKNOWN terminal. Unresolved manual closure preserves the tombstone and conflicting-operation block.
3. **Recovery cannot resurrect security inventory.** Document 35 step 6 denies every restored entitlement by default and explicitly forbids automatic re-signing into the new epoch. Fresh keys/authenticators, current provider ownership, independently approved policy/analyzer inventory and independent offline recovery roots are required. Document 48 disclaims a complete security-mutation journal. This closes the lost-revocation counterexample through fresh bootstrap, with a real availability cost.
4. **Byte loss fails closed.** Documents 14, 24 and 35 require exact immutable patch/material custody through display, approval, publication and uncertain-effect reconciliation. Loss/expiry prevents future authority; compulsory deletion records evidence loss and retains unresolved effect metadata. A digest or regenerated working-tree patch cannot substitute for missing approved bytes.
5. **Provider safety includes delayed writes.** Document 54 requires discovery of potentially authoritative duplicate/orphan checks, all relevant associations, explicit withdrawal, readback and terminality of older writes before safe convergence. A visible failure while an old success can still arrive does not release freeze. The external unsafe interval and potentially indefinite operator/provider reconciliation remain explicit.
6. **Lease expiry is independent of cleanup.** Documents 07, 19 and 29 sample the database clock after locks and require strict expiry together with owner/generation/epoch/state. Reaper delay, stale heartbeat, lock wait and late result cannot restore authority. Late actual spend remains separately settleable.

## Independent closure addendum to Distributed Systems review

| Item | Independent read-back | Disposition |
|---|---|---|
| DS-01 security inventory rollback | 35 step 6, 48 rollback containment, and both representations of P0-10I deny restored grants even when every post-backup revocation is omitted. P0-10I attempts bulk fresh-epoch re-signing and old-key/authenticator replay, then separately removes recovery-root and ownership proof. Its positive control requires complete fresh bootstrap and preserves effect tombstones. | RESOLVED_IN_V2_1 at contract level; P0-10I remains NOT_RUN. |
| DS-01 reliability propagation | 17 explicitly limits the four-hour target to infrastructure regional recovery and says authoritative capability recovery can exceed it. This agrees with 35's mandatory fresh bootstrap and unbounded provider uncertainty. | RESOLVED_IN_V2_1; no four-hour security-recovery promise remains in the checked reliability contract. |
| DS-02 retry consistency | 18's relational rules admit only the exact proof-backed exception in 50, use fresh call_attempt/current authority and keep the original UNKNOWN unresolved. | Prior independent closure corroborated; no re-open. |
| DS-03 remaining deep-scan diagram | 26 now seals a logical analysis plan first, allocates immutable AttemptBindingV2 through Runner Control, seals its attempt-bound manifest, and only then releases the binding/manifest to the runner. 25 has the same order. 51 requires new attempt/new manifest after expired allocation and retains COLLECTING rather than introducing extra evaluation states. | RESOLVED_IN_V2_1. The original report's PARTIALLY_RESOLVED entry describes its earlier read; this addendum closes the final sibling-diagram propagation. |

## All twenty repairs examined through the recovery lens

| Finding | Attempted counterexample and current containment |
|---|---|
| 1 | Replayed or retargeted planner work after restart: independently controlled gateway derives exact approved calls and rechecks current per-effect authority; stolen-token residual is disclosed (53). |
| 2 | Restored old success or delayed pre-revocation write: journal recovery, complete check discovery, generation reconciliation and confirmed freeze/withdrawal precede safe release (35, 54). |
| 3 | Reopened ceremony displays substituted bytes: AA retrieves and hashes its own canonical plan/custody and binds display/challenge; stale/missing inputs reject (14, 52). |
| 4 | Login recovery or lost revocation grants approval: independent enrollment/recovery and full fresh post-restore bootstrap prevent inherited restored authority (35, 52). |
| 5 | Restored or API-forged trusted-looking rows reach signer: protected writers, signed role-bound provenance, current pointers and new bootstrap inventory are required (07, 35). |
| 6 | Lost zero-finding receipt or replaced attempt accidentally passes: exact mandatory-unit completion set is required; manifest follows allocated attempt and is re-sealed on replacement (25, 26, 51). |
| 7 | Repeated agent-authored expectation becomes trusted after restart: immutable oracle provenance remains separate from causality and independence; new scoped approval/verification is needed (13, 56). |
| 8 | Unreaped or lock-delayed lease gains authority: strict actual DB-clock expiry after locks denies acceptance and same-generation renewal (07, 29). |
| 9 | RPO restore erases an already-sent effect: independent complete witnessed history enumerates it; old sender fencing and no re-arm precede admission (35, 50). |
| 10 | A transient not-found result permits duplicate ref/PR/check: UNKNOWN persists absent exact outcome or strong terminal/idempotency proof (50, 56). |
| 11 | Parent progress authorizes ref/PR after revocation: each actual call has its own intent and STARTED check; orphaned immutable objects confer no sibling authority (19, 27, 50, 53). |
| 12 | Mutable state or reconstructed identity changes the original signed request: immutable intent, separate state snapshots, explicit epoch-fence type and strict registered encoding preserve distinctions; complete conformance remains Contract Consistency/P0-06 scope. |
| 13 | Expired/deleted custody is regenerated under old approval: pinned exact bytes, read hashes and explicit loss/deletion states prevent substitution; uncertain metadata remains (14, 24). |
| 14 | Recovery consolidates verifier administration but retains C3 label: capability matching requires actual independent administrative/recovery domains; same-domain deployment remains C0–C2 (13, 31). |
| 15 | Restarted download resets budget or promotes late bytes: one-use grants, root accounting, current authority and expiry bind acquisition; late cost can settle without artifact promotion (55). |
| 16 | Operator closes uncertainty to clear dedupe or directly mint PASS: scoped resolution binds evidence and current state; unresolved tombstone persists; legitimate evidence/policy changes require fresh evaluation (56). |
| 17 | Model outage blocks deterministic recovery: FAST has no model prerequisite in work/architecture DAGs, while required security/bootstrap/evidence gates remain (05, 35, 39). |
| 18 | An umbrella result hides missing recovery evidence: atomic claims bind exact cell/version/fixture, observations, evidence and independent reviewer; relevant restore/expiry/ambiguity claims remain NOT_RUN (42, 57). |
| 19 | Old alias interpreted as alternate authority after upgrade: canonical registry, closed versions/options and explicit alias mapping prohibit such wire acceptance (47, 49, SCHEMAS/ENCODING.md). |
| 20 | Recovery path gives runner/sandbox direct DB credentials: sequence diagrams route submissions through trusted Runner Control/Evidence authorities; only trusted server roles reach PostgreSQL (25–27). |

## Remaining qualification boundary

Journal fsync and failure-domain independence, sender fencing, provider terminality/idempotency/check selection and effective merge freeze, custody durability, authenticator recovery and independent-domain claims require their applicable future atomic P0 evidence. Simultaneous loss/compromise of all journal copies remains outside the admitted zero-loss envelope and freezes privileged capability. Indefinite uncertainty can retain tombstones/custody holds and delay capability recovery indefinitely. These are explicit limits, not permission to retry or declare recovery safe.

Recovery review supports a narrow external architecture audit with no remaining recovery repair blocker found. It grants no production freeze, prototype PASS or implementation authorization.
