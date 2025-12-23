use crate::app_error::AppError;
use crate::domain::commands::list_articles_query::ListArticlesQuery;
use crate::http::AppState;
use crate::http::dto::article::{
    ArticleListItem, ArticleListQuery, ArticlesResponse,
};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::{ Query, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn list_articles(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Query(params): Query<ArticleListQuery>,
) -> Result<Json<ArticlesResponse>, AppError> {
    info!(params = ?params, "List articles with filters");

    let query = ListArticlesQuery::from_request(params);
    let user_id = auth.as_ref().map(|u| u.user_id);

    let articles = state
        .article_service
        .list_articles(query.clone(), user_id)
        .await?;
    let articles_count = state.article_service.count_articles(query, user_id).await?;

    let views: Vec<_> = articles
        .iter()
        .map(ArticleListItem::from_article_view)
        .collect();

    Ok(Json(ArticlesResponse {
        articles: views,
        articles_count,
    }))
}
