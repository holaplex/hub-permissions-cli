use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    id: Uuid,
    project_id: Uuid,
}
impl FromRow for Collection {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            project_id: row.get("project_id"),
        }
    }
}

impl RelationPayload for Collection {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Collection.to_string(),
            object: self.id.to_string(),
            relation: Parents.to_string(),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                namespace: Project.to_string(),
                object: self.project_id.to_string(),
                relation: String::new(),
            })),
        }
    }
}

pub async fn get(id: Option<String>, all: bool) -> Result<Vec<Relationship>> {
    let config = Config::read();
    let db = config.get_instance("nfts")?;

    let query = match (id, all) {
        (Some(id), false) => format!(
            "SELECT id, project_id FROM collections WHERE id = '{id}'",
            id = Uuid::parse_str(&id)?
        ),
        _ => "SELECT id, project_id FROM collections".to_string(),
    };

    let items: Vec<Collection> = from_row::query_and_map(db, &query).await?;

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
