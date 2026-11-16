use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_comments(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT ON (c.id)
            c.id,
            c.content,
            c.user_id,
            c.like_count,
            c.created_at,
            u.username,
            o.profile_picture
        FROM comments c
        LEFT JOIN users u ON c.user_id = u.id
        LEFT JOIN oauth_identities o ON u.id = o.user_id
        WHERE c.blog_id = $1
        ORDER BY c.id
        "#,
    )
    .bind(&post_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error fetching comments: {e}")))?;

    let comments: Vec<Value> = rows
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "content": row.get::<String, _>("content"),
                "userId": row.get::<i32, _>("user_id"),
                "likeCount": row.get::<i32, _>("like_count"),
                "createdAt": row.get::<chrono::NaiveDateTime, _>("created_at"),
                "username": row.get::<Option<String>, _>("username"),
                "profilePicture": row.get::<Option<String>, _>("profile_picture"),
            })
        })
        .collect();

    Ok(Json(json!({ "comments": comments })))
}