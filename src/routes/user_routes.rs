use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::user_handlers::create_user;

pub fn user_routes() -> Router {
    Router::new()
        .route("/user", post(create_user))
}