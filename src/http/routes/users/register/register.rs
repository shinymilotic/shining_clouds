use crate::app_error::AppError;
use crate::domain::commands::register_command::RegisterCommand;
use crate::http::AppState;
use crate::http::dto::register::RegisterRequest;
use crate::http::dto::user::{UserData, UserResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json};
use tracing::info;

pub(crate) async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    info!(
        "Registration attempt for email: {}, username: {}, hash of password: {}",
        payload.user.email, payload.user.username, payload.user.password
    );

    let command = RegisterCommand::from_request(payload);

    let user = app_state.user_service.register_user(command).await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData {
        email: user.email,
        token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok((StatusCode::CREATED, Json(UserResponse { user })))
}
