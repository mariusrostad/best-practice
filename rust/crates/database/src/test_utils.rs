//! Utilities for database integration tests.
//!
//! These helpers use the PostgreSQL instance configured in the repository's
//! `compose.yml`. Start it from the repository root, then run the database
//! tests from the Rust workspace:
//!
//! ```text
//! podman compose up -d
//! cd rust
//! cargo test -p database
//! ```
//!
//! Generate and open this documentation with:
//!
//! ```text
//! cargo doc -p database --open
//! ```
//!
//! Use [`test_pool`] when a test can share the compose app database. Tests that
//! exercise [`crate::setup`] can pass [`TEST_DATABASE_URL`] directly.
//!
//! Use [`create_isolated_pool`] when a test needs a clean database, such as
//! when applying a controlled migrator from `tests/migrations`. Close the
//! isolated pool before calling [`drop_test_database`] so PostgreSQL can remove
//! it cleanly:
//!
//! ```no_run
//! use database::test_utils::{
//!     create_isolated_pool, drop_test_database, test_pool,
//! };
//!
//! # async fn isolated_database_test() {
//! let admin_pool = test_pool().await;
//! let pool = create_isolated_pool(&admin_pool).await;
//!
//! // Run migrations and assertions against `pool`.
//!
//! pool.close().await;
//! drop_test_database(&admin_pool).await;
//! # }
//! ```
//!
//! The isolated helpers use one fixed database name. Tests that use them must
//! not run concurrently.

use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;

/// URL of the compose app database used by integration tests.
pub const TEST_DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/app";

/// Name of the disposable database used by isolated integration tests.
pub const TEST_DATABASE_NAME: &str = "database_migration_probe_test";

/// Connects to the compose app database for integration tests.
///
/// Use this pool directly for tests that can safely share the app database, or
/// use it as the administrative pool passed to [`create_isolated_pool`] and
/// [`drop_test_database`].
///
/// # Panics
///
/// Panics if PostgreSQL is unavailable or the connection cannot be
/// established.
pub async fn test_pool() -> PgPool {
    crate::connect(TEST_DATABASE_URL)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "failed to connect to the test database; run `podman compose up -d` first: {error:#}"
            )
        })
}

/// Creates and connects to a clean disposable database.
///
/// Any existing database named [`TEST_DATABASE_NAME`] is dropped first. Close
/// the returned pool and call [`drop_test_database`] after the test.
///
/// # Panics
///
/// Panics if the disposable database cannot be dropped, created, or connected
/// to.
pub async fn create_isolated_pool(admin_pool: &PgPool) -> PgPool {
    drop_test_database(admin_pool).await;

    sqlx::query(&format!("CREATE DATABASE {TEST_DATABASE_NAME}"))
        .execute(admin_pool)
        .await
        .expect("test database should be creatable");

    let options = PgConnectOptions::from_str(TEST_DATABASE_URL)
        .expect("test database URL should be valid")
        .database(TEST_DATABASE_NAME);

    PgPoolOptions::new()
        .connect_with(options)
        .await
        .expect("test database should be connectable")
}

/// Removes the disposable database if it exists.
///
/// Close pools connected to the disposable database before calling this
/// helper. PostgreSQL forcibly terminates any connections that remain.
///
/// # Panics
///
/// Panics if PostgreSQL cannot remove the database.
pub async fn drop_test_database(admin_pool: &PgPool) {
    sqlx::query(&format!(
        "DROP DATABASE IF EXISTS {TEST_DATABASE_NAME} WITH (FORCE)"
    ))
    .execute(admin_pool)
    .await
    .expect("test database should be removable");
}
