// middleware/security.rs
use axum::http::{header, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn x_content_type_options() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    )
}

pub fn x_frame_options() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("SAMEORIGIN"),
    )
}

pub fn referrer_policy() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    )
}

pub fn hsts() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=15552000; includeSubDomains"),
    )
}