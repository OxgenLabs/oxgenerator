use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn create_oxgen_project() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", "test-api", "--database", "mock"])
        .assert()
        .success();

    temp_dir
}

#[test]
fn generate_model_creates_model_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    let model_path = project.join("src/modules/user/model.rs");

    assert!(model_path.is_file());

    let content = fs::read_to_string(model_path).unwrap();

    assert!(content.contains("use serde::{Deserialize, Serialize};"));
    assert!(content.contains("#[derive(Serialize, Deserialize, Clone)]"));
    assert!(content.contains("pub struct User"));
    assert!(content.contains("pub id: String"));
    assert!(content.contains("pub created_at: i64"));
    assert!(content.contains("pub updated_at: Option<i64>"));
    assert!(!content.contains("{{capitalized_resource_name}}"));
    assert!(!content.contains("{{resource_name}}"));
    assert!(!content.contains("{{name}}"));
}

#[test]
fn generate_model_creates_module_directory() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    assert!(project.join("src/modules/user").is_dir());
}

#[test]
fn generate_model_updates_root_modules_mod_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    let root_modules_mod = project.join("src/modules/mod.rs");

    assert!(root_modules_mod.is_file());

    let content = fs::read_to_string(root_modules_mod).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod user;"));
}

#[test]
fn generate_model_creates_resource_module_mod_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    let resource_module_mod = project.join("src/modules/user/mod.rs");

    assert!(resource_module_mod.is_file());

    let content = fs::read_to_string(resource_module_mod).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod model;"));
}

#[test]
fn generate_model_uses_snake_case_for_module_and_pascal_case_for_struct() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user-profile"])
        .assert()
        .success();

    let model_path = project.join("src/modules/user_profile/model.rs");

    assert!(model_path.is_file());

    let content = fs::read_to_string(model_path).unwrap();
    let root_modules_mod = fs::read_to_string(project.join("src/modules/mod.rs")).unwrap();

    assert!(content.contains("pub struct UserProfile"));
    assert!(
        root_modules_mod
            .lines()
            .any(|line| line.trim() == "pub mod user_profile;")
    );
}

#[test]
fn generate_model_fails_if_model_already_exists_without_force() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file already exists"));
}

#[test]
fn generate_model_overwrites_existing_model_with_force() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    let module_path = project.join("src/modules/user");

    fs::create_dir_all(&module_path).unwrap();
    fs::write(module_path.join("model.rs"), "old content").unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(module_path.join("model.rs")).unwrap();

    assert!(!content.contains("old content"));
    assert!(content.contains("pub struct User"));
}

#[test]
fn generate_model_dry_run_creates_nothing() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[CREATE]"));

    assert!(!project.join("src/modules/user").exists());
    assert!(!project.join("src/modules/user/model.rs").exists());
}

#[test]
fn generate_model_does_not_duplicate_root_module_declaration() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(project.join("src/modules/mod.rs")).unwrap();

    let count = content
        .lines()
        .filter(|line| line.trim() == "pub mod user;")
        .count();

    assert_eq!(count, 1);
}

#[test]
fn generate_model_does_not_duplicate_resource_model_declaration() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(project.join("src/modules/user/mod.rs")).unwrap();

    let count = content
        .lines()
        .filter(|line| line.trim() == "pub mod model;")
        .count();

    assert_eq!(count, 1);
}

#[test]
fn generate_model_fails_outside_rust_project() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["generate", "model", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no Rust project found"));
}

#[test]
fn generate_model_fails_inside_non_oxgen_rust_project() {
    let temp_dir = tempdir().unwrap();
    let project = temp_dir.path().join("plain-rust-api");

    fs::create_dir_all(project.join("src")).unwrap();
    fs::write(
        project.join("Cargo.toml"),
        r#"[package]
name = "plain-rust-api"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(project.join("src/main.rs"), "fn main() {}\n").unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no oxgen project found"));
}

#[test]
fn generate_model_rejects_invalid_model_name() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "model", "user profile"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid name"));
}
