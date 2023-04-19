use crate::entities::Tweet as TweetEntity;

pub struct Tweet {
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl From<TweetEntity> for Tweet {
    fn from(e: TweetEntity) -> Self {
        Tweet {
            name: "太郎".to_string(),
            message: e.message,
            posted_at: e.posted_at.format("%Y/%m/%d %H:%M").to_string(),
        }
    }
}
