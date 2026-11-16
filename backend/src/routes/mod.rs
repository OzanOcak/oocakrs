use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub mod likes;
pub mod comments;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/api/hello", get(hello))
        .merge(likes::router())
        .merge(comments::router())
}

async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, World!" }))
}