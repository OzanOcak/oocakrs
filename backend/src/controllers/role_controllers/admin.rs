use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AdminListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub search_query: String,
    #[serde(default = "default_criteria")]
    pub search_criteria: String,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 10 }
fn default_criteria() -> String { "name".into() }

pub async fn admin(
    State(state): State<AppState>,
    Query(q): Query<AdminListQuery>,
) -> Result<Json<Value>, ApiError> {
    if q.page < 1 || q.limit < 1 {
        return Err(ApiError::BadRequest("Invalid page or limit value".into()));
    }

    let offset = (q.page - 1) * q.limit;
    let like_pattern = format!("%{}%", q.search_query);

    let (rows, total): (Vec<sqlx::postgres::PgRow>, i64) =
        match q.search_criteria.as_str() {
            "email" => {
                let rows = sqlx::query(
                    "SELECT id, username, email, role
                     FROM users
                     WHERE email ILIKE $1
                     ORDER BY id
                     LIMIT $2 OFFSET $3",
                )
                .bind(&like_pattern)
                .bind(q.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?;

                let (count,): (i64,) =
                    sqlx::query_as("SELECT COUNT(*) FROM users WHERE email ILIKE $1")
                        .bind(&like_pattern)
                        .fetch_one(&state.db)
                        .await?;

                (rows, count)
            }
            _ => {
                let rows = sqlx::query(
                    "SELECT id, username, email, role
                     FROM users
                     WHERE username ILIKE $1
                     ORDER BY id
                     LIMIT $2 OFFSET $3",
                )
                .bind(&like_pattern)
                .bind(q.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?;

                let (count,): (i64,) =
                    sqlx::query_as("SELECT COUNT(*) FROM users WHERE username ILIKE $1")
                        .bind(&like_pattern)
                        .fetch_one(&state.db)
                        .await?;

                (rows, count)
            }
        };

    let users: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<i32, _>("id"),
                "username": r.get::<String, _>("username"),
                "email": r.get::<String, _>("email"),
                "role": r.get::<String, _>("role"),
            })
        })
        .collect();

    Ok(Json(json!({ "users": users, "total": total })))
}