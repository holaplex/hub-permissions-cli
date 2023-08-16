use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "hub-permissions-cli",
    about = "A CLI to troubleshoot and fix Hub-Permissions (Keto) relationships"
)]

pub struct Opt {
    #[structopt(flatten)]
    pub global: GlobalOptions,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub struct GlobalOptions {
    #[structopt(
        long,
        global = true,
        help = "config path with db instances and keto read/write endpoints",
        default_value = "./config.json",
        env = "CONFIG_PATH",
        parse(from_os_str)
    )]
    pub config: PathBuf,
    #[structopt(global = true, long = "fix", help = "Fixes missing relations")]
    pub fix: bool,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "check")]
    Check {
        #[structopt(subcommand)]
        subcmd: Namespace,
    },
}

#[derive(Debug, StructOpt)]
pub enum Namespace {
    #[structopt(alias = "webhooks")]
    Webhook {
        #[structopt(name = "id", help = "Webhook ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all members",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "credentials")]
    Credential {
        #[structopt(name = "id", help = "Credential ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all members",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "users")]
    User {
        #[structopt(name = "id", help = "User ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(short, long = "relation", help = "members or owners")]
        relation: UserRelation,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all members",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "projects")]
    Project {
        #[structopt(name = "id", help = "Project ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all members",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "customers")]
    Customer {
        #[structopt(name = "id", help = "Customer ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all customers",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "drops")]
    Drop {
        #[structopt(name = "drop_id", help = "Drop ID", required_unless = "all")]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all drops",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "collections")]
    Collection {
        #[structopt(
            name = "collection_id",
            help = "Collection ID",
            required_unless = "all"
        )]
        id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all collections",
            conflicts_with = "id"
        )]
        all: bool,
    },
    #[structopt(alias = "mints")]
    Mint {
        #[structopt(
            name = "id",
            help = "Mint ID",
            required_unless = "all",
            required_unless = "project_id",
            conflicts_with = "collection_id",
            required_unless = "drop_id"
        )]
        id: Option<String>,
        #[structopt(short, long = "relation", help = "drop or collection")]
        relation: MintRelation,
        #[structopt(
            long = "project",
            short = "p",
            name = "project_id",
            help = "Project ID",
            conflicts_with = "id",
            conflicts_with = "all",
            conflicts_with = "collection_id",
            conflicts_with = "drop_id"
        )]
        project_id: Option<String>,
        #[structopt(
            long = "collection",
            short = "c",
            name = "collection_id",
            help = "Collection ID",
            conflicts_with = "id",
            conflicts_with = "all",
            conflicts_with = "project_id",
            conflicts_with = "drop_id"
        )]
        collection_id: Option<String>,
        #[structopt(
            long = "drop",
            short = "d",
            name = "drop_id",
            help = "Drop ID",
            conflicts_with = "id",
            conflicts_with = "all",
            conflicts_with = "project_id",
            conflicts_with = "collection_id"
        )]
        drop_id: Option<String>,
        #[structopt(
            short = "A",
            long = "all",
            help = "Retrieve all relations",
            conflicts_with = "id",
            conflicts_with = "project_id",
            conflicts_with = "collection_id",
            conflicts_with = "drop_id"
        )]
        all: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum UserRelation {
    #[structopt(alias = "owners")]
    Owner,
    #[structopt(alias = "members")]
    Member,
}

#[derive(Debug, StructOpt)]
pub enum MintRelation {
    #[structopt(alias = "drop")]
    Drop,
    #[structopt(alias = "collection")]
    Collection,
}

impl FromStr for UserRelation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owners" | "owner" => Ok(UserRelation::Owner),
            "members" | "member" => Ok(UserRelation::Member),
            _ => Err(format!("Invalid role: {s}")),
        }
    }
}

impl FromStr for MintRelation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "drops" | "drop" => Ok(MintRelation::Drop),
            "collections" | "collection" => Ok(MintRelation::Collection),
            _ => Err(format!("Invalid role: {s}")),
        }
    }
}
