use axum::{
    extract::{Extension, Path},
    Json, response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::node::UpdateNodeRequest;
use super::utils::node_utils::{create_node_db, fetch_node_db, fetch_all_nodes_db, delete_node_db, update_node_db};


pub async fn create_node(
    pool: &PgPool,
    graph_id: Uuid,
    name: String,
) -> Response {
    match create_node_db(&pool, graph_id, name).await {
        Ok(graph) => (StatusCode::CREATED, Json(graph)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create graph").into_response(),
    }
}


pub async fn fetch_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
) -> Response {
    match fetch_node_db(&pool, node_id).await {
        Ok(node) => (StatusCode::OK, Json(node)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Node not found").into_response(),
    }
}

pub async fn fetch_all_nodes(
    Extension(pool): Extension<PgPool>,
    Path(graph_id): Path<Uuid>,
) -> Response {
    match fetch_all_nodes_db(&pool, graph_id).await {
        Ok(nodes) => (StatusCode::OK, Json(nodes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch nodes").into_response(),
    }
}

pub async fn delete_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
) -> Response {
    match delete_node_db(&pool, node_id).await {
        Ok(rows) if rows > 0 => StatusCode::OK.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
    Json(input): Json<UpdateNodeRequest>,
) -> Response {
    match update_node_db(&pool, node_id, input).await {
        Ok(node) => (StatusCode::OK, Json(node)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update node").into_response(),
    }
}