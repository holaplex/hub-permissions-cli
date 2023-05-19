#![allow(clippy::unused_async)]
use crate::prelude::*;
pub mod credentials;
pub mod members;
pub mod owners;
pub mod webhooks;

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    id: Uuid,
    owners: Option<Vec<Uuid>>,
    credentials: Option<Vec<Uuid>>,
    webhooks: Option<Vec<Uuid>>,
}

impl FromRow for Organization {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            owners: None,
            credentials: None,
            webhooks: None,
        }
    }
}
