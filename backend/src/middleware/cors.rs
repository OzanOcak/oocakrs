use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn build(allowed_origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::HEAD,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::HeaderName::from_static("refresh-token-id"),
        ])
        .expose_headers([
            header::AUTHORIZATION,
            header::HeaderName::from_static("refresh-token-id"),
        ])
}