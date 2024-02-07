use axum::{
    routing::get,
    Router,
};
use log::{debug, error, log_enabled, info, Level};

#[tokio::main]
async fn main() {
    env_logger::init();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
