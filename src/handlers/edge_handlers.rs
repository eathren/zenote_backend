use super::utils::edge_utils::{
    create_edge_db, delete_edge_db, fetch_all_edges_db, fetch_edge_db,
    fetch_edges_by_source_and_target_db, fetch_edges_by_source_db, fetch_edges_by_target_db,
};
use crate::models::edge::NewEdgeRequest;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Handler for creating an edge
pub async fn create_edge(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewEdgeRequest>,
) -> impl IntoResponse {
    match create_edge_db(&pool, input).await {
        Ok(edge) => (StatusCode::CREATED, Json(edge)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create edge").into_response(),
    }
}

/// Handler for fetching an edge by ID
pub async fn fetch_edge(Extension(pool): Extension<PgPool>, Path(edge_id): Path<Uuid>) -> impl IntoResponse {
    match fetch_edge_db(&pool, edge_id).await {
        Ok(edge) => (StatusCode::OK, Json(edge)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Edge not found").into_response(),
    }
}

/// Handler for fetching all edges for a graph
pub async fn fetch_all_edges(
    Extension(pool): Extension<PgPool>,
    Path(graph_id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_all_edges_db(&pool, graph_id).await {
        Ok(edges) => (StatusCode::OK, Json(edges)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch edges").into_response(),
    }
}

/// Handler for deleting an edge
pub async fn delete_edge(
    Extension(pool): Extension<PgPool>,
    Path(edge_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_edge_db(&pool, edge_id).await {
        Ok(rows) if rows > 0 => StatusCode::OK.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Edge not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete edge").into_response(),
    }
}

/// Handler for fetching all edges for a source node
pub async fn fetch_edges_by_source(
    Extension(pool): Extension<PgPool>,
    Path((graph_id, source_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match fetch_edges_by_source_db(&pool, graph_id, source_id).await {
        Ok(edges) => (StatusCode::OK, Json(edges)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch edges").into_response(),
    }
}

/// Handler for fetching all edges for a target node
pub async fn fetch_edges_by_target(
    Extension(pool): Extension<PgPool>,
    Path((graph_id, target_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match fetch_edges_by_target_db(&pool, graph_id, target_id).await {
        Ok(edges) => (StatusCode::OK, Json(edges)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch edges").into_response(),
    }
}

pub async fn fetch_edges_by_source_and_target(
    Extension(pool): Extension<PgPool>,
    Path((graph_id, source_id, target_id)): Path<(Uuid, Uuid, Uuid)>,
) -> impl IntoResponse {
    match fetch_edges_by_source_and_target_db(&pool, graph_id, source_id, target_id).await {
        Ok(edges) => (StatusCode::OK, Json(edges)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch edges").into_response(),
    }
}
