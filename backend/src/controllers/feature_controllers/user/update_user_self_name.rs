use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UpdateNameBody {
    #[serde(rename = "newName")]
    pub new_name: String,
}

pub async fn update_user_self_name(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UpdateNameBody>,
) -> Result<Json<Value>, ApiError> {
    sqlx::query("UPDATE users SET username = $1 WHERE id = $2")
        .bind(&body.new_name)
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Error updating role: {e}")))?;

    Ok(Json(json!({ "message": "User name updated successfully" })))
}