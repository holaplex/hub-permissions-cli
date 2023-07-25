#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
use cli::{
    Command::Check,
    Namespace::{Collection, Credential, Customer, Drop, Mint, Project, User, Webhook},
    UserRelation::{Member, Owner},
};
use config::Config;
use env_logger::Builder;
use log::LevelFilter;
use prelude::{create_relations, info, warn, Result};
use structopt::StructOpt;

mod cli;
mod config;
mod db;
pub mod from_row;
mod keto;
mod organization;
pub mod prelude;
mod project;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Opt::from_args();
    Builder::new().filter(None, LevelFilter::Info).init();
    Config::load(cli.global.config)?;

    match cli.cmd {
        Check { subcmd } => {
            let payloads = match subcmd {
                User { id, relation, all } => match relation {
                    Member => organization::members::check(id, all).await?,
                    Owner => organization::owners::check(id, all).await?,
                },
                Credential { id, all } => organization::credentials::check(id, all).await?,
                Webhook { id, all } => organization::webhooks::check(id, all).await?,
                Project { id, all } => project::check(id, all).await?,
                Customer { id, all } => project::customers::check(id, all).await?,
                Drop { id, all } => project::drops::check(id, all).await?,
                Collection { id, all } => project::collections::check(id, all).await?,
                Mint {
                    id,
                    project_id,
                    drop_id,
                    all,
                } => project::mints::check(id, project_id, drop_id, all).await?,
            };
            if let Some(payload_count) = (!payloads.is_empty()).then_some(payloads.len()) {
                info!("{}", serde_json::to_string_pretty(&payloads)?);
                if cli.global.fix {
                    info!("--fix flag detected. Creating {} relations", payload_count);
                    let results = create_relations(&payloads).await?;
                    info!("{}", serde_json::to_string_pretty(&results).unwrap());
                } else {
                    warn!(
                        "Found {} broken relations. Re-run the command with --fix flag to create them",
                        payload_count
                    );
                }
            } else {
                info!("OK. No missing/broken relations");
            }
        },
    };

    Ok(())
}
