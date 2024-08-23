use anyhow::Result;
use std::fmt::Debug;
use std::sync::OnceLock;
use tokio_rusqlite::ToSql;
use tokio_rusqlite::{params, Connection};
use tracing::{info, instrument, trace};

#[derive(Debug)]
pub enum TableType {
    Temperature,
    Humidity,
    Motion,
    Light,
}

impl TableType {
    pub fn table_name(&self) -> String {
        match self {
            TableType::Temperature => "temperature".to_string(),
            TableType::Humidity => "humidity".to_string(),
            TableType::Motion => "motion".to_string(),
            TableType::Light => "light".to_string(),
        }
    }
    pub fn column_type(&self) -> String {
        match self {
            TableType::Temperature => "temp".to_string(),
            TableType::Humidity => "humd".to_string(),
            TableType::Motion => "detected_motion".to_string(),
            TableType::Light => "is_light".to_string(),
        }
    }
}

#[instrument(level = "debug", skip(conn))]
pub async fn insert<T: ToSql + Debug + Send + Sync + 'static>(
    timestamp: u32,
    data: T,
    conn: &OnceLock<Connection>,
    table_type: TableType,
) -> Result<()> {
    trace!("Inserting into database...");

    let conn = conn.get().unwrap();
    let table = table_type.table_name();
    let column_name = table_type.column_type();

    conn.call(move |conn| {
        conn.execute(
            format!("INSERT INTO {table} (time, {column_name}) VALUES (?1, ?2)").as_str(),
            params![timestamp, data],
        )?;
        Ok(())
    })
    .await?;

    trace!("Insertion succeded");

    Ok(())
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
