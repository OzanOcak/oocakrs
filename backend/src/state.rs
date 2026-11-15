use std::sync::Arc;
use sqlx::PgPool;
use crate::config::Config;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}