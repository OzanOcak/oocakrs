use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub mod admin;
pub mod likes;
pub mod comments;
pub mod user;
pub mod visitor_count; 

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/api/hello", get(hello))
        .merge(likes::router())
        .merge(comments::router())
        .merge(user::router())
        .merge(admin::router())
        .merge(visitor_count::router()) 
}

async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, World!" }))
}