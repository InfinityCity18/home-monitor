use std::sync::OnceLock;

use anyhow::Result;
use tokio_rusqlite::Connection;
use tracing::info;

enum TableType {
    Temperature,
    Humidity,
    Motion,
    Light,
}

impl TableType {
    pub fn into_text(&self) -> &str {
        match self {
            TableType::Temperature => "temperature",
            TableType::Humidity => "humidity",
            TableType::Motion => "motion",
            TableType::Light => "light",
        }
    }
    pub fn column_type(&self) -> &str {
        match self {
            TableType::Temperature => "temp",
            TableType::Humidity => "humd",
            TableType::Motion => "detected_motion",
            TableType::Light => "is_light",
        }
    }
}

pub async fn insert(conn: OnceLock<Connection>, type: TableType) -> Result<()> {
    let conn = conn.get().unwrap();

    conn.call(|conn| conn.execute("", params))
}

pub async fn init_database(path: &str) -> Result<Connection> {
    info!("Connecting to database...");

    let conn = Connection::open(path).await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS temperature (
                    time INTEGER,
                    temp REAL
            )",
            (),
        )?;
        return Ok(());
    })
    .await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS humidity (
                    time INTEGER,
                    humd REAL
            )",
            (),
        )?;
        return Ok(());
    })
    .await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS light (
                    time INTEGER,
                    is_light INTEGER
            )",
            (),
        )?;
        return Ok(());
    })
    .await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS motion (
                    time INTEGER,
                    detected_motion INTEGER
            )",
            (),
        )?;
        return Ok(());
    })
    .await?;

    info!("Connection successful");

    return Ok(conn);
}
