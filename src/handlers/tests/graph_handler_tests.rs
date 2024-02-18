#[cfg(test)]
pub mod graph_handler_tests {
    use crate::handlers::{tests::common, utils::graph_utils::{create_graph_db, fetch_all_graphs_db, fetch_graph_db}};
    use uuid::Uuid;
    use crate::models::graph::NewGraphRequest;
    use common::setup_test_db;


    #[tokio::test]
    async fn test_create_graph_db() {
        let pool = setup_test_db().await; 
        let new_graph_request = NewGraphRequest {
            name: "Test Graph".to_string(),
            owner_id: Uuid::new_v4(),
        };
        let result = create_graph_db(&pool, new_graph_request).await;
        print!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_graph_db() {
        let pool = setup_test_db().await;
        // Assuming there is a graph with this ID; you might need to insert one first
        let graph_id = Uuid::new_v4();
        let result = fetch_graph_db(&pool, graph_id).await;
        assert!(result.is_err()); 
    }

}