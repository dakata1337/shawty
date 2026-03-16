use std::{sync::Arc, time::Duration};

use serde::Deserialize;

use axum::{Form, extract::State, response::Html};
use tracing::info;
use url::Url;

use crate::url_handler::AppState;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
    expiry: Option<String>,
}

pub async fn shorten(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<ShortenRequest>,
) -> Html<String> {
    let trimmed = payload.url.trim();
    if trimmed.is_empty() {
        return Html("<div class='error'>Invalid URL: empty</div>".into());
    }

    match Url::parse(trimmed) {
        Ok(parsed) => {
            if parsed.scheme() != "http" && parsed.scheme() != "https" {
                return Html(
                    "<div class='error'>Invalid URL: must start with http:// or https://</div>"
                        .into(),
                );
            }
        }
        Err(_) => {
            return Html("<div class='error'>Invalid URL format</div>".into());
        }
    }

    let expiry_duration: Duration = match payload.expiry.as_deref() {
        Some("1h") => Duration::from_hours(1),
        Some("6h") => Duration::from_hours(6),
        Some("3d") => Duration::from_hours(24 * 3),
        Some("7d") => Duration::from_hours(24 * 7),
        Some("24h") | None => Duration::from_hours(24), // default
        Some(_) => {
            return Html("<div class='error'>Invalid expiry selection</div>".into());
        }
    };

    let short = state.create_short_url(trimmed, Some(expiry_duration));
    match short {
        Some(short) => {
            let short_url = short.get_shortended_url();
            info!(
                "generated {} -> {} ({:?})",
                short_url,
                short.get_original_url(),
                expiry_duration
            );
            Html(format!(
                include_str!("../../templates/shorten/success.html"),
                short_url
            ))
        }
        None => Html("<div class='error'>Unable to create short url</div>".into()),
    }
}
