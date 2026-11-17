use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::services::delete_self_profile::delete_self_profile;
use crate::state::AppState;

pub async fn delete_self_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    delete_self_profile(&state.db, user_id).await?;

    Ok(Json(json!({ "message": "User deleted successfully" })))
}