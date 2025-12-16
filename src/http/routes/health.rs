use crate::http::AppState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health",
    responses(
        (status = 200, description = "API is healthy")
    )
)]
pub(crate) async fn health_check() -> impl IntoResponse {
    Json::from("OK").into_response()
}
