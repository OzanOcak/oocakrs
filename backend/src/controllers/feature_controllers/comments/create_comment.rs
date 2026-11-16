use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCommentBody {
    pub content: String,
    pub user_id: i32,
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
    Json(body): Json<CreateCommentBody>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let row = sqlx::query(
        r#"
        INSERT INTO comments (content, user_id, blog_id, like_count)
        VALUES ($1, $2, $3, 0)
        RETURNING id, content, user_id, blog_id, like_count, created_at
        "#,
    )
    .bind(&body.content)
    .bind(body.user_id)
    .bind(&post_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error creating comment: {e}")))?;

    let comment = json!({
        "id": row.get::<i32, _>("id"),
        "content": row.get::<String, _>("content"),
        "userId": row.get::<i32, _>("user_id"),
        "blogId": row.get::<String, _>("blog_id"),
        "likeCount": row.get::<i32, _>("like_count"),
        "createdAt": row.get::<chrono::NaiveDateTime, _>("created_at"),
    });

    Ok((StatusCode::CREATED, Json(json!({ "comment": comment }))))
}