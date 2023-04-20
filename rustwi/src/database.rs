use axum::extract::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

use tokio_postgres::NoTls;

use crate::constants::database_url;
use crate::repos_impl::{AccountsImpl, TweetsImpl};

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> Extension<RepoProvider> {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    Extension(RepoProvider(pool))
}

#[derive(Clone)]
pub struct RepoProvider(ConnectionPool);

impl RepoProvider {
    pub fn accounts(&self) -> AccountsImpl {
        AccountsImpl { pool: &self.0 }
    }

    pub fn tweets(&self) -> TweetsImpl {
        TweetsImpl { pool: &self.0 }
    }
}
