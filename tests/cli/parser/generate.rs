use oxgen::cli::command::{Command, GeneratorCommand};
use oxgen::cli::parser::parse_args;
use oxgen::core::error::OxgenError;
use oxgen::core::naming::Name;

use super::shared::{CurrentDirGuard, create_oxgen_project, current_dir_lock};

#[test]
fn parser_parses_generate_module_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "module".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Module {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_g_module_alias_command() {
    let command = parse_args(vec![
        "g".to_string(),
        "mod".to_string(),
        "user".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Module {
                name: Name::new("user").unwrap(),
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
                name: Name::new("user").unwrap(),
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
                name: Name::new("user").unwrap(),
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
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_service_alias_command() {
    let command = parse_args(vec!["g".to_string(), "svc".to_string(), "user".to_string()]).unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Service {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_model_command() {
    let _lock = current_dir_lock().lock().unwrap();
    let temp_dir = create_oxgen_project("mock");
    let _guard = CurrentDirGuard::enter(temp_dir.path().to_path_buf());

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
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
                database: "mock".to_string(),
            }
        }
    );
}

#[test]
fn parser_parses_generate_model_command_with_force_and_dry_run() {
    let _lock = current_dir_lock().lock().unwrap();
    let temp_dir = create_oxgen_project("mock");
    let _guard = CurrentDirGuard::enter(temp_dir.path().to_path_buf());

    let command = parse_args(vec![
        "generate".to_string(),
        "model".to_string(),
        "user".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Model {
                name: Name::new("user").unwrap(),
                force: true,
                dry_run: true,
                database: "mock".to_string(),
            }
        }
    );
}

#[test]
fn parser_parses_generate_model_command_with_mongo_database_from_config() {
    let _lock = current_dir_lock().lock().unwrap();
    let temp_dir = create_oxgen_project("mongodb");
    let _guard = CurrentDirGuard::enter(temp_dir.path().to_path_buf());

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
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
                database: "mongodb".to_string(),
            }
        }
    );
}

#[test]
fn parser_returns_error_when_generate_model_is_used_outside_oxgen_project() {
    let _lock = current_dir_lock().lock().unwrap();
    let temp_dir = tempfile::tempdir().unwrap();
    let _guard = CurrentDirGuard::enter(temp_dir.path().to_path_buf());

    let result = parse_args(vec![
        "generate".to_string(),
        "model".to_string(),
        "user".to_string(),
    ]);

    assert!(matches!(
        result,
        Err(OxgenError::ProjectNotFound | OxgenError::OxgenProjectNotFound)
    ));
}

#[test]
fn parser_parses_generate_dto_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "dto".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Dto {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_g_dto_command() {
    let command = parse_args(vec!["g".to_string(), "dto".to_string(), "user".to_string()]).unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Dto {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_dto_command_with_force() {
    let command = parse_args(vec![
        "generate".to_string(),
        "dto".to_string(),
        "user".to_string(),
        "--force".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Dto {
                name: Name::new("user").unwrap(),
                force: true,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_dto_command_with_dry_run() {
    let command = parse_args(vec![
        "generate".to_string(),
        "dto".to_string(),
        "user".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Dto {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: true,
            }
        }
    );
}

#[test]
fn parser_parses_generate_dto_command_with_force_and_dry_run() {
    let command = parse_args(vec![
        "generate".to_string(),
        "dto".to_string(),
        "user".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Dto {
                name: Name::new("user").unwrap(),
                force: true,
                dry_run: true,
            }
        }
    );
}

#[test]
fn parser_parses_generate_route_command() {
    let command = parse_args(vec![
        "generate".to_string(),
        "route".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Route {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_g_route_command() {
    let command = parse_args(vec![
        "g".to_string(),
        "route".to_string(),
        "user".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Route {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_route_command_with_force() {
    let command = parse_args(vec![
        "generate".to_string(),
        "route".to_string(),
        "user".to_string(),
        "--force".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Route {
                name: Name::new("user").unwrap(),
                force: true,
                dry_run: false,
            }
        }
    );
}

#[test]
fn parser_parses_generate_route_command_with_dry_run() {
    let command = parse_args(vec![
        "generate".to_string(),
        "route".to_string(),
        "user".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Route {
                name: Name::new("user").unwrap(),
                force: false,
                dry_run: true,
            }
        }
    );
}

#[test]
fn parser_parses_generate_route_command_with_force_and_dry_run() {
    let command = parse_args(vec![
        "generate".to_string(),
        "route".to_string(),
        "user".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::Generate {
            generator: GeneratorCommand::Route {
                name: Name::new("user").unwrap(),
                force: true,
                dry_run: true,
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
    let result = parse_args(vec!["generate".to_string(), "resource".to_string()]);

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
