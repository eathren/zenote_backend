use super::utils::user_utils::{create_user_db, delete_user_db, fetch_all_users_db, fetch_user_db};
use crate::models::user::NewUserRequest;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::{error, info};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewUserRequest>,
) -> impl IntoResponse {
    match create_user_db(&pool, input.user_id, input.email).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("Fetching user: {:?}", user_id);
    match fetch_user_db(&pool, user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            error!("User not found: {:?}", e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_user_db(&pool, user_id).await {
        Ok(rows) if rows > 0 => StatusCode::OK.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    info!("Fetching all users.");
    match fetch_all_users_db(&pool).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            error!("Failed to fetch users: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
