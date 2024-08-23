use axum::http::HeaderMap;
use tracing::{instrument, trace};

use crate::error::AppError;

#[instrument(level = "trace")]
pub async fn cors() -> Result<HeaderMap, AppError> {
    trace!("Sending CORS headers");
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse()?);
    headers.insert("Access-Control-Allow-Headers", "*".parse()?);
    Ok(headers)
}
