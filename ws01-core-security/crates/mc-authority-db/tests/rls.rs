// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
mod common;
use common::*;
use sqlx::PgPool;
use uuid::Uuid;

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
async fn force_rls_on_every_base_table(pool: PgPool) {
    migrated(&pool).await;
    let t = tables(&pool).await;
    assert!(t.len() >= 14, "{t:?}");
    for table in t {
        let (on, forced, policies): (bool, bool, i64) = sqlx::query_as(
            "SELECT c.relrowsecurity, c.relforcerowsecurity, \
                    (SELECT count(*) FROM pg_policy p WHERE p.polrelid = c.oid) \
             FROM pg_class c WHERE c.oid = $1::regclass",
        )
        .bind(format!("mc_authority.{table}"))
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(on && forced, "{table}: RLS enabled={on} forced={forced}");
        assert!(policies >= 1, "{table}: FORCE RLS without a policy");
    }
}

#[sqlx::test(migrations = false)]
async fn runtime_role_cannot_disable_rls(pool: PgPool) {
    provisioned(&pool).await;
    let all = tables(&pool).await;
    for role in RUNTIME_ROLES {
        for table in &all {
            let policy: String = sqlx::query_scalar(
                "SELECT polname::text FROM pg_policy WHERE polrelid = $1::regclass ORDER BY 1 LIMIT 1",
            )
            .bind(format!("mc_authority.{table}"))
            .fetch_one(&pool)
            .await
            .unwrap();
            for stmt in [
                format!("ALTER TABLE mc_authority.{table} DISABLE ROW LEVEL SECURITY"),
                format!("ALTER TABLE mc_authority.{table} NO FORCE ROW LEVEL SECURITY"),
                format!("DROP POLICY {policy} ON mc_authority.{table}"),
            ] {
                let mut tx = begin_tenant_as(&pool, role, TENANT_A).await;
                let err = sqlx::query(sqlx::AssertSqlSafe(stmt.clone()))
                    .execute(&mut *tx)
                    .await
                    .expect_err(&format!("{role}: {stmt} must fail"));
                assert_eq!(sqlstate(&err), "42501", "{role}: {stmt}: {err}");
            }
        }
        // `SET row_security = off` cannot be used to look past the policy either.
        let mut tx = begin_tenant_as(&pool, role, TENANT_A).await;
        if role == "readonly" {
            sqlx::query("SET LOCAL row_security = off")
                .execute(&mut *tx)
                .await
                .unwrap();
            let err = sqlx::query("SELECT count(*) FROM mc_authority.evaluation")
                .fetch_one(&mut *tx)
                .await
                .expect_err("row_security=off must not bypass the policy");
            assert_eq!(sqlstate(&err), "42501", "{err}");
        }
    }
    // Still enforced afterwards.
    let still: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class WHERE relnamespace = 'mc_authority'::regnamespace AND relkind = 'r' \
         AND NOT (relrowsecurity AND relforcerowsecurity)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(still, 0);
}

#[sqlx::test(migrations = false)]
async fn cross_tenant_select_returns_zero(pool: PgPool) {
    provisioned(&pool).await;
    seed_tenant(&pool, TENANT_A, 1).await;
    seed_tenant(&pool, TENANT_B, 2).await;

    for (tenant, own, other) in [(TENANT_A, 1u8, 2u8), (TENANT_B, 2, 1)] {
        let mut tx = begin_tenant_as(&pool, "readonly", tenant).await;
        for table in ["evaluation", "expected_manifest"] {
            let n: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
                "SELECT count(*) FROM mc_authority.{table}"
            )))
            .fetch_one(&mut *tx)
            .await
            .unwrap();
            assert_eq!(n, 1, "{tenant}: sees exactly its own {table} row");
        }
        let own_rows: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM mc_authority.evaluation WHERE evaluation_id = $1",
        )
        .bind(id(own))
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        let other_rows: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM mc_authority.evaluation WHERE evaluation_id = $1",
        )
        .bind(id(other))
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        assert_eq!((own_rows, other_rows), (1, 0));
    }

    // No authenticated context: every table is empty for every readable role.
    for table in tables(&pool).await {
        let mut tx = begin_as(&pool, "readonly").await;
        if table == "tenant_context" || table == "tenant_authorization" {
            continue; // not readable by runtime roles at all (see grants::matrix_matches_doc07)
        }
        let n: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT count(*) FROM mc_authority.{table}"
        )))
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        assert_eq!(
            n, 0,
            "{table} must be empty without an authenticated tenant"
        );
    }

    // A caller-set GUC is not authentication: forging one grants nothing.
    for forge in [
        format!("SELECT set_config('mc_authority.tenant_id', '{TENANT_A}', true)"),
        format!("SELECT set_config('mc_authority.current_tenant', '{TENANT_A}', true)"),
        format!("SELECT set_config('app.tenant_id', '{TENANT_A}', true)"),
    ] {
        let mut tx = begin_as(&pool, "readonly").await;
        sqlx::query(sqlx::AssertSqlSafe(forge))
            .execute(&mut *tx)
            .await
            .unwrap();
        let n: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_authority.evaluation")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(n, 0);
        let t: Option<Uuid> = sqlx::query_scalar("SELECT mc_authority.current_tenant()")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(t, None);
    }
}

#[sqlx::test(migrations = false)]
async fn cross_tenant_fk_rejected(pool: PgPool) {
    provisioned(&pool).await;
    seed_tenant(&pool, TENANT_A, 1).await; // manifest 1 + evaluation 1 belong to tenant A

    // Tenant B pins tenant A's manifest digest: the composite (tenant_id, ..., manifest_digest) FK rejects it,
    // even though the referenced row exists (FK checks are not subject to RLS).
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_B).await;
    let err = insert_evaluation(&mut tx, TENANT_B, 9, 1)
        .await
        .expect_err("cross-tenant FK");
    assert_eq!(sqlstate(&err), "23503", "{err}");
    drop(tx);

    // Same for a decision on tenant A's evaluation id.
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_B).await;
    let err = insert_decision(&mut tx, TENANT_B, 1)
        .await
        .expect_err("cross-tenant FK");
    assert_eq!(sqlstate(&err), "23503", "{err}");
    drop(tx);

    // Writing a row that names another tenant is rejected by RLS WITH CHECK.
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_B).await;
    let err = insert_evaluation(&mut tx, TENANT_A, 9, 1)
        .await
        .expect_err("foreign tenant_id");
    assert_eq!(sqlstate(&err), "42501", "{err}");
    assert!(message(&err).contains("row-level security"), "{err}");
    drop(tx);

    // Positive controls: the same statements are valid inside tenant A, so the FK/RLS is what rejected them.
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_A).await;
    insert_evaluation(&mut tx, TENANT_A, 9, 1)
        .await
        .expect("same-tenant evaluation");
    insert_decision(&mut tx, TENANT_A, 1)
        .await
        .expect("same-tenant decision");
    tx.commit().await.unwrap();
}
