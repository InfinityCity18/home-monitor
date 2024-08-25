use crate::database::{self, TableType};
use crate::error::AppError;
use crate::period::Period;
use crate::DB_CONNECTION;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Days, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, instrument};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRequest {
    period: Period,
    table_type: TableType,
}

#[instrument(level = "info")]
pub async fn send_data(
    Json(ClientRequest { period, table_type }): Json<ClientRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Received request for data...");

    let timestamp = Local::now();
    let timestamp = (timestamp - Days::new(period.amount_of_days())).timestamp();

    let v: Value;
    match table_type {
        TableType::Temperature => {
            v = serde_json::to_value(
                database::select::<f64>(&DB_CONNECTION, table_type, timestamp).await?,
            )?;
        }
        TableType::Humidity => {
            v = serde_json::to_value(
                database::select::<f64>(&DB_CONNECTION, table_type, timestamp).await?,
            )?;
        }
        TableType::Motion => {
            v = serde_json::to_value(
                database::select::<i64>(&DB_CONNECTION, table_type, timestamp).await?,
            )?;
        }
        TableType::Light => {
            v = serde_json::to_value(
                database::select::<i64>(&DB_CONNECTION, table_type, timestamp).await?,
            )?;
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse()?);

    info!("Request succesfully fulfilled");
    return Ok((headers, Json(v)));
}
