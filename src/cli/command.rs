use crate::core::database::DatabaseEngine;
use crate::core::generator::Generator;
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::result::OxgenResult;
use crate::generators::{
    controller::ControllerGenerator, dto::DtoGenerator, model::ModelGenerator,
    module::ModuleGenerator, new_project::NewProjectGenerator, route::RouteGenerator,
    service::ServiceGenerator,
};

pub enum Command {
    New {
        name: Name,
        force: bool,
        dry_run: bool,
        database: Option<DatabaseEngine>,
    },
    Generate {
        generator: GeneratorCommand,
    },
    Help,
    Version,
    Update,
}

pub enum GeneratorCommand {
    Module {
        name: Name,
        context: GeneratorContext,
        collection: Option<String>,
    },
    Controller {
        name: Name,
        context: GeneratorContext,
        collection: Option<String>,
    },
    Service {
        name: Name,
        context: GeneratorContext,
    },
    Model {
        name: Name,
        context: GeneratorContext,
    },
    Dto {
        name: Name,
        context: GeneratorContext,
    },
    Route {
        name: Name,
        context: GeneratorContext,
    },
}

impl Command {
    pub fn execute(self) -> OxgenResult<()> {
        match self {
            Command::New {
                name,
                force,
                dry_run,
                database,
            } => NewProjectGenerator::new(name, force, dry_run, database).generate(),

            Command::Generate { generator } => generator.execute(),

            Command::Help => {
                crate::cli::help::print_help();
                Ok(())
            }

            Command::Version => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }

            Command::Update => crate::core::updater::update(),
        }
    }
}

impl GeneratorCommand {
    pub fn execute(self) -> OxgenResult<()> {
        match self {
            GeneratorCommand::Module {
                name,
                context,
                collection,
            } => ModuleGenerator::new(name, context, collection).generate(),

            GeneratorCommand::Controller {
                name,
                context,
                collection,
            } => ControllerGenerator::new(name, context, collection).generate(),

            GeneratorCommand::Service { name, context } => {
                ServiceGenerator::new(name, context).generate()
            }

            GeneratorCommand::Model { name, context } => {
                ModelGenerator::new(name, context).generate()
            }

            GeneratorCommand::Dto { name, context } => DtoGenerator::new(name, context).generate(),

            GeneratorCommand::Route { name, context } => {
                RouteGenerator::new(name, context).generate()
            }
        }
    }
}
