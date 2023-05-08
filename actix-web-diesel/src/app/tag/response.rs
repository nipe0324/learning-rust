use super::model::Tag;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl From<Vec<Tag>> for TagsResponse {
    fn from(tags: Vec<Tag>) -> Self {
        let tags = tags.iter().map(move |tag| tag.name.to_string()).collect();
        Self { tags }
    }
}
