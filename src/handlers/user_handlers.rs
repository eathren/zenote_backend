use axum::{
    extract::Extension,
    Json, response::IntoResponse, http::StatusCode,
};
use sqlx::PgPool;
use crate::models::user::{User, NewUserRequest};
use uuid::Uuid;
use time::OffsetDateTime;

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<NewUserRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (email) VALUES ($1) RETURNING *",
        &input.email,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User{
            id: Uuid::nil(), // Use nil UUID to indicate "empty"
            email: "".to_string(),
            date_created: Some(OffsetDateTime::now_utc()),
            date_updated: Some(OffsetDateTime::now_utc()),
            deleted: false,
        })),
    }
}

