use oxgen::cli::command::Command;
use oxgen::cli::parser::parse_args;
use oxgen::core::error::OxgenError;
use oxgen::core::naming::Name;

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
