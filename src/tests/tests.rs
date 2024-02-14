use crate::main;
use sqlx::{PgPool, Executor};
use uuid::Uuid;
use once_cell::sync::Lazy;
use std::net::SocketAddr;

static POOL: Lazy<PgPool> = Lazy::new(|| {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgPool::connect_lazy(&database_url).expect("Failed to connect to the database")
});

async fn setup() -> anyhow::Result<()> {
    // Run migrations if necessary
    // Clean up the database or set up test data
    Ok(())
}

#[tokio::test]
async fn test_create_graph() -> anyhow::Result<()> {
    setup().await?;

    // Use `reqwest` or another HTTP client to make requests to your application
    // Assert on the responses
    Ok(())
}

#[tokio::test]
async fn test_fetch_all_graphs() -> anyhow::Result<()> {
    setup().await?;

    // Similar setup to `test_create_graph`, targeting the fetch functionality
    Ok(())
}
