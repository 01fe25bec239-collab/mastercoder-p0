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
memberships allowed are the offline login roles below). Role names are cluster-global: when `0001_roles.sql` is
actually executed it creates missing roles, re-asserts these attributes on existing ones and revokes any membership
between these roles, so do not reuse these names for anything else on the cluster. `migrate()` records applied
files and skips unchanged ones, so re-running the migration runner does NOT repair role drift: it only executes
`0001_roles.sql` on a database where that file is not yet recorded. Role drift is detected and corrected by the
operator re-running that file (or the equivalent `ALTER ROLE` / `REVOKE`) under the privileged bootstrap connection.

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

A caller-set GUC is not authentication, and neither is a Rust type, a digest shape or a role label. The policies
read `mc_authority.current_tenant()`, which is backed by a per-transaction row that only
`mc_authority.set_tenant_context(tenant_id, workload_id, assertion_digest)` can write. The setter writes it only if
`mc_authority.tenant_authorization` holds a row binding the **authenticated database session** (`session_user`, the
login; not `current_user`, which inside a SECURITY DEFINER function is the owner and after `SET ROLE` is the runtime
role) to exactly that tenant, workload and digest. Unknown login, no row or any disagreement raises SQLSTATE `42501`
and leaves no context. In Rust: authenticate the caller, build a `VerifiedTenantAssertion`, then
`mc_authority_db::with_tenant(&mut tx, &TenantContext::from_verified(&assertion))` as the first statement of every
tenant transaction. A second call in the same transaction with a different assertion fails with `TenantMismatch`
(SQLSTATE `MCT01`). Without a context every policy denies all rows.

A context is *eligible* only inside the transaction (and backend) that established it: `current_tenant()` matches
the row on `(backend pid, current transaction id)`, so a later transaction on a pooled connection, or a rolled-back
transaction/savepoint, has no tenant. This is separate from *storage*: a committed context row is not deleted at
commit; it stays in the UNLOGGED `tenant_context` table, unusable, until the same backend's next
`set_tenant_context` call prunes it (or a crash clears the table). No cleanup job exists or is needed.

### Provisioning tenant authorization (trusted, offline)

`tenant_authorization` is owned by `mc_authority_owner`, FORCE RLS, and **no runtime role has any privilege on it**
(and none can `SET ROLE` to the owner or change its `session_user`). Only the trusted operator / verifier supplies
rows, on the privileged bootstrap connection, after it has verified the workload (mTLS, enrollment, signed receipt;
those pipelines are external and are NOT implemented here):

```sql
SET LOCAL ROLE mc_authority_owner;
INSERT INTO mc_authority.tenant_authorization (principal, tenant_id, workload_id, assertion_digest)
VALUES ('<workload login role>', '<tenant uuid>', '<workload uuid>', '<32-byte verified digest>');
```

One row per login and tuple; remove a row to revoke. There is no default row, no name-derived tenant and no
fallback. Runtime sessions reach the database only as the login role (member of exactly one runtime role).

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
two tenant-context functions (EXECUTE is not the trust boundary; the session authorization above is); nobody has
DELETE or TRUNCATE, and nobody but the owner has any privilege on `tenant_context` or `tenant_authorization`.

## Not in this base

Gate currentness events, the immutable-observation table behind `current_candidate`, intake/quarantine tables
and "approved views" for `ordinary_api` belong to the owning modules.
