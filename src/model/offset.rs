use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, ToSchema)]
pub struct Offset(u64);

impl Offset {
    pub fn new(limit: u64) -> Self {
        Offset(limit)
    }

    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}
