// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
mod common;
use common::*;
use sqlx::PgPool;
use std::collections::BTreeSet;

/// Doc 07 matrix as the test's own oracle: (table, role, privileges). Anything not listed must be denied.
/// Every table is insert-only except the three pointers; nobody has DELETE/TRUNCATE/REFERENCES/TRIGGER.
const MATRIX: &[(&str, &str, &[&str])] = &[
    ("evaluation", "gate_writer", &["INSERT", "SELECT"]),
    ("evaluation", "readonly", &["SELECT"]),
    ("gate_decision", "gate_writer", &["INSERT", "SELECT"]),
    ("gate_decision", "readonly", &["SELECT"]),
    (
        "analysis_completion_receipt",
        "evidence_ingest_writer",
        &["INSERT"],
    ),
    ("analysis_completion_receipt", "gate_writer", &["SELECT"]),
    ("analysis_completion_receipt", "readonly", &["SELECT"]),
    ("evidence_receipt", "evidence_ingest_writer", &["INSERT"]),
    ("evidence_receipt", "gate_writer", &["SELECT"]),
    ("evidence_receipt", "readonly", &["SELECT"]),
    (
        "verification_receipt",
        "verification_ingest_writer",
        &["INSERT"],
    ),
    ("verification_receipt", "gate_writer", &["SELECT"]),
    ("verification_receipt", "readonly", &["SELECT"]),
    ("verification_set", "verification_set_writer", &["INSERT"]),
    ("verification_set", "gate_writer", &["SELECT"]),
    ("verification_set", "readonly", &["SELECT"]),
    ("expected_manifest", "manifest_writer", &["INSERT"]),
    ("expected_manifest", "gate_writer", &["SELECT"]),
    ("expected_manifest", "readonly", &["SELECT"]),
    ("revocation", "revocation_writer", &["INSERT"]),
    ("revocation", "gate_writer", &["SELECT"]),
    ("revocation", "readonly", &["SELECT"]),
    ("policy_version", "policy_writer", &["INSERT"]),
    ("policy_version", "gate_writer", &["SELECT"]),
    ("policy_version", "readonly", &["SELECT"]),
    (
        "policy_pointer",
        "policy_writer",
        &["INSERT", "SELECT", "UPDATE"],
    ),
    ("policy_pointer", "gate_writer", &["SELECT"]),
    ("policy_pointer", "readonly", &["SELECT"]),
    ("trust_bundle", "trust_writer", &["INSERT"]),
    ("trust_bundle", "gate_writer", &["SELECT"]),
    ("trust_bundle", "readonly", &["SELECT"]),
    (
        "trust_pointer",
        "trust_writer",
        &["INSERT", "SELECT", "UPDATE"],
    ),
    ("trust_pointer", "gate_writer", &["SELECT"]),
    ("trust_pointer", "readonly", &["SELECT"]),
    (
        "current_candidate",
        "candidate_writer",
        &["INSERT", "SELECT", "UPDATE"],
    ),
    ("current_candidate", "gate_writer", &["SELECT"]),
    ("current_candidate", "readonly", &["SELECT"]),
];

const OPS: [&str; 7] = [
    "SELECT",
    "INSERT",
    "UPDATE",
    "DELETE",
    "TRUNCATE",
    "REFERENCES",
    "TRIGGER",
];

fn allowed(table: &str, role: &str, op: &str) -> bool {
    MATRIX
        .iter()
        .any(|(t, r, ops)| *t == table && *r == role && ops.contains(&op))
}

async fn tables(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT relname::text FROM pg_class WHERE relnamespace = 'mc_authority'::regnamespace \
         AND relkind = 'r' ORDER BY 1",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrations = false)]
async fn matrix_matches_doc07(pool: PgPool) {
    migrated(&pool).await;

    // The oracle must cover exactly the base tables (a new table forces a matrix decision).
    let found: BTreeSet<String> = tables(&pool).await.into_iter().collect();
    let mut expected: BTreeSet<String> = MATRIX.iter().map(|(t, _, _)| t.to_string()).collect();
    for t in ["tenant_context", "evaluation", "gate_decision"] {
        expected.insert(t.to_owned());
    }
    assert_eq!(found, expected);

    // role x table x op, computed by the database.
    for role in RUNTIME_ROLES {
        for table in &found {
            for op in OPS {
                let sql = if op == "UPDATE" {
                    "SELECT has_any_column_privilege($1::name, $2, 'UPDATE')"
                } else {
                    "SELECT has_table_privilege($1::name, $2, $3)"
                };
                let q = sqlx::query_scalar::<_, bool>(sqlx::AssertSqlSafe(sql))
                    .bind(role)
                    .bind(format!("mc_authority.{table}"));
                let got = if op == "UPDATE" { q } else { q.bind(op) }
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                assert_eq!(got, allowed(table, role, op), "{role} {op} {table}");
            }
        }
    }

    // Nothing for PUBLIC on any relation or function; the identity columns are never updatable.
    let public: i64 = sqlx::query_scalar(
        "SELECT (SELECT count(*) FROM pg_class c, aclexplode(c.relacl) a \
                  WHERE c.relnamespace = 'mc_authority'::regnamespace AND a.grantee = 0) \
              + (SELECT count(*) FROM pg_attribute t JOIN pg_class c ON c.oid = t.attrelid, aclexplode(t.attacl) a \
                  WHERE c.relnamespace = 'mc_authority'::regnamespace AND a.grantee = 0)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(public, 0);
    for role in RUNTIME_ROLES {
        for table in &found {
            for col in [
                "tenant_id",
                "repository_id",
                "repository_tenant_epoch",
                "authority_epoch",
            ] {
                let has_col: bool = sqlx::query_scalar(
                    "SELECT EXISTS (SELECT 1 FROM pg_attribute WHERE attrelid = $1::regclass AND attname = $2)",
                )
                .bind(format!("mc_authority.{table}"))
                .bind(col)
                .fetch_one(&pool)
                .await
                .unwrap();
                if has_col {
                    let upd: bool = sqlx::query_scalar(
                        "SELECT has_column_privilege($1::name, $2, $3, 'UPDATE')",
                    )
                    .bind(role)
                    .bind(format!("mc_authority.{table}"))
                    .bind(col)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                    assert!(!upd, "{role} must not UPDATE {table}.{col}");
                }
            }
        }
    }

    // Real statements: every role without INSERT is denied at the database (permission denied, 42501).
    for role in RUNTIME_ROLES {
        for table in &found {
            if allowed(table, role, "INSERT") {
                continue;
            }
            let mut tx = begin_as(&pool, role).await;
            let err = sqlx::query(sqlx::AssertSqlSafe(format!(
                "INSERT INTO mc_authority.{table} DEFAULT VALUES"
            )))
            .execute(&mut *tx)
            .await
            .expect_err(&format!("{role} INSERT {table} must be denied"));
            assert_eq!(sqlstate(&err), "42501", "{role} {table}: {err}");
            assert!(
                message(&err).contains(&format!("permission denied for table {table}")),
                "{err}"
            );
        }
    }
}

#[sqlx::test(migrations = false)]
async fn ordinary_api_cannot_insert_gate_decision(pool: PgPool) {
    migrated(&pool).await;
    seed_tenant(&pool, TENANT_A, 1).await;

    // ordinary_api, fully authenticated for the tenant, is denied by the database itself.
    let mut tx = begin_tenant_as(&pool, "ordinary_api", TENANT_A).await;
    let err = insert_decision(&mut tx, TENANT_A, 1)
        .await
        .expect_err("must be denied");
    assert_eq!(sqlstate(&err), "42501");
    assert!(
        message(&err).contains("permission denied for table gate_decision"),
        "{err}"
    );
    drop(tx);

    // Positive control: the identical statement succeeds for gate_writer, so privilege is what blocked it.
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_A).await;
    insert_decision(&mut tx, TENANT_A, 1)
        .await
        .expect("gate_writer inserts the decision");
    tx.commit().await.unwrap();
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_authority.gate_decision")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows, 1);
}

#[sqlx::test(migrations = false)]
async fn insert_only_tables_reject_update_and_delete(pool: PgPool) {
    migrated(&pool).await;
    for (table, role, ops) in MATRIX {
        if !ops.contains(&"INSERT") || ops.contains(&"UPDATE") {
            continue;
        }
        for stmt in [
            format!("UPDATE mc_authority.{table} SET tenant_id = tenant_id"),
            format!("DELETE FROM mc_authority.{table}"),
        ] {
            let mut tx = begin_tenant_as(&pool, role, TENANT_A).await;
            let err = sqlx::query(sqlx::AssertSqlSafe(stmt.clone()))
                .execute(&mut *tx)
                .await
                .expect_err(&format!("{role}: {stmt} must be denied"));
            assert_eq!(sqlstate(&err), "42501", "{role}: {stmt}: {err}");
        }
    }
}
