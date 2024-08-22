use std::sync::OnceLock;

use anyhow::Result;
use tokio_rusqlite::Connection;
use tracing::info;
use tracing::Level;

static DB_CONNECTION: OnceLock<Connection> = OnceLock::new();

mod database;

const DB_PATH: &str = "data.db";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Tracing initialized");

    let conn = database::init_database(DB_PATH).await?;
    let _ = DB_CONNECTION.set(conn);

    Ok(())
}
