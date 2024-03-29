#![allow(clippy::unused_async)]
use crate::prelude::*;

pub mod collections;
pub mod customers;
pub mod drops;
pub mod mints;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    id: Uuid,
    organization_id: Uuid,
}
impl FromRow for Project {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
        }
    }
}

impl RelationPayload for Project {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Project.to_string(),
            object: self.id.to_string(),
            relation: Parents.to_string(),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                namespace: Organization.to_string(),
                object: self.organization_id.to_string(),
                relation: String::new(),
            })),
        }
    }
}

pub async fn get(id: Option<String>, all: bool) -> Result<Vec<Relationship>> {
    let config = Config::read();
    let db = config.get_instance("orgs")?;

    let query = match (id, all) {
        (Some(id), false) => format!(
            "SELECT id, organization_id FROM projects WHERE id = '{id}'",
            id = Uuid::parse_str(&id)?
        ),

        _ => "SELECT id, organization_id FROM projects".to_string(),
    };

    let items: Vec<Project> = from_row::query_and_map(db, &query).await?;

    let payloads: Vec<Relationship> = items
        .into_iter()
        .map(|item| item.create_payload())
        .collect();

    Ok(payloads)
}

pub async fn check(id: Option<String>, all: bool) -> Result<Vec<Relationship>> {
    let items = get(id, all).await?;
    let results: Vec<CheckResponse> = check_relations(items).await?;

    let missing: Vec<Relationship> = results
        .into_iter()
        .filter(|result| !result.allowed)
        .map(|result| result.relationship)
        .collect();
    Ok(missing)
}
