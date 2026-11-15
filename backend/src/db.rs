use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("failed to create Postgres pool")?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .context("Postgres ping failed")?;

    println!("Connected to PostgreSQL database.");
    Ok(pool)
}