use crate::http::AppState;
use crate::http::routes::articles::list_articles;
use crate::http::routes::articles::articles_feed;
use crate::http::routes::articles::get_article;
use crate::http::routes::articles::update_article;
use crate::http::routes::articles::delete_article;
use crate::http::routes::articles::favorite_article;
use crate::http::routes::articles::unfavorite_article;
use crate::http::routes::articles::create_article;

use axum::{ Router};
use axum::routing::{delete, get, post, put};

pub(crate) fn article_routes() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list_articles::list_articles))
        .route("/articles", post(create_article::create_article))
        .route("/articles/feed", get(articles_feed::feed_articles))
        .route("/articles/{slug}", get(get_article::get_article))
        .route("/articles/{slug}", put(update_article::update_article))
        .route("/articles/{slug}", delete(delete_article::delete_article))
        .route("/articles/{slug}/favorite", post(favorite_article::favorite_article))
        .route("/articles/{slug}/favorite", delete(unfavorite_article::unfavorite_article))
}