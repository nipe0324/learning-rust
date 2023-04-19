use crate::entities::Tweet;

#[cfg_attr(test, mockall::automock)]
#[axum::async_trait]
pub trait Tweets {
    async fn list(&self) -> Vec<Tweet>;
    async fn store(&self, entity: &Tweet);
}
