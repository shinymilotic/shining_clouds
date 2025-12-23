use crate::http::AppState;
use crate::http::routes::tags::get_tags::get_tags::get_tags;
use axum::routing::get;
use axum::{ Router};

pub(crate) fn tag_routes() -> Router<AppState> {
    Router::new().route("/tags", get(get_tags))
}
