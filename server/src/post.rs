use axum::{response::IntoResponse, Json};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MonitorData {
    #[serde(rename = "t")]
    temperature: f64,
    #[serde(rename = "h")]
    humidity: f64,
    #[serde(rename = "m")]
    motion: bool,
    #[serde(rename = "l")]
    light: bool,
}

pub async fn post(Json(payload): Json<MonitorData>) -> impl IntoResponse {
    let timestamp = Local::now().timestamp();
}
