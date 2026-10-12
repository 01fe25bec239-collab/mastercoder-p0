# A4-V2_1-SECURITY — independent architecture review

Reviewer: fresh independent A4-V2_1-SECURITY; not the repair author. Date: 2026-09-12. Scope: bounded SOL V2 → V2.1 correction, not implementation or prototype qualification.

Status: **RECONCILIATION_REQUIRED**. The review found security-contract propagation gaps. The repair author is amending the package; no final approval is asserted until the resulting contracts are independently reread. All P0 evidence remains NOT_RUN.

## Findings

### SEC-01 — High — enrollment and recovery lack a complete, constructible ceremony chain

Original anchors: `52_APPROVAL_AUTHORITY_CONTRACT.md`, Enrollment and recovery authority; `SCHEMAS/ApprovalCredentialBindingV1.schema.json`, `registration_ceremony_digest`; `SCHEMAS/WebAuthnCeremonyV1.schema.json`, description. Related accepted findings: 4 and 12.

The initial schema set contained only an authentication ceremony (`webauthn.get`), while the authority contract required `webauthn.create`, administrator-signed enrollment and independently approved recovery. There was no typed registration response carrying the attestation object, nor a typed enrollment action binding the intended user, operation, scope and authorizers. A valid authentication response cannot supply a registration's newly generated public key and registration authorization.

Concrete counterexample: an implementer has to invent what `registration_ceremony_digest` hashes, and how an administrator's approval of enrolling user A in scope X is kept distinct from enrolling user B in scope Y. The document's permission table is not a complete interoperable signed protocol.

During reconciliation, the author added registration/action schemas. The intermediate `ApprovalEnrollmentActionV1` then contained `new_credential_id` and `new_public_key_cose`, while document 52 said the action is fixed before the create ceremony and its challenge binds that action. A normal authenticator generates those values only after receiving the create challenge. That ordering cannot be constructed without a second, unspecified ceremony or bypassing the stated binding.

Smallest repair: define a pre-registration intent containing existing identities, operation, exact scope/policy/current epoch and fresh session/nonce; bind the create challenge to that intent; bind the resulting registration response and new key into a final action; collect the required administrator/existing-credential authorization over that final action and commit once. Preserve add/replace owner UV, recovery quorum/hold, bootstrap identity proof, scope non-expansion and old-binding revocation. Register every digest target and add cross-purpose, changed-key/scope, stale-epoch and replay rejection vectors. Authentication-only proof must never stand in for new-key registration.

Status: OPEN pending final reread of the corrected ordering and authorization receipts.

### SEC-02 — High — maintainer oracle approval initially had no distinct signed purpose

Original anchors: `13_REVIEW_VERIFICATION.md`, V2.1 oracle trust; `SCHEMAS/OracleTrustClassV1.schema.json`, `approval_record_digest`; publication-only `ApprovalChallengeV2` and `HumanApprovalAssertionV2`. Related accepted findings: 7, 12 and 16.

Document 13 required an independently approved oracle with a distinct approval purpose. The original schema set offered only a Git publication assertion, whose required plan, custody, destination and effects fields cannot represent accepting an exact test expectation as a valid requirement. A publication signature or ordinary review comment must not be interpreted as oracle authorization.

Concrete counterexample: a maintainer approves a patch publication; an implementation treats that record as the `approval_record_digest` of a new agent-authored expectation. Reproduction can then qualify the expectation even though the maintainer never approved its requirement semantics. Another implementer may invent a different challenge layout, preventing interoperation.

Smallest repair: reuse a fully typed manual-resolution action with a closed oracle purpose, exact candidate/requirement/test evidence, current scoped resolver credential, independently fetched display and a separately domain-bound challenge. The final resolution must be the only allowed digest target for MAINTAINER_APPROVED. Publication approval cannot be reused.

Reconciliation observed: document 13 now names `ManualResolutionActionV1(kind=UNTRUSTED_ORACLE, decision=PROVIDE_ORACLE)`, `ResolutionChallengeV1` and `ManualResolutionV1`. Those schemas separate the pre-ceremony action from its final signed resolution and avoid a digest cycle. Final reference/authority mapping and vectors still require reread before closure.

### SEC-03 — High — eligibility policy did not reach the explicit publication acceptance predicate

Original anchors: `15_IDENTITY_SECRETS.md`, Author eligibility (lines 9–11 at initial review); `53_GIT_PUBLICATION_CAPABILITY.md`, Authorization and send boundary (line 29); `SCHEMAS/VerifiedPublicationPlanV2.schema.json`, `GitPublicationCapabilityV2.schema.json`, `PublicationOperationV1.schema.json`, `ModelIntentV2.schema.json`. Related accepted finding: 12 and preservation of the sound V2 eligibility decision.

Document 15 requires a current EligibilityProofV2 in reservation/publication inputs. The initial explicit publication guard checked active approver scope, policy, plan, ownership and epochs, but no current eligibility proof. None of the named publication admission/effect records bound such a proof. A broad entitlement to approve within a repository is different from MY_PRS_ONLY or a current ORG_MEMBERS eligibility decision.

Concrete counterexample: a repository-scoped approver requests a patch for another person's PR under MY_PRS_ONLY; or organization membership is revoked after plan approval but before CREATE_REF. The documented per-effect guard could accept a current credential and plan without reevaluating the now-false eligibility condition.

Smallest repair: explicitly require independently authenticated current eligibility at review/resource admission, plan/admission and each publication STARTED boundary. Bind the proof digest, or specify an authoritative lookup keyed by exact current subject, action actor, policy, tenant/repository epoch and membership generation; require actor equality, eligible=true and actual-time freshness. Old proof signatures and broad AA scope do not override current eligibility. Add mismatch/revoked-membership/stale-proof cases to the relevant vectors and P0-04/P0-09 claims.

Status: OPEN pending final reread.

## Security review of all twenty repairs

| Accepted finding | Security assessment of repaired contract |
|---|---|
| 1 | Independent gateway, credential-free planner, closed exact Git requests and no-bypass rulesets are explicit. Compromised gateway/stolen-token residual is honestly distinct from planner compromise. Actual provider permission evidence remains P0-gated. |
| 2 | Currentness invalidation, projection generation, shared-SHA ambiguity, prior-write terminality, withdrawal and provider freeze cover the stale-success defect at the contract level. Emergency unsafe interval remains explicit. |
| 3 | Separate AA origin/assets/session/RP, independent plan/custody fetch, complete rendering, frame/CSRF controls and endpoint residual address ordinary-UI substitution. |
| 4 | Lifecycle authority table is materially improved; SEC-01 blocks claiming the wire contract complete. |
| 5 | Document 07 enumerates producer/authentication/signature/writer/immutability/consumer/currentness for gate-critical rows. Dedicated roles, no ordinary API EXECUTE path, role-bound signatures and locked current pointers address signer-input fabrication. |
| 6 | Expected unit set is sealed before execution; each unit including zero findings requires matching qualified completion/scope/coverage. Missing execution cannot reduce to an empty successful finding set. |
| 7 | Causality and oracle trust are distinct; protected-baseline provenance and independent applicability checks prevent agent labels from becoming authority. SEC-02 concerns the maintainer-approval wire path. |
| 8 | Actual `clock_timestamp()` after lock, strict expiry equality, heartbeat rejection and owner/generation/epoch/state predicates address unreaped expiry. |
| 9 | Independent two-copy witnessed intent metadata before send and complete restore enumeration address loss of whole intents from PostgreSQL. Failure envelope is expressly bounded. |
| 10 | Not-found/time elapsed/token expiry do not prove nonexecution; UNKNOWN/manual tombstones persist until operation-specific terminal proof. |
| 11 | Each actual provider call has its own immutable intent, STARTED transaction and one-use SEND_ARMED claim. Parent progress does not authorize children; client deadline does not claim provider cancellation. |
| 12 | Canonical fixed arrays, width/option/enum/domain rules, dedicated schemas and detached signatures are substantial repairs. SEC-01–03 show remaining propagation checks; this review does not independently qualify Rust/TypeScript codecs. |
| 13 | Exact immutable tenant custody, authenticated hash-checked reads, pins through uncertainty, deletion/hold/loss and residency consequences are explicit. |
| 14 | Individual and shared-administrator profiles remain at most C2; unsupported required C3 cannot become PASS. No whole-system redesign was introduced. |
| 15 | Credential-free resolver, bounded credential-bearing dialer, exact locked closure, endpoint/redirect/DNS checks and sandbox-only extraction/scripts close the described authority gap. Each adapter/registry cell remains disabled pending P0 evidence. |
| 16 | Scoped resolution, trusted action display, expiry/replay control, independent terminal-nonexecution review and permanent uncertainty tombstones prevent manual PASS/retry laundering. Typed resolution correction intersects SEC-02. |
| 17 | Deterministic FAST has no model prerequisite in the DAG or admission description. |
| 18 | Atomic claim specifications and evidence conditions are present; inspected security claims require positive control and separate injections. Exact execution manifests and independent P0 evidence remain future obligations. |
| 19 | Glossary/registry establish canonical names. Transitional stale names were reported for the author's mechanical consistency pass; they do not establish demonstrated security enforcement. |
| 20 | Review/deep/publication sequences route runner and verifier submissions through trusted ingress. No runner/sandbox/agent PostgreSQL credential or direct DB access is authorized. |

## Actual review scope and limits

Read the supplied repair instructions; current numbered architecture documents 00/01–56 (including the security, state, API/event, authority, recovery, custody, dependency and compatibility sections); schema encoding and the fields/semantic rules of the registered initial contracts; all ADR decisions and amendments; and selected security atomic claims in 57/its JSON source. Compared the frozen V2 publication, approval, identity, data and API passages to reproduce the inherited defects. Inspected all 55 atomic claim identities and security-relevant procedures/observations; did not execute them. No product code, database, provider mutation, sandbox prototype, Rust or TypeScript implementation was run. The review wrote only this report and communicated proposed repairs; the repair author owns normative edits.

The package was being completed during this review. Temporarily absent final-status/traceability/vector artifacts were not treated as substantive security defects. Historical BASELINE_HISTORY reviews were not used as current approval evidence. Final readiness requires rereading the reconciled schemas/reference map and retaining independent contract/distributed reviews; this report does not supersede them.

Primary-source spot checks: GitHub documents Contents write for both creating and updating references, supporting the honest stolen-token residual ([GitHub Git references](https://docs.github.com/en/rest/git/refs)); WebAuthn registration and authentication have distinct response/validation paths, supporting the need for a constructible registration ceremony ([W3C WebAuthn](https://www.w3.org/TR/webauthn-3/)); check-run API documentation does not itself prove the proposed concurrency/freeze protocol ([GitHub check runs](https://docs.github.com/en/rest/checks/runs)). No provider semantics were experimentally verified.
