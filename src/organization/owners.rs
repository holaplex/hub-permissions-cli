use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    id: Uuid,
    organization_id: Uuid,
}
impl FromRow for Owner {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("user_id"),
            organization_id: row.get("organization_id"),
        }
    }
}

impl RelationPayload for Owner {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Organization.to_string(),
            object: self.organization_id.to_string(),
            relation: Owners.to_string(),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                namespace: User.to_string(),
                object: self.id.to_string(),
                relation: String::new(),
            })),
        }
    }
}

pub async fn get(id: Option<String>, all: bool) -> Result<Vec<Relationship>> {
    let config = Config::read();
    let db = config.get_instance("orgs")?;

    let query = match (id, all) {
        (Some(user_id), false) => {
            let user_id = Uuid::parse_str(&user_id)?;
            format!("SELECT user_id, organization_id FROM owners WHERE  user_id = '{user_id}'")
        }
        _ => "SELECT user_id, organization_id FROM owners".to_string(),
    };

    let items: Vec<Owner> = from_row::query_and_map(db, &query).await?;
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
