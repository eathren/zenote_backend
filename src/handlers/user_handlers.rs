use axum::{
    extract::{Extension, Path},
    Json, response::IntoResponse, http::StatusCode,
};
use log::error;
use sqlx::PgPool;
use crate::models::user::{User, NewUserRequest};
use uuid::Uuid;

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (email) VALUES ($1) RETURNING *",
        &input.email,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            error!("User not found: {:?}", e);
            Err(StatusCode::NOT_FOUND) 
        },
    }
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => {
            error!("User not found: {:?}", e);
            Err(StatusCode::NOT_FOUND) 
        },
    }
}