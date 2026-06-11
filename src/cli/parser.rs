use crate::cli::command::{Command, GeneratorCommand};
use crate::core::error::OxgenError;
use crate::core::naming::Name;
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
    let name_input = args
        .first()
        .ok_or_else(|| OxgenError::MissingArgument("project name".to_string()))?
        .clone();

    if name_input.starts_with("-") {
        return Err(OxgenError::MissingArgument("project name".to_string()));
    }

    let name = Name::new(&name_input)?;

    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    let database = if has_flag(args, "--database") {
        parse_database_engine(args)?
    } else {
        "none".to_string()
    };

    Ok(Command::New {
        name,
        force,
        dry_run,
        database,
    })
}

fn parse_database_engine(args: &[String]) -> OxgenResult<String> {
    let database = get_required_next_arg(args, "--database", "database engine name")?;

    match database.trim().to_lowercase().as_str() {
        "mongo" | "mongodb" => Ok("mongo".to_string()),
        "mock" => Ok("mock".to_string()),
        _ => Err(OxgenError::UnknownDatabase),
    }
}

fn get_required_next_arg(args: &[String], arg: &str, argument_name: &str) -> OxgenResult<String> {
    args.iter()
        .position(|current_arg| current_arg == arg)
        .and_then(|index| args.get(index + 1))
        .filter(|value| !value.starts_with('-'))
        .cloned()
        .ok_or_else(|| OxgenError::MissingArgument(argument_name.to_string()))
}

fn parse_generate_command(args: &[String]) -> OxgenResult<Command> {
    if args.is_empty() {
        return Err(OxgenError::MissingArgument("generator".to_string()));
    }

    let generator = args[0].as_str();

    let name_input = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'))
        .ok_or_else(|| OxgenError::MissingArgument("name".to_string()))?
        .to_string();

    let name = Name::new(&name_input)?;

    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    let generator = match generator {
        "module" | "mod" => GeneratorCommand::Module {
            name,
            force,
            dry_run,
        },
        "controller" | "ctrl" => GeneratorCommand::Controller {
            name,
            force,
            dry_run,
        },
        "service" | "svc" => GeneratorCommand::Service {
            name,
            force,
            dry_run,
        },
        "model" => GeneratorCommand::Model {
            name,
            force,
            dry_run,
        },
        "dto" => GeneratorCommand::Dto {
            name,
            force,
            dry_run,
        },
        "route" => GeneratorCommand::Route {
            name,
            force,
            dry_run,
        },
        unknown => return Err(OxgenError::UnknownGenerator(unknown.to_string())),
    };

    Ok(Command::Generate { generator })
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}
