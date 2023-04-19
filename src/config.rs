use super::db::Instance;
use crate::prelude::error;
use anyhow::Result;
use once_cell::sync::OnceCell;
use ory_keto_client::apis::configuration::Configuration as KetoConfig;
use serde::Deserialize;
use std::{fs, io, path::PathBuf, sync::Arc};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keto: Keto,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Deserialize)]
pub struct Keto {
    pub read_url: Url,
    pub write_url: Url,
}

static CONFIG: OnceCell<Arc<Config>> = OnceCell::new();

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if unable to read config file
    pub fn load(path: PathBuf) -> Result<(), io::Error> {
        let config = serde_json::from_str::<Config>(&fs::read_to_string(path)?)?;
        CONFIG
            .set(Arc::new(config))
            .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "Config already loaded"))?;
        Ok(())
    }

    pub fn read() -> &'static Config {
        CONFIG.get().map_or_else(
            || {
                error!("Unable to read config. Exiting");
                std::process::exit(1)
            },
            std::convert::AsRef::as_ref,
        )
    }
    /// # Errors
    ///
    /// Will return `Err` if unable find instance on config file
    pub fn get_instance(&self, db_name: &str) -> Result<&Instance> {
        self.instances
            .iter()
            .find(|c| c.database == db_name)
            .ok_or_else(|| anyhow::anyhow!("Unable to find {} DB details", db_name))
    }
}
impl Keto {
    pub fn build_config(url: &Url) -> KetoConfig {
        KetoConfig {
            base_path: url.to_string(),
            user_agent: None,
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
