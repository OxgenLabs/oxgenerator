use crate::core::error::OxgeneratorError;
use crate::core::result::OxgeneratorResult;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
#[command(about="Rust API project generator")]
pub struct Cli {
    /// Preview changes without writing files
    #[arg(short, long, global = true)]
    dry_run: bool,

    /// Overwrite files that already exist
    #[arg(short, long, global = true)]
    force: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Oxgen Project
    New {
        /// Name of the project
        name: String,

        /// Select the database engine or opens an interactive selection when omitted (supported value: `mock`, `mongodb`)
        #[arg(long)]
        database: Option<String>,
    },

    /// Generate an Oxgen resource
    Generate {
        #[command(subcommand)]
        command: GeneratorCommands,
    },

    /// Update Oxgen CLI
    Update
}

#[derive(Subcommand)]
pub enum GeneratorCommands {
    /// Generate an Oxgen module
    Module {
        /// Name of the module
        name: String,

        /// Name of the collection used by the database or the name of the module when omitted
        #[arg(long)]
        collection: Option<String>,
    },
    /// Generate an Oxgen controller
    Controller {
        /// Name of the resource
        name: String,

        /// Name of the collection used by the database or the name of the resource when omitted
        #[arg(long)]
        collection: Option<String>,
    },

    ///Generate an Oxgen service
    Service {
        /// Name of the resource
        name: String,
    },

    ///Generate an Oxgen model
    Model {
        /// Name of the resource
        name: String,
    },

    /// Generate an Oxgen dto
    Dto {
        /// Name of the resource
        name: String,
    },

    /// Generate an Oxgen route
    Route {
        /// Name of the resource
        name: String,
    },
}

pub fn parse_cli() -> OxgeneratorResult<(), OxgeneratorError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, database } => {
            println!(
                "New with {} and {:?} and option {} and {}",
                name, database, cli.force, cli.dry_run
            );
            Ok(())
        }
        Commands::Generate { command } => match command {
            GeneratorCommands::Module { name, collection } => {
                println!(
                    "module: {}, {:?}, {}, {}",
                    name, collection, cli.force, cli.dry_run
                );
                Ok(())
            }
            GeneratorCommands::Controller { name, collection } => {
                println!(
                    "controller: {}, {:?}, {}, {}",
                    name, collection, cli.force, cli.dry_run
                );
                Ok(())
            }
            GeneratorCommands::Service { name } => {
                println!("service: {}, {}, {}", name, cli.force, cli.dry_run);
                Ok(())
            }
            GeneratorCommands::Model { name } => {
                println!("model: {}, {}, {}", name, cli.force, cli.dry_run);
                Ok(())
            }
            GeneratorCommands::Dto { name } => {
                println!("dto: {}, {}, {}", name, cli.force, cli.dry_run);
                Ok(())
            }
            GeneratorCommands::Route { name } => {
                println!("route: {}, {}, {}", name, cli.force, cli.dry_run);
                Ok(())
            }
        },
        Commands::Update => {
            println!("update!");
            Ok(())
        }
    }
}
