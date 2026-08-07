use database::{
    migrate, setup,
    test_utils::{TEST_DATABASE_URL, create_isolated_pool, drop_test_database, test_pool},
};

const TEST_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./tests/migrations");

#[tokio::test]
async fn controlled_migration_applies_expected_schema() {
    let admin_pool = test_pool().await;
    let pool = create_isolated_pool(&admin_pool).await;

    TEST_MIGRATOR
        .run(&pool)
        .await
        .expect("controlled test migration should succeed");

    let probe_table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
              AND table_name = 'migration_probe'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("migration probe table lookup should succeed");

    assert!(
        probe_table_exists,
        "migration should create migration_probe"
    );

    let applied_migrations: Vec<(i64, String)> =
        sqlx::query_as("SELECT version, description FROM _sqlx_migrations ORDER BY version")
            .fetch_all(&pool)
            .await
            .expect("migration journal lookup should succeed");

    assert_eq!(
        applied_migrations,
        vec![(20260807190000, "create migration probe".to_owned())]
    );

    pool.close().await;
    drop_test_database(&admin_pool).await;
}

#[tokio::test]
async fn migrate_is_idempotent() {
    let pool = test_pool().await;

    migrate(&pool).await.expect("migration should succeed");
    migrate(&pool)
        .await
        .expect("running migrations again should succeed");
}

#[tokio::test]
async fn setup_connects_and_runs_migrations() {
    let pool = setup(TEST_DATABASE_URL).await.unwrap_or_else(|error| {
        panic!("failed to set up the test database; run `podman compose up -d` first: {error:#}")
    });

    let value: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("SELECT 1 should succeed after setup");

    assert_eq!(value, 1);
}
