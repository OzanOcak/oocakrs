// backend/src/services/get_profile.rs
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use crate::error::ApiError;

pub async fn get_user_profile(
    db: &PgPool,
    user_id: i32,
) -> Result<Option<Value>, ApiError> {
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
    .fetch_optional(db)
    .await
    .map_err(|e| ApiError::Internal(format!("Error fetching user profile: {e}")))?;

    Ok(row.map(|r| {
        json!({
            "id": r.get::<i32, _>("id"),
            "username": r.get::<String, _>("username"),
            "email": r.get::<String, _>("email"),
            "role": r.get::<String, _>("role"),
            "profilePicture": r.get::<Option<String>, _>("profile_picture"),
        })
    }))
}