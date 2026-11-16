use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_visitor_count(
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT total_visits FROM website_visits WHERE id = 1")
            .fetch_optional(&state.db)
            .await
            .map_err(|e| ApiError::Internal(format!("Error fetching visitor count: {e}")))?;

    match row {
        Some((total_visits,)) => Ok(Json(json!({ "total_visits": total_visits }))),
        None => Err(ApiError::Internal("Visitor count row not found".into())),
    }
}