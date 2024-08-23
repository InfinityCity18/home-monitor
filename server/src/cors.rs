use axum::{http::HeaderMap, response::IntoResponse};
use tracing::{instrument, trace};

#[instrument(level = "trace")]
pub async fn cors() -> impl IntoResponse {
    trace!("Sending CORS headers");
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap());
    headers
}
