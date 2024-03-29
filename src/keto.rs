use std::{collections::HashMap, fmt};

use futures::{stream, StreamExt};
use reqwest::StatusCode;
use serde_json::Value;

use crate::prelude::*;

pub trait RelationPayload {
    fn create_payload(&self) -> Relationship;
}

pub enum Namespace {
    Organization,
    Webhook,
    Credential,
    User,
    Member,
    Project,
    Customer,
    Collection,
    Drop,
    Mint,
}
pub enum Relation {
    Parents,
    Owners,
    Editors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckResponse {
    pub relationship: Relationship,
    pub allowed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub relationship: Relationship,
    pub status: u16,
    pub error: Option<String>,
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Organization => "Organization",
            Self::Webhook => "Webhook",
            Self::Credential => "Credential",
            Self::User => "User",
            Self::Member => "Member",
            Self::Project => "Project",
            Self::Customer => "Customer",
            Self::Drop => "Drop",
            Self::Collection => "Collection",
            Self::Mint => "Mint",
        };
        write!(f, "{name}")
    }
}
impl Default for Relation {
    fn default() -> Self {
        Self::Parents
    }
}
impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Parents => "parents",
            Self::Editors => "editors",
            Self::Owners => "owners",
        };
        write!(f, "{name}")
    }
}
/// # Errors
///
/// Will return `Err` if unable to contact Keto-read endpoint
pub async fn check_relations(items: Vec<Relationship>) -> Result<Vec<CheckResponse>> {
    let url = format!("{}/relation-tuples/check", Config::read().keto.read_url);

    let results: Vec<Result<CheckResponse>> = stream::iter(items.into_iter().map(|payload| {
        let url = url.clone();
        async move {
            let mut query_params = HashMap::new();
            query_params.insert("namespace", payload.namespace.to_string());
            query_params.insert("object", payload.object.to_string());
            query_params.insert("relation", payload.relation.to_string());

            if let Some(ss) = &payload.subject_set {
                query_params.insert("subject_set.object", ss.object.to_string());
                query_params.insert("subject_set.namespace", ss.namespace.to_string());
                query_params.insert("subject_set.relation", String::new());
            }
            let client = reqwest::Client::new();
            let response = client.get(&url).query(&query_params).send().await?;
            let status = response.status();
            let allowed = if status == StatusCode::OK {
                let json: Value = response.json().await?;
                json["allowed"].as_bool().unwrap_or(false)
            } else {
                false
            };

            Ok(CheckResponse {
                relationship: payload,
                allowed,
            })
        }
    }))
    .buffer_unordered(8)
    .collect()
    .await;

    results.into_iter().collect()
}

/// # Errors
///
/// Will return `Err` if unable to contact Keto-write endpoint
pub async fn create_relations(items: &[Relationship]) -> Result<Vec<CreateResponse>> {
    let url = format!("{}/admin/relation-tuples", Config::read().keto.write_url);

    let results: Vec<Result<CreateResponse>> = stream::iter(items.iter().map(|payload| {
        let url = url.clone();
        async move {
            let client = reqwest::Client::new();
            match client.put(&url).json(&payload).send().await {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let error = if status == 201 {
                        None
                    } else {
                        response
                            .json::<serde_json::Value>()
                            .await
                            .ok()
                            .and_then(|v| v.get("error").and_then(|e| e.get("message").cloned()))
                            .and_then(|e| e.as_str().map(String::from))
                    };
                    Ok(CreateResponse {
                        relationship: payload.clone(),
                        status,
                        error,
                    })
                },
                Err(_) => Ok(CreateResponse {
                    relationship: payload.clone(),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    error: Some("Internal server error".to_string()),
                }),
            }
        }
    }))
    .buffer_unordered(8)
    .collect::<Vec<_>>()
    .await;

    let all_results: Vec<CreateResponse> = results.into_iter().map(Result::unwrap).collect();

    Ok(all_results)
}
