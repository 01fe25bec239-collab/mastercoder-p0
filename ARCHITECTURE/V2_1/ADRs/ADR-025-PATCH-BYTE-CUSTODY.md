# ADR-025: Patch-byte custody through reconciliation

Status: ACCEPTED_V2_1_CONTRACT; capability qualification pending atomic prototypes.

Finding 13: ephemeral patch production plus digests does not ensure later exact display/replay. Amend ADR-012: PatchCustodyV1 retains immutable encrypted complete bytes/manifest/base rendering material in tenant-authorized durable custody, pins through approval/publication and unresolved effects, validates every read, and defines deletion/hold/loss. Local custody preserves the source-residency default; managed custody is opt-in. Missing permitted custody disables publication rather than assuming hidden retention. Normative contracts: 14 and 24; P0-05/P0-09/P0-10 test corruption, deletion and restore.

Rust/Axum/Tokio/SQLx/Serde, PostgreSQL, Next.js/React/TypeScript and Rust runner remain unchanged. No product implementation or prototype execution is authorized by this ADR.
