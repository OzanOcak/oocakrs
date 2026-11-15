use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/api/hello", get(hello))
}

async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, World!" }))
}