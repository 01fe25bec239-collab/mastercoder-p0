# V2 Initial Scope

Despite the historical filename, this document defines the first production scope of the V2 architecture.

## Included after gates pass

- GitHub pull-request review for explicitly proven merge modes/repository postures.
- Customer-controlled Rust runners on proven Linux/macOS/Windows capability cells.
- Deterministic analyzers and gated model-assisted findings through proven API/harness cells.
- Exact current required-check publication.
- Local-first private-source handling and opt-in encrypted artifacts.
- Immutable patch proposal, independent verification, WebAuthn approval, and create-only fix PR publication.
- PostgreSQL durable workflow, budget, tenant RLS, audit, recovery, and observability.

## Explicitly excluded

- Automatic merge or direct source/default/protected-ref updates.
- Managed execution of private source in Mastercoder-owned sandboxes.
- Credentials inside repository guests.
- Unrestricted model/HTTP proxies.
- GitLab/Bitbucket parity.
- Active-active multi-region writes.
- Cross-tenant analysis-value caches.
- Pre-runner repository analysis promises.
- Self-repair loops beyond one explicitly enabled fresh cycle.
- Unsupported native builds treated as success.
- Implementation before P0 closure.

## Degradation

Capabilities are enabled cell by cell. A tenant may use deterministic review while model, native platform, publication, or high-independence cells remain disabled. Degradation is explicit and required-check policy determines whether the resulting evaluation is non-permitting.

## V2.1 scope clarification

Production scope remains conditional; no prototypes or implementation are authorized by this correction. FAST deterministic review is independently deliverable with all customer-owned model modes disabled. Model-assisted modes retain their separate API/harness compatibility gates. C3 is available only in a proven independent-administrative-domain deployment; individuals and same-domain teams receive explicit HUMAN_REVIEW_REQUIRED/BLOCKED when policy requires unavailable C3. Publication's create-only behavior is enforced at the independent gateway boundary with provider-token residual risk disclosed in 53. Dependency acquisition, expected coverage, trusted oracle provenance and provider check withdrawal are required security contracts, not optional convenience features.
