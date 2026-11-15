mod app;
mod config;
mod db;
mod env;
mod logging;
mod middleware;
mod routes;
mod state;

use anyhow::Result;
use state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    env::load();
    logging::init();

    let config = Arc::new(config::Config::from_env()?);
    let pool = db::connect(&config.database_url()).await?;

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    let router = app::build(state, &config);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server is running on http://localhost:{}", config.port);

    axum::serve(listener, router).await?;
    Ok(())
}