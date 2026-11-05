// PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::error::DbError;

/// A tenant/workload assertion that an authenticating caller has already verified
/// (mutually authenticated workload identity; doc 07). Construct it only after that verification:
/// this crate cannot check the proof. The database does not trust these fields either: they take effect only
/// if the trusted operator provisioned exactly this tuple for the connected session's login
/// (`mc_authority.tenant_authorization`, db/authority/base/0005); otherwise `with_tenant` fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedTenantAssertion {
    pub tenant_id: Uuid,
    pub workload_id: Uuid,
    pub assertion_digest: [u8; 32],
}

/// Tenant context derived from a [`VerifiedTenantAssertion`]; there is no other constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TenantContext(VerifiedTenantAssertion);

impl TenantContext {
    pub fn from_verified(assertion: &VerifiedTenantAssertion) -> Self {
        Self(*assertion)
    }
}

/// Binds `tx` to the authenticated tenant through the SECURITY DEFINER function
/// `mc_authority.set_tenant_context`, which requires a protected authorization for the session's login
/// (an unauthorized tuple is a redacted [`DbError::Sqlx`] and establishes nothing). The binding is only
/// honoured for this transaction (never carried over by a pooled connection). A different assertion already
/// bound to this transaction yields [`DbError::TenantMismatch`].
pub async fn with_tenant(
    tx: &mut Transaction<'_, Postgres>,
    ctx: &TenantContext,
) -> Result<(), DbError> {
    sqlx::query("SELECT mc_authority.set_tenant_context($1, $2, $3)")
        .bind(ctx.0.tenant_id)
        .bind(ctx.0.workload_id)
        .bind(&ctx.0.assertion_digest[..])
        .execute(&mut **tx)
        .await?;
    Ok(())
}
