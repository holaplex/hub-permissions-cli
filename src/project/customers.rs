use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    id: Uuid,
    project_id: Uuid,
}
impl FromRow for Customer {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            project_id: row.get("project_id"),
        }
    }
}

impl RelationPayload for Customer {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Customer.to_string(),
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
    let db = config.get_instance("customers")?;

    let query = match (id, all) {
        (Some(id), false) => {
            let id = Uuid::parse_str(&id)?;
            format!("SELECT id, project_id FROM customers WHERE id = '{id}'")
        }
        _ => "SELECT id, project_id FROM customers".to_string(),
    };

    let items: Vec<Customer> = from_row::query_and_map(db, &query).await?;
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
