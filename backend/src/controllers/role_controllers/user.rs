use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ProfileQuery {
    pub user_id: i32,
}

pub async fn user(
    State(state): State<AppState>,
    Query(q): Query<ProfileQuery>,
) -> Result<Json<Value>, ApiError> {
    let row = sqlx::query(
        r#"
        SELECT
            u.id,
            u.username,
            u.email,
            u.role,
            o.profile_picture
        FROM users u
        LEFT JOIN oauth_identities o ON u.id = o.user_id
        WHERE u.id = $1
        "#,
    )
    .bind(q.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error fetching data: {e}")))?;

    match row {
        Some(r) => {
            let profile = json!({
                "id": r.get::<i32, _>("id"),
                "username": r.get::<String, _>("username"),
                "email": r.get::<String, _>("email"),
                "role": r.get::<String, _>("role"),
                "profilePicture": r.get::<Option<String>, _>("profile_picture"),
            });
            Ok(Json(json!({ "profile": profile })))
        }
        None => Err(ApiError::NotFound("Profile not found.".into())),
    }
}