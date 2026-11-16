use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn delete_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error deleting user: {e}")))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("User not found".into()));
    }

    Ok(Json(json!({ "message": "User deleted successfully" })))
}