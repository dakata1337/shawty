use std::sync::Arc;

use axum::{extract::State, response::Html};

use crate::url_handler::AppState;

pub async fn homapage(State(_state): State<Arc<AppState>>) -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}
