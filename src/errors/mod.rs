use axum::{
    response::IntoResponse,
    http::StatusCode,
    BoxError,
};
use serde::Serialize;

#[derive(Serialize)]
struct GenericErrorResponse {
    error: String,
}

// impl fmt::Display for Box<dyn std::error::Error + Send + Sync> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// A generic error handler that can be used across the application
pub async fn handle_generic_error(
    err: BoxError,
) -> impl IntoResponse {
    let error_response = GenericErrorResponse {
        error: format!("Something went wrong: {}", err),
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        serde_json::to_string(&error_response).unwrap(),
    )
}
