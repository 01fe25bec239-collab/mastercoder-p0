-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA. Tenant context set ONLY through a SECURITY DEFINER function (INV-15, doc 07).
-- A caller-set GUC is not authentication: the tenant policies do NOT read any GUC. They read a per-transaction
-- row in mc_authority.tenant_context that no runtime role can write directly; the only writer is
-- set_tenant_context(), which the Rust API calls with the fields of a VerifiedTenantAssertion.
-- The row is keyed by (backend pid, transaction id): it is invisible to the next transaction on a pooled
-- connection (no leak) and disappears with a rollback / savepoint rollback. UNLOGGED: cleared on crash.
-- tenant_context is internal per-transaction authentication state, not tenant-owned data, so it is the one
-- table whose primary key has no tenant_id.

SET LOCAL ROLE mc_authority_owner;

CREATE UNLOGGED TABLE mc_authority.tenant_context (
    backend_pid integer NOT NULL,
    xact_id xid8 NOT NULL,
    tenant_id uuid NOT NULL,
    workload_id uuid NOT NULL,
    assertion_digest mc_authority.digest NOT NULL,
    PRIMARY KEY (backend_pid, xact_id)
);
REVOKE ALL ON mc_authority.tenant_context FROM PUBLIC;
ALTER TABLE mc_authority.tenant_context ENABLE ROW LEVEL SECURITY;
ALTER TABLE mc_authority.tenant_context FORCE ROW LEVEL SECURITY;
-- Only the definer functions (owner) can ever touch it, and only their own backend's rows.
CREATE POLICY own_backend ON mc_authority.tenant_context TO mc_authority_owner
    USING (backend_pid = pg_catalog.pg_backend_pid())
    WITH CHECK (backend_pid = pg_catalog.pg_backend_pid() AND xact_id = pg_catalog.pg_current_xact_id());

-- Establishes the transaction-local tenant from a verified assertion. SQLSTATE MCT01 = tenant mismatch.
CREATE FUNCTION mc_authority.set_tenant_context(p_tenant_id uuid, p_workload_id uuid, p_assertion_digest bytea)
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, mc_authority, pg_temp
AS $fn$
DECLARE
    v_xact xid8 := pg_catalog.pg_current_xact_id();
    v_existing mc_authority.tenant_context%ROWTYPE;
BEGIN
    IF p_tenant_id IS NULL OR p_workload_id IS NULL OR p_assertion_digest IS NULL
       OR pg_catalog.octet_length(p_assertion_digest) <> 32
       OR p_tenant_id = '00000000-0000-0000-0000-000000000000'::uuid THEN
        RAISE EXCEPTION 'invalid tenant assertion' USING ERRCODE = 'invalid_parameter_value';
    END IF;

    -- Forget earlier transactions of this (pooled) connection.
    DELETE FROM mc_authority.tenant_context
     WHERE backend_pid = pg_catalog.pg_backend_pid() AND xact_id <> v_xact;

    SELECT * INTO v_existing FROM mc_authority.tenant_context
     WHERE backend_pid = pg_catalog.pg_backend_pid() AND xact_id = v_xact;
    IF FOUND THEN
        IF v_existing.tenant_id = p_tenant_id AND v_existing.workload_id = p_workload_id
           AND v_existing.assertion_digest = p_assertion_digest THEN
            RETURN; -- idempotent re-assertion of the same verified assertion
        END IF;
        RAISE EXCEPTION 'tenant context mismatch' USING ERRCODE = 'MCT01';
    END IF;

    INSERT INTO mc_authority.tenant_context (backend_pid, xact_id, tenant_id, workload_id, assertion_digest)
    VALUES (pg_catalog.pg_backend_pid(), v_xact, p_tenant_id, p_workload_id, p_assertion_digest);
END
$fn$;

-- The tenant of the current transaction, or NULL (=> every policy denies). No GUC involved.
CREATE FUNCTION mc_authority.current_tenant()
RETURNS uuid
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = pg_catalog, mc_authority, pg_temp
AS $fn$
    SELECT tenant_id FROM mc_authority.tenant_context
     WHERE backend_pid = pg_catalog.pg_backend_pid() AND xact_id = pg_catalog.pg_current_xact_id_if_assigned()
$fn$;

-- Narrow EXECUTE: only the 22 runtime roles; never PUBLIC.
REVOKE ALL ON FUNCTION mc_authority.set_tenant_context(uuid, uuid, bytea) FROM PUBLIC;
REVOKE ALL ON FUNCTION mc_authority.current_tenant() FROM PUBLIC;
DO $exec$
DECLARE
    r text;
BEGIN
    FOREACH r IN ARRAY ARRAY[
        'ordinary_api', 'runner_control', 'evidence_ingest_writer', 'verification_ingest_writer',
        'verification_set_writer', 'gate_writer', 'policy_writer', 'candidate_writer',
        'projection_writer', 'trust_writer', 'revocation_writer', 'manifest_writer',
        'eligibility_writer', 'approval_writer', 'custody_writer', 'plan_writer',
        'publication_writer', 'effect_writer', 'budget_writer', 'dependency_writer',
        'resolution_writer', 'readonly'
    ] LOOP
        EXECUTE format('GRANT EXECUTE ON FUNCTION mc_authority.set_tenant_context(uuid, uuid, bytea) TO %I', r);
        EXECUTE format('GRANT EXECUTE ON FUNCTION mc_authority.current_tenant() TO %I', r);
    END LOOP;
END
$exec$;

-- Tenant isolation on every tenant-owned base table: rows are visible/writable only for the transaction's
-- authenticated tenant (USING for reads/updates/deletes, WITH CHECK for inserts/updates).
DO $policy$
DECLARE
    t text;
BEGIN
    FOR t IN SELECT c.relname FROM pg_catalog.pg_class c
             WHERE c.relnamespace = 'mc_authority'::regnamespace AND c.relkind = 'r'
               AND c.relname <> 'tenant_context' LOOP
        EXECUTE format(
            'CREATE POLICY tenant_isolation ON mc_authority.%I '
            'USING (tenant_id = mc_authority.current_tenant()) '
            'WITH CHECK (tenant_id = mc_authority.current_tenant())', t);
    END LOOP;
END
$policy$;
