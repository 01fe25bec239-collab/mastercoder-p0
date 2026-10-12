# Decision Provenance

| Decision | Origin | OPUS position | SOL position | Audit pressure | V2 choice and tradeoff |
|---|---|---|---|---|---|
| V2-D001 Rust modular monolith | MODIFIED_OPUS | Rust/customer-hosted execution, but package topology drifted | Modular monolith, originally TypeScript | Cross-document drift | Frozen Rust stack; one codebase, several authority-scoped roles; fewer protocols than microservices |
| V2-D002 PostgreSQL workflow | BOTH | Durable jobs/outbox/fencing | Sole authority/queue/budget ledger | No material rejection | Keep; accept single primary write authority and engineer DR around it |
| V2-D003 Separate Git authorities | MODIFIED_SOL | Shared signer in one revision | Review/Publisher separation | SOL-OPUS-001 | Separate Apps, keys, egress, and effect vocabularies |
| V2-D004 Exact MergeSubjectV2 | NEW_SYNTHESIS | Head/merge-base focus | Typed identity but candidate ambiguity | SOL2-SOL1-003, SOL-OPUS-004/009, OPUS-SOL-006 | Bind provider candidate commit and tree; block when equivalence is unproved |
| V2-D005 Canonical binding | MODIFIED_SOL | Digest-bound workflows | Full binding but circular | OPUS-SOL-003, SOL2-SOL1-006 | RFC 8949 deterministic CBOR, domain-separated hashes, non-self-referential types |
| V2-D006 Evidence independence | BOTH | Closed grades | Independent immutable receipts | SOL-OPUS-006/007/017, OPUS-SOL-011/013 | C0-C4 capability classes and causal reproduction; reduced availability on small plans |
| V2-D007 Sandbox tiers | MODIFIED_SOL | Linux tiers with disputed shared kernel | Cross-platform capability matrix | SOL-OPUS-003/012 | VM-grade S2 default for hostile code; greater cost for defensible isolation |
| V2-D008 ModelTurnV2 | NEW_SYNTHESIS | Host brokerage/pinned harness | Credential/parser split but fixed envelope | SOL-OPUS-002/011, OPUS-SOL-002/004 | Structured provider-neutral tool loop plus secret dialer; parity remains gated |
| V2-D009 Resource graph envelope | NEW_SYNTHESIS | Graph limits partly asserted | Graph manager and budgets | OPUS-SOL-011, SOL-OPUS-010 | Broker-enforced root budget; hidden native subagents only outer-limited |
| V2-D010 EligibilityProofV2 | NEW_SYNTHESIS | Policy modes underspecified | Eligibility state | SOL2-SOL1-004, OPUS-SOL-009 | Stable provider IDs and current proof; reduced offline automation |
| V2-D011 WebAuthn Approval Authority | NEW_SYNTHESIS | Digest-specific user approval | Immutable approval but forgeable API path | SOL2-SOL1-002, SOL-OPUS-008/015 | Separate RP/key; direct user verification; additional service and recovery burden |
| V2-D012 Verified publication plan | NEW_SYNTHESIS | Isolated publisher but excess authority | Safe effect broker without source path | OPUS-SOL-001, SOL-OPUS-001/008 | Verifier provides base/candidate/object plan; broker checks returned tree equality |
| V2-D013 Tenant epochs and RLS | MODIFIED_SOL | Tenant isolation, inconsistent public caching | RLS/local-first custody | SOL-OPUS-013/014 | Composite tenant keys, RLS, repo ownership epochs, no cross-tenant value cache |
| V2-D014 Late settlement | NEW_SYNTHESIS | Reservations | Transactional ledger | SOL-OPUS-010, OPUS-SOL-007 | Meter actual spend after fencing; debt/freeze without result resurrection |
| V2-D015 Revocation and skew | NEW_SYNTHESIS | Revocation present but race vague | Generations/protocols incomplete | SOL-OPUS-015, OPUS-SOL-004/017 and unnumbered skew audit | Generation-bound sessions, STARTED linearization, N/N-1 fail-closed negotiation |
| V2-D016 Cross-platform cells | BOTH | Rust runner and local custody | Explicit capability matrix | SOL-OPUS-003, OPUS-SOL-005/014 | OS support is not native build support; unsupported cells cannot pass |

Source hashes and package identities are recorded in TRACEABILITY/INPUT_MANIFEST.md. Detailed rationale is in the ADR set. Prototype-dependent decisions remain architecturally selected but operationally disabled.

