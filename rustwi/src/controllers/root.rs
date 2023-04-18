use axum::{response::IntoResponse, routing, Router};

use crate::controllers::tweets;
use crate::response;
use crate::views::{Home, Tweet};

pub fn app() -> Router {
    Router::new()
        .route("/", routing::get(get))
        .nest("/tweets", tweets::tweets())
}

async fn get() -> impl IntoResponse {
    let tweets = (1..=20)
        .into_iter()
        .map(|_| Tweet {
            name: "太郎".to_string(),
            message: "こんにちは！".to_string(),
            posted_at: "2020-01-01 12:34".to_string(),
        })
        .collect();
    let home = Home { tweets };
    response::from_template(home)
}
