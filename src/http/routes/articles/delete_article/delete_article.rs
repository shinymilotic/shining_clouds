use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::info;

pub(crate) async fn delete_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
) -> Result<StatusCode, AppError> {
    info!(slug = %slug, "Delete article: {}", slug);

    state
        .article_service
        .delete_article(slug, auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}