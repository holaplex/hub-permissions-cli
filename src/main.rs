use cli::{Command::*, Namespace::*, UserRelation::*};
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
    Config::load(cli.global.config).await?;

    match cli.cmd {
        Get { subcmd } => {
            let payloads = match subcmd {
                User { id, relation, all } => match relation {
                    Member => organization::members::get(id, all).await?,
                    Owner => organization::owners::get(id, all).await?,
                },
                Credential { id, all } => organization::credentials::get(id, all).await?,
                Webhook { id, all } => organization::webhooks::get(id, all).await?,
                Project { id, all } => project::get(id, all).await?,
                Customer { id, all } => project::customers::get(id, all).await?,
                Drop { id, all } => project::drops::get(id, all).await?,
                Mint { id, project_id, drop_id, all } => project::mints::get(id, project_id, drop_id, all).await?,
            };
            info!("{}", serde_json::to_string_pretty(&payloads)?);
        }
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
                Mint { id, project_id, drop_id, all } => project::mints::check(id, project_id, drop_id, all).await?,
            };
            if let Some(payload_count) = (!payloads.is_empty()).then_some(payloads.len()) {
                info!("{}", serde_json::to_string_pretty(&payloads).unwrap());

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
                info!("No missing relations found. If you think this is an error double check the provided ID");
            }
        }
    };

    Ok(())
}
