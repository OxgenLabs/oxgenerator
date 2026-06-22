use crate::cli::command::{Command, GeneratorCommand};
use crate::core::database::DatabaseEngine;
use crate::core::error::OxgenError;
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::project_detector::which_db_engine;
use crate::core::result::OxgenResult;

pub fn parse_args(args: Vec<String>) -> OxgenResult<Command> {
    if args.is_empty() {
        return Ok(Command::Help);
    }

    match args[0].as_str() {
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

    let mut name_input = None;
    let mut database = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--force" | "--dry-run" => {
                index += 1;
            }

            "--database" => {
                let value = get_flag_value(args, index, "database")?;

                database = Some(normalize_database(value)?);
                index += 2;
            }

            argument if argument.starts_with('-') => {
                return Err(OxgenError::InvalidCommand(argument.to_string()));
            }

            argument => {
                if name_input.is_some() {
                    return Err(OxgenError::InvalidCommand(argument.to_string()));
                }

                name_input = Some(argument.to_string());
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

fn parse_generate_command(args: &[String]) -> OxgenResult<Command> {
    if args.is_empty() {
        return Err(OxgenError::MissingArgument("generator".to_string()));
    }

    let generator_name = args[0].as_str();
    let options = parse_generate_options(&args[1..])?;

    let name_input = options
        .name
        .ok_or_else(|| OxgenError::MissingArgument("name".to_string()))?;

    let name = Name::new(&name_input)?;
    let database = which_db_engine()?;

    validate_collection(generator_name, database, options.collection.as_ref())?;

    let context = GeneratorContext::new(options.force, options.dry_run, database);

    let generator = match generator_name {
        "module" | "mod" => GeneratorCommand::Module {
            name,
            context,
            collection: options.collection,
        },

        "controller" | "ctrl" => GeneratorCommand::Controller {
            name,
            context,
            collection: options.collection,
        },

        "service" | "svc" => GeneratorCommand::Service { name, context },

        "model" => GeneratorCommand::Model { name, context },

        "dto" => GeneratorCommand::Dto { name, context },

        "route" => GeneratorCommand::Route { name, context },

        unknown => {
            return Err(OxgenError::UnknownGenerator(unknown.to_string()));
        }
    };

    Ok(Command::Generate { generator })
}

struct GenerateOptions {
    name: Option<String>,
    force: bool,
    dry_run: bool,
    collection: Option<String>,
}

fn parse_generate_options(args: &[String]) -> OxgenResult<GenerateOptions> {
    let mut name = None;
    let mut force = false;
    let mut dry_run = false;
    let mut collection = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--force" => {
                force = true;
                index += 1;
            }

            "--dry-run" => {
                dry_run = true;
                index += 1;
            }

            "--collection" => {
                let value = get_flag_value(args, index, "collection")?;

                collection = Some(value.to_string());
                index += 2;
            }

            argument if argument.starts_with('-') => {
                return Err(OxgenError::InvalidCommand(argument.to_string()));
            }

            argument => {
                if name.is_some() {
                    return Err(OxgenError::InvalidCommand(argument.to_string()));
                }

                name = Some(argument.to_string());
                index += 1;
            }
        }
    }

    Ok(GenerateOptions {
        name,
        force,
        dry_run,
        collection,
    })
}

fn validate_collection(
    generator: &str,
    database: DatabaseEngine,
    collection: Option<&String>,
) -> OxgenResult<()> {
    if collection.is_none() {
        return Ok(());
    }

    if !database.supports_collection() {
        return Err(OxgenError::CollectionRequiresMongoDb);
    }

    if !matches!(generator, "module" | "mod" | "controller" | "ctrl") {
        return Err(OxgenError::CollectionUnsupportedGenerator(
            generator.to_string(),
        ));
    }

    Ok(())
}

fn normalize_database(database: &str) -> OxgenResult<DatabaseEngine> {
    match database.trim().to_lowercase().as_str() {
        "mock" => Ok(DatabaseEngine::Mock),
        "mongo" | "mongodb" => Ok(DatabaseEngine::MongoDb),
        _ => Err(OxgenError::UnknownDatabase),
    }
}

fn get_flag_value<'a>(
    args: &'a [String],
    flag_index: usize,
    argument_name: &str,
) -> OxgenResult<&'a str> {
    args.get(flag_index + 1)
        .filter(|value| !value.starts_with('-'))
        .map(String::as_str)
        .ok_or_else(|| OxgenError::MissingArgument(argument_name.to_string()))
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|argument| argument == flag)
}
