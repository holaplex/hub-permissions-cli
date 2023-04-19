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

/*
pub async fn get(id: Option<String>, all: bool) -> Result<Vec<CheckResponse>> {
    let config = Config::read();
    let db = config.get_instance("orgs")?;

    let query = match (id, all) {
        (Some(id), false) => {
            let id = Uuid::parse_str(&id)?;
            format!("SELECT id FROM organizations WHERE id = '{id}'")
        }
        _ => "SELECT id FROM organizations".to_string()
    };

    let items: Vec<Organization> = from_row::query_and_map(db, &query).await?;
}*/
