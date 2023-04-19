use tokio_postgres::Row;

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

    async fn find(&self, id: i32) -> Option<Tweet> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM tweets WHERE id = $1", &[&id])
            .await
            .unwrap();
        row.map(|r| r.into())
    }

    async fn create(&self, entity: &Tweet) {
        let conn = self.pool.get().await.unwrap();
        conn.execute(
            "INSERT INTO tweets (message, posted_at) VALUES ($1, $2)",
            &[&entity.message, &entity.posted_at],
        )
        .await
        .unwrap();
    }

    async fn delete(&self, entity: &Tweet) {
        let conn = self.pool.get().await.unwrap();
        if let Some(id) = entity.id() {
            if entity.is_deleted() {
                conn.execute("DELETE FROM tweets WHERE id = $1", &[&id])
                    .await
                    .unwrap();
            }
        }
    }
}

impl From<Row> for Tweet {
    fn from(row: Row) -> Self {
        Tweet::new(row.get("id"), row.get("message"), row.get("posted_at"))
    }
}
