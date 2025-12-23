use crate::http::AppState;
use crate::http::routes::articles::articles_list::articles_list::list_articles;
use crate::http::routes::articles::articles_feed::articles_feed::feed_articles;
use crate::http::routes::articles::get_article::get_article::get_article;
use crate::http::routes::articles::update_article::update_article::update_article;
use crate::http::routes::articles::delete_article::delete_article::delete_article;
use crate::http::routes::articles::favorite_article::favorite_article::favorite_article;
use crate::http::routes::articles::unfavorite_article::unfavorite_article::unfavorite_article;
use crate::http::routes::articles::create_article::create_article::create_article;

use axum::{ Router};
use axum::routing::{delete, get, post, put};

pub(crate) fn article_routes() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list_articles))
        .route("/articles", post(create_article))
        .route("/articles/feed", get(feed_articles))
        .route("/articles/{slug}", get(get_article))
        .route("/articles/{slug}", put(update_article))
        .route("/articles/{slug}", delete(delete_article))
        .route("/articles/{slug}/favorite", post(favorite_article))
        .route("/articles/{slug}/favorite", delete(unfavorite_article))
}