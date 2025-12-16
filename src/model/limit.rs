use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema)]
pub struct Limit(u64);

impl Limit {
    pub fn new(limit: u64) -> Self {
        Limit(limit)
    }

    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

impl Default for Limit {
    fn default() -> Self {
        Limit(50)
    }
}
