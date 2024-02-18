#[cfg(test)]
pub mod graph_handler_tests {
    use crate::handlers::{tests::common, utils::{graph_utils::{create_graph_db, fetch_graph_db, fetch_all_graphs_db}, user_utils::create_user_db}};
    use crate::models::{graph::NewGraphRequest, user::NewUserRequest};
    use uuid::Uuid;
    use common::setup_test_db;

    async fn setup_user_and_graph(pool: &sqlx::PgPool) -> (Uuid, Uuid) {
        let user_email = format!("user_{}@example.com", Uuid::new_v4());
        let user = create_user_db(pool, user_email).await.expect("Failed to create user");
        let graph_request = NewGraphRequest {
            name: "Test Graph".to_string(),
            owner_id: user.id,
        };
        let graph = create_graph_db(pool, graph_request).await.expect("Failed to create graph");
        (user.id, graph.id)
    }

    #[tokio::test]
    async fn test_create_graph_db() {
        let pool = setup_test_db().await; 
        let user_email = format!("user_{}@example.com", Uuid::new_v4());
        let user = create_user_db(&pool, user_email).await.expect("Failed to create user");
        
        let new_graph_request = NewGraphRequest {
            name: "Test Graph".to_string(),
            owner_id: user.id,
        };
        let result = create_graph_db(&pool, new_graph_request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_graph_db() {
        let pool = setup_test_db().await;
        let (user_id, graph_id) = setup_user_and_graph(&pool).await;
        
        let result = fetch_graph_db(&pool, graph_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().owner_id, user_id);
    }

    #[tokio::test]
    async fn test_fetch_all_graphs_db() {
        let pool = setup_test_db().await;
        setup_user_and_graph(&pool).await; // Setup initial graph
        
        let result = fetch_all_graphs_db(&pool).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty(), "Should fetch at least one graph");
    }

    #[tokio::test]
    async fn test_fetch_nonexistent_graph_db() {
        let pool = setup_test_db().await;
        let non_existent_graph_id = Uuid::new_v4(); // Assuming this ID has not been used
        
        let result = fetch_graph_db(&pool, non_existent_graph_id).await;
        assert!(result.is_err(), "Should error when fetching a nonexistent graph");
    }
}
