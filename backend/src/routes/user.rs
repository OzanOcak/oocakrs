use axum::routing::{delete, patch};
use axum::Router;

use crate::controllers::feature_controllers::user::{
    delete_self_user_profile, update_user_self_name,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/profile/{user_id}",
            delete(delete_self_user_profile::delete_self_user_profile),
        )
        .route(
            "/api/profile/{user_id}/name",
            patch(update_user_self_name::update_user_self_name),
        )
}