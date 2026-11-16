use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_comment_count(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if post_id.is_empty() {
        return Err(ApiError::BadRequest("Post ID is required".into()));
    }

    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM comments WHERE blog_id = $1")
        .bind(&post_id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(json!({ "count": row.0 })))
}