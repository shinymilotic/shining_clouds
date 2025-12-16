use crate::model::values::tag_name::TagName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TagsResponse {
    pub tags: Vec<TagName>,
}
