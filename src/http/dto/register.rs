use crate::model::values::email::Email;
use crate::model::values::password::Password;
use crate::model::values::username::Username;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub user: RegisterUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterUser {
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
