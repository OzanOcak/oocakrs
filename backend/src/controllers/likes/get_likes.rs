use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_like_blog(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if post_id.is_empty() {
        return Err(ApiError::BadRequest("Post ID is required".into()));
    }

    let row: Option<(i32,)> = sqlx::query_as("SELECT likes_count FROM likes WHERE slug = $1")
        .bind(&post_id)
        .fetch_optional(&state.db)
        .await?;

    match row {
        Some((likes_count,)) => Ok(Json(json!({ "likes": likes_count }))),
        None => Err(ApiError::NotFound("Post not found".into())),
    }
}