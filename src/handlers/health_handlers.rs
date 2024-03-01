use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> impl IntoResponse {
    let health_response = HealthResponse {
        status: "ok".to_string(),
    };

    (StatusCode::OK, Json(health_response))
}
