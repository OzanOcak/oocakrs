use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::services::delete_profile::delete_profile;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn delete_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    delete_profile(&state.db, user_id).await?;
    Ok(Json(json!({ "message": "User deleted successfully" })))
}