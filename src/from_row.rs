use std::fmt::Debug;

use anyhow::Result;
use tokio_postgres::Row;

use crate::db::Instance;

pub trait FromRow {
    fn from_row(row: &Row) -> Self;
}
/// # Errors
///
/// Will return `Err` if unable map row to struct
pub async fn query_and_map<T: FromRow + Debug>(db: &Instance, query: &str) -> Result<Vec<T>> {
    let db_client = Instance::connect(db).await;
    let rows = db_client
        .query(query, &[])
        .await
        .expect("Failed to execute query");
    let items: Vec<T> = rows.iter().map(T::from_row).collect();
    Ok(items)
}
