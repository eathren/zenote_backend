use axum::extract::Extension;
use axum::{Json, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::graph::{Graph, NewGraphRequest};
use time::OffsetDateTime;
use log::error;



pub async fn create_graph(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewGraphRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Graph,
        "INSERT INTO graphs (name, owner_id) VALUES ($1, $2) RETURNING id, name, date_created, date_updated, deleted, owner_id",
        &input.name,
        input.owner_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(graph) => (StatusCode::CREATED, Json(graph)),
        Err(e) => {
            error!("Failed to create graph: {:?}", e);
            // return empty graph for now.
            (StatusCode::INTERNAL_SERVER_ERROR,  Json(Graph {
                id: Uuid::nil(), // Use nil UUID to indicate "empty"
                name: "".to_string(),
                date_created: Some(OffsetDateTime::now_utc()), 
                date_updated: Some(OffsetDateTime::now_utc()),
                deleted: false,
                owner_id: Uuid::nil(), // Use nil UUID
            }))
        },
    }
}
pub async fn fetch_all_graphs(
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse   {
    let result = sqlx::query_as!(Graph,
        "SELECT * FROM graphs"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(graphs) => (StatusCode::OK, Json(graphs)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}
