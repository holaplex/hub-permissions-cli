use crate::db::Instance;
use anyhow::Result;
use std::fmt::Debug;
use tokio_postgres::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Self;
}

pub async fn query_and_map<T: FromRow + Debug>(db: &Instance, query: &str) -> Result<Vec<T>> {
    let db_client = Instance::connect(db).await;
    let rows = db_client
        .query(query, &[])
        .await
        .expect("Failed to execute query");
    let items: Vec<T> = rows.iter().map(T::from_row).collect();
    Ok(items)
}
