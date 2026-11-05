// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
mod common;
use common::*;
use mc_authority_db::{DbError, migrate};
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// A scratch modules tree under the OS temp dir, removed on drop.
struct Tree(PathBuf);
impl Tree {
    fn new(modules: &str, files: &[(&str, &str, &str)]) -> Self {
        let root = std::env::temp_dir().join(format!("mc-authority-db-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("MODULES"), modules).unwrap();
        for (module, name, sql) in files {
            std::fs::create_dir_all(root.join(module)).unwrap();
            std::fs::write(root.join(module).join(name), sql).unwrap();
        }
        Tree(root)
    }
    fn modules(&self) -> PathBuf {
        self.0.join("MODULES")
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn listed_modules(modules_file: &Path) -> Vec<String> {
    std::fs::read_to_string(modules_file)
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

#[sqlx::test(migrations = false)]
async fn modules_applied_in_order_on_empty_db(pool: PgPool) {
    migrate(&pool, &modules_file())
        .await
        .expect("migrate on an empty database");

    let applied: Vec<(String, String)> =
        sqlx::query_as("SELECT module, file FROM mc_migration.applied ORDER BY seq")
            .fetch_all(&pool)
            .await
            .unwrap();
    let ours: Vec<(&str, &str)> = applied
        .iter()
        .filter(|(m, _)| m == "roles" || m == "base")
        .map(|(m, f)| (m.as_str(), f.as_str()))
        .collect();
    assert_eq!(
        ours,
        [
            ("roles", "0001_roles.sql"),
            ("base", "0001_schema.sql"),
            ("base", "0002_tables.sql"),
            ("base", "0003_rls_and_grants.sql"),
            ("base", "0004_tenant_context.sql"),
            ("base", "0005_tenant_authorization.sql"),
        ]
    );
    // Whatever modules exist, they were applied in MODULES order.
    let order = listed_modules(&modules_file());
    let positions: Vec<usize> = applied
        .iter()
        .map(|(m, _)| order.iter().position(|o| o == m).unwrap())
        .collect();
    assert!(positions.windows(2).all(|w| w[0] <= w[1]), "{applied:?}");

    // The result is a working schema, and a re-run is a no-op.
    let tables: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class WHERE relnamespace = 'mc_authority'::regnamespace AND relkind = 'r'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(tables >= 14);
    migrate(&pool, &modules_file())
        .await
        .expect("idempotent re-run");
    let again: i64 = sqlx::query_scalar("SELECT count(*) FROM mc_migration.applied")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(again as usize, applied.len());
}

#[sqlx::test(migrations = false)]
async fn modules_order_follows_modules_file_not_alphabet(pool: PgPool) {
    // `zz_first` must run before `aa_second` because MODULES lists it first.
    let t = Tree::new(
        "zz_first\naa_second\n",
        &[
            (
                "zz_first",
                "0001.sql",
                "CREATE TABLE public.mc_order_probe (x int); INSERT INTO public.mc_order_probe VALUES (1);",
            ),
            ("zz_first", "README.md", "ignored: not a .sql file"),
            (
                "aa_second",
                "0001.sql",
                "UPDATE public.mc_order_probe SET x = x + 1;",
            ),
        ],
    );
    migrate(&pool, &t.modules()).await.unwrap();
    let x: i32 = sqlx::query_scalar("SELECT x FROM public.mc_order_probe")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(x, 2);
}

#[sqlx::test(migrations = false)]
async fn failures_name_the_file_only_and_edited_files_are_rejected(pool: PgPool) {
    // A failing statement: the error carries the file name, not the SQL or the server message.
    let bad = Tree::new(
        "m\n",
        &[(
            "m",
            "0001_bad.sql",
            "SELECT secret_marker FROM nonexistent_table;",
        )],
    );
    let err = migrate(&pool, &bad.modules()).await.unwrap_err();
    assert_eq!(err, DbError::Migration("0001_bad.sql".to_owned()));
    assert!(!format!("{err} {err:?}").contains("nonexistent_table"));

    // A missing module directory and an unsafe module name fail before anything is applied.
    let missing = Tree::new("nope\n", &[]);
    assert_eq!(
        migrate(&pool, &missing.modules()).await.unwrap_err(),
        DbError::Migration("nope".to_owned())
    );
    let escape = Tree::new("../etc\n", &[]);
    assert!(matches!(
        migrate(&pool, &escape.modules()).await,
        Err(DbError::Migration(_))
    ));

    // An applied file that is edited afterwards is rejected, not silently skipped or re-run.
    let t = Tree::new(
        "m\n",
        &[(
            "m",
            "0001.sql",
            "CREATE TABLE public.mc_edit_probe (x int);",
        )],
    );
    migrate(&pool, &t.modules()).await.unwrap();
    migrate(&pool, &t.modules()).await.unwrap();
    std::fs::write(
        t.0.join("m/0001.sql"),
        "CREATE TABLE public.mc_edit_probe (x int, y int);",
    )
    .unwrap();
    assert_eq!(
        migrate(&pool, &t.modules()).await.unwrap_err(),
        DbError::Migration("0001.sql".to_owned())
    );
}
