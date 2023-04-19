use crate::entities::Tweet;

#[axum::async_trait]
pub trait Tweets {
    async fn list(&self) -> Vec<Tweet>;
}
