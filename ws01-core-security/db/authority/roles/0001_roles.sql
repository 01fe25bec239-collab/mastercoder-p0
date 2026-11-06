-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA. Source: doc 07 "Authority and DB-write matrix", ADR-013/ADR-019, INV-06/INV-15.
-- Roles are cluster-global, so this file is idempotent and race-tolerant (parallel databases on one cluster).
-- Every role is NOLOGIN NOINHERIT NOBYPASSRLS and no role here is a member of another (no SET ROLE path
-- between runtime roles); stray memberships between these roles are revoked on every run.
-- Login identities are created offline by the operator; see roles/README.md.

DO $roles$
DECLARE
    r text;
    m record;
    i integer;
    -- mc_authority_owner: the offline migration owner (owns every mc_authority object),
    -- then the 22 runtime roles of doc 07, exactly.
    v_roles constant text[] := ARRAY[
        'mc_authority_owner',
        'ordinary_api', 'runner_control', 'evidence_ingest_writer', 'verification_ingest_writer',
        'verification_set_writer', 'gate_writer', 'policy_writer', 'candidate_writer',
        'projection_writer', 'trust_writer', 'revocation_writer', 'manifest_writer',
        'eligibility_writer', 'approval_writer', 'custody_writer', 'plan_writer',
        'publication_writer', 'effect_writer', 'budget_writer', 'dependency_writer',
        'resolution_writer', 'readonly'
    ];
BEGIN
    FOREACH r IN ARRAY v_roles LOOP
        BEGIN
            EXECUTE format('CREATE ROLE %I NOLOGIN NOINHERIT NOBYPASSRLS', r);
        EXCEPTION WHEN duplicate_object OR unique_violation THEN
            NULL; -- already exists (or created concurrently by another database's migration)
        END;
        -- Re-assert the exact attributes only when they differ (avoids concurrent ALTER ROLE races).
        IF EXISTS (
            SELECT 1 FROM pg_catalog.pg_roles
            WHERE rolname = r
              AND (rolcanlogin OR rolinherit OR rolbypassrls OR rolsuper OR rolcreatedb
                   OR rolcreaterole OR rolreplication)
        ) THEN
            FOR i IN 1..10 LOOP
                BEGIN
                    EXECUTE format(
                        'ALTER ROLE %I NOLOGIN NOINHERIT NOBYPASSRLS NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION', r);
                    EXIT;
                EXCEPTION WHEN internal_error THEN -- "tuple concurrently updated": another database is healing it too
                    IF i = 10 THEN RAISE; END IF;
                    PERFORM pg_catalog.pg_sleep(0.05 * i);
                END;
            END LOOP;
        END IF;
    END LOOP;

    FOR m IN
        SELECT g.rolname AS grp, u.rolname AS mem
        FROM pg_catalog.pg_auth_members x
        JOIN pg_catalog.pg_roles g ON g.oid = x.roleid
        JOIN pg_catalog.pg_roles u ON u.oid = x.member
        WHERE g.rolname = ANY (v_roles) AND u.rolname = ANY (v_roles)
    LOOP
        FOR i IN 1..10 LOOP
            BEGIN
                EXECUTE format('REVOKE %I FROM %I', m.grp, m.mem);
                EXIT;
            EXCEPTION WHEN internal_error THEN
                IF i = 10 THEN RAISE; END IF;
                PERFORM pg_catalog.pg_sleep(0.05 * i);
            END;
        END LOOP;
    END LOOP;
END
$roles$;
