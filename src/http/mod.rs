pub(crate) mod dto;
pub(crate) mod extractors;
pub(crate) mod routes;

use axum::extract::Request;
use routes::users::user_routes;
use routes::comments::comment_routes;
use routes::profiles::profile_routes;
use routes::articles::article_routes;
use routes::tags::tag_routes;
use crate::{app_config::AppConfig};
use crate::domain::article_service::ArticleService;
use crate::domain::comment_service::CommentService;
use crate::domain::profile_service::ProfileService;
use crate::domain::tag_service::TagService;
use crate::domain::user_service::UserService;
use crate::utils::jwt::JwtHandler;
use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, MakeSpan, TraceLayer};
use tracing::Span;
#[derive(Clone)]
struct FilteringMakeSpan<'a> {
    inner: DefaultMakeSpan,
    exceptions: Vec<&'a str>,
}

impl<'a> FilteringMakeSpan<'a> {
    fn except_routes(exceptions: Vec<&'a str>) -> Self {
        Self {
            exceptions,
            inner: DefaultMakeSpan::new().level(tracing::Level::INFO),
        }
    }
}

impl<B> MakeSpan<B> for FilteringMakeSpan<'_> {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        if self.exceptions.contains(&request.uri().path()) {
            Span::none()
        } else {
            self.inner.make_span(request)
        }
    }
}

pub fn router(state: AppState) -> Router {
    let api_routes = Router::new()
        .merge(user_routes::user_routes())
        .merge(profile_routes::profile_routes())
        .merge(article_routes::article_routes())
        .merge(comment_routes::comment_routes())
        .merge(tag_routes::tag_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(FilteringMakeSpan::except_routes(vec!["/api/health"]))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    Router::new()
        .nest("/api", api_routes)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub article_service: ArticleService,
    pub comment_service: CommentService,
    pub tag_service: TagService,
    pub profile_service: ProfileService,
    pub jwt: JwtHandler,
}
