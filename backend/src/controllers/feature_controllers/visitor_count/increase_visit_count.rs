use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn increase_visitor_count(
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    sqlx::query("UPDATE website_visits SET total_visits = total_visits + 1 WHERE id = 1")
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error increasing visitor count: {e}")))?;

    Ok(Json(json!({ "message": "Visitor count increased successfully" })))
}