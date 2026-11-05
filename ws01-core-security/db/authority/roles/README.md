# Authority database role framework v1

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

Source: doc 07 "Authority and DB-write matrix" and "Grants and tenant isolation", doc 18 relational constraints,
ADR-013 / ADR-019, INV-06, INV-15. Implemented by W01-T007-DB-ROLES-SCHEMA. For other workstreams that add
module tables under `db/authority/<module>/`.

## Roles (cluster-global names)

`mc_authority_owner` — the **offline migration owner**; owns every `mc_authority` object. NOLOGIN: the operator
applies migrations with a privileged connection (superuser, or a role that may `SET ROLE mc_authority_owner`).
The migration files use `SET LOCAL ROLE mc_authority_owner`.

The 22 runtime roles, exactly: `ordinary_api`, `runner_control`, `evidence_ingest_writer`,
`verification_ingest_writer`, `verification_set_writer`, `gate_writer`, `policy_writer`, `candidate_writer`,
`projection_writer`, `trust_writer`, `revocation_writer`, `manifest_writer`, `eligibility_writer`,
`approval_writer`, `custody_writer`, `plan_writer`, `publication_writer`, `effect_writer`, `budget_writer`,
`dependency_writer`, `resolution_writer`, `readonly`.

Every role is `NOLOGIN NOINHERIT NOBYPASSRLS`, not superuser, no CREATEDB/CREATEROLE/REPLICATION, owns nothing,
and is **not a member of any other of these roles** (no `SET ROLE` path between runtime roles; the only
memberships allowed are the offline login roles below). Role names are cluster-global: `0001_roles.sql` creates
missing roles, re-asserts these attributes on existing ones and revokes any membership between these roles on
every run, so do not reuse these names for anything else on the cluster.

Login identities are an offline deployment step and are not created here (no secrets in the repo). Each
workload gets its own login role that is a member of exactly ONE runtime role (`GRANT <runtime_role> TO <login>`)
and uses it via `SET ROLE`; one login role must never be a member of two runtime roles.

## Rules for module tables

1. Create tables as `mc_authority_owner` (`SET LOCAL ROLE mc_authority_owner;` at the top of the file).
2. `tenant_id` in every primary key and every foreign key; repository-scoped FKs also bind `repository_id` and
   `repository_tenant_epoch`. Use the `mc_authority.digest` / `mc_authority.u64` domains.
3. `ENABLE` + `FORCE ROW LEVEL SECURITY` and this policy on every tenant-owned table:
   `CREATE POLICY tenant_isolation ON mc_authority.<t> USING (tenant_id = mc_authority.current_tenant()) WITH CHECK (tenant_id = mc_authority.current_tenant());`
4. Grant the minimum per doc 07, to the one dedicated writer role. Immutable tables: INSERT only. Mutable
   pointers: column-level `UPDATE` on non-identity columns only. Never grant to PUBLIC; never DELETE/TRUNCATE.
5. `SECURITY DEFINER` only with `SET search_path = pg_catalog, mc_authority, pg_temp`, schema-qualified names, no
   dynamic SQL from caller input, `REVOKE ALL ... FROM PUBLIC` and a narrow `GRANT EXECUTE`.
6. Add new migration files to your module directory (listed in `db/authority/MODULES`, applied in that order, files
   in lexical order). Applied files are recorded with a SHA-256 in `mc_migration.applied`; editing an applied file
   is an error. Run `migrate()` as the offline owner only.

## Tenant context (INV-15)

A caller-set GUC is not authentication. The policies read `mc_authority.current_tenant()`, which is backed by a
per-transaction row that only `mc_authority.set_tenant_context(tenant_id, workload_id, assertion_digest)` can
write. In Rust: authenticate the caller, build a `VerifiedTenantAssertion`, then
`mc_authority_db::with_tenant(&mut tx, &TenantContext::from_verified(&assertion))` as the first statement of every
tenant transaction. A second call in the same transaction with a different assertion fails with `TenantMismatch`
(SQLSTATE `MCT01`). The context ends with the transaction; a pooled connection never carries a tenant over.
Without a context every policy denies all rows.

## Grants in this base (doc 07)

| Table | INSERT | UPDATE | SELECT |
|---|---|---|---|
| evaluation, gate_decision | gate_writer | — | gate_writer, readonly |
| analysis_completion_receipt, evidence_receipt | evidence_ingest_writer | — | gate_writer, readonly |
| verification_receipt | verification_ingest_writer | — | gate_writer, readonly |
| verification_set | verification_set_writer | — | gate_writer, readonly |
| expected_manifest | manifest_writer | — | gate_writer, readonly |
| revocation | revocation_writer | — | gate_writer, readonly |
| policy_version | policy_writer | — | gate_writer, readonly |
| policy_pointer | policy_writer | policy_writer (policy_id, generation, updated_at) | policy_writer, gate_writer, readonly |
| trust_bundle | trust_writer | — | gate_writer, readonly |
| trust_pointer | trust_writer | trust_writer (trust_bundle_id, generation, updated_at) | trust_writer, gate_writer, readonly |
| current_candidate | candidate_writer | candidate_writer (observation columns) | candidate_writer, gate_writer, readonly |

`ordinary_api` has no privilege on any gate table (and no EXECUTE on any gate transition); all other roles have
none on these tables (their tables come from other modules). Everyone has USAGE on the schema and EXECUTE on the
two tenant-context functions; nobody has DELETE or TRUNCATE.

## Not in this base

Gate currentness events, the immutable-observation table behind `current_candidate`, intake/quarantine tables
and "approved views" for `ordinary_api` belong to the owning modules.
