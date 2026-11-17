use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::services::update_role::update_role;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UpdateRoleBody {
    #[serde(rename = "newRole")]
    pub new_role: String,
}

pub async fn update_user_role(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<Value>, ApiError> {
    update_role(&state.db, user_id, &body.new_role).await?;
    Ok(Json(json!({ "message": "User role updated successfully" })))
}