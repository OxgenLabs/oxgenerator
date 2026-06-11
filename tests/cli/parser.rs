use oxgen::cli::command::{Command, GeneratorCommand};
use oxgen::cli::parser::parse_args;
use oxgen::core::error::OxgenError;
use oxgen::core::naming::Name;

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
fn parser_parses_update_flag() {
    let command = parse_args(vec!["update".to_string()]).unwrap();

    assert_eq!(command, Command::Update);
}

#[test]
fn parser_parses_new_command() {
    let command = parse_args(vec!["new".to_string(), "test-api".to_string()]).unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "none".to_string(),
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
            name: Name::new("test-api").unwrap(),
            force: true,
            dry_run: false,
            database: "none".to_string(),
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
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: true,
            database: "none".to_string(),
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
            name: Name::new("test-api").unwrap(),
            force: true,
            dry_run: true,
            database: "none".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_mock_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "mock".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mock".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_mongo_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "mongo".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_mongodb_database_alias() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "mongodb".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_uppercase_mock_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "Mock".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mock".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_uppercase_mongo_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "Mongo".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_uppercase_mongodb_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "MongoDB".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_database_value_containing_spaces() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        " MongoDB ".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: false,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_force_and_mock_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--force".to_string(),
        "--database".to_string(),
        "mock".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: true,
            dry_run: false,
            database: "mock".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_dry_run_and_mock_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--dry-run".to_string(),
        "--database".to_string(),
        "mock".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: false,
            dry_run: true,
            database: "mock".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_force_dry_run_and_mock_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
        "--database".to_string(),
        "mock".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: true,
            dry_run: true,
            database: "mock".to_string(),
        }
    );
}

#[test]
fn parser_parses_new_command_with_force_dry_run_and_mongo_database() {
    let command = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
        "--database".to_string(),
        "mongo".to_string(),
    ])
    .unwrap();

    assert_eq!(
        command,
        Command::New {
            name: Name::new("test-api").unwrap(),
            force: true,
            dry_run: true,
            database: "mongo".to_string(),
        }
    );
}

#[test]
fn parser_returns_error_when_new_command_has_no_project_name() {
    let result = parse_args(vec!["new".to_string()]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_new_command_has_only_force_flag() {
    let result = parse_args(vec!["new".to_string(), "--force".to_string()]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_new_command_has_only_dry_run_flag() {
    let result = parse_args(vec!["new".to_string(), "--dry-run".to_string()]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_new_command_has_only_flags() {
    let result = parse_args(vec![
        "new".to_string(),
        "--force".to_string(),
        "--dry-run".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_database_flag_has_no_value() {
    let result = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_database_flag_value_is_another_flag() {
    let result = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "--force".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::MissingArgument(_))));
}

#[test]
fn parser_returns_error_when_database_engine_is_unknown() {
    let result = parse_args(vec![
        "new".to_string(),
        "test-api".to_string(),
        "--database".to_string(),
        "postgres".to_string(),
    ]);

    assert!(matches!(result, Err(OxgenError::UnknownDatabase)));
}

#[test]
fn parser_returns_error_for_unknown_command() {
    let result = parse_args(vec!["unknown".to_string()]);

    assert!(matches!(result, Err(OxgenError::UnknownCommand(_))));
}

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
