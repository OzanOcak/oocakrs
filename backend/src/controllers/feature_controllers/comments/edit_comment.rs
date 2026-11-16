use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct EditCommentBody {
    pub content: String,
}

pub async fn edit_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<i32>,
    Json(body): Json<EditCommentBody>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query("UPDATE comments SET content = $1 WHERE id = $2")
        .bind(&body.content)
        .bind(comment_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error updating comment: {e}")))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Comment not found".into()));
    }

    Ok(Json(json!({ "message": "Comment updated successfully" })))
}