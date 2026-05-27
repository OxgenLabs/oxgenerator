use crate::core::generator::Generator;
use crate::core::result::OxgenResult;
use crate::generators::{
    controller::ControllerGenerator, model::ModelGenerator, new_project::NewProjectGenerator,
    resource::ResourceGenerator, service::ServiceGenerator,
};

#[derive(Debug, PartialEq)]
pub enum Command {
    New {
        name: String,
        force: bool,
        dry_run: bool,
    },
    Generate {
        generator: GeneratorCommand,
    },
    Help,
    Version,
}

#[derive(Debug, PartialEq)]
pub enum GeneratorCommand {
    Resource {
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
}

impl Command {
    pub fn execute(self) -> OxgenResult<()> {
        match self {
            Command::New {
                name,
                force,
                dry_run,
            } => NewProjectGenerator::new(name, force, dry_run).generate(),

            Command::Generate { generator } => match generator {
                GeneratorCommand::Resource {
                    name,
                    force,
                    dry_run,
                } => ResourceGenerator::new(name, force, dry_run).generate(),

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
            },

            Command::Help => {
                crate::cli::help::print_help();
                Ok(())
            }

            Command::Version => {
                println!("oxgen {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
        }
    }
}
