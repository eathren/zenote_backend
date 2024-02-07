use axum::extract::Extension;
use axum::{Json, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
struct GraphResponse {
    id: Uuid,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct NewGraphRequest {
    name: String,
    owner_id: Uuid,
}

// pub async fn create_graph(
//     Extension(pool): Extension<PgPool>,
//     Json(input): Json<NewGraphRequest>,
// ) -> impl IntoResponse {
//     let result = sqlx::query_as!(
//         GraphResponse,
//         "INSERT INTO graphs (name, owner_id) VALUES ($1, $2) RETURNING id, name",
//         &input.name,
//         input.owner_id
//     )
//     .fetch_one(&pool)
//     .await;

//     match result {
//         Ok(graph) => (StatusCode::CREATED, Json(graph)),
//         Err(e) => {
//             println!("Failed to create graph: {:?}", e); // Replace with appropriate logging
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(ErrorResponse::from("Failed to create graph"))
//             )
//         },
//     }
// }

pub async fn fetch_all_graphs(
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse   {
    let result = sqlx::query_as!(GraphResponse,
        "SELECT id, name FROM graphs"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(graphs) => (StatusCode::OK, Json(graphs)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}