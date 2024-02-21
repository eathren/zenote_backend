use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::node_handlers::{create_node, fetch_all_nodes, fetch_node, update_node};

pub fn node_routes() -> Router {
    Router::new()
        .route("/nodes/:graph_id", get(fetch_all_nodes))
        .route("/node", post(create_node))
        .route("/node/:id", get(fetch_node))
        .route("/node/:id", post(update_node))
}