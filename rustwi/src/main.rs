use dotenvy::dotenv;
use std::net::SocketAddr;

#[tokio::main] // main関数を非同期関数にするために必要
async fn main() {
    dotenv().expect(".env file not found");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();

    let app = rustwi::app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 13001));
    tracing::debug!("listing on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
