use axum::{extract::Extension, response::IntoResponse, routing, Router};
use chrono::{DateTime, Utc};

use crate::controllers::tweets;
use crate::database::{self, ConnectionPool};
use crate::response;
use crate::views::{Home, Tweet};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/tweets", tweets::tweets())
        .layer(database_layer)
}

async fn get(pool: Extension<ConnectionPool>) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();
    let rows = conn
        .query("SELECT * FROM tweets ORDER BY posted_at DESC", &[])
        .await
        .unwrap();
    let tweets = rows
        .into_iter()
        .map(|r| Tweet {
            name: "太郎".to_string(),
            message: r.get("message"),
            posted_at: r
                .get::<&str, DateTime<Utc>>("posted_at")
                .format("%Y/%m/%d %H:%M")
                .to_string(),
        })
        .collect();

    let home = Home { tweets };
    response::from_template(home)
}
