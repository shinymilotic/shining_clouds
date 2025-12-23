use axum::extract::State;
use crate::http::AppState;
use crate::app_error::AppError;
use crate::domain::commands::get_feed_query::GetFeedQuery;
use crate::http::dto::article::{
    ArticleFeedListQuery, ArticleListItem, ArticlesResponse,
};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::{Query};
use axum::{Json};
use tracing::info;


pub(crate) async fn feed_articles(
    State(state): State<AppState>,
    auth: AuthToken,
    Query(params): Query<ArticleFeedListQuery>,
) -> Result<Json<ArticlesResponse>, AppError> {
    info!(params = ?params, "Get article feed");

    let query = GetFeedQuery::from_request(params, auth.user_id);

    let articles = state.article_service.get_feed(query).await?;

    let views: Vec<_> = articles
        .iter()
        .map(ArticleListItem::from_article_view)
        .collect();

    let articles_count = state
        .article_service
        .count_feed_articles(auth.user_id)
        .await?;
    Ok(Json(ArticlesResponse {
        articles: views,
        articles_count,
    }))
}