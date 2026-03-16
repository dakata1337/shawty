use std::{
    env,
    time::{Duration, Instant},
};

use dashmap::{DashMap, mapref::one::Ref};
use rand::RngExt;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct ShortUrl {
    #[allow(unused)]
    cleanup_at: Option<Instant>,
    shortened_url: String,
    original_url: String,
}

impl ShortUrl {
    pub fn new(original_url: &str, short_url: &str, duration: Option<Duration>) -> Self {
        Self {
            cleanup_at: duration.map(|x| Instant::now() + x),
            shortened_url: short_url.to_string(),
            original_url: original_url.to_string(),
        }
    }

    pub fn get_original_url(&self) -> &str {
        &self.original_url
    }
    pub fn get_shortended_url(&self) -> &str {
        &self.shortened_url
    }
    pub fn get_expiry_time(&self) -> Option<Instant> {
        self.cleanup_at
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AppState {
    shortened: DashMap<String, ShortUrl>,
}

#[allow(unused)]
impl AppState {
    const DEFAULT_URL_GEN_RETRY_ATTEMPTS: usize = 5;
    const DEFAULT_URL_GEN_LENGTH: usize = 5;

    pub fn new() -> Self {
        Self {
            shortened: DashMap::new(),
        }
    }
    fn generate_random_sequence(len: usize) -> String {
        let mut rng = rand::rng();
        let mut buf = String::with_capacity(len);
        for _ in 0..len {
            buf.push(rng.sample(rand::distr::Alphanumeric) as char);
        }
        buf
    }

    pub fn create_short_url(&self, url: &str, duration: Option<Duration>) -> Option<ShortUrl> {
        let url_gen_retry = env::var("URL_GEN_RETRY_ATTEMPTS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(Self::DEFAULT_URL_GEN_RETRY_ATTEMPTS);

        let url_gen_length = env::var("URL_GEN_LENGTH")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(Self::DEFAULT_URL_GEN_LENGTH);

        info!("len {}  retry {}", url_gen_length, url_gen_retry);

        for try_cnt in 0..url_gen_retry {
            let short_url_str = Self::generate_random_sequence(url_gen_length + try_cnt);

            // NOTE: There is a small probability this would happen BUT
            if short_url_str == "shorten" || self.shortened.get(&short_url_str).is_some() {
                warn!("WOW!? this guy should play the lottary");
                continue;
            }

            let short_url = ShortUrl::new(url, &short_url_str, duration);
            self.shortened.insert(short_url_str, short_url.clone());
            return Some(short_url);
        }
        None
    }

    pub fn lookup_url(&self, short_url: &str) -> Option<Ref<'_, String, ShortUrl>> {
        self.shortened.get(short_url)
    }

    pub fn purge_expired_urls(&self) -> usize {
        let mut removed = 0;

        let now = Instant::now();
        self.shortened
            .retain(|_, value| match value.get_expiry_time() {
                Some(v) if v < now => {
                    removed += 1;
                    false
                }
                _ => true,
            });

        removed
    }
}
