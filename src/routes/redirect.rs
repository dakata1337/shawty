use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use tracing::info;

use crate::url_handler::AppState;

pub async fn redirect(
    State(state): State<Arc<AppState>>,
    Path(short_url): Path<String>,
) -> impl IntoResponse {
    let maybe_url = state.lookup_url(&short_url);

    match maybe_url {
        Some(url) => {
            info!(
                "user gave {} returning -> {:?}",
                short_url,
                url.get_original_url()
            );
            Redirect::temporary(url.get_original_url()).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Html(format!(include_str!("../../templates/error.html"), "404 - this page does not exist")),
        )
            .into_response(),
    }
}
