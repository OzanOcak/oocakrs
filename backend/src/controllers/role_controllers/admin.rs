use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::services::count_users::count_users;
use crate::services::get_all_users::get_all_users;
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

    let users = get_all_users(&state.db, q.page, q.limit, &q.search_query, &q.search_criteria).await?;
    let total = count_users(&state.db, &q.search_query, &q.search_criteria).await?;

    Ok(Json(json!({ "users": users, "total": total })))
}