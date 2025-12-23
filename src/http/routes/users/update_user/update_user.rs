use crate::app_error::AppError;
use crate::domain::commands::update_user_command::UpdateUserCommand;
use crate::http::AppState;
use crate::http::dto::user::{UpdateUserRequest, UserData, UserResponse};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::State;
use axum::{Json};
use tracing::info;

pub(crate) async fn update_user(
    State(app_state): State<AppState>,
    auth_user: AuthToken,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!(user_id = %{auth_user.user_id}, payload = ?payload, "Update user with id: {}", auth_user.user_id);

    let command = UpdateUserCommand::from_request(payload, auth_user.user_id);

    let user = app_state.user_service.update_user(command).await?;

    let user_date = UserData::new(user, auth_user.raw_token);

    Ok(Json(UserResponse { user: user_date }))
}
