use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn create_oxgen_project(project_name: &str) -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["new", project_name])
        .assert()
        .success();

    temp_dir
}

#[test]
fn generate_dto_creates_dto_file() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    let dto_path = project.join("src/modules/user/dto.rs");

    assert!(dto_path.is_file());

    let content = fs::read_to_string(dto_path).unwrap();

    assert!(content.contains("use serde::Serialize;"));
    assert!(content.contains("use super::model::User;"));
    assert!(content.contains("pub struct UserResponse"));
    assert!(content.contains("pub id: String"));
    assert!(content.contains("pub created_at: i64"));
    assert!(content.contains("pub updated_at: Option<i64>"));
    assert!(content.contains("impl From<User> for UserResponse"));
    assert!(content.contains("pub struct UserDeleteResponse"));
    assert!(!content.contains("{{capitalized_resource_name}}"));
    assert!(!content.contains("{{resource_name}}"));
    assert!(!content.contains("{{name}}"));
}

#[test]
fn generate_dto_creates_module_directory() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    assert!(project.join("src/modules/user").is_dir());
}

#[test]
fn generate_dto_updates_root_modules_mod_file() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    let root_modules_mod = project.join("src/modules/mod.rs");

    assert!(root_modules_mod.is_file());

    let content = fs::read_to_string(root_modules_mod).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod user;"));
}

#[test]
fn generate_dto_creates_resource_module_mod_file() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    let resource_module_mod = project.join("src/modules/user/mod.rs");

    assert!(resource_module_mod.is_file());

    let content = fs::read_to_string(resource_module_mod).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod dto;"));
}

#[test]
fn generate_dto_uses_snake_case_for_module_and_pascal_case_for_struct() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user-profile"])
        .assert()
        .success();

    let dto_path = project.join("src/modules/user_profile/dto.rs");

    assert!(dto_path.is_file());

    let content = fs::read_to_string(dto_path).unwrap();
    let root_modules_mod = fs::read_to_string(project.join("src/modules/mod.rs")).unwrap();

    assert!(content.contains("pub struct UserProfileResponse"));
    assert!(content.contains("impl From<UserProfile> for UserProfileResponse"));
    assert!(content.contains("pub struct UserProfileDeleteResponse"));
    assert!(
        root_modules_mod
            .lines()
            .any(|line| line.trim() == "pub mod user_profile;")
    );
}

#[test]
fn generate_dto_fails_if_dto_already_exists_without_force() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file already exists"));
}

#[test]
fn generate_dto_overwrites_existing_dto_with_force() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    let module_path = project.join("src/modules/user");

    fs::create_dir_all(&module_path).unwrap();
    fs::write(module_path.join("dto.rs"), "old content").unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(module_path.join("dto.rs")).unwrap();

    assert!(!content.contains("old content"));
    assert!(content.contains("pub struct UserResponse"));
}

#[test]
fn generate_dto_dry_run_creates_nothing() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[CREATE]"))
        .stdout(predicate::str::contains(
            "[ADD] `pub mod user;` to src/modules/mod.rs",
        ))
        .stdout(predicate::str::contains(
            "[ADD] `pub mod dto;` to src/modules/user/mod.rs",
        ));

    assert!(!project.join("src/modules/user").exists());
    assert!(!project.join("src/modules/user/dto.rs").exists());
}

#[test]
fn generate_dto_does_not_duplicate_root_module_declaration() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user", "--force"])
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
fn generate_dto_does_not_duplicate_resource_dto_declaration() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(project.join("src/modules/user/mod.rs")).unwrap();

    let count = content
        .lines()
        .filter(|line| line.trim() == "pub mod dto;")
        .count();

    assert_eq!(count, 1);
}

#[test]
fn generate_dto_fails_outside_rust_project() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["generate", "dto", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no Rust project found"));
}

#[test]
fn generate_dto_fails_inside_non_oxgen_rust_project() {
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
        .args(["generate", "dto", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no oxgen project found"));
}

#[test]
fn generate_dto_rejects_invalid_dto_name() {
    let temp_dir = create_oxgen_project("test-api");
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "dto", "user profile"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid name"));
}
