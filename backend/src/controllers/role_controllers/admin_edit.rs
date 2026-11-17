use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::services::get_profile::get_user_profile;
use crate::state::AppState;

pub async fn admin_edit(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
   
    match get_user_profile(&state.db, user_id).await? {
        Some(user) => Ok(Json(json!({ "user": user }))),
        None => Err(ApiError::NotFound("User not found".into())),
    }
}