use crate::http::AppState;
use crate::http::routes::profiles:: {
    follow_user::follow_user::follow_user,
    get_profile::get_profile::get_profile,
    unfollow_user::unfollow_user::unfollow_user
};
use axum::routing::{delete, get, post};
use axum::{Router};

pub(crate) fn profile_routes() -> Router<AppState> {
    Router::new()
        .route("/profiles/{username}", get(get_profile))
        .route("/profiles/{username}/follow", post(follow_user))
        .route("/profiles/{username}/follow", delete(unfollow_user))
}
