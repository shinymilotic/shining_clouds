use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::profile::{Profile, ProfileResponse};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::username::Username;
use axum::extract::{Path, State};
use axum::{Json};
use tracing::info;

pub(crate) async fn get_profile(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    let maybe_user_id = auth.as_ref().map(|u| u.user_id);

    info!(user_id = ?maybe_user_id, username = %username, "Get profile for username: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username.clone())
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let following = if let Some(user_id) = maybe_user_id {
        state.profile_service.is_following(user_id, user.id).await?
    } else {
        false
    };

    let profile = Profile::from_user(user, following);

    Ok(Json(ProfileResponse { profile }))
}