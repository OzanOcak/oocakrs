use axum::extract::Path;
use axum::Json;
use serde_json::{json, Value};
use std::fs;

use super::set_published;           // ← import from mod.rs

use crate::error::ApiError;

pub async fn unpublish_post(
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let file_path = format!("../frontend/src/content/{slug}.md");

    let file_content = fs::read_to_string(&file_path)
        .map_err(|e| ApiError::Internal(format!("Error reading file: {e}")))?;
    let updated = set_published(&file_content, false);
    fs::write(&file_path, updated)
        .map_err(|e| ApiError::Internal(format!("Error writing file: {e}")))?;

    Ok(Json(json!({
        "message": format!("Post {slug} unpublished successfully")
    })))
}
