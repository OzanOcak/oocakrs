use axum::routing::{get, post};
use axum::Router;

use crate::controllers::likes::{get_likes, like, unlike};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/blogpost/{post_id}/bloglikes",
            get(get_likes::get_like_blog),
        )
        .route("/api/blogpost/{post_id}/likeblog", post(like::like_blog))
        .route(
            "/api/blogpost/{post_id}/unlikeblog",
            post(unlike::unlike_blog),
        )
}