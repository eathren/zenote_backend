#[cfg(test)]
mod edge_handler_tests{
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::{handlers::{tests::common::setup_test_db, utils::{edge_utils::{create_edge_db, fetch_edge_db}, graph_utils::create_graph_db, node_utils::create_node_db, user_utils::create_user_db}}, models::edge::NewEdgeRequest};
    use crate::models::graph::NewGraphRequest;

    async fn setup_user_and_graph_and_nodes(pool: &PgPool) -> (Uuid, Uuid, Uuid, Uuid) {
        let user_email = format!("user_{}@example.com", Uuid::new_v4());
        let user = create_user_db(pool, user_email).await.expect("Failed to create user");
        
        let new_graph_request = NewGraphRequest {
            name: "Test Graph".to_string(),
            owner_id: user.id,
        };
        let graph = create_graph_db(pool, new_graph_request).await.expect("Failed to create graph");
        let node_name = "Test Node 1".to_string();
        let node = create_node_db(pool, graph.id, node_name.clone()).await.expect("Failed to create node");
        let node_name_2 = "Test Node 2".to_string();
        let node_2 = create_node_db(pool, graph.id, node_name_2.clone()).await.expect("Failed to create node");
        (user.id, graph.id, node.id, node_2.id)
    }

    #[tokio::test]
    async fn test_create_and_fetch_edge() {
        let pool = setup_test_db().await;
        let (_, graph_id, node_id, node_id_2) = setup_user_and_graph_and_nodes(&pool).await;

        let edge_request = NewEdgeRequest {
            graph_id,
            source_id: node_id,
            target_id: node_id_2,
            label: None,
        };
        let created_edge = create_edge_db(&pool, edge_request).await.expect("Failed to create edge");

        let fetched_edge = fetch_edge_db(&pool, created_edge.id).await.expect("Failed to fetch edge");

        assert_eq!(fetched_edge.id, created_edge.id, "Fetched edge should have the same ID");
    }



    #[tokio::test]
    async fn test_create_edge_with_nonexistent_nodes() {
        let pool = setup_test_db().await;
        let (_, graph_id, _, _) = setup_user_and_graph_and_nodes(&pool).await;
        let non_existent_node_id = Uuid::new_v4();

        let edge_request = NewEdgeRequest {
            graph_id,
            source_id: non_existent_node_id,
            target_id: non_existent_node_id,
            label: None,
        };
        let result = create_edge_db(&pool, edge_request).await;

        assert!(result.is_err(), "Should error when creating edge with nonexistent nodes");
    }



}