use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main] // main関数を非同期関数にするために必要
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listing on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    let tweets = (1..=20)
        .into_iter()
        .map(|_| TweetView {
            name: "太郎".to_string(),
            message: "こんにちは！".to_string(),
            posted_at: "2020-01-01 12:34".to_string(),
        })
        .collect();
    let home = Home { tweets };
    Html(home.render().unwrap()).into_response()
}

struct TweetView {
    name: String,
    message: String,
    posted_at: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    tweets: Vec<TweetView>,
}
