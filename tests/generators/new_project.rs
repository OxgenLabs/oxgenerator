use crate::common::collect_files;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::tempdir;

fn run_cargo_command(project_path: &std::path::Path, args: &[&str]) {
    let output = StdCommand::new("cargo")
        .args(args)
        .current_dir(project_path)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "command `cargo {}` failed\n\nstdout:\n{}\n\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn new_command_creates_project() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success()
        .stdout(predicate::str::contains("created project `test-api`"));

    let project = temp_dir.path().join("test-api");

    assert!(project.is_dir());
    assert!(project.join("Cargo.toml").is_file());
    assert!(project.join("src/main.rs").is_file());
    assert!(project.join("src/lib.rs").is_file());
    assert!(project.join("src/state.rs").is_file());
}

#[test]
fn new_command_replaces_project_name() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let cargo_toml = fs::read_to_string(temp_dir.path().join("test-api/Cargo.toml")).unwrap();

    assert!(cargo_toml.contains(r#"name = "test-api""#));
    assert!(!cargo_toml.contains("{{project_name}}"));
}

#[test]
fn new_command_replaces_crate_name() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let main_rs = fs::read_to_string(temp_dir.path().join("test-api/src/main.rs")).unwrap();

    assert!(main_rs.contains("test_api"));
    assert!(!main_rs.contains("{{crate_name}}"));
}

#[test]
fn new_command_removes_ox_extensions() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let project = temp_dir.path().join("test-api");
    let mut files = Vec::new();

    collect_files(&project, &mut files);

    let has_ox_file = files
        .iter()
        .any(|path| path.extension().is_some_and(|extension| extension == "ox"));

    assert!(!has_ox_file);
}

#[test]
fn new_command_fails_if_project_already_exists_without_force() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project already exists"));
}

#[test]
fn new_command_overwrites_project_with_force() {
    let temp_dir = tempdir().unwrap();
    let project = temp_dir.path().join("test-api");

    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("old-file.txt"), "old content").unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api", "--force"])
        .assert()
        .success();

    assert!(project.join("Cargo.toml").is_file());
    assert!(!project.join("old-file.txt").exists());
}

#[test]
fn new_command_dry_run_creates_nothing() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "dry-api", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dry run"));

    assert!(!temp_dir.path().join("dry-api").exists());
}

#[test]
fn new_command_rejects_invalid_project_name() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "my api"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid package name"));
}

#[test]
fn new_command_generates_code_compliant_with_cargo_fmt() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let project = temp_dir.path().join("test-api");

    run_cargo_command(&project, &["fmt", "--", "--check"]);
}

#[test]
fn new_command_generates_project_that_passes_cargo_check() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let project = temp_dir.path().join("test-api");

    run_cargo_command(&project, &["check"]);
}

#[test]
fn new_command_generates_project_that_passes_clippy() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let project = temp_dir.path().join("test-api");

    run_cargo_command(&project, &["clippy", "--", "-D", "warnings"]);
}

#[test]
fn new_command_initializes_git_repository() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api"])
        .assert()
        .success();

    let project = temp_dir.path().join("test-api");

    assert!(project.join(".git").is_dir());

    let output = StdCommand::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(&project)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "expected generated project to be a git repository\n\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "true");
}

#[test]
fn new_command_rejects_built_in_library_package_name() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("built-in"));
}

#[test]
fn new_command_rejects_confusing_package_name_std() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "std"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("confusing"));
}

#[test]
fn new_command_rejects_rust_keyword_package_name() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "async"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("keyword"));
}
