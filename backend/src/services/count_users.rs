use sqlx::PgPool;

use crate::error::ApiError;

pub async fn count_users(
    db: &PgPool,
    search_query: &str,
    search_criteria: &str,
) -> Result<i64, ApiError> {
    let like_pattern = format!("%{search_query}%");

    let count: (i64,) = match (search_query.is_empty(), search_criteria) {
        (true, _) => sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(db)
            .await?,

        (false, "name") => {
            sqlx::query_as("SELECT COUNT(*) FROM users WHERE username ILIKE $1")
                .bind(&like_pattern)
                .fetch_one(db)
                .await?
        }
        (false, "email") => {
            sqlx::query_as("SELECT COUNT(*) FROM users WHERE email ILIKE $1")
                .bind(&like_pattern)
                .fetch_one(db)
                .await?
        }
        (false, "role") => {
            sqlx::query_as("SELECT COUNT(*) FROM users WHERE role = $1")
                .bind(search_query)
                .fetch_one(db)
                .await?
        }
        (false, _) => sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(db)
            .await?,
    };

    Ok(count.0)
}