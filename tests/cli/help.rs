use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_command_prints_help_output() {
    let mut cmd = Command::cargo_bin("oxgen").unwrap();

    cmd.arg("help")
        .assert()
        .success()
        .stdout(contains("Oxgen - Rust project generator"))
        .stdout(contains("USAGE:"))
        .stdout(contains("oxgen <COMMAND> [OPTIONS]"))
        .stdout(contains("COMMANDS:"))
        .stdout(contains("new <name>"))
        .stdout(contains("generate <type> <name>"));
}

#[test]
fn help_flag_prints_help_output() {
    let mut cmd = Command::cargo_bin("oxgen").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Oxgen - Rust project generator"))
        .stdout(contains("USAGE:"))
        .stdout(contains("oxgen <COMMAND> [OPTIONS]"))
        .stdout(contains("COMMANDS:"))
        .stdout(contains("new <name>"))
        .stdout(contains("generate <type> <name>"));
}

#[test]
fn short_help_flag_prints_help_output() {
    let mut cmd = Command::cargo_bin("oxgen").unwrap();

    cmd.arg("-h")
        .assert()
        .success()
        .stdout(contains("Oxgen - Rust project generator"))
        .stdout(contains("USAGE:"))
        .stdout(contains("oxgen <COMMAND> [OPTIONS]"))
        .stdout(contains("COMMANDS:"))
        .stdout(contains("new <name>"))
        .stdout(contains("generate <type> <name>"));
}
