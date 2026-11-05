-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA r1 (repair of A3 finding F-1). Supersedes the setter of 0004 (appended, because applied
-- migration files are immutable: roles/README.md rule 6).
--
-- 0004 let ANY runtime session choose its own tenant: the setter checked only that the arguments were well formed.
-- Now set_tenant_context() additionally requires a row in mc_authority.tenant_authorization that binds the
-- AUTHENTICATED DATABASE SESSION (session_user: fixed by the login, unchanged by SECURITY DEFINER, SET ROLE and
-- unprivileged callers cannot change it with SET SESSION AUTHORIZATION) to exactly this tenant, workload and
-- assertion digest. Nothing the caller passes can create such a row: the table is owned by the offline migration
-- owner, FORCE RLS, and no runtime role has any privilege on it. Rows are provisioned by the trusted operator /
-- verifier under the privileged bootstrap connection (roles/README.md "Provisioning tenant authorization").
-- This is a grant/session binding, not the external mTLS / enrollment / signed-receipt pipeline.

SET LOCAL ROLE mc_authority_owner;

CREATE TABLE mc_authority.tenant_authorization (
    principal name NOT NULL,                    -- the login (session_user) that may assert the tuple below
    tenant_id uuid NOT NULL CHECK (tenant_id <> '00000000-0000-0000-0000-000000000000'::uuid),
    workload_id uuid NOT NULL,
    assertion_digest mc_authority.digest NOT NULL,
    PRIMARY KEY (principal, tenant_id, workload_id, assertion_digest)
);
REVOKE ALL ON mc_authority.tenant_authorization FROM PUBLIC;
ALTER TABLE mc_authority.tenant_authorization ENABLE ROW LEVEL SECURITY;
ALTER TABLE mc_authority.tenant_authorization FORCE ROW LEVEL SECURITY;
-- Only the owner (provisioning, and the definer setter below) can see or change rows; runtime roles have no grant.
CREATE POLICY owner_only ON mc_authority.tenant_authorization TO mc_authority_owner
    USING (true) WITH CHECK (true);

-- Same signature, owner and EXECUTE grants as 0004. SQLSTATE MCT01 = tenant mismatch, 42501 = not authorized.
CREATE OR REPLACE FUNCTION mc_authority.set_tenant_context(p_tenant_id uuid, p_workload_id uuid, p_assertion_digest bytea)
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
        -- A context only ever exists here after the authorization below succeeded in this transaction.
        IF v_existing.tenant_id = p_tenant_id AND v_existing.workload_id = p_workload_id
           AND v_existing.assertion_digest = p_assertion_digest THEN
            RETURN; -- idempotent re-assertion of the same authorized assertion
        END IF;
        RAISE EXCEPTION 'tenant context mismatch' USING ERRCODE = 'MCT01';
    END IF;

    -- The authorization predicate: the authenticated session must be provisioned for exactly this tuple.
    IF NOT EXISTS (
        SELECT 1 FROM mc_authority.tenant_authorization a
         WHERE a.principal = session_user
           AND a.tenant_id = p_tenant_id
           AND a.workload_id = p_workload_id
           AND a.assertion_digest = p_assertion_digest
    ) THEN
        RAISE EXCEPTION 'tenant authorization denied' USING ERRCODE = 'insufficient_privilege';
    END IF;

    INSERT INTO mc_authority.tenant_context (backend_pid, xact_id, tenant_id, workload_id, assertion_digest)
    VALUES (pg_catalog.pg_backend_pid(), v_xact, p_tenant_id, p_workload_id, p_assertion_digest);
END
$fn$;
