use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use crate::error::ApiError;

pub async fn get_all_users(
    db: &PgPool,
    page: i64,
    limit: i64,
    search_query: &str,
    search_criteria: &str,
) -> Result<Vec<Value>, ApiError> {
    let offset = (page - 1) * limit;
    let like_pattern = format!("%{search_query}%");

    let rows = match (search_query.is_empty(), search_criteria) {
        (true, _) => {
            sqlx::query(
                "SELECT id, username, email, password, role FROM users
                 ORDER BY id LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?
        }
        (false, "name") => {
            sqlx::query(
                "SELECT id, username, email, password, role FROM users
                 WHERE username ILIKE $1 ORDER BY id LIMIT $2 OFFSET $3",
            )
            .bind(&like_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?
        }
        (false, "email") => {
            sqlx::query(
                "SELECT id, username, email, password, role FROM users
                 WHERE email ILIKE $1 ORDER BY id LIMIT $2 OFFSET $3",
            )
            .bind(&like_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?
        }
        (false, "role") => {
            sqlx::query(
                "SELECT id, username, email, password, role FROM users
                 WHERE role = $1 ORDER BY id LIMIT $2 OFFSET $3",
            )
            .bind(search_query)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?
        }
        (false, _) => {
            sqlx::query(
                "SELECT id, username, email, password, role FROM users
                 ORDER BY id LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?
        }
    };

    Ok(rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<i32, _>("id"),
                "username": r.get::<String, _>("username"),
                "email": r.get::<String, _>("email"),
                "password": r.get::<Option<String>, _>("password"),
                "role": r.get::<String, _>("role"),
            })
        })
        .collect())
}