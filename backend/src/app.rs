use crate::config::Config;
use crate::state::AppState;
use crate::{middleware, routes};
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;

pub fn build(state: AppState, config: &Arc<Config>) -> Router {
    let cors = middleware::cors::build(&config.allowed_origins);

    let security = ServiceBuilder::new()
        .layer(middleware::security::x_content_type_options())
        .layer(middleware::security::x_frame_options())
        .layer(middleware::security::referrer_policy())
        .layer(middleware::security::hsts());

    routes::router()
        .layer(security)
        .layer(cors)
        .with_state(state)
}