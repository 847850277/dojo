use clap::{Args, Parser, Subcommand};

use self::build::BuildArgs;
use self::init::InitArgs;
use self::migrate::MigrateArgs;

pub(crate) mod build;
pub(crate) mod init;
pub(crate) mod migrate;

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Build the project's ECS, outputting smart contracts artifacts for deployment"
    )]
    Build(BuildArgs),
    #[command(about = "Initialize a new project")]
    Init(InitArgs),
    #[command(about = "Run a migration, declaring and deploying contracts as necessary to \
                       update the world")]
    Migrate(MigrateArgs),
    #[command(about = "Generate rust contract bindings")]
    Bind(BindArgs),
    #[command(about = "Retrieve an entity's state by entity ID")]
    Inspect(InspectArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct BindArgs {}

#[derive(Args)]
pub struct InspectArgs {
    #[clap(short, long, help = "Entity ID to retrieve state for")]
    id: String,
    #[clap(short, long, help = "World address to retrieve entity state from")]
    world_address: String,
}