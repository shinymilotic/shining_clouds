use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::article::{ArticleItem, ArticleResponse};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn get_article(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(slug): Path<Slug>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, "Get article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, auth.map(|u| u.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let article = ArticleItem::from_article_view(&article);

    Ok(Json(ArticleResponse { article }))
}