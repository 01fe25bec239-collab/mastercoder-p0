//! mc-authority-db — PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
//!
//! PostgreSQL authority framework (W01-T007): the module-ordered migration runner and the
//! authenticated tenant-context API. Roles, grants and RLS live in `db/authority/**`
//! (see `db/authority/roles/README.md`). A caller-set tenant GUC is not authentication:
//! the tenant is established only from a [`VerifiedTenantAssertion`] (INV-15, doc 07).

mod error;
mod migrate;
mod tenant;

pub use error::DbError;
pub use migrate::migrate;
pub use tenant::{TenantContext, VerifiedTenantAssertion, with_tenant};
