use crate::app_error::AppError;
use crate::domain::commands::login_command::LoginCommand;
use crate::http::AppState;
use crate::http::dto::login::LoginRequest;
use crate::http::dto::user::{UserData, UserResponse};
use axum::extract::State;
use axum::{Json};
use tracing::info;

pub(crate) async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!("Login attempt for email: {}", payload.user.email);

    let command = LoginCommand::from_request(payload);

    let user = app_state.user_service.login_user(command).await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData::new(user, token);

    Ok(Json(UserResponse { user }))
}