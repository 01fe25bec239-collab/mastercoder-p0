# Integrated V2.1 independent review log

Final reconciliation date: 2026-10-03. Disposition: V2_1_READY_FOR_NARROW_EXTERNAL_AUDIT. No outstanding architecture repair blocker remains in the completed internal reviews. No runtime capability is enabled and no P0 experiment was executed.

Five fresh independent review roles examined the complete normative architecture through their respective lenses. Authoring agents and integrating author did not approve their own repairs. Account-limit interruptions are not approvals; checkpoint reports retained their original pending conclusions until later independent read-back. Original report text is preserved; closure is by explicit addendum in another independent report where necessary.

| Role | Initial issues | Independent final disposition |
|---|---|---|
| A4-V2_1-SECURITY | SEC-01 enrollment wire/ordering; SEC-02 oracle purpose; SEC-03 eligibility propagation | All three independently CLOSED by the later Contract Consistency report's security closure addendum. Original security report remains an honest checkpoint, not rewritten to pretend its author performed the later reread. |
| A4-V2_1-DISTRIBUTED-SYSTEMS | DS-01 stale restored entitlements; DS-02 retry rule contradiction; DS-03 manifest ordering | DS-01/02 independently resolved in original report. Recovery report independently closes final DS-03 sibling diagram and corroborates bootstrap/P0/RTO propagation. |
| A4-V2_1-CONTRACT-CONSISTENCY | CC-01 pre-proof check records; CC-02 obsolete negotiation bitmap; CC-03 root ceilings; operation-specific enrollment quorum | PASS_FOR_NARROW_EXTERNAL_AUDIT. All repaired and independently reread. Complete registry/schema projection and committed vectors checked. |
| A4-V2_1-RECOVERY | Lost intent, old sender, unknown effects, lost revocation/custody and provider write races attempted | NO_NEW_ARCHITECTURE_BLOCKER_FOUND; closes DS-03 and records failure-envelope limits. |
| A4-V2_1-PRODUCT-OPERABILITY | PO-01 prototype lifecycle versus outcome; PO-02 stale interrupted claim snapshot | PASS_FOR_NARROW_EXTERNAL_AUDIT; PO-01 closed, PO-02 withdrawn after current-snapshot reread. |

## Corrections arising from internal review

Enrollment uses pre-registration intent→create challenge→new key→final action→operation-specific owner/admin consent or offline signatures→authorization. WebAuthn users are not assumed to hold AA signing keys. Oracle acceptance uses a separate resolution purpose. Per-effect eligibility includes actor/current membership. Restore denies all historical entitlements pending fresh independent bootstrap rather than assuming a nonexistent complete security log. Manifest follows allocation before execution. Check payloads allow absent pre-verdict identity/proof only for nonpermitting states. Root resource ceilings include model calls/events/dependency quantities. Negotiation binds exact schema registry bytes. Polymorphic audit/revocation subjects and manual prior state have explicit record-qualified types. Acquisition calls have exact per-call request bodies rather than treating aggregate grants as request digests.

## Mechanical evidence versus runtime evidence

The Contract Consistency reviewer independently checked all 85 schema projections, 440 wire fixtures,40 declarative semantic specification cases, reconstructed six Ed25519 signature inputs, and verified six valid/rejected six corrupted signatures using OpenSSL. These are local architecture-artifact checks; they do not constitute independent Rust/TypeScript implementations, tested PostgreSQL permissions, provider semantics, WebAuthn hardware or sandbox experiments. All 55 atomic P0 claims remain NOT_RUN.

The original frozen V2 reviews are kept separately in BASELINE_HISTORY and do not count as V2.1 approval. Complete signed/hashed contract semantics and real failure-domain/provider properties remain appropriate targets for narrow external audit, then separately authorized P0 qualification.
