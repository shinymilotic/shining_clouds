use crate::http::AppState;
use crate::http::routes::users::get_current_user::get_current_user::get_current_user;
use crate::http::routes::users::login::login::login;
use crate::http::routes::users::register::register::register;
use crate::http::routes::users::update_user::update_user::update_user;
use axum::routing::{get, post, put};
use axum::{ Router};

pub(crate) fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
        .route("/user", get(get_current_user))
        .route("/user", put(update_user))
}
