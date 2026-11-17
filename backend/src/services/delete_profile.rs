use sqlx::PgPool;

use crate::error::ApiError;

pub async fn delete_profile(db: &PgPool, user_id: i32) -> Result<(), ApiError> {
    let exists: Option<(i32,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;

    if exists.is_none() {
        return Err(ApiError::NotFound("User not found".into()));
    }

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(db)
        .await?;

    Ok(())
}