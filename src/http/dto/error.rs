use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub errors: ErrorBody,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorBody {
    pub body: Vec<String>,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            errors: ErrorBody {
                body: vec![message],
            },
        }
    }
}
