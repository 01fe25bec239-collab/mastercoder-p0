// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
// Shared helpers for the mc-authority-db integration tests. Each test runs on a fresh database
// (`#[sqlx::test]`); the connecting user must be a superuser (it applies the offline migrations and
// uses SET SESSION AUTHORIZATION to act as a runtime role). Synthetic data only.
#![allow(dead_code)]

use std::path::PathBuf;

use mc_authority_db::{TenantContext, VerifiedTenantAssertion, migrate, with_tenant};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub const OWNER: &str = "mc_authority_owner";

/// The 22 runtime roles of doc 07, typed here independently of the SQL under test.
pub const RUNTIME_ROLES: [&str; 22] = [
    "ordinary_api",
    "runner_control",
    "evidence_ingest_writer",
    "verification_ingest_writer",
    "verification_set_writer",
    "gate_writer",
    "policy_writer",
    "candidate_writer",
    "projection_writer",
    "trust_writer",
    "revocation_writer",
    "manifest_writer",
    "eligibility_writer",
    "approval_writer",
    "custody_writer",
    "plan_writer",
    "publication_writer",
    "effect_writer",
    "budget_writer",
    "dependency_writer",
    "resolution_writer",
    "readonly",
];

pub const TENANT_A: Uuid = Uuid::from_u128(0xA);
pub const TENANT_B: Uuid = Uuid::from_u128(0xB);

pub fn modules_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../db/authority/MODULES")
}

pub async fn migrated(pool: &PgPool) {
    migrate(pool, &modules_file()).await.expect("migrate");
}

/// Trusted provisioning, run on the privileged bootstrap connection BEFORE any untrusted session runs: binds the
/// authenticated session `principal` to exactly this assertion (what the operator/verifier does; see roles/README.md).
/// It grants only what the caller names explicitly; nothing here reacts to an assertion an attack later supplies.
pub async fn authorize(pool: &PgPool, principal: &str, a: &VerifiedTenantAssertion) {
    let mut tx = pool.begin().await.expect("begin");
    sqlx::query(sqlx::AssertSqlSafe(format!("SET LOCAL ROLE {OWNER}")))
        .execute(&mut *tx)
        .await
        .expect("set role owner");
    sqlx::query(
        "INSERT INTO mc_authority.tenant_authorization (principal, tenant_id, workload_id, assertion_digest) \
         VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
    )
    .bind(principal)
    .bind(a.tenant_id)
    .bind(a.workload_id)
    .bind(&a.assertion_digest[..])
    .execute(&mut *tx)
    .await
    .expect("authorize");
    tx.commit().await.expect("commit");
}

/// Migrates, then authorizes every runtime role (as itself, i.e. `SET SESSION AUTHORIZATION role`) for the two
/// canonical tenants A and B with their canonical `assertion()`. For the pre-existing RLS/grant tests only: any
/// other tuple (invented tenant/workload/digest) stays unauthorized.
pub async fn provisioned(pool: &PgPool) {
    migrated(pool).await;
    for role in RUNTIME_ROLES {
        for tenant in [TENANT_A, TENANT_B] {
            authorize(pool, role, &assertion(tenant)).await;
        }
    }
}

pub fn assertion(tenant_id: Uuid) -> VerifiedTenantAssertion {
    VerifiedTenantAssertion {
        tenant_id,
        workload_id: Uuid::from_u128(0x100 + tenant_id.as_u128()),
        assertion_digest: [tenant_id.as_bytes()[15]; 32],
    }
}

/// A transaction acting as `role` (SET LOCAL SESSION AUTHORIZATION: session_user becomes the role, so
/// SET ROLE / RLS / privilege checks behave exactly as for a real login of that role).
pub async fn begin_as(pool: &PgPool, role: &str) -> Transaction<'static, Postgres> {
    assert!(role.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'));
    let mut tx = pool.begin().await.expect("begin");
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "SET LOCAL SESSION AUTHORIZATION {role}"
    )))
    .execute(&mut *tx)
    .await
    .expect("set session authorization");
    tx
}

/// `begin_as` plus an authenticated tenant context.
pub async fn begin_tenant_as(
    pool: &PgPool,
    role: &str,
    tenant: Uuid,
) -> Transaction<'static, Postgres> {
    let mut tx = begin_as(pool, role).await;
    with_tenant(&mut tx, &TenantContext::from_verified(&assertion(tenant)))
        .await
        .expect("with_tenant");
    tx
}

pub fn sqlstate(e: &sqlx::Error) -> String {
    match e {
        sqlx::Error::Database(d) => d.code().map(|c| c.into_owned()).unwrap_or_default(),
        other => panic!("expected a database error, got {other}"),
    }
}

pub fn message(e: &sqlx::Error) -> String {
    match e {
        sqlx::Error::Database(d) => d.message().to_owned(),
        other => panic!("expected a database error, got {other}"),
    }
}

pub fn authority_epoch() -> Uuid {
    Uuid::from_u128(0xE0)
}

pub const INSERT_MANIFEST: &str = "INSERT INTO mc_authority.expected_manifest \
    (tenant_id, repository_id, repository_tenant_epoch, authority_epoch, manifest_id, manifest_digest, \
     attempt_binding_digest, candidate_tree, policy_digest, analysis_plan_digest, issuer_id, issued_at) \
    VALUES ($1, 1, 1, $2, $3, $4, $4, $5, $4, $4, $2, 0)";

pub const INSERT_EVALUATION: &str = "INSERT INTO mc_authority.evaluation \
    (tenant_id, repository_id, repository_tenant_epoch, authority_epoch, evaluation_id, evaluation_sequence, \
     subject_digest, policy_digest, capability_digest, trust_set_digest, manifest_digest, verification_set_digest) \
    VALUES ($1, 1, 1, $2, $3, 1, $4, $4, $4, $4, $5, $4)";

pub const INSERT_DECISION: &str = "INSERT INTO mc_authority.gate_decision \
    (tenant_id, repository_id, repository_tenant_epoch, authority_epoch, evaluation_id, conclusion, \
     proof_digest, gate_authority_id, gate_generation, issued_at, expires_at) \
    VALUES ($1, 1, 1, $2, $3, 'PASS', $4, $2, 1, 0, 1)";

pub fn id(n: u8) -> Uuid {
    Uuid::from_u128(n as u128)
}

pub fn digest(n: u8) -> Vec<u8> {
    vec![n; 32]
}

/// Inserts an expected manifest `n` for `tenant` through manifest_writer (the doc 07 path).
pub async fn insert_manifest(
    tx: &mut Transaction<'static, Postgres>,
    tenant: Uuid,
    n: u8,
) -> Result<(), sqlx::Error> {
    sqlx::query(INSERT_MANIFEST)
        .bind(tenant)
        .bind(authority_epoch())
        .bind(id(n))
        .bind(digest(n))
        .bind(vec![n; 20])
        .execute(&mut **tx)
        .await
        .map(|_| ())
}

/// Inserts evaluation `n` (pinning manifest `m`) for `tenant` through gate_writer.
pub async fn insert_evaluation(
    tx: &mut Transaction<'static, Postgres>,
    tenant: Uuid,
    n: u8,
    m: u8,
) -> Result<(), sqlx::Error> {
    sqlx::query(INSERT_EVALUATION)
        .bind(tenant)
        .bind(authority_epoch())
        .bind(id(n))
        .bind(digest(n))
        .bind(digest(m))
        .execute(&mut **tx)
        .await
        .map(|_| ())
}

pub async fn insert_decision(
    tx: &mut Transaction<'static, Postgres>,
    tenant: Uuid,
    n: u8,
) -> Result<(), sqlx::Error> {
    sqlx::query(INSERT_DECISION)
        .bind(tenant)
        .bind(authority_epoch())
        .bind(id(n))
        .bind(digest(n))
        .execute(&mut **tx)
        .await
        .map(|_| ())
}

/// Seeds manifest `n` and evaluation `n` for `tenant` through the proper writers, each committed.
pub async fn seed_tenant(pool: &PgPool, tenant: Uuid, n: u8) {
    let mut tx = begin_tenant_as(pool, "manifest_writer", tenant).await;
    insert_manifest(&mut tx, tenant, n).await.expect("manifest");
    tx.commit().await.expect("commit");
    let mut tx = begin_tenant_as(pool, "gate_writer", tenant).await;
    insert_evaluation(&mut tx, tenant, n, n)
        .await
        .expect("evaluation");
    tx.commit().await.expect("commit");
}
