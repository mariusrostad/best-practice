//! PostgreSQL connection and migration utilities.
//!
//! Use [`connect`] when only a connection pool is needed and [`setup`] to
//! connect and apply all embedded migrations.
//! Database integration tests can use the helpers in [`test_utils`].

use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub mod test_utils;

/// The database migrations embedded in this crate at compile time.
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// Creates a PostgreSQL connection pool for `database_url`.
///
/// The pool uses [`PgPoolOptions`]' default configuration and establishes its
/// first connection before returning.
///
/// # Errors
///
/// Returns an error if a connection to PostgreSQL cannot be established.
pub async fn connect(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .connect(database_url)
        .await
        .context("failed to connect to Postgres")
}

/// Applies all pending embedded migrations to `pool`.
///
/// Migrations that have already been applied are not run again.
///
/// # Errors
///
/// Returns an error if migration metadata cannot be read or a pending
/// migration cannot be applied.
pub async fn migrate(pool: &PgPool) -> Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("failed to run database migrations")
}

/// Connects to PostgreSQL and applies all pending migrations.
///
/// # Errors
///
/// Returns an error if the database connection cannot be established or a
/// migration cannot be applied.
pub async fn setup(database_url: &str) -> Result<PgPool> {
    let pool = connect(database_url).await?;
    migrate(&pool).await?;
    Ok(pool)
}
