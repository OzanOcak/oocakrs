use axum::routing::{get, post};
use axum::Router;

use crate::controllers::feature_controllers::visitor_count::{
    get_visit_count, increase_visit_count,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/admin/get-visit-count",
            get(get_visit_count::get_visitor_count),
        )
        .route(
            "/api/increase-visit-count",
            post(increase_visit_count::increase_visitor_count),
        )
}