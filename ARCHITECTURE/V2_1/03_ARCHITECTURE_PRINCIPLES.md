# Architecture Principles

1. **One authority, many projections.** PostgreSQL commits the state transition. Provider checks, object storage, web views, metrics, and search are projections or effects.
2. **Positive interfaces, not prohibition lists.** Every privileged boundary exposes a closed operation vocabulary that is sufficient for its legitimate work.
3. **Bind before acting.** Subject, policy, capability, budget, protocol, and generation are immutable inputs to an attempt or effect.
4. **Recheck at the effect boundary.** Approval or an earlier check does not excuse the final authority from validating expiry, currentness, generation, and exact target.
5. **Credentials and hostile parsing do not meet.** Injectors and dialers handle opaque authorized bytes; parsers and guests remain credential free.
6. **Claims are not facts.** Webhooks, runners, agents, model responses, and signatures establish provenance, never correctness by themselves.
7. **Fail closed visibly.** Unsupported, partial, unknown, or stale work is a named state with a user-visible reason and non-permitting provider result.
8. **No silent capability downgrade.** Sandbox, independence, checkout, provider, and platform cells are bound and explicitly supported or unsupported.
9. **External reality is reconciled.** Ambiguous provider calls enter UNKNOWN and are inspected read-only before any new mutation.
10. **Revocation has a linearization point.** Pre-STARTED authorization is rejectable; post-STARTED completion is observable and reconciled.
11. **Least topology.** The control plane remains a modular monolith. A separate process exists only to isolate a credential, signer, egress path, or hostile workload.
12. **Evidence before authorization.** A selected design can be ready for external audit while production freeze and implementation remain blocked. P0 artifacts, not confidence, unlock capability cells.

