use crate::app_error::AppError;
use crate::domain::commands::add_comment_command::AddCommentCommand;
use crate::http::AppState;
use crate::http::dto::comment::{
    CommentItem, CommentResponse, CreateCommentRequest,
};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json};
use tracing::info;

pub(crate) async fn create_comment(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), AppError> {
    info!(user_id=%{auth.user_id}, payload=?payload, "Add comment to article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, Some(auth.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let command = AddCommentCommand::from_request(payload, article.id, auth.user_id);

    let comment_view = state
        .comment_service
        .add_comment(command, auth.user_id)
        .await?;

    let comment = CommentItem::from_comment_view(comment_view);

    Ok((StatusCode::CREATED, Json(CommentResponse { comment })))
}