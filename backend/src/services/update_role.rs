use sqlx::PgPool;

use crate::error::ApiError;

const VALID_ROLES: &[&str] = &["user", "admin", "editor"];

pub async fn update_role(db: &PgPool, user_id: i32, new_role: &str) -> Result<(), ApiError> {
    if !VALID_ROLES.contains(&new_role) {
        return Err(ApiError::BadRequest("Invalid role".into()));
    }

    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(new_role)
        .bind(user_id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("User not found".into()));
    }

    Ok(())
}