// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
mod common;
use common::*;
use sqlx::{PgPool, Row};

fn names() -> Vec<String> {
    RUNTIME_ROLES.iter().map(|r| r.to_string()).collect()
}

#[sqlx::test(migrations = false)]
async fn all_22_roles_exist_nologin_noinherit_nobypassrls(pool: PgPool) {
    migrated(&pool).await;
    assert_eq!(RUNTIME_ROLES.len(), 22);
    let mut all = names();
    all.push(OWNER.to_owned());
    let rows = sqlx::query(
        "SELECT rolname, rolcanlogin, rolinherit, rolbypassrls, rolsuper, rolcreatedb, rolcreaterole, \
         rolreplication FROM pg_roles WHERE rolname = ANY($1) ORDER BY rolname",
    )
    .bind(&all)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows.len(),
        23,
        "22 runtime roles + the migration owner must exist"
    );
    for r in rows {
        let n: String = r.get("rolname");
        for attr in [
            "rolcanlogin",
            "rolinherit",
            "rolbypassrls",
            "rolsuper",
            "rolcreatedb",
            "rolcreaterole",
            "rolreplication",
        ] {
            assert!(!r.get::<bool, _>(attr), "{n} must have {attr} = false");
        }
    }
}

#[sqlx::test(migrations = false)]
async fn no_runtime_role_owns_tables(pool: PgPool) {
    migrated(&pool).await;
    for (what, sql) in [
        (
            "relations",
            "SELECT count(*) FROM pg_class c JOIN pg_roles r ON r.oid = c.relowner WHERE r.rolname = ANY($1)",
        ),
        (
            "functions",
            "SELECT count(*) FROM pg_proc p JOIN pg_roles r ON r.oid = p.proowner WHERE r.rolname = ANY($1)",
        ),
        (
            "schemas",
            "SELECT count(*) FROM pg_namespace n JOIN pg_roles r ON r.oid = n.nspowner WHERE r.rolname = ANY($1)",
        ),
        (
            "types",
            "SELECT count(*) FROM pg_type t JOIN pg_roles r ON r.oid = t.typowner WHERE r.rolname = ANY($1)",
        ),
    ] {
        let n: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(sql))
            .bind(names())
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(n, 0, "a runtime role owns {what}");
    }
    // Positive control: the offline owner owns every mc_authority table, function and the schema.
    let not_owner: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class c JOIN pg_roles r ON r.oid = c.relowner \
         WHERE c.relnamespace = 'mc_authority'::regnamespace AND c.relkind = 'r' AND r.rolname <> $1",
    )
    .bind(OWNER)
    .fetch_one(&pool)
    .await
    .unwrap();
    let tables: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class WHERE relnamespace = 'mc_authority'::regnamespace AND relkind = 'r'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(tables >= 14);
    assert_eq!(not_owner, 0);
    let foreign: i64 = sqlx::query_scalar(
        "SELECT (SELECT count(*) FROM pg_proc p JOIN pg_roles r ON r.oid = p.proowner \
                  WHERE p.pronamespace = 'mc_authority'::regnamespace AND r.rolname <> $1) \
              + (SELECT count(*) FROM pg_namespace n JOIN pg_roles r ON r.oid = n.nspowner \
                  WHERE n.nspname = 'mc_authority' AND r.rolname <> $1)",
    )
    .bind(OWNER)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(foreign, 0);
}

#[sqlx::test(migrations = false)]
async fn set_role_between_runtime_roles_fails(pool: PgPool) {
    migrated(&pool).await;
    // No membership exists between these roles (in either direction), so no SET ROLE path can exist.
    let mut all = names();
    all.push(OWNER.to_owned());
    let members: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_auth_members m JOIN pg_roles a ON a.oid = m.roleid \
         JOIN pg_roles b ON b.oid = m.member WHERE a.rolname = ANY($1) AND b.rolname = ANY($1)",
    )
    .bind(&all)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(members, 0);

    for from in RUNTIME_ROLES {
        // Positive control: SET ROLE to itself works, so the harness really acts as `from`.
        let mut tx = begin_as(&pool, from).await;
        sqlx::query(sqlx::AssertSqlSafe(format!("SET ROLE {from}")))
            .execute(&mut *tx)
            .await
            .unwrap();
        for to in RUNTIME_ROLES.iter().filter(|t| **t != from) {
            let mut tx = begin_as(&pool, from).await;
            let err = sqlx::query(sqlx::AssertSqlSafe(format!("SET ROLE {to}")))
                .execute(&mut *tx)
                .await
                .expect_err(&format!("SET ROLE {from} -> {to} must fail"));
            assert_eq!(sqlstate(&err), "42501", "{from} -> {to}: {err}");
        }
        // ... and into the migration owner.
        let mut tx = begin_as(&pool, from).await;
        let err = sqlx::query(sqlx::AssertSqlSafe(format!("SET ROLE {OWNER}")))
            .execute(&mut *tx)
            .await
            .expect_err("SET ROLE to the owner must fail");
        assert_eq!(sqlstate(&err), "42501", "{from} -> owner: {err}");
    }
}
