mod url_handler;
use dotenvy::dotenv;
use tracing::info;
use url_handler::AppState;

use std::{env, sync::Arc, time::Duration};

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

    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    let state = Arc::new(AppState::new());

    spawn_purge_task(Arc::clone(&state));

    let app = Router::new()
        .route("/", get(homepage::homapage))
        .route("/shorten", post(shorten::shorten))
        .route("/{short_url}", get(redirect::redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
