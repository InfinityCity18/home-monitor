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
    pub fn column_name(&self) -> String {
        match self {
            TableType::Temperature => "temp".to_string(),
            TableType::Humidity => "humd".to_string(),
            TableType::Motion => "detected_motion".to_string(),
            TableType::Light => "is_light".to_string(),
        }
    }
}

pub trait ToSqlDebug: ToSql + Debug + Send + Sync {}
impl ToSqlDebug for f64 {}
impl ToSqlDebug for bool {}

#[instrument(level = "debug", skip(conn))]
pub async fn insert<T: ToSql + Debug + Send + Sync + 'static>(
    timestamp: i64,
    data: T,
    conn: &OnceLock<Connection>,
    table_type: TableType,
) -> Result<()> {
    trace!("Inserting into database...");

    let conn = conn.get().unwrap();
    let table = table_type.table_name();
    let column_name = table_type.column_name();

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

#[instrument(level = "debug")]
pub async fn select(conn: &OnceLock<Connection>, table_type: TableType) -> Result<Vec<(i64, f64)>> {
    trace!("Selecting from database...");

    let conn = conn.get().unwrap();
    let table = table_type.table_name();

    let data = conn
        .call(move |conn| {
            let mut stmt = conn.prepare(format!("SELECT * FROM {table} ORDER BY time").as_str())?;
            let i = stmt
                .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
                .map(|r| match r {
                    Ok(a) => Ok(a),
                    Err(e) => Err(tokio_rusqlite::Error::from(e)),
                })
                .collect::<std::result::Result<Vec<(i64, f64)>, tokio_rusqlite::Error>>()?;
            Ok(i)
        })
        .await?;

    trace!("Selecting successful, returning data");

    Ok(data)
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
