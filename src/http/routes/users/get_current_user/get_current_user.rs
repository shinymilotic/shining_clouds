use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::user::{UserData, UserResponse};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::State;
use axum::{Json};
use tracing::info;

pub(crate) async fn get_current_user(
    State(app_state): State<AppState>,
    auth_user: AuthToken,
) -> Result<Json<UserResponse>, AppError> {
    info!(user_id = %{auth_user.user_id}, "Get current user with id: {}", auth_user.user_id);

    let user = app_state
        .user_service
        .get_user_by_id(auth_user.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = UserData {
        email: user.email,
        token: auth_user.raw_token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok(Json(UserResponse { user }))
}