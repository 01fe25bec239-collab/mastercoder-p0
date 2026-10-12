# ADR-027: Authorized Manual Resolution Without Evidence Promotion

Status: ACCEPTED_V2_1_ARCHITECTURE; capability validation remains pending.

V2 exposed human/manual states without complete authorization, evidence, expiry and resolution paths.

Decision: ManualResolutionV1 records an immutable signed scoped decision through Resolution Authority. Document 56 defines maintainer/policy/reconciliation roles, trusted evidence, one-use ceremony, expiry, audit and fresh-evaluation transitions. No manual decision converts missing/unsupported/untrusted evidence into deterministic PASS. Unknown external effects may close only with exact outcome/nonexecution proof or an uncertainty tombstone that continues blocking duplicate effects.

Consequence: explicit accepted-risk and non-pass outcomes are usable product states; they do not impersonate gate success. New publication requires fresh independent approval and per-effect authorization. Atomic validation includes unauthorized/replayed/expired decisions and false-absence retries. Amends ADR-014; finding 16.
