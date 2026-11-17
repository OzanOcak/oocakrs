use sqlx::PgPool;

use crate::error::ApiError;

pub async fn delete_self_profile(db: &PgPool, user_id: i32) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(db)
        .await?;

    Ok(())
}