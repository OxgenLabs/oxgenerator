use crate::cli::command::{Command, GeneratorCommand};
use crate::core::error::OxgenError;
use crate::core::naming::Name;
use crate::core::project_detector::which_db_engine;
use crate::core::result::OxgenResult;

pub fn parse_args(args: Vec<String>) -> OxgenResult<Command> {
    if args.is_empty() {
        return Ok(Command::Help);
    }

    let command = args[0].as_str();

    match command {
        "new" => parse_new_command(&args[1..]),
        "generate" | "g" => parse_generate_command(&args[1..]),
        "help" | "--help" | "-h" => Ok(Command::Help),
        "version" | "--version" | "-v" => Ok(Command::Version),
        "update" => Ok(Command::Update),
        unknown => Err(OxgenError::UnknownCommand(unknown.to_string())),
    }
}

fn parse_new_command(args: &[String]) -> OxgenResult<Command> {
    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    let mut name_input: Option<String> = None;
    let mut database = "none".to_string();

    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--force" | "--dry-run" => {
                index += 1;
            }

            "--database" => {
                let value = args
                    .get(index + 1)
                    .filter(|value| !value.starts_with('-'))
                    .ok_or_else(|| OxgenError::MissingArgument("database".to_string()))?;

                database = normalize_database(value)?;
                index += 2;
            }

            arg if arg.starts_with('-') => {
                index += 1;
            }

            arg => {
                if name_input.is_none() {
                    name_input = Some(arg.to_string());
                }

                index += 1;
            }
        }
    }

    let name_input =
        name_input.ok_or_else(|| OxgenError::MissingArgument("project name".to_string()))?;
    let name = Name::new(&name_input)?;

    Ok(Command::New {
        name,
        force,
        dry_run,
        database,
    })
}

fn get_generate_name(args: &[String]) -> OxgenResult<String> {
    let mut index = 1;

    while index < args.len() {
        match args[index].as_str() {
            "--force" | "--dry-run" => {
                index += 1;
            }

            "--collection" => {
                let _ = args
                    .get(index + 1)
                    .filter(|value| !value.starts_with('-'))
                    .ok_or_else(|| OxgenError::MissingArgument("collection".to_string()))?;

                index += 2;
            }

            arg if arg.starts_with('-') => {
                index += 1;
            }

            arg => return Ok(arg.to_string()),
        }
    }

    Err(OxgenError::MissingArgument("name".to_string()))
}

pub fn get_collection_name(args: &[String]) -> Option<String> {
    args.iter()
        .position(|arg| arg == "--collection")
        .and_then(|index| args.get(index + 1))
        .filter(|value| !value.starts_with('-'))
        .cloned()
}

fn normalize_database(database: &str) -> OxgenResult<String> {
    match database.trim().to_lowercase().as_str() {
        "none" => Ok("none".to_string()),
        "mock" => Ok("mock".to_string()),
        "mongo" | "mongodb" => Ok("mongo".to_string()),
        _ => Err(OxgenError::UnknownDatabase),
    }
}

fn parse_generate_command(args: &[String]) -> OxgenResult<Command> {
    if args.is_empty() {
        return Err(OxgenError::MissingArgument("generator".to_string()));
    }

    let generator = args[0].as_str();
    let name_input = get_generate_name(args)?;
    let name = Name::new(&name_input)?;

    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    let database = which_db_engine()?;
    let collection = match database.as_str() {
        "mongodb" => get_collection_name(args),
        _ => None,
    };

    let generator = match generator {
        "module" | "mod" => GeneratorCommand::Module {
            name,
            force,
            dry_run,
            database,
            collection
        },

        "controller" | "ctrl" => GeneratorCommand::Controller {
            name,
            force,
            dry_run,
            database,
            collection,
        },

        "service" | "svc" => GeneratorCommand::Service {
            name,
            force,
            dry_run,
            database,
        },

        "model" => GeneratorCommand::Model {
            name,
            force,
            dry_run,
            database,
        },

        "dto" => GeneratorCommand::Dto {
            name,
            force,
            dry_run,
            database,
        },

        "route" => GeneratorCommand::Route {
            name,
            force,
            dry_run,
            database,
        },

        unknown => return Err(OxgenError::UnknownGenerator(unknown.to_string())),
    };

    Ok(Command::Generate { generator })
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}
