use crate::core::generator::Generator;
use crate::core::result::OxgenResult;
use crate::generators::dto::DtoGenerator;
use crate::generators::route::RouteGenerator;
use crate::generators::{
    controller::ControllerGenerator, model::ModelGenerator, module::ModuleGenerator,
    new_project::NewProjectGenerator, service::ServiceGenerator,
};

#[derive(Debug, PartialEq)]
pub enum Command {
    New {
        name: String,
        force: bool,
        dry_run: bool,
        database: Option<String>,
    },
    Generate {
        generator: GeneratorCommand,
    },
    Help,
    Version,
    Update,
}

#[derive(Debug, PartialEq)]
pub enum GeneratorCommand {
    Module {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Controller {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Service {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Model {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Dto {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Route {
        name: String,
        force: bool,
        dry_run: bool,
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

            Command::Generate { generator } => match generator {
                GeneratorCommand::Module {
                    name,
                    force,
                    dry_run,
                } => ModuleGenerator::new(name, force, dry_run).generate(),

                GeneratorCommand::Controller {
                    name,
                    force,
                    dry_run,
                } => ControllerGenerator::new(name, force, dry_run).generate(),

                GeneratorCommand::Service {
                    name,
                    force,
                    dry_run,
                } => ServiceGenerator::new(name, force, dry_run).generate(),

                GeneratorCommand::Model {
                    name,
                    force,
                    dry_run,
                } => ModelGenerator::new(name, force, dry_run).generate(),
                GeneratorCommand::Dto {
                    name,
                    force,
                    dry_run,
                } => DtoGenerator::new(name, force, dry_run).generate(),
                GeneratorCommand::Route {
                    name,
                    force,
                    dry_run,
                } => RouteGenerator::new(name, force, dry_run).generate(),
            },

            Command::Help => {
                crate::cli::help::print_help();
                Ok(())
            }

            Command::Version => {
                println!("oxgen {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            Command::Update => {
                crate::core::updater::update()?;
                Ok(())
            }
        }
    }
}
