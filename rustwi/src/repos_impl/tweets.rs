use crate::database::ConnectionPool;
use crate::entities::Tweet;
use crate::repos::Tweets;

pub struct TweetsImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Tweets for TweetsImpl<'a> {
    async fn list(&self) -> Vec<Tweet> {
        let conn = self.pool.get().await.unwrap();
        let rows = conn
            .query("SELECT * FROM tweets ORDER BY posted_at DESC", &[])
            .await
            .unwrap();
        rows.into_iter()
            .map(|r| Tweet::new(r.get("id"), r.get("message"), r.get("posted_at")))
            .collect()
    }
}
