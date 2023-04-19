use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mint {
    id: Uuid,
    drop_id: Uuid,
}
impl FromRow for Mint {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("mint_id"),
            drop_id: row.get("drop_id"),
        }
    }
}

impl RelationPayload for Mint {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Mint.to_string(),
            object: self.id.to_string(),
            relation: Parents.to_string(),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                namespace: Drop.to_string(),
                object: self.drop_id.to_string(),
                relation: String::new(),
            })),
        }
    }
}

pub async fn get(
    id: Option<String>,
    project_id: Option<String>,
    drop_id: Option<String>,
    all: bool,
) -> Result<Vec<Relationship>> {
    let config = Config::read();
    let db = config.get_instance("nfts")?;

    let query = match (id, project_id, drop_id, all) {
        (Some(id), None, None, false) => {
            let id = Uuid::parse_str(&id)?;
            format!(
                "
        SELECT d.id AS drop_id, cm.id AS mint_id
        FROM drops d JOIN collection_mints cm ON d.collection_id = cm.collection_id
        WHERE cm.id = '{id}' GROUP BY d.id, cm.id;
        ",
                id = id
            )
        }
        (None, Some(project_id), None, _) => {
            format!(
                "
        SELECT d.id AS drop_id, cm.id AS mint_id FROM drops d
        JOIN collection_mints cm ON d.collection_id = cm.collection_id
        WHERE d.project_id = '{id}';
        ",
                id = project_id
            )
        }
        (None, None, Some(drop_id), _) => {
            format!(
                "
        SELECT d.id AS drop_id, cm.id AS mint_id FROM drops d
        JOIN collection_mints cm ON d.collection_id = cm.collection_id
        WHERE d.id = '{id}';
        ",
                id = drop_id
            )
        }
        _ => "
        SELECT d.id AS drop_id, cm.id AS mint_id FROM drops d JOIN collection_mints cm
        ON d.collection_id = cm.collection_id GROUP BY d.id, cm.id;
        "
        .to_string(),
    };

    let items: Vec<Mint> = from_row::query_and_map(db, &query).await?;

    let payloads: Vec<Relationship> = items
        .into_iter()
        .map(|item| item.create_payload())
        .collect();

    Ok(payloads)
}



pub async fn check(id: Option<String>, project_id: Option<String>, drop_id: Option<String>, all: bool) -> Result<Vec<Relationship>> {
    let items = get(id, project_id, drop_id, all).await?;
    let results: Vec<CheckResponse> = check_relations(items).await?;

    let missing: Vec<Relationship> = results
        .into_iter()
        .filter(|result| !result.allowed)
        .map(|result| result.relationship)
        .collect();
    Ok(missing)
}
