// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
// W01-T007 r1, A3 finding F-1: caller-invented tenant/workload/digest fields must not grant tenant access.
// Trusted state (`authorize`) is provisioned explicitly on the privileged bootstrap connection BEFORE any untrusted
// session runs; nothing here authorizes whatever an attack under test supplies.
mod common;
use common::*;
use mc_authority_db::{DbError, TenantContext, VerifiedTenantAssertion, with_tenant};
use sqlx::{
    PgPool, Postgres, Transaction,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use uuid::Uuid;

/// Invented by the attacker: victim tenant B, made-up workload and an arbitrary digest (A3's reproduction).
const FORGED: VerifiedTenantAssertion = VerifiedTenantAssertion {
    tenant_id: TENANT_B,
    workload_id: Uuid::from_u128(99),
    assertion_digest: [0xff; 32],
};

fn ta() -> VerifiedTenantAssertion {
    assertion(TENANT_A)
}
fn tb() -> VerifiedTenantAssertion {
    assertion(TENANT_B)
}

/// Every tuple an untrusted session might try: invented, wrong workload, wrong digest, and the canonical tuples.
fn attempts() -> Vec<VerifiedTenantAssertion> {
    vec![
        FORGED,
        VerifiedTenantAssertion {
            tenant_id: TENANT_A,
            ..FORGED
        },
        VerifiedTenantAssertion {
            workload_id: Uuid::from_u128(99),
            ..tb()
        },
        VerifiedTenantAssertion {
            assertion_digest: [0xff; 32],
            ..tb()
        },
        VerifiedTenantAssertion {
            workload_id: Uuid::from_u128(99),
            ..ta()
        },
        VerifiedTenantAssertion {
            assertion_digest: [0xff; 32],
            ..ta()
        },
        ta(),
        tb(),
    ]
}

/// `with_tenant` inside a savepoint, so the transaction stays usable to prove what a failure left behind.
async fn attempt(
    tx: &mut Transaction<'static, Postgres>,
    a: &VerifiedTenantAssertion,
) -> Result<(), DbError> {
    sqlx::query("SAVEPOINT attempt")
        .execute(&mut **tx)
        .await
        .unwrap();
    let r = with_tenant(tx, &TenantContext::from_verified(a)).await;
    if r.is_err() {
        sqlx::query("ROLLBACK TO SAVEPOINT attempt")
            .execute(&mut **tx)
            .await
            .unwrap();
    }
    r
}

/// The same call as literal SQL (no Rust API in the path); returns the SQLSTATE of the rejection.
async fn raw_set_context(
    tx: &mut Transaction<'static, Postgres>,
    a: &VerifiedTenantAssertion,
) -> Result<(), sqlx::Error> {
    let hex: String = a
        .assertion_digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    sqlx::query("SAVEPOINT raw").execute(&mut **tx).await?;
    let r = sqlx::query(sqlx::AssertSqlSafe(format!(
        "SELECT mc_authority.set_tenant_context('{}', '{}', '\\x{hex}')",
        a.tenant_id, a.workload_id
    )))
    .execute(&mut **tx)
    .await
    .map(|_| ());
    if r.is_err() {
        sqlx::query("ROLLBACK TO SAVEPOINT raw")
            .execute(&mut **tx)
            .await?;
    }
    r
}

async fn no_context(tx: &mut Transaction<'static, Postgres>) {
    let t: Option<Uuid> = sqlx::query_scalar("SELECT mc_authority.current_tenant()")
        .fetch_one(&mut **tx)
        .await
        .unwrap();
    assert_eq!(t, None, "a failed authorization must leave no context");
}

async fn manifests(tx: &mut Transaction<'static, Postgres>) -> Vec<Uuid> {
    sqlx::query_scalar("SELECT manifest_id FROM mc_authority.expected_manifest ORDER BY 1")
        .fetch_all(&mut **tx)
        .await
        .unwrap()
}

/// Trusted fixture: manifest_writer is authorized for both tenants (it seeds one manifest each);
/// readonly only for A (ta); gate_writer only for B (tb). Nothing else is authorized.
async fn fixture(pool: &PgPool) {
    migrated(pool).await;
    authorize(pool, "manifest_writer", &ta()).await;
    authorize(pool, "manifest_writer", &tb()).await;
    authorize(pool, "readonly", &ta()).await;
    authorize(pool, "gate_writer", &tb()).await;
    for (t, n) in [(TENANT_A, 1), (TENANT_B, 2)] {
        let mut tx = begin_tenant_as(pool, "manifest_writer", t).await;
        insert_manifest(&mut tx, t, n).await.expect("seed");
        tx.commit().await.unwrap();
    }
}

fn is_authorized(role: &str, a: &VerifiedTenantAssertion) -> bool {
    match role {
        "manifest_writer" => *a == ta() || *a == tb(),
        "readonly" => *a == ta(),
        "gate_writer" => *a == tb(),
        _ => false,
    }
}

#[sqlx::test(migrations = false)]
async fn forged_assertions_cannot_establish_tenant_authority(pool: PgPool) {
    fixture(&pool).await;

    // Genuine positive controls through the public API: A and (separately) B are each readable by exactly
    // their authorized principal, and only for their own tenant.
    let mut tx = begin_tenant_as(&pool, "readonly", TENANT_A).await;
    assert_eq!(manifests(&mut tx).await, vec![id(1)]);
    drop(tx);
    let mut tx = begin_tenant_as(&pool, "gate_writer", TENANT_B).await;
    assert_eq!(manifests(&mut tx).await, vec![id(2)]);
    drop(tx);

    // Every runtime identity x every invented/copied tuple that is not provisioned for that identity is denied,
    // through the Rust API and as direct SQL, and leaves no usable context behind.
    let mut denied = 0;
    for role in RUNTIME_ROLES {
        for a in attempts() {
            if is_authorized(role, &a) {
                continue;
            }
            let mut tx = begin_as(&pool, role).await;
            assert_eq!(
                attempt(&mut tx, &a).await,
                Err(DbError::Sqlx),
                "{role} {a:?}"
            );
            let e = raw_set_context(&mut tx, &a).await.expect_err("raw forgery");
            assert_eq!(sqlstate(&e), "42501", "{role} {a:?}: {e}");
            assert!(message(&e).contains("authorization denied"), "{e}");
            no_context(&mut tx).await;
            if role == "readonly" {
                // The victim read: 0 rows before and after the forged assertion (A3's control showed 1).
                assert_eq!(manifests(&mut tx).await, Vec::<Uuid>::new(), "{a:?}");
            }
            denied += 1;
        }
    }
    assert!(denied > 150, "attack matrix too small: {denied}");

    // A principal authorized for tenant A (readonly) cannot read tenant B with a made-up assertion,
    // nor by copying another principal's (gate_writer's) valid tuple for B.
    for a in [FORGED, tb()] {
        let mut tx = begin_as(&pool, "readonly").await;
        assert_eq!(attempt(&mut tx, &a).await, Err(DbError::Sqlx));
        assert_eq!(manifests(&mut tx).await, Vec::<Uuid>::new());
    }
    // ... and the reverse copy: gate_writer (tenant B) cannot use readonly's tuple for A.
    let mut tx = begin_as(&pool, "gate_writer").await;
    assert_eq!(attempt(&mut tx, &ta()).await, Err(DbError::Sqlx));
    assert_eq!(manifests(&mut tx).await, Vec::<Uuid>::new());
    drop(tx);

    // A writer authorized for tenant A cannot insert a manifest for the victim tenant B under an invented
    // assertion, and its authorized context for A cannot be used to write B rows either (RLS WITH CHECK).
    let mut tx = begin_as(&pool, "manifest_writer").await;
    with_tenant(&mut tx, &TenantContext::from_verified(&ta()))
        .await
        .unwrap();
    let e = insert_manifest(&mut tx, TENANT_B, 7)
        .await
        .expect_err("B row under A");
    assert_eq!(sqlstate(&e), "42501");
    assert!(message(&e).contains("row-level security"), "{e}");
    drop(tx);

    // Missing context stays deny-by-default for the same principals.
    let mut tx = begin_as(&pool, "readonly").await;
    assert_eq!(manifests(&mut tx).await, Vec::<Uuid>::new());
}

#[sqlx::test(migrations = false)]
async fn unauthorized_writer_cannot_insert_victim_manifest(pool: PgPool) {
    migrated(&pool).await; // trusted state: manifest_writer is deliberately NOT authorized for anything yet

    let mut tx = begin_as(&pool, "manifest_writer").await;
    assert_eq!(attempt(&mut tx, &FORGED).await, Err(DbError::Sqlx));
    assert_eq!(attempt(&mut tx, &tb()).await, Err(DbError::Sqlx));
    no_context(&mut tx).await;
    let e = insert_manifest(&mut tx, TENANT_B, 5)
        .await
        .expect_err("victim insert");
    assert_eq!(sqlstate(&e), "42501");
    assert!(message(&e).contains("row-level security"), "{e}");
    drop(tx);
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_authority.expected_manifest")
        .fetch_one(&pool) // superuser bootstrap bypasses RLS: sees what really got stored
        .await
        .unwrap();
    assert_eq!(n, 0);

    // Positive control: once the trusted operator provisions tb for this principal, the identical insert works.
    authorize(&pool, "manifest_writer", &tb()).await;
    let mut tx = begin_tenant_as(&pool, "manifest_writer", TENANT_B).await;
    insert_manifest(&mut tx, TENANT_B, 5)
        .await
        .expect("authorized insert");
    tx.commit().await.unwrap();
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_authority.expected_manifest")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, 1);
}

/// A real non-superuser LOGIN connection on the pinned service. The login (session_user) is the authenticated
/// identity; it enters a runtime role with SET ROLE as in roles/README.md. Fresh random credentials, dropped after.
#[sqlx::test(migrations = false)]
async fn real_login_session_binds_tenant_authority(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) {
    let pool = pool_opts
        .max_connections(2)
        .connect_with(conn_opts.clone())
        .await
        .unwrap();
    migrated(&pool).await;
    authorize(&pool, "manifest_writer", &ta()).await;
    authorize(&pool, "manifest_writer", &tb()).await;
    for (t, n) in [(TENANT_A, 1), (TENANT_B, 2)] {
        let mut tx = begin_tenant_as(&pool, "manifest_writer", t).await;
        insert_manifest(&mut tx, t, n).await.expect("seed");
        tx.commit().await.unwrap();
    }

    let suffix = Uuid::new_v4().simple().to_string();
    let mut logins = Vec::new(); // (login, password)
    for who in ["a", "b", "x"] {
        let name = format!("mc_t007_{who}_{suffix}");
        let pw = Uuid::new_v4().simple().to_string();
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "CREATE ROLE {name} LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT \
             PASSWORD '{pw}'"
        )))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(sqlx::AssertSqlSafe(format!("GRANT readonly TO {name}")))
            .execute(&pool)
            .await
            .unwrap();
        logins.push((name, pw));
    }
    // Trusted provisioning: login a -> tenant A, login b -> tenant B, login x -> nothing.
    authorize(&pool, &logins[0].0, &ta()).await;
    authorize(&pool, &logins[1].0, &tb()).await;

    let mut pools = Vec::new();
    for (name, pw) in &logins {
        pools.push(
            PgPoolOptions::new()
                .max_connections(1)
                .connect_with(conn_opts.clone().username(name).password(pw))
                .await
                .expect("login connects"),
        );
    }
    let (pa, pb, px) = (&pools[0], &pools[1], &pools[2]);

    // Each login: real session identity, not superuser, not BYPASSRLS, and runtime role after SET ROLE.
    for (pool, (name, _)) in pools.iter().zip(&logins) {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE readonly")
            .execute(&mut *tx)
            .await
            .unwrap();
        let (s, c, privileged): (String, String, i64) = sqlx::query_as(
            "SELECT session_user::text, current_user::text, \
                    (SELECT count(*) FROM pg_roles WHERE rolname IN (session_user, current_user) \
                       AND (rolsuper OR rolbypassrls))",
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        assert_eq!(
            (s.as_str(), c.as_str(), privileged),
            (name.as_str(), "readonly", 0)
        );
    }

    async fn runtime_tx(pool: &PgPool) -> Transaction<'static, Postgres> {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE readonly")
            .execute(&mut *tx)
            .await
            .unwrap();
        tx
    }

    // Denials: unprovisioned login with every tuple (incl. valid tuples of the other logins), and a provisioned
    // login with the other tenant's valid tuple or invented fields. Each leaves no context and reads 0 rows.
    for (p, tuples) in [
        (px, attempts()),
        (
            pa,
            vec![
                FORGED,
                tb(),
                VerifiedTenantAssertion {
                    workload_id: Uuid::from_u128(99),
                    ..ta()
                },
            ],
        ),
        (
            pb,
            vec![
                FORGED,
                ta(),
                VerifiedTenantAssertion {
                    assertion_digest: [0xff; 32],
                    ..tb()
                },
            ],
        ),
    ] {
        for a in tuples {
            let mut tx = runtime_tx(p).await;
            assert_eq!(attempt(&mut tx, &a).await, Err(DbError::Sqlx), "{a:?}");
            let e = raw_set_context(&mut tx, &a).await.expect_err("raw forgery");
            assert_eq!(sqlstate(&e), "42501", "{a:?}: {e}");
            no_context(&mut tx).await;
            assert_eq!(manifests(&mut tx).await, Vec::<Uuid>::new(), "{a:?}");
        }
    }

    // The login cannot manufacture its own authorization, become the owner, or change its session identity.
    let mut tx = runtime_tx(px).await;
    let e = sqlx::query(
        "INSERT INTO mc_authority.tenant_authorization (principal, tenant_id, workload_id, assertion_digest) \
         VALUES (session_user, $1, $2, $3)",
    )
    .bind(TENANT_B)
    .bind(FORGED.workload_id)
    .bind(&FORGED.assertion_digest[..])
    .execute(&mut *tx)
    .await
    .expect_err("self-authorization");
    assert_eq!(sqlstate(&e), "42501");
    drop(tx);
    for stmt in [
        format!("SET ROLE {OWNER}"),
        format!("SET SESSION AUTHORIZATION {}", logins[0].0),
        "SET SESSION AUTHORIZATION readonly".to_owned(),
        "SET SESSION AUTHORIZATION DEFAULT".to_owned(),
    ] {
        let mut tx = px.begin().await.unwrap();
        let r = sqlx::query(sqlx::AssertSqlSafe(stmt.clone()))
            .execute(&mut *tx)
            .await;
        // DEFAULT to itself is a no-op for a plain login; every other change must be refused.
        if stmt.ends_with("DEFAULT") {
            r.expect("reset to own session user is a no-op");
        } else {
            assert_eq!(sqlstate(&r.expect_err(&stmt)), "42501", "{stmt}");
        }
    }
    // ... and after all of that the unprovisioned login still has no access.
    let mut tx = runtime_tx(px).await;
    assert_eq!(attempt(&mut tx, &tb()).await, Err(DbError::Sqlx));
    drop(tx);

    // Positive controls through the public API on the real logins: a sees only A, b sees only B.
    let mut tx = runtime_tx(pa).await;
    with_tenant(&mut tx, &TenantContext::from_verified(&ta()))
        .await
        .unwrap();
    assert_eq!(manifests(&mut tx).await, vec![id(1)]);
    drop(tx);
    let mut tx = runtime_tx(pb).await;
    with_tenant(&mut tx, &TenantContext::from_verified(&tb()))
        .await
        .unwrap();
    assert_eq!(manifests(&mut tx).await, vec![id(2)]);
    drop(tx);

    // Teardown of only the fixtures this test created.
    for p in &pools {
        p.close().await;
    }
    for (name, _) in &logins {
        sqlx::query(sqlx::AssertSqlSafe(format!("DROP ROLE {name}")))
            .execute(&pool)
            .await
            .unwrap();
    }
}

/// The authorization state is unreachable for every runtime role and owned by the offline owner only.
#[sqlx::test(migrations = false)]
async fn authorization_state_is_owner_only(pool: PgPool) {
    provisioned(&pool).await;
    for role in RUNTIME_ROLES {
        for stmt in [
            "SELECT count(*) FROM mc_authority.tenant_authorization",
            "INSERT INTO mc_authority.tenant_authorization (principal, tenant_id, workload_id, assertion_digest) \
             VALUES ('x', gen_random_uuid(), gen_random_uuid(), '\\x00'::bytea)",
            "UPDATE mc_authority.tenant_authorization SET principal = session_user",
            "DELETE FROM mc_authority.tenant_authorization",
            "TRUNCATE mc_authority.tenant_authorization",
        ] {
            let mut tx = begin_as(&pool, role).await;
            let e = sqlx::query(sqlx::AssertSqlSafe(stmt))
                .execute(&mut *tx)
                .await
                .expect_err(stmt);
            assert_eq!(sqlstate(&e), "42501", "{role}: {stmt}: {e}");
        }
    }
    let (owner, non_owner_acl, rls, forced): (String, i64, bool, bool) = sqlx::query_as(
        "SELECT r.rolname::text, \
                (SELECT count(*) FROM aclexplode(c.relacl) a WHERE a.grantee <> c.relowner), \
                c.relrowsecurity, c.relforcerowsecurity \
         FROM pg_class c JOIN pg_roles r ON r.oid = c.relowner \
         WHERE c.oid = 'mc_authority.tenant_authorization'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (owner.as_str(), non_owner_acl, rls, forced),
        (OWNER, 0, true, true)
    );
}

/// Guard-removal control (fresh scratch database only): drop the authorization predicate from the setter and the
/// very same forged assertion succeeds and exposes the victim row, which is what the denial tests above prevent.
#[sqlx::test(migrations = false)]
async fn removing_the_authorization_predicate_reopens_the_forgery(pool: PgPool) {
    fixture(&pool).await;
    let forge_and_read = |pool: PgPool| async move {
        let mut tx = begin_as(&pool, "readonly").await;
        let r = attempt(&mut tx, &FORGED).await;
        let rows = if r.is_ok() {
            manifests(&mut tx).await
        } else {
            vec![]
        };
        (r, rows)
    };
    // Intact predicate: denied, 0 victim rows.
    assert_eq!(
        forge_and_read(pool.clone()).await,
        (Err(DbError::Sqlx), vec![])
    );

    // Scratch mutation: the predicate guard can never fire.
    let def: String = sqlx::query_scalar(
        "SELECT pg_get_functiondef('mc_authority.set_tenant_context(uuid, uuid, bytea)'::regprocedure)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let mutated = def.replace("IF NOT EXISTS (", "IF FALSE AND NOT EXISTS (");
    assert_ne!(mutated, def, "the predicate must be present to be removed");
    let mut tx = pool.begin().await.unwrap();
    sqlx::query(sqlx::AssertSqlSafe(format!("SET LOCAL ROLE {OWNER}")))
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(sqlx::AssertSqlSafe(mutated))
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Guard removed: the identical forged call is accepted and tenant B's manifest becomes visible.
    assert_eq!(forge_and_read(pool.clone()).await, (Ok(()), vec![id(2)]));
}
