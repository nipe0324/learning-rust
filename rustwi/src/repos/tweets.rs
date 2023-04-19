use crate::entities::Tweet;

#[cfg_attr(test, mockall::automock)]
#[axum::async_trait]
pub trait Tweets {
    async fn list(&self) -> Vec<Tweet>;
    async fn find(&self, id: i32) -> Option<Tweet>;
    async fn create(&self, entity: &Tweet);
    async fn delete(&self, entity: &Tweet);
}
