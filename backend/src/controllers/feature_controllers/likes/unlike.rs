use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn unlike_blog(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if post_id.is_empty() {
        return Err(ApiError::BadRequest("Slug is required".into()));
    }

    sqlx::query("UPDATE likes SET likes_count = likes_count - 1 WHERE slug = $1")
        .bind(&post_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "message": "Post unliked successfully" })))
}