use axum::extract::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

use std::env;
use tokio_postgres::NoTls;

use crate::repos_impl::TweetsImpl;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> Extension<RepoProvider> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    Extension(RepoProvider(pool))
}

#[derive(Clone)]
pub struct RepoProvider(ConnectionPool);

impl RepoProvider {
    pub fn tweets(&self) -> TweetsImpl {
        TweetsImpl { pool: &self.0 }
    }
}
