use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    id: Uuid,
    user_id: Uuid,
    organization_id: Uuid,
    deactivated_at: Option<DateTime<Utc>>,
}

impl FromRow for Member {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            organization_id: row.get("organization_id"),
            deactivated_at: row.get("deactivated_at"),
        }
    }
}

impl Member {
    fn create_role_payload(&self) -> Relationship {
        Relationship {
            namespace: Organization.to_string(),
            object: self.organization_id.to_string(),
            relation: Editors.to_string(),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                namespace: User.to_string(),
                object: self.user_id.to_string(),
                relation: String::new(),
            })),
        }
    }
}

impl RelationPayload for Member {
    fn create_payload(&self) -> Relationship {
        Relationship {
            namespace: Member.to_string(),
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
    let query = {
        match (id, all) {
            (Some(user_id), false) => format!(
                "SELECT id, user_id, organization_id, deactivated_at FROM members WHERE user_id = '{id}'",
                id = Uuid::parse_str(&user_id)?
            ),
            _ => "SELECT id, user_id, organization_id, deactivated_at FROM members".to_string(),
        }
    };

    let items: Vec<Member> = from_row::query_and_map(db, &query).await?;
    let payloads: Vec<Relationship> = items
        .into_iter()
        .flat_map(|item| {
            let resource_payload = item.create_payload();
            if item.deactivated_at.is_none() {
                let role_payload = item.create_role_payload();
                vec![resource_payload, role_payload]
            } else {
                vec![resource_payload]
            }
        })
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
