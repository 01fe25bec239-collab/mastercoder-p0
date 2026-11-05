// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
mod common;
use common::*;
use mc_authority_db::{DbError, TenantContext, VerifiedTenantAssertion, with_tenant};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use uuid::Uuid;

#[sqlx::test(migrations = false)]
async fn mismatched_assertion_rejected(pool: PgPool) {
    provisioned(&pool).await;
    let a = assertion(TENANT_A);
    let ctx = |x: &VerifiedTenantAssertion| TenantContext::from_verified(x);

    // Same assertion twice is fine; a different tenant in the same transaction is rejected.
    let mut tx = begin_as(&pool, "ordinary_api").await;
    with_tenant(&mut tx, &ctx(&a)).await.unwrap();
    with_tenant(&mut tx, &ctx(&a)).await.unwrap();
    assert_eq!(
        with_tenant(&mut tx, &ctx(&assertion(TENANT_B))).await,
        Err(DbError::TenantMismatch)
    );
    drop(tx);

    // Same tenant but a different workload, or a different digest, is also a disagreement.
    for other in [
        VerifiedTenantAssertion {
            workload_id: Uuid::from_u128(0xBAD),
            ..a
        },
        VerifiedTenantAssertion {
            assertion_digest: [0xEE; 32],
            ..a
        },
    ] {
        let mut tx = begin_as(&pool, "ordinary_api").await;
        with_tenant(&mut tx, &ctx(&a)).await.unwrap();
        assert_eq!(
            with_tenant(&mut tx, &ctx(&other)).await,
            Err(DbError::TenantMismatch)
        );
    }

    // An invalid assertion is a database error, never a tenant context.
    let mut tx = begin_as(&pool, "ordinary_api").await;
    let nil = VerifiedTenantAssertion {
        tenant_id: Uuid::nil(),
        ..a
    };
    assert_eq!(with_tenant(&mut tx, &ctx(&nil)).await, Err(DbError::Sqlx));
}

#[sqlx::test(migrations = false)]
async fn pooled_connection_does_not_leak_tenant(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) {
    // One physical connection, so every transaction below reuses the same backend.
    let pool = pool_opts
        .max_connections(1)
        .connect_with(conn_opts)
        .await
        .unwrap();
    provisioned(&pool).await;
    seed_tenant(&pool, TENANT_A, 1).await;
    seed_tenant(&pool, TENANT_B, 2).await;

    let mut tx = begin_tenant_as(&pool, "readonly", TENANT_A).await;
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let seen: Vec<Uuid> = sqlx::query_scalar("SELECT evaluation_id FROM mc_authority.evaluation")
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    assert_eq!(seen, vec![id(1)]);
    tx.commit().await.unwrap();

    // Next transaction on the same connection: no tenant, no rows (A is not carried over).
    let mut tx = begin_as(&pool, "readonly").await;
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT pg_backend_pid()")
            .fetch_one(&mut *tx)
            .await
            .unwrap(),
        pid
    );
    let t: Option<Uuid> = sqlx::query_scalar("SELECT mc_authority.current_tenant()")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(t, None);
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_authority.evaluation")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(n, 0);
    tx.rollback().await.unwrap();

    // A rolled-back context does not survive either.
    let tx = begin_tenant_as(&pool, "readonly", TENANT_B).await;
    tx.rollback().await.unwrap();
    let mut tx = begin_as(&pool, "readonly").await;
    let t: Option<Uuid> = sqlx::query_scalar("SELECT mc_authority.current_tenant()")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(t, None);
    tx.rollback().await.unwrap();

    // A different tenant can now be bound on the same connection without a spurious mismatch, and sees only B.
    let mut tx = begin_tenant_as(&pool, "readonly", TENANT_B).await;
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT pg_backend_pid()")
            .fetch_one(&mut *tx)
            .await
            .unwrap(),
        pid
    );
    let seen: Vec<Uuid> = sqlx::query_scalar("SELECT evaluation_id FROM mc_authority.evaluation")
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    assert_eq!(seen, vec![id(2)]);
}

#[sqlx::test(migrations = false)]
async fn definer_functions_have_fixed_search_path_and_narrow_execute(pool: PgPool) {
    migrated(&pool).await;
    let funcs: Vec<(String, bool, Vec<String>, Vec<String>)> = sqlx::query_as(
        "SELECT p.proname::text, p.prosecdef, coalesce(p.proconfig, '{}'), \
                coalesce((SELECT array_agg(coalesce(r.rolname::text, 'PUBLIC') ORDER BY coalesce(r.rolname::text, 'PUBLIC')) \
                          FROM aclexplode(p.proacl) a LEFT JOIN pg_roles r ON r.oid = a.grantee \
                          WHERE a.privilege_type = 'EXECUTE'), '{}') \
         FROM pg_proc p WHERE p.pronamespace = 'mc_authority'::regnamespace ORDER BY 1",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let names: Vec<&str> = funcs.iter().map(|f| f.0.as_str()).collect();
    assert_eq!(names, ["current_tenant", "set_tenant_context"]);
    let mut want: Vec<String> = RUNTIME_ROLES.iter().map(|r| r.to_string()).collect();
    want.push(OWNER.to_owned());
    want.sort();
    // EXECUTE reaching all runtime roles is NOT the trust boundary (r0 treated it as one, which was finding F-1):
    // the boundary is the setter's session_user check against the owner-only tenant_authorization state
    // (behaviour: tenant_authority.rs). Here we only pin that the setter really consults it.
    let setter: String = sqlx::query_scalar(
        "SELECT prosrc FROM pg_proc WHERE pronamespace = 'mc_authority'::regnamespace AND proname = 'set_tenant_context'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        setter.contains("tenant_authorization") && setter.contains("session_user"),
        "the setter must authorize the authenticated session"
    );
    for (name, definer, config, execute) in funcs {
        assert!(definer, "{name} must be SECURITY DEFINER");
        assert!(
            config.contains(&"search_path=pg_catalog, mc_authority, pg_temp".to_owned()),
            "{name}: {config:?}"
        );
        assert_eq!(
            execute, want,
            "{name}: EXECUTE must be exactly the runtime roles (never PUBLIC)"
        );
    }
}

#[sqlx::test(migrations = false)]
async fn runtime_role_cannot_write_tenant_context_directly(pool: PgPool) {
    migrated(&pool).await;
    for role in RUNTIME_ROLES {
        let mut tx = begin_as(&pool, role).await;
        let err = sqlx::query(
            "INSERT INTO mc_authority.tenant_context (backend_pid, xact_id, tenant_id, workload_id, assertion_digest) \
             VALUES (pg_backend_pid(), pg_current_xact_id(), $1, $1, $2)",
        )
        .bind(TENANT_A)
        .bind(digest(1))
        .execute(&mut *tx)
        .await
        .expect_err("direct tenant_context write must fail");
        assert_eq!(sqlstate(&err), "42501", "{role}: {err}");
    }
}
