use sqlx::postgres::PgPool;
use std::env;
use sqlx::migrate::Migrator;
use log::{info, error};
static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Attempting to establish a connection to the database.");

    match PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to the database.");
            Ok(pool)
        },
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            Err(e)
        }
    }
}

pub async fn run_migrations_for_url(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

pub async fn migrate_databases() -> Result<(), Box<dyn std::error::Error>> {
    // Migrate the main database
    let main_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    run_migrations_for_url(&main_database_url).await.expect("Failed to run migrations on main database");

    // Migrate the test database
    let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    run_migrations_for_url(&test_database_url).await.expect("Failed to run migrations on test database");
    info!("Migrations run");
    Ok(())
}