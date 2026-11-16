use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

const VALID_ROLES: &[&str] = &["admin", "editor", "user"];

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
    if !VALID_ROLES.contains(&body.new_role.as_str()) {
        return Err(ApiError::BadRequest("Invalid role".into()));
    }

    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(&body.new_role)
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error updating role: {e}")))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("User not found".into()));
    }

    Ok(Json(json!({ "message": "User role updated successfully" })))
}