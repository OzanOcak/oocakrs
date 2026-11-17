use axum::routing::{delete,get, patch, post};
use axum::Router;

use crate::controllers::feature_controllers::admin::{
    delete_user_profile, publish_post, unpublish_post, update_user_role,
};
use crate::controllers::role_controllers::{admin, admin_edit};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin", get(admin::admin))
        .route("/api/admin/users/{user_id}", get(admin_edit::admin_edit))
        .route(
            "/api/admin/{user_id}",
            delete(delete_user_profile::delete_user_profile),
        )
        .route(
            "/api/admin/{user_id}/role",
            patch(update_user_role::update_user_role),
        )
        .route(
            "/api/admin/blog/{slug}/publish",
            post(publish_post::publish_post),
        )
        .route(
            "/api/admin/blog/{slug}/unpublish",
            post(unpublish_post::unpublish_post),
        )
}