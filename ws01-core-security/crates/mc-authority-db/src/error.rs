// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use thiserror::Error;

/// Database errors. No SQL text or server message is carried (nothing sensitive can leak through it).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DbError {
    /// A migration could not be read or applied; the payload is a module name or file name only.
    #[error("migration failed: {0}")]
    Migration(String),
    /// The transaction already has a different authenticated tenant context.
    #[error("tenant context mismatch")]
    TenantMismatch,
    /// Any other database error.
    #[error("database error")]
    Sqlx,
}

/// SQLSTATE raised by `mc_authority.set_tenant_context` on a mismatch (db/authority/base/0004).
const TENANT_MISMATCH_SQLSTATE: &str = "MCT01";

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::Database(d) if d.code().as_deref() == Some(TENANT_MISMATCH_SQLSTATE) => {
                DbError::TenantMismatch
            }
            _ => DbError::Sqlx,
        }
    }
}
