use crate::cli::command::{Command, GeneratorCommand};
use crate::core::error::OxgenError;
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
        unknown => Err(OxgenError::UnknownCommand(unknown.to_string())),
    }
}

fn parse_new_command(args: &[String]) -> OxgenResult<Command> {
    let name = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .ok_or_else(|| OxgenError::MissingArgument("project name".to_string()))?
        .to_string();

    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    Ok(Command::New {
        name,
        force,
        dry_run,
    })
}

fn parse_generate_command(args: &[String]) -> OxgenResult<Command> {
    if args.is_empty() {
        return Err(OxgenError::MissingArgument("generator".to_string()));
    }

    let generator = args[0].as_str();

    let name = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'))
        .ok_or_else(|| OxgenError::MissingArgument("name".to_string()))?
        .to_string();

    let force = has_flag(args, "--force");
    let dry_run = has_flag(args, "--dry-run");

    let generator = match generator {
        "resource" | "res" => GeneratorCommand::Resource {
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
        unknown => return Err(OxgenError::UnknownGenerator(unknown.to_string())),
    };

    Ok(Command::Generate { generator })
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}
