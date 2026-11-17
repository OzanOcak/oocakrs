use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn admin_edit(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
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
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error fetching user profile: {e}")))?;

    match row {
        Some(r) => {
            let user = json!({
                "id": r.get::<i32, _>("id"),
                "username": r.get::<String, _>("username"),
                "email": r.get::<String, _>("email"),
                "role": r.get::<String, _>("role"),
                "profilePicture": r.get::<Option<String>, _>("profile_picture"),
            });
            Ok(Json(json!({ "user": user })))
        }
        None => Err(ApiError::NotFound("User not found".into())),
    }
}