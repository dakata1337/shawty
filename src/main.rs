mod url_handler;
use tracing::info;
use url_handler::AppState;

use std::{sync::Arc, time::Duration};

mod routes;
use routes::{homepage, redirect, shorten};

use axum::{
    Router,
    routing::{get, post},
};

fn spawn_purge_task(state: Arc<AppState>) {
    tokio::spawn(async move {
        loop {
            std::thread::sleep(Duration::from_secs(10));

            let purged_count = state.purge_expired_urls();
            info!("Purged {} expired short URLs", purged_count);
        }
    });
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = Arc::new(AppState::new());

    spawn_purge_task(Arc::clone(&state));

    let app = Router::new()
        .route("/", get(homepage::homapage))
        .route("/shorten", post(shorten::shorten))
        .route("/{short_url}", get(redirect::redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("192.168.1.7:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
