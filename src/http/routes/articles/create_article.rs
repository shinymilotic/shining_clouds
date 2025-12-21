use crate::domain::commands::create_article_command::CreateArticleCommand;
use crate::http::AppState;
use crate::http::dto::article::{
    ArticleItem, ArticleResponse, CreateArticleRequest,
};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::{State};
use axum::http::StatusCode;
use axum::{Json};
use tracing::info;
use crate::app_error::AppError;

pub(crate) async fn create_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<(StatusCode, Json<ArticleResponse>), AppError> {
    info!(payload = ?payload, "Create article");

    let command = CreateArticleCommand::from_request(payload, auth.user_id);

    let article_view = state.article_service.create_article(command).await?;

    let article = ArticleItem::from_article_view(&article_view);

    Ok((StatusCode::CREATED, Json(ArticleResponse { article })))
}