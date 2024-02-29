use sqlx;
use std::env;

// Ensure this function returns a Pool directly, handling the Result internally.
pub async fn setup_test_db() -> sqlx::PgPool {
    let database_url = format!(
        "postgres://postgres:{}@localhost:5433/{}",
        env::var("POSTGRES_PASSWORD_TEST").unwrap_or_else(|_| "test_password".into()),
        env::var("POSTGRES_DB_TEST").unwrap_or_else(|_| "test_db".into())
    );

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let tables = vec!["users", "graphs", "nodes", "edges"];
    for table in tables.iter().rev() {
        // `graphs` references `users`, clear `graphs` first
        let query = format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE;", table);
        sqlx::query(&query)
            .execute(&pool)
            .await
            .expect("Failed to truncate table");
    }

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
