use crate::database::{self, TableType};
use crate::error::AppError;
use crate::period::Period;
use crate::DB_CONNECTION;
use axum::Json;
use chrono::{Days, Local};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

#[derive(Debug, Serialize, Deserialize)]
struct ClientRequest {
    period: Period,
    table_type: TableType,
}

#[instrument(level = "info")]
pub async fn send_data(
    Json(ClientRequest { period, table_type }): Json<ClientRequest>,
) -> Result<(), AppError> {
    info!("Received request for data...");

    let timestamp = Local::now();
    let timestamp = (timestamp - Days::new(period.amount_of_days())).timestamp();

    let data = database::select(&DB_CONNECTION, table_type, timestamp).await?;

    info!("Request succesfully fulfilled");
    return Ok(());
}
