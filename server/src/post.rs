use crate::error::AppError;
use crate::{
    database::{insert, TableType, ToSqlDebug},
    DB_CONNECTION,
};
use axum::Json;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{info, instrument};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MonitorData {
    #[serde(rename = "t")]
    temperature: f64,
    #[serde(rename = "h")]
    humidity: f64,
    #[serde(rename = "m")]
    motion: bool,
    #[serde(rename = "l")]
    light: f64,
}

#[instrument(level = "info")]
pub async fn monitor_post(Json(payload): Json<MonitorData>) -> Result<(), AppError> {
    info!("Received monitor post data...");
    let timestamp = Local::now().timestamp();
    let data_types = vec![
        TableType::Temperature,
        TableType::Humidity,
        TableType::Motion,
        TableType::Light,
    ]
    .into_iter();
    let payload_datas: Vec<Box<dyn ToSqlDebug>> = vec![
        Box::new(payload.temperature),
        Box::new(payload.humidity),
        Box::new(payload.motion),
        Box::new(payload.light),
    ];

    let data_types = data_types.zip(payload_datas.into_iter());
    for (t, d) in data_types {
        insert(timestamp, d, &DB_CONNECTION, t).await?;
    }
    info!("Monitor post data successfully inserted");
    return Ok(());
}
