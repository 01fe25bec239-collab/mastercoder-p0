# A4-V2_1-DISTRIBUTED-SYSTEMS — independent architecture review

Reviewer: fresh A4-V2_1-DISTRIBUTED-SYSTEMS agent; not an author of the repairs. Reviewed the user correction brief, current numbered architecture 00–57, ADR-001–028, the canonical schema registry and encoding rules, and atomic prototype claims. Compared relevant original PostgreSQL, fencing and recovery contracts in the frozen `_baseline` tree. Read-only review except this report; no product implementation, database experiment, provider experiment or P0 execution.

Scope: attempted counterexamples against all twenty accepted repairs, emphasizing lease expiry, authority ordering, provider projection, per-call dispatch, ambiguity and recovery. Package status/traceability/vector artifacts were being integrated concurrently and are not certified by this review. This report is architectural reasoning, not empirical provider or runtime qualification. No claim is made to be the separately requested Recovery reviewer.

## Findings and reconciliation

### DS-01 — HIGH — Recovery cannot establish that restored security grants are still valid

Original affected locations: `35_DISASTER_RECOVERY.md` recovery step 6; `48_COMPONENT_REVOCATION_POLICY.md` revocation commit rule; `50_EXTERNAL_EFFECT_RECOVERY.md` journal scope; `SCHEMAS/REGISTRY.json` RevocationRecordV1 and journal record contracts. Related findings: 4, 5, 9 and 12; invariants INV-06, INV-10, INV-13, INV-17.

Counterexample: take a business-DB backup with analyzer/key or approver binding authorized. Revoke it after the backup, then lose the DB before asynchronous security/audit export and without any later provider effect. The effect journal is complete but contains no requirement to preserve that revocation. Restoring and rotating an epoch does not, by itself, prove which restored entitlements may be reissued under the new epoch. The original instruction to recover the “independent latest” security inventory had no completeness/watermark protocol. A prototype that deliberately omits a revocation cannot reliably detect the omission through the specified effect-only journal.

Requested minimum repair: either synchronously preserve all security mutations with independently verifiable completeness, or refuse all restored entitlements and require a complete independently authorized fresh bootstrap. Do not claim ordinary backup signatures prove latest state.

**Recheck: RESOLVED_IN_V2_1.** The repair author selected the second option. Current document 35 step 6 explicitly denies every restored authorization inventory and forbids bulk copying or re-signing it into the new epoch. Independently held offline recovery roots, current provider ownership, newly approved policy/analyzer inventory, new workload/signer keys, and new authenticator bindings are mandatory. Document 48 now explicitly disclaims a lossless security-mutation journal. The former revoked key/binding therefore cannot become current merely because its revocation was lost. This is a substantial availability tradeoff: privileged recovery has no guaranteed four-hour completion. P0-10I must test the new bootstrap rule, including omitted revocations and attempts to re-sign restored grants; it remains NOT_RUN.

### DS-02 — MEDIUM — Relational retry rule contradicted the provider-idempotency exception

Original location: `18_DATA_MODEL.md:25`, versus `50_EXTERNAL_EFFECT_RECOVERY.md` dispatch step 6 and replacement proof rule. Related findings: 9–12; INV-13 and INV-17.

Counterexample: a provider supplies an explicitly qualified same-token/same-body idempotency guarantee, the first call times out, and the original remains UNKNOWN. Document 50 permitted a fresh recorded call under that guarantee, while document 18 allowed a new attempt only after a terminal outcome. Implementers would have to either reject the documented safe recovery path or incorrectly mark the unknown original terminal to satisfy the data model.

Requested minimum repair: align the relational constraint with the exact provider-specific exception and preserve the unresolved original's identity/state.

**Recheck: RESOLVED_IN_V2_1.** Current document 18 explicitly permits only document 50's proof-backed exception, preserves original UNKNOWN, requires fresh call_attempt/current authority, and retains logical request/token/account binding. Neither an internal UUID nor a not-found read supplies the provider guarantee. Unqualified GitHub operations remain disabled for such retry.

### DS-03 — MEDIUM — Attempt-bound manifest was sealed before its attempt could exist

Original locations: `25_PR_SEQUENCE_DIAGRAMS.md:18`, `26_DEEP_SCAN_SEQUENCE_DIAGRAMS.md:18`, `51_ANALYSIS_COVERAGE_CONTRACT.md:21`, and `SCHEMAS/REGISTRY.json` AttemptBindingV2/ExpectedAnalysisManifestV1. Related findings: 6, 12, 17 and 20; INV-02 and INV-03.

Counterexample: the original sequence freezes ExpectedAnalysisManifestV1 before allocating a runner lease. Its required attempt_binding_digest must already include runner ID, runner session, allocated lease generation and expiry. No valid value is available at the depicted freeze point. Reassignment additionally changes that binding. Document 51 also introduced evaluation PLANNING→READY states absent from document 19.

Requested minimum repair: seal the logical analysis plan first; allocate an immutable attempt/lease; seal that attempt's manifest before releasing execution; keep evaluation state vocabulary consistent; repeat sealing for a replacement attempt.

**Recheck: PARTIALLY_RESOLVED, awaiting sibling diagram propagation.** Document 25 now shows the correct plan→allocation→manifest→work-release order, and document 51 keeps evaluation COLLECTING and requires a fresh manifest on a replacement allocation. At the latest review read, document 26 still froze the expected manifest before its lease allocation. The remaining correction is the same order in the deep-scan diagram, not a new design. This status must be updated after that read-back before final package acceptance.

## Twenty-repair attack coverage

| Finding | Counterexample examined / assessment |
|---|---|
| 1 | Compromised planner submits update/delete/merge or retargeted body. Positive gateway vocabulary, independent configuration/credential boundary and honest stolen-token residual address the architectural defect; actual permissions remain P0-gated. |
| 2 | Confirmed success becomes stale; late earlier success arrives after withdrawal; concurrent/shared-SHA checks. Documents 54/50 require generation ordering, exact readback, old-write terminality, freeze and an explicitly unsafe external interval. No instantaneous provider revocation is claimed. |
| 3 | Ordinary UI displays A and submits B. AA fetches and renders its own complete canonical plan/custody; independent origin and challenge binding address substitution, with endpoint compromise residual disclosed. |
| 4 | Account recovery restores an approver or rollback resurrects a revoked credential. Independent AA lifecycle and DS-01 fresh-bootstrap repair prevent inherited ordinary-login authority. |
| 5 | Ordinary API inserts trusted-looking evidence or races revocation against gate commit. Dedicated DB writers, signed source roles and common authority locking prevent that path under the stated trusted-DB boundary. DS-01 addressed restoration. |
| 6 | Missing zero-finding analyzer, substituted coverage or changed attempt. Complete-set reducer rejects absent/substituted receipts. DS-03 requires matching manifest-allocation sequencing in both diagrams. |
| 7 | Arbitrary agent expectation reproduces across independent runs. Oracle trust is separate from causality and independence; repetition cannot create a trusted requirement. |
| 8 | Heartbeat/completion waits beyond expiry while reaper is disabled. DB clock sampled after locks, strict greater-than expiry and new generation on reassignment close the old acceptance defect. |
| 9 | Restore loses a sent intent; witness acknowledgment is lost; old sender survives failover. Complete journal enumeration, two-copy durability before send, no re-arming and externally proved sender fencing address lost sends; DS-01 addressed security inventory recovery. |
| 10 | Not-found read precedes a delayed provider completion. UNKNOWN persists absent exact positive outcome or strong terminal/idempotency proof. DS-02 aligns the exceptional safe-retry path. |
| 11 | Approval revoked after commit creation but before ref creation. Each child has separate current STARTED authority; parent progress grants no later call. Bounded client lifetime does not claim provider cancellation. |
| 12 | Wire identity differs from mutable effect state or reconstructed recovery identity. Immutable EffectIntentV2, separate state revisions and a distinct JournalEpochFenceV1 resolve the known discriminator/state issues. Full vector and schema completeness acceptance belongs to the independent contract review. |
| 13 | Patch bytes expire, mutate or disappear after approval. Exact tenant custody, read hashing, pins and loss/deletion states block subsequent publication and retain uncertain-effect metadata. |
| 14 | Two same-admin runners claim C3. Explicit deployment profiles cap them at C2; required higher assurance remains non-permitting. |
| 15 | Dependency retries evade limits or delayed downloads gain artifact authority. Grants/root accounting retain consumed budgets; expiry fences promotion; credentials stay in the dialer and mandatory unsupported resolution fails closed. |
| 16 | Operator closes UNKNOWN to clear dedupe or promotes missing evidence to PASS. Resolution retains uncertainty tombstones, requires scoped evidence-bound authority and creates a fresh evaluation after legitimate changes. |
| 17 | All model services unavailable during deterministic FAST. DAGs do not place model integration before deterministic verification/review. |
| 18 | An umbrella PASS hides missing atomic evidence. Separate claim/cell/run, independent review, expiry and all-applicable-claim qualification are specified. No prototype results were inferred from these contracts. |
| 19 | Canonical aliases create alternate accepted protocols. Closed registry/encoding rejects unknown names; final mechanical documentation sweep is still the integrator's task. |
| 20 | Runner or sandbox appears to hold DB credentials. Current diagrams route through trusted ingress; DS-03 is the remaining semantic sequence consistency item, not a direct DB-access arrow. |

## Scope conclusion

No additional unresolved distributed-system safety blocker was established beyond DS-03's remaining deep-scan diagram propagation at this read. DS-01 and DS-02 were independently rechecked after repair; the reviewer did not author their fixes. Provider semantics, witness durability, trusted-domain independence and hostile runtime behavior remain capability-gated pending the separately authorized atomic P0 evidence. This report does not grant production freeze or implementation permission.
