#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Postgres, Executor};
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    async fn setup_database() -> Pool<Postgres> {
        let database_url = "postgres://username:password@localhost/test_database";
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await
            .expect("Failed to connect to the database");

        // Run migrations or setup test data here if needed

        pool
    }

    #[tokio::test]
    async fn test_create_organization_db() {
        let pool = setup_database().await;
        
        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        assert_eq!(created_org.name, "Test Organization");

        // Clean up test data if not using transactional tests
        // sqlx::query("DELETE FROM organizations WHERE id = $1", created_org.id)
        //     .execute(&pool)
        //     .await
        //     .expect("Failed to clean up test organization");
    }

    // Additional tests for fetch, update, and delete
}
