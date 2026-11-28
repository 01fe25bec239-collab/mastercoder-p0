-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA. Schema, owned by the offline migration owner. No PUBLIC access.
-- The migrating connection must be a superuser (or a member of mc_authority_owner with SET).

CREATE SCHEMA mc_authority AUTHORIZATION mc_authority_owner;
REVOKE ALL ON SCHEMA mc_authority FROM PUBLIC;
