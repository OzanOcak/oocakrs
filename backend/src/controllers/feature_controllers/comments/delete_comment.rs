use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let exists: Option<(i32,)> = sqlx::query_as("SELECT id FROM comments WHERE id = $1")
        .bind(comment_id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(ApiError::NotFound("Comment not found".into()));
    }

    sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(comment_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "message": "Comment deleted successfully" })))
}