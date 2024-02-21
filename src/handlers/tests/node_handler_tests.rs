#[cfg(test)]
mod node_handler_tests {
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::{handlers::{tests::common::setup_test_db, utils::{graph_utils::create_graph_db, node_utils::{create_node_db, delete_node_db, fetch_all_nodes_db, fetch_node_db, update_node_db}, user_utils::create_user_db}}, models::node::{NewNodeRequest, UpdateNodeRequest}};
    use crate::models::graph::NewGraphRequest;

    async fn setup_user_and_graph(pool: &PgPool) -> (Uuid, Uuid) {
        let user_email = format!("user_{}@example.com", Uuid::new_v4());
        let user = create_user_db(pool, user_email).await.expect("Failed to create user");
        
        let new_graph_request = NewGraphRequest {
            name: "Test Graph".to_string(),
            owner_id: user.id,
        };
        let graph = create_graph_db(pool, new_graph_request).await.expect("Failed to create graph");

        (user.id, graph.id)
    }

    #[tokio::test]
    async fn test_create_and_fetch_node() {
        let pool = setup_test_db().await;
        let (_, graph_id) = setup_user_and_graph(&pool).await;

        let node_name = "Test Node".to_string();
        let node_options = NewNodeRequest {
            graph_id,
            name: Some(node_name.clone()),
        };
        let created_node = create_node_db(&pool, node_options).await.expect("Failed to create node");

        assert_eq!(created_node.name, node_name, "Node name should match");

        let fetched_node = fetch_node_db(&pool, created_node.id).await.expect("Failed to fetch node");

        assert_eq!(fetched_node.id, created_node.id, "Fetched node should have the same ID");
    }

    #[tokio::test]
    async fn test_update_node() {
        let pool = setup_test_db().await;
        let (_, graph_id) = setup_user_and_graph(&pool).await;
    
        let node_name = "Initial Node".to_string();
        let node_options = NewNodeRequest {
            graph_id,
            name: Some(node_name.clone()),
        };
        let created_node = create_node_db(&pool, node_options).await.expect("Failed to create node");
    
        let updated_name = "Updated Node".to_string();
        let update_request = UpdateNodeRequest {
            name: Some(updated_name.clone()),
            deleted: None, // Not updating 'deleted' in this test, so set it to None
        };
        update_node_db(&pool, created_node.id, update_request).await.expect("Failed to update node");
    
        let updated_node = fetch_node_db(&pool, created_node.id).await.expect("Failed to fetch updated node");
    
        assert_eq!(updated_node.name, updated_name, "Node name should be updated");
    }

    #[tokio::test]
    async fn test_delete_node() {
        let pool = setup_test_db().await;
        let (_, graph_id) = setup_user_and_graph(&pool).await;

        let node_name = "Node to Delete".to_string();
        let node_options = NewNodeRequest {
            graph_id,
            name: Some(node_name.clone()),
        };
        let created_node = create_node_db(&pool, node_options).await.expect("Failed to create node");

        delete_node_db(&pool, created_node.id).await.expect("Failed to delete node");

        let result = fetch_node_db(&pool, created_node.id).await;

        assert!(result.is_err(), "Node should not exist after deletion");
    }

    #[tokio::test]
    async fn test_fetch_all_nodes_for_graph() {
        let pool = setup_test_db().await;
        let (_, graph_id) = setup_user_and_graph(&pool).await;
        let node_options_1 = NewNodeRequest {
            graph_id,
            name: Some("Node 1".to_string()),
        };
        let node_options_2 = NewNodeRequest {
            graph_id,
            name: Some("Node 2".to_string()),
        };
        create_node_db(&pool, node_options_1).await.expect("Failed to create first node");
        create_node_db(&pool, node_options_2).await.expect("Failed to create second node");

        let nodes = fetch_all_nodes_db(&pool, graph_id).await.expect("Failed to fetch nodes for graph");

        assert!(nodes.len() >= 2, "Should fetch at least two nodes for the graph");
    }
}
