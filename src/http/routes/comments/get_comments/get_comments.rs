use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::comment::{
    CommentItem, CommentsResponse,
};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn get_comments(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(slug): Path<Slug>,
) -> Result<Json<CommentsResponse>, AppError> {
    let maybe_user_id = auth.as_ref().map(|a| a.user_id);
    info!(slug = %slug, user_id = ?maybe_user_id, "Get comments for article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, None)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let comment_views = state
        .comment_service
        .get_comments(article.id, maybe_user_id)
        .await?;

    let comments = comment_views
        .into_iter()
        .map(CommentItem::from_comment_view)
        .collect();

    Ok(Json(CommentsResponse { comments }))
}