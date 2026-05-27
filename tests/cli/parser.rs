use oxgen::cli::command::{Command, GeneratorCommand};
use oxgen::cli::parser::parse_args;
use oxgen::core::error::OxgenError;

#[test]
fn parser_returns_help_when_no_args_are_provided() {
    let command = parse_args(vec![]).unwrap();

    assert_eq!(command, Command::Help);
}

#[test]
fn parser_parses_help_command() {
    let command = parse_args(vec!["help".to_string()]).unwrap();

    assert_eq!(command, Command::Help);
}

#[test]
fn parser_parses_help_short_flag() {
    let command = parse_args(vec!["-h".to_string()]).unwrap();

    assert_eq!(command, Command::Help);
}

#[test]
fn parser_parses_help_long_flag() {
    let command = parse_args(vec!["--help".to_string()]).unwrap();

    assert_eq!(command, Command::Help);
}

#[test]
fn parser_parses_version_command() {
    let command = parse_args(vec!["version".to_string()]).unwrap();

    assert_eq!(command, Command::Version);
}

#[test]
fn parser_parses_version_short_flag() {
    let command = parse_args(vec!["-v".to_string()]).unwrap();

    assert_eq!(command, Command::Version);
}

#[test]
fn parser_parses_version_long_flag() {
    let command = parse_args(vec!["--version".to_string()]).unwrap();

    assert_eq!(command, Command::Version);
}

#[test]
fn parser_parses_new_command() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: "test-api".to_string(),
            force: false,
            dry_run: false,
        }
    );
}

#[test]
fn parser_parses_new_command_with_force() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--force".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: "test-api".to_string(),
            force: true,
            dry_run: false,
        }
    );
}

#[test]
fn parser_parses_new_command_with_dry_run() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: "test-api".to_string(),
            force: false,
            dry_run: true,
        }
    );
}

#[test]
fn parser_parses_new_command_with_force_and_dry_run() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: "test-api".to_string(),
            force: true,
            dry_run: true,
        }
    );
}

#[test]
fn parser_parses_new_command_when_flags_are_before_name() {
    let command = parse_args(vec![
        "new".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
        "test-api".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: "test-api".to_string(),
            force: true,
            dry_run: true,
        }
    );
}

#[test]
fn parser_returns_error_when_new_command_has_no_project_name() {
    let result = parse_args(vec!["new".to_string()]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_for_unknown_command() {
    let result = parse_args(vec!["unknown".to_string()]);

    assert!(matches!(result, Err(OxgenError::UnknownCommand(_))));
}

#[test]
fn parser_parses_generate_resource_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "resource".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Resource {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_g_resource_alias_command() {
    let command = parse_args(vec![
        "g".to_string(),
        "res".to_string(),
        "user".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Resource {
                name: "user".to_string(),
                force: true,
                dry_run: true,
            }
        }
    );
}

#[test]
fn parser_parses_generate_controller_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "controller".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Controller {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_controller_alias_command() {
    let command = parse_args(vec![
        "g".to_string(),
        "ctrl".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Controller {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_service_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "service".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Service {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_service_alias_command() {
    let command = parse_args(vec![
        "g".to_string(),
        "svc".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Service {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_model_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "model".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Model {
                name: "user".to_string(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_returns_error_when_generate_has_no_generator() {
    let result = parse_args(vec!["generate".to_string()]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_generate_has_no_name() {
    let result = parse_args(vec![
        "generate".to_string(),
        "resource".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_for_unknown_generator() {
    let result = parse_args(vec![
        "generate".to_string(),
        "unknown".to_string(),
        "user".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::UnknownGenerator(_))));
}
