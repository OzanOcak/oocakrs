use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::services::get_profile::get_user_profile;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ProfileQuery {
    pub user_id: Option<i32>,
}

pub async fn user(
    State(state): State<AppState>,
    Query(q): Query<ProfileQuery>,
) -> Result<Json<Value>, ApiError> {
    let user_id = q
        .user_id
        .ok_or_else(|| ApiError::BadRequest("Missing user_id".into()))?;

    match get_user_profile(&state.db, user_id).await? {
        Some(profile) => Ok(Json(json!({ "profile": profile }))),
        None => Err(ApiError::NotFound("Profile not found.".into())),
    }
}