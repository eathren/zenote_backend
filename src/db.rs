use sqlx::postgres::PgPool;
use std::env;
use log::{info, error};

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
