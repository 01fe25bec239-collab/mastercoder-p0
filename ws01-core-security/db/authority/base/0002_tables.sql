-- PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
-- W01-T007-DB-ROLES-SCHEMA. Representative gate-critical tables (doc 07 matrix, doc 18 constraints).
-- Column names/enums are copied from SCHEMAS/REGISTRY.json (ScopeV1, EvaluationIdentityV2, GateProofV2,
-- AnalysisCompletionReceiptV1, EvidenceReceiptV2, VerificationReceiptV2, VerificationSetV2, PolicyVersionV1,
-- TrustBundleV1, RevocationRecordV1, CurrentCandidateV1, ExpectedAnalysisManifestV1). Only the columns the
-- constraints need are kept; owning modules extend these later. Tenant isolation is structural:
-- tenant_id is in every primary key and every foreign key; repository-scoped FKs also bind
-- repository_id + repository_tenant_epoch (doc 18). RLS + grants are in 0003/0004.

SET LOCAL ROLE mc_authority_owner;

CREATE DOMAIN mc_authority.digest AS bytea CHECK (octet_length(VALUE) = 32);
CREATE DOMAIN mc_authority.u64 AS numeric(20, 0) CHECK (VALUE >= 0 AND VALUE <= 18446744073709551615);

-- Immutable signed manifest pinned before execution (manifest_writer).
CREATE TABLE mc_authority.expected_manifest (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    manifest_id uuid NOT NULL,
    manifest_digest mc_authority.digest NOT NULL,
    attempt_binding_digest mc_authority.digest NOT NULL,
    candidate_tree bytea NOT NULL CHECK (octet_length(candidate_tree) IN (20, 32)),
    policy_digest mc_authority.digest NOT NULL,
    analysis_plan_digest mc_authority.digest NOT NULL,
    issuer_id uuid NOT NULL,
    issued_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, manifest_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
);

-- Evaluation identity (gate_writer, insert only).
CREATE TABLE mc_authority.evaluation (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    evaluation_id uuid NOT NULL,
    evaluation_sequence mc_authority.u64 NOT NULL,
    subject_digest mc_authority.digest NOT NULL,
    policy_digest mc_authority.digest NOT NULL,
    capability_digest mc_authority.digest NOT NULL,
    trust_set_digest mc_authority.digest NOT NULL,
    manifest_digest mc_authority.digest NOT NULL,
    verification_set_digest mc_authority.digest NOT NULL,
    PRIMARY KEY (tenant_id, evaluation_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, evaluation_id),
    -- ASSUMPTION (A-1): the evaluation sequence is per exact subject (doc 18 "evaluation sequence").
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, subject_digest, evaluation_sequence),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
        REFERENCES mc_authority.expected_manifest (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
);

-- Immutable decision per evaluation identity (gate_writer, insert only). Currentness is a separate gate-module concern.
CREATE TABLE mc_authority.gate_decision (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    evaluation_id uuid NOT NULL,
    conclusion text NOT NULL CHECK (conclusion IN
        ('PASS', 'FAIL', 'BLOCKED', 'HUMAN_REVIEW_REQUIRED', 'PARTIALLY_AVAILABLE', 'STALE', 'REVOKED', 'INVALID')),
    proof_digest mc_authority.digest NOT NULL,
    gate_authority_id uuid NOT NULL,
    gate_generation mc_authority.u64 NOT NULL,
    issued_at bigint NOT NULL,
    expires_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, evaluation_id),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, evaluation_id)
        REFERENCES mc_authority.evaluation (tenant_id, repository_id, repository_tenant_epoch, evaluation_id)
);

-- evidence_ingest_writer, insert only; unique tenant/attempt/manifest/work-unit (doc 07, doc 18).
CREATE TABLE mc_authority.analysis_completion_receipt (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    receipt_id uuid NOT NULL,
    receipt_digest mc_authority.digest NOT NULL,
    manifest_digest mc_authority.digest NOT NULL,
    unit_id text NOT NULL CHECK (unit_id <> ''),
    attempt_binding_digest mc_authority.digest NOT NULL,
    producer_id uuid NOT NULL,
    outcome text NOT NULL CHECK (outcome IN ('COMPLETED', 'FAILED', 'UNSUPPORTED', 'CANCELLED')),
    finding_count bigint NOT NULL CHECK (finding_count BETWEEN 0 AND 4294967295),
    PRIMARY KEY (tenant_id, receipt_id),
    UNIQUE (tenant_id, attempt_binding_digest, manifest_digest, unit_id),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
        REFERENCES mc_authority.expected_manifest (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
);

-- evidence_ingest_writer, insert only.
CREATE TABLE mc_authority.evidence_receipt (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    receipt_id uuid NOT NULL,
    receipt_digest mc_authority.digest NOT NULL,
    attempt_binding_digest mc_authority.digest NOT NULL,
    manifest_digest mc_authority.digest NOT NULL,
    unit_id text NOT NULL CHECK (unit_id <> ''),
    producer_id uuid NOT NULL,
    producer_generation mc_authority.u64 NOT NULL,
    input_digest mc_authority.digest NOT NULL,
    environment_digest mc_authority.digest NOT NULL,
    output_digest mc_authority.digest NOT NULL,
    grade text NOT NULL CHECK (grade IN ('E0', 'E1', 'E2', 'E3', 'E4')),
    created_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, receipt_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, receipt_digest),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
        REFERENCES mc_authority.expected_manifest (tenant_id, repository_id, repository_tenant_epoch, manifest_digest)
);

-- verification_ingest_writer, insert only per verification assignment/attempt (doc 07).
CREATE TABLE mc_authority.verification_receipt (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    receipt_id uuid NOT NULL,
    receipt_digest mc_authority.digest NOT NULL,
    attempt_binding_digest mc_authority.digest NOT NULL,
    evidence_digest mc_authority.digest NOT NULL,
    verifier_id uuid NOT NULL,
    verifier_generation mc_authority.u64 NOT NULL,
    subject_digest mc_authority.digest NOT NULL,
    policy_digest mc_authority.digest NOT NULL,
    independence_class text NOT NULL CHECK (independence_class IN ('C0', 'C1', 'C2', 'C3', 'C4')),
    outcome text NOT NULL CHECK (outcome IN ('CONFIRMED', 'REJECTED', 'UNSUPPORTED', 'INCONCLUSIVE')),
    PRIMARY KEY (tenant_id, receipt_id),
    UNIQUE (tenant_id, attempt_binding_digest, verifier_id),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, evidence_digest)
        REFERENCES mc_authority.evidence_receipt (tenant_id, repository_id, repository_tenant_epoch, receipt_digest)
);

-- verification_set_writer, insert only; replacements have a new identity (doc 07).
CREATE TABLE mc_authority.verification_set (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    verification_set_id uuid NOT NULL,
    set_digest mc_authority.digest NOT NULL,
    attempt_binding_digest mc_authority.digest NOT NULL,
    subject_digest mc_authority.digest NOT NULL,
    policy_digest mc_authority.digest NOT NULL,
    trust_set_digest mc_authority.digest NOT NULL,
    issuer_id uuid NOT NULL,
    issued_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, verification_set_id),
    UNIQUE (tenant_id, set_digest)
);

-- policy_writer: immutable versions + a CAS pointer.
CREATE TABLE mc_authority.policy_version (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    policy_id uuid NOT NULL,
    generation mc_authority.u64 NOT NULL,
    policy_digest mc_authority.digest NOT NULL,
    previous_digest mc_authority.digest,
    created_at bigint NOT NULL,
    activated_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, policy_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, policy_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, generation)
);

CREATE TABLE mc_authority.policy_pointer (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    policy_id uuid NOT NULL,
    generation mc_authority.u64 NOT NULL,
    updated_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, repository_id, repository_tenant_epoch),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, policy_id)
        REFERENCES mc_authority.policy_version (tenant_id, repository_id, repository_tenant_epoch, policy_id)
);

-- trust_writer: immutable bundles + a monotonic pointer.
CREATE TABLE mc_authority.trust_bundle (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    trust_bundle_id uuid NOT NULL,
    generation mc_authority.u64 NOT NULL,
    bundle_digest mc_authority.digest NOT NULL,
    created_at bigint NOT NULL,
    expires_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, trust_bundle_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, trust_bundle_id),
    UNIQUE (tenant_id, repository_id, repository_tenant_epoch, generation)
);

CREATE TABLE mc_authority.trust_pointer (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    trust_bundle_id uuid NOT NULL,
    generation mc_authority.u64 NOT NULL,
    updated_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, repository_id, repository_tenant_epoch),
    FOREIGN KEY (tenant_id, repository_id, repository_tenant_epoch, trust_bundle_id)
        REFERENCES mc_authority.trust_bundle (tenant_id, repository_id, repository_tenant_epoch, trust_bundle_id)
);

-- revocation_writer: append-only; unique subject/generation; no delete or un-revoke (doc 07).
CREATE TABLE mc_authority.revocation (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    revocation_id uuid NOT NULL,
    subject_type text NOT NULL CHECK (subject_type <> ''),
    subject_digest mc_authority.digest NOT NULL,
    revoked_generation mc_authority.u64 NOT NULL,
    reason text NOT NULL CHECK (reason <> ''),
    effective_at bigint NOT NULL,
    authority_id uuid NOT NULL,
    PRIMARY KEY (tenant_id, revocation_id),
    UNIQUE (tenant_id, subject_digest, revoked_generation)
);

-- candidate_writer: CAS pointer on the authoritative provider generation.
-- ASSUMPTION (A-2): one pointer row per exact subject; a base/head/queue change yields a new subject_digest
-- (doc 07 "supersede identity"), hence a new row. Only the observation columns are updatable.
CREATE TABLE mc_authority.current_candidate (
    tenant_id uuid NOT NULL,
    repository_id mc_authority.u64 NOT NULL,
    repository_tenant_epoch mc_authority.u64 NOT NULL,
    authority_epoch uuid NOT NULL,
    subject_digest mc_authority.digest NOT NULL,
    provider_generation mc_authority.u64 NOT NULL,
    observation_digest mc_authority.digest NOT NULL,
    writer_id uuid NOT NULL,
    observed_at bigint NOT NULL,
    expires_at bigint NOT NULL,
    PRIMARY KEY (tenant_id, repository_id, repository_tenant_epoch, subject_digest)
);
