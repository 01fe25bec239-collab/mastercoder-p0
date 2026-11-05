// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;

use sqlx::{AssertSqlSafe, PgPool};

use crate::error::DbError;

/// Advisory-lock key serializing concurrent migrators (arbitrary constant, per database).
const MIGRATION_LOCK_KEY: i64 = 0x4d43_5f41_5554_4801;

const LEDGER_DDL: &str = "\
CREATE SCHEMA IF NOT EXISTS mc_migration;
CREATE TABLE IF NOT EXISTS mc_migration.applied (
    seq bigint GENERATED ALWAYS AS IDENTITY,
    module text NOT NULL,
    file text NOT NULL,
    sha256 bytea NOT NULL,
    applied_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (module, file)
);";

fn file_name(p: &Path) -> String {
    p.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("?")
        .to_owned()
}

fn valid_module_name(n: &str) -> bool {
    !n.is_empty()
        && n.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// Applies every module listed in `modules_file`, in listed order; each module is a sibling directory whose
/// `*.sql` files run in lexical order, one transaction per file. Applied files are recorded with their SHA-256
/// in `mc_migration.applied`: a re-run skips them and an edited applied file is rejected. Everything is read
/// before the database is touched. Run it as the offline migration owner (see roles/README.md).
/// Failures name the module or file only (`DbError::Migration`), never SQL or server text.
pub async fn migrate(pool: &PgPool, modules_file: &Path) -> Result<(), DbError> {
    let base = modules_file.parent().unwrap_or_else(|| Path::new("."));
    let list = std::fs::read_to_string(modules_file)
        .map_err(|_| DbError::Migration(file_name(modules_file)))?;

    // (module, file name, sql)
    let mut plan: Vec<(String, String, String)> = Vec::new();
    let mut seen = HashSet::new();
    for module in list
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        if !valid_module_name(module) || !seen.insert(module) {
            return Err(DbError::Migration(module.to_owned()));
        }
        let dir = base.join(module);
        let entries = std::fs::read_dir(&dir).map_err(|_| DbError::Migration(module.to_owned()))?;
        let mut files = Vec::new();
        for entry in entries {
            let name = file_name(
                &entry
                    .map_err(|_| DbError::Migration(module.to_owned()))?
                    .path(),
            );
            if name.ends_with(".sql") {
                files.push(name);
            }
        }
        files.sort();
        for name in files {
            let sql = std::fs::read_to_string(dir.join(&name))
                .map_err(|_| DbError::Migration(name.clone()))?;
            plan.push((module.to_owned(), name, sql));
        }
    }

    for (module, name, sql) in plan {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(MIGRATION_LOCK_KEY)
            .execute(&mut *tx)
            .await?;
        sqlx::raw_sql(LEDGER_DDL).execute(&mut *tx).await?;
        let same: Option<bool> = sqlx::query_scalar(
            "SELECT sha256 = sha256($1::bytea) FROM mc_migration.applied WHERE module = $2 AND file = $3",
        )
        .bind(sql.as_bytes())
        .bind(&module)
        .bind(&name)
        .fetch_optional(&mut *tx)
        .await?;
        match same {
            Some(true) => continue, // already applied; dropping tx rolls back the no-op
            Some(false) => return Err(DbError::Migration(name)), // applied file was edited
            None => {}
        }
        // The file comes from the repository checkout under the operator's control, not from a caller.
        sqlx::raw_sql(AssertSqlSafe(sql.as_str()))
            .execute(&mut *tx)
            .await
            .map_err(|_| DbError::Migration(name.clone()))?;
        // A file may `SET LOCAL ROLE mc_authority_owner`; the ledger is written as the migrating user.
        sqlx::query("RESET ROLE").execute(&mut *tx).await?;
        sqlx::query("INSERT INTO mc_migration.applied (module, file, sha256) VALUES ($1, $2, sha256($3::bytea))")
            .bind(&module)
            .bind(&name)
            .bind(sql.as_bytes())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
    }
    Ok(())
}
