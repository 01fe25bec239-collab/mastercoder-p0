-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA. FORCE ROW LEVEL SECURITY on every base table and per-role grants from doc 07.
-- The tenant policies themselves are created in 0004_tenant_context.sql, after the function they call exists;
-- between the two files (same migrate() run) a table with RLS forced and no policy denies every row.
-- Deny by default: nothing is granted to PUBLIC; no role gets DELETE or TRUNCATE; only the three pointer
-- tables are updatable, and only on their non-identity columns (tenant/repository/epoch/key are immutable).

SET LOCAL ROLE mc_authority_owner;

DO $rls$
DECLARE
    t text;
BEGIN
    FOR t IN SELECT c.relname FROM pg_catalog.pg_class c
             WHERE c.relnamespace = 'mc_authority'::regnamespace AND c.relkind = 'r' LOOP
        EXECUTE format('ALTER TABLE mc_authority.%I ENABLE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE mc_authority.%I FORCE ROW LEVEL SECURITY', t);
    END LOOP;
END
$rls$;

REVOKE ALL ON ALL TABLES IN SCHEMA mc_authority FROM PUBLIC;

-- Schema USAGE for the 22 runtime roles so denials happen at the table, not at the schema.
DO $usage$
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
        EXECUTE format('GRANT USAGE ON SCHEMA mc_authority TO %I', r);
    END LOOP;
END
$usage$;

-- ordinary_api, runner_control, projection_writer, eligibility_writer, approval_writer, custody_writer,
-- plan_writer, publication_writer, effect_writer, budget_writer, dependency_writer, resolution_writer:
-- no privilege on these tables (ordinary_api writes requests only; the other roles own other modules' tables).

-- Doc 07 row: Evaluation and GateDecision -> gate_writer (insert only). gate_writer also reads every gate input.
GRANT INSERT, SELECT ON mc_authority.evaluation, mc_authority.gate_decision TO gate_writer;
GRANT SELECT ON
    mc_authority.expected_manifest, mc_authority.analysis_completion_receipt, mc_authority.evidence_receipt,
    mc_authority.verification_receipt, mc_authority.verification_set, mc_authority.policy_version,
    mc_authority.policy_pointer, mc_authority.trust_bundle, mc_authority.trust_pointer,
    mc_authority.revocation, mc_authority.current_candidate
    TO gate_writer;

-- Doc 07 rows: AnalysisCompletionReceiptV1, EvidenceReceiptV2 -> evidence_ingest_writer (insert only).
GRANT INSERT ON mc_authority.analysis_completion_receipt, mc_authority.evidence_receipt TO evidence_ingest_writer;
-- VerificationReceiptV2 -> verification_ingest_writer (insert only).
GRANT INSERT ON mc_authority.verification_receipt TO verification_ingest_writer;
-- VerificationSet -> verification_set_writer (insert only).
GRANT INSERT ON mc_authority.verification_set TO verification_set_writer;
-- Expected manifests -> manifest_writer (insert only).
GRANT INSERT ON mc_authority.expected_manifest TO manifest_writer;
-- Revocation state -> revocation_writer (append-only: no UPDATE, no DELETE).
GRANT INSERT ON mc_authority.revocation TO revocation_writer;

-- PolicyVersion and pointer -> policy_writer (immutable versions; CAS pointer, mutable columns only).
GRANT INSERT ON mc_authority.policy_version TO policy_writer;
GRANT INSERT, SELECT ON mc_authority.policy_pointer TO policy_writer;
GRANT UPDATE (policy_id, generation, updated_at) ON mc_authority.policy_pointer TO policy_writer;

-- Trust bundle and pointer -> trust_writer.
GRANT INSERT ON mc_authority.trust_bundle TO trust_writer;
GRANT INSERT, SELECT ON mc_authority.trust_pointer TO trust_writer;
GRANT UPDATE (trust_bundle_id, generation, updated_at) ON mc_authority.trust_pointer TO trust_writer;

-- Current candidate pointer -> candidate_writer.
GRANT INSERT, SELECT ON mc_authority.current_candidate TO candidate_writer;
GRANT UPDATE (provider_generation, observation_digest, writer_id, observed_at, expires_at)
    ON mc_authority.current_candidate TO candidate_writer;

-- readonly: SELECT on every base table below.
GRANT SELECT ON
    mc_authority.expected_manifest, mc_authority.evaluation, mc_authority.gate_decision,
    mc_authority.analysis_completion_receipt, mc_authority.evidence_receipt, mc_authority.verification_receipt,
    mc_authority.verification_set, mc_authority.policy_version, mc_authority.policy_pointer,
    mc_authority.trust_bundle, mc_authority.trust_pointer, mc_authority.revocation,
    mc_authority.current_candidate
    TO readonly;
