use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn unlike_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let row = sqlx::query(
        r#"
        UPDATE comments SET like_count = like_count - 1
        WHERE id = $1
        RETURNING id, content, user_id, blog_id, like_count, created_at
        "#,
    )
    .bind(comment_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error unliking comment: {e}")))?;

    match row {
        Some(row) => {
            let comment = json!({
                "id": row.get::<i32, _>("id"),
                "content": row.get::<String, _>("content"),
                "userId": row.get::<i32, _>("user_id"),
                "blogId": row.get::<String, _>("blog_id"),
                "likeCount": row.get::<i32, _>("like_count"),
                "createdAt": row.get::<chrono::NaiveDateTime, _>("created_at"),
            });
            Ok(Json(json!({ "comment": comment })))
        }
        None => Err(ApiError::NotFound("Comment not found".into())),
    }
}