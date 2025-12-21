use crate::app_error::AppError;
use crate::domain::commands::update_article_command::UpdateArticleCommand;
use crate::http::AppState;
use crate::http::dto::article::{
    ArticleItem, ArticleResponse, UpdateArticleRequest,
};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn update_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, payload = ?payload , "Update article: {}", slug);

    let command = UpdateArticleCommand::from_request(payload, slug);

    let updated_article = state
        .article_service
        .update_article(command, auth.user_id)
        .await?;

    let article = ArticleItem::from_article_view(&updated_article);

    Ok(Json(ArticleResponse { article }))
}