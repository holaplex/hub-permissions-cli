use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    id: Uuid,
    organization_id: Uuid,
}
impl FromRow for Credential {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            organization_id: row.get("owner"),
        }
    }
}

impl RelationPayload for Credential {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Credential.to_string(),
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
    let db = config.get_instance("hydra")?;

    let query = match (id, all) {
        (Some(id), false) => {
            let id = Uuid::parse_str(&id)?;
            format!("SELECT id::uuid, owner::uuid FROM hydra_client WHERE id = '{id}'")
        }
        _ => "SELECT id::uuid, owner::uuid FROM hydra_client".to_string(),
    };

    let items: Vec<Credential> = from_row::query_and_map(db, &query).await?;
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
