# Canonical protocol glossary

`SCHEMAS/REGISTRY.json` is the machine-readable type registry and exact field-order authority. The per-type schemas and ENCODING.md form the complete wire specification. The validator rejects undeclared versioned names in normative Markdown. A human domain noun (review, patch, publication) is not implicitly a distinct wire type.

| Historical alias | Only canonical type | Rule |
|---|---|---|
| ProducerBindingV2 | ProducerEventV2 | Historical spelling only; no accepted wire alias |
| PublicationPlanV2 | VerifiedPublicationPlanV2 | Plan must be independently verified before this type is valid |
| ApprovalProofV2 | HumanApprovalAssertionV2 | Independent AA assertion following trusted display + fresh UV |
| EffectIntent / external effect | EffectIntentV2 + EffectStateV1 | Immutable request separate from append-only runtime state |
| UNKNOWN_EFFECT | UNKNOWN | Historical state label; not a new wire enum |
| CHECK_CREATE / CHECK_UPDATE | CREATE_CHECK / UPDATE_CHECK | Canonical EffectIntentV2 discriminants |
| Publisher / Publication Effect Broker | Credential-free Planner + Git Publication Capability Gateway | Historical combined role split in ADR-016 |

Aliases are allowed in this mapping and BASELINE_HISTORY only. Unknown fields, names, revisions and enums reject. `ABSENT_CONFIRMED` is deliberately absent from the state schema; references elsewhere explain its removal. C0–C4 denote independence and E0–E4 denote evidence grade, not synonymous dimensions. OracleTrustClassV1 is a third independent dimension. `Mastercoder Review` is the sole canonical required-check name for the GitHub profile; its expected App identity and exact subject are also mandatory.

Protocol structs refine V2's intended deterministic fixed-array representation without claiming compatibility with an implemented V2 serializer. Any signed/digested contract amendment changes registry hash and evidence scope. Run the architecture documentation checker before exporting; P0-06 independently implements Rust/TypeScript after separate authorization.
