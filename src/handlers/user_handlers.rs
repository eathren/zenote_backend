// src/handlers/user_handlers.rs

use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Response},
    Json, http::StatusCode,
};
use log::error;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::NewUserRequest;
use super::utils::user_utils::{create_user_db, fetch_user_db}; 

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewUserRequest>,
) -> Response {
    match create_user_db(&pool, input.email).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Response {
    match fetch_user_db(&pool, user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            error!("User not found: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        },
    }
}
