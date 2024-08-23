use anyhow::Result;
use axum::Router;
use std::sync::OnceLock;
use tokio_rusqlite::Connection;
use tracing::info;
use tracing::Level;

mod cors;
mod database;
mod post;

const DB_PATH: &str = "data.db";
const BIND_SOCK_ADDR: &str = "0.0.0.0:8138";

static DB_CONNECTION: OnceLock<Connection> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Tracing initialized");

    let conn = database::init_database(DB_PATH).await?;
    DB_CONNECTION.set(conn).unwrap();

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
