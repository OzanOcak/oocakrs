use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use std::fs;

use super::set_published; // ← import from mod.rs

use crate::error::ApiError;
use crate::state::AppState;

pub async fn publish_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let file_path = format!("../frontend/src/content/{slug}.md");

    // Read the markdown file and flip published: true
    let file_content = fs::read_to_string(&file_path)
        .map_err(|e| ApiError::Internal(format!("Error reading file: {e}")))?;
    let updated = set_published(&file_content, true);
    fs::write(&file_path, updated)
        .map_err(|e| ApiError::Internal(format!("Error writing file: {e}")))?;

    // Insert default likes row (no-op if exists)
    sqlx::query("INSERT INTO likes (slug, likes_count) VALUES ($1, 0) ON CONFLICT DO NOTHING")
        .bind(&slug)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error inserting likes: {e}")))?;

    Ok(Json(json!({
        "message": format!("Post {slug} published successfully")
    })))
}
