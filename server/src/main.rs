use anyhow::Result;
use tokio_rusqlite::Connection;
use tracing::info;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

const DB_PATH: &str = "data.db";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Tracing initialized");

    info!("Trying to connect to database...");
    let conn = Connection::open(DB_PATH).await?;
    Ok(())
}
