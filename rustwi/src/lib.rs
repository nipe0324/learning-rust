mod constants;
mod controllers;
mod database;
mod entities;
mod repos;
mod repos_impl;
mod request;
mod response;
mod services;
mod views;

pub use controllers::app;

pub async fn setup_session_store() {
    let database_url = constants::database_url();
    let store = async_sqlx_session::PostgresSessionStore::new(&database_url)
        .await
        .unwrap();
    store.migrate().await.unwrap();

    store.spawn_cleanup_task(std::time::Duration::from_secs(3600));
}
