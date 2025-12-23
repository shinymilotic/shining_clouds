use crate::http::AppState;
use crate::http::routes::comments:: {
    create_comment::create_comment::create_comment,
    delete_comment::delete_comment::delete_comment,
    get_comments::get_comments::get_comments
};
use axum::routing::{delete, get, post};
use axum::{Router};

pub(crate) fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/articles/{slug}/comments", post(create_comment))
        .route("/articles/{slug}/comments", get(get_comments))
        .route("/articles/{slug}/comments/{id}", delete(delete_comment))
}