use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::profile::{Profile, ProfileResponse};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::username::Username;
use axum::extract::{Path, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn follow_user(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!(user_id = %{auth.user_id}, username = %username, "Follow user: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    state
        .profile_service
        .follow_user(auth.user_id, user.id)
        .await?;

    let profile = Profile::from_user(user, true);

    Ok(Json(ProfileResponse { profile }))
}
