#[cfg(test)]
pub mod user_handler_tests {
    use crate::handlers::{tests::common, utils::user_utils::{create_user_db, fetch_user_db}};
    use common::setup_test_db;

    #[tokio::test]
    async fn test_create_user_db() {
        let pool = setup_test_db().await; 
        let result = create_user_db(&pool, "test@example.com".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_user_db() {
        let pool = setup_test_db().await; 
        // Assuming there's a user with id = Uuid::new_v4(), replace with actual logic
        let email = "test1@example.com".to_string();
        let user =create_user_db(&pool, email).await.unwrap();
        let user_id = user.id;
        let result = fetch_user_db(&pool, user_id).await;
        assert!(result.is_ok());
    }

      
    
}
