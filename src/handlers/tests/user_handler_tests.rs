#[cfg(test)]
pub mod user_handler_tests {
    use crate::handlers::{tests::common, utils::user_utils::{create_user_db, delete_user_db, fetch_user_db}};
    use common::setup_test_db;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_user_db() {
        let pool = setup_test_db().await;
        let unique_email = format!("test_{}@example.com", Uuid::new_v4());
        let result = create_user_db(&pool, unique_email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_same_user_db() {
        let pool = setup_test_db().await;
        let email = format!("test_same_{}@example.com", Uuid::new_v4());
        let first_attempt = create_user_db(&pool, email.clone()).await;
        assert!(first_attempt.is_ok());

        // Try to create the same user again
        let second_attempt = create_user_db(&pool, email).await;
        assert!(second_attempt.is_err()); // Expecting error due to duplicate email
    }

    #[tokio::test]
    async fn test_fetch_user_db() {
        let pool = setup_test_db().await;
        let email = format!("fetch_user_{}@example.com", Uuid::new_v4());
        let user = create_user_db(&pool, email.clone()).await.expect("Failed to create user for fetch test");
    
        let result = fetch_user_db(&pool, user.id).await;
        
        assert!(result.is_ok(), "Failed to fetch user: {:?}", result.err());
        let fetched_user = result.expect("Failed to unwrap fetched user");
        assert_eq!(fetched_user.email, email, "Fetched user email does not match");
    }

    #[tokio::test]
    async fn test_delete_user_db() {
        let pool = setup_test_db().await;
        let email = format!("delete_{}@example.com", Uuid::new_v4());
        let user = create_user_db(&pool, email).await.unwrap();

        let result = delete_user_db(&pool, user.id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
    
    #[tokio::test]
    async fn test_delete_nonexistent_user_db() {
        let pool = setup_test_db().await;
        let non_existent_user_id = Uuid::new_v4();
        let result = delete_user_db(&pool, non_existent_user_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
}
