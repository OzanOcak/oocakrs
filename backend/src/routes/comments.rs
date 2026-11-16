use axum::routing::{delete, get, post};
use axum::Router;

use crate::controllers::feature_controllers::comments::{
    create_comment, delete_comment, edit_comment, fetch_comment_count, fetch_comments,
    like_comment, unlike_comment,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/posts/{post_id}/comments",
            post(create_comment::create_comment).get(fetch_comments::get_comments),
        )
        .route(
            "/api/blogpost/{post_id}/commentcount",
            get(fetch_comment_count::get_comment_count),
        )
        .route(
            "/api/comments/{comment_id}/likecomment",
            post(like_comment::like_comment),
        )
        .route(
            "/api/comments/{comment_id}/unlikecomment",
            post(unlike_comment::unlike_comment),
        )
        .route(
            "/api/comments/{comment_id}/reply",
            post(create_comment::create_comment), // Express reuses createComment for reply
        )
        .route(
            "/api/comments/{comment_id}",
            delete(delete_comment::delete_comment).patch(edit_comment::edit_comment),
        )
}