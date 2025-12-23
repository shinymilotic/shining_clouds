use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::comment_id::CommentId;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::info;

pub(crate) async fn delete_comment(
    State(state): State<AppState>,
    auth: AuthToken,
    Path((slug, comment_id)): Path<(Slug, CommentId)>,
) -> Result<StatusCode, AppError> {
    info!(user_id=%{auth.user_id}, slug = %slug, comment_id = %comment_id, "Delete comment {} from article: {}", comment_id, slug);

    state
        .comment_service
        .delete_comment(comment_id, auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
