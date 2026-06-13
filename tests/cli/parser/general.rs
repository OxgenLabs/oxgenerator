use oxgen::cli::command::Command;
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
fn parser_parses_update_flag() {
    let command = parse_args(vec!["update".to_string()]).unwrap();

    assert_eq!(command, Command::Update);
}

#[test]
fn parser_returns_error_for_unknown_command() {
    let result = parse_args(vec!["unknown".to_string()]);

    assert!(matches!(result, Err(OxgenError::UnknownCommand(_))));
}
