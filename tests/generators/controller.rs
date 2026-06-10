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
fn generate_controller_creates_controller_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    let controller_path = project.join("src/modules/user/controller.rs");

    assert!(controller_path.is_file());

    let content = fs::read_to_string(controller_path).unwrap();

    assert!(content.contains("use axum::extract::Path;"));
    assert!(content.contains("use crate::common::response::{ApiResponse, AppResult};"));
    assert!(content.contains("UserDeleteResponse"));
    assert!(content.contains("UserListResponse"));
    assert!(content.contains("UserResponse"));
    assert!(content.contains("model::User"));
    assert!(content.contains("service"));
    assert!(content.contains("fn mock_user_list() -> Vec<User>"));
    assert!(content.contains("id: String::from(\"user_1\")"));
    assert!(content.contains("id: String::from(\"user_2\")"));
    assert!(content.contains("pub async fn get_all_user_handler"));
    assert!(content.contains("pub async fn get_user_handler"));
    assert!(content.contains("pub async fn create_user_handler"));
    assert!(content.contains("pub async fn update_user_handler"));
    assert!(content.contains("pub async fn delete_user_handler"));
    assert!(content.contains("AppResult<UserListResponse>"));
    assert!(content.contains("AppResult<UserResponse>"));
    assert!(content.contains("AppResult<UserDeleteResponse>"));
    assert!(content.contains("service::get_all_user"));
    assert!(content.contains("service::get_user"));
    assert!(content.contains("service::create_user"));
    assert!(content.contains("service::update_user"));
    assert!(content.contains("service::delete_user"));
    assert!(content.contains("Ok(ApiResponse::success(response))"));
    assert!(!content.contains("{{capitalized_resource_name}}"));
    assert!(!content.contains("{{resource_name}}"));
    assert!(!content.contains("{{name}}"));
}

#[test]
fn generate_controller_creates_module_directory() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    assert!(project.join("src/modules/user").is_dir());
}

#[test]
fn generate_controller_updates_root_modules_mod_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    let root_modules_mod = project.join("src/modules/mod.rs");

    assert!(root_modules_mod.is_file());

    let content = fs::read_to_string(root_modules_mod).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod user;"));
}

#[test]
fn generate_controller_creates_resource_module_mod_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    let resource_module_mod = project.join("src/modules/user/mod.rs");

    assert!(resource_module_mod.is_file());

    let content = fs::read_to_string(resource_module_mod).unwrap();

    assert!(
        content
            .lines()
            .any(|line| line.trim() == "pub mod controller;")
    );
}

#[test]
fn generate_controller_uses_snake_case_for_module_and_pascal_case_for_struct() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user-profile"])
        .assert()
        .success();

    let controller_path = project.join("src/modules/user_profile/controller.rs");

    assert!(controller_path.is_file());

    let content = fs::read_to_string(controller_path).unwrap();
    let root_modules_mod = fs::read_to_string(project.join("src/modules/mod.rs")).unwrap();

    assert!(content.contains("UserProfileDeleteResponse"));
    assert!(content.contains("UserProfileListResponse"));
    assert!(content.contains("UserProfileResponse"));
    assert!(content.contains("model::UserProfile"));
    assert!(content.contains("fn mock_user_profile_list() -> Vec<UserProfile>"));
    assert!(content.contains("id: String::from(\"user_profile_1\")"));
    assert!(content.contains("id: String::from(\"user_profile_2\")"));
    assert!(content.contains("pub async fn get_all_user_profile_handler"));
    assert!(content.contains("pub async fn get_user_profile_handler"));
    assert!(content.contains("pub async fn create_user_profile_handler"));
    assert!(content.contains("pub async fn update_user_profile_handler"));
    assert!(content.contains("pub async fn delete_user_profile_handler"));
    assert!(content.contains("service::get_all_user_profile"));
    assert!(content.contains("service::get_user_profile"));
    assert!(content.contains("service::create_user_profile"));
    assert!(content.contains("service::update_user_profile"));
    assert!(content.contains("service::delete_user_profile"));

    assert!(
        root_modules_mod
            .lines()
            .any(|line| line.trim() == "pub mod user_profile;")
    );
}

#[test]
fn generate_controller_fails_if_controller_already_exists_without_force() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("file already exists"));
}

#[test]
fn generate_controller_overwrites_existing_controller_with_force() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    let module_path = project.join("src/modules/user");

    fs::create_dir_all(&module_path).unwrap();
    fs::write(module_path.join("controller.rs"), "old content").unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(module_path.join("controller.rs")).unwrap();

    assert!(!content.contains("old content"));
    assert!(content.contains("pub async fn get_all_user_handler"));
    assert!(content.contains("pub async fn get_user_handler"));
    assert!(content.contains("pub async fn create_user_handler"));
    assert!(content.contains("pub async fn update_user_handler"));
    assert!(content.contains("pub async fn delete_user_handler"));
}

#[test]
fn generate_controller_dry_run_creates_nothing() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[CREATE]"))
        .stdout(predicate::str::contains(
            "[ADD] `pub mod user;` to src/modules/mod.rs",
        ))
        .stdout(predicate::str::contains(
            "[ADD] `pub mod controller;` to src/modules/user/mod.rs",
        ));

    assert!(!project.join("src/modules/user").exists());
    assert!(!project.join("src/modules/user/controller.rs").exists());
}

#[test]
fn generate_controller_does_not_duplicate_root_module_declaration() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user", "--force"])
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
fn generate_controller_does_not_duplicate_resource_controller_declaration() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user"])
        .assert()
        .success();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(project.join("src/modules/user/mod.rs")).unwrap();

    let count = content
        .lines()
        .filter(|line| line.trim() == "pub mod controller;")
        .count();

    assert_eq!(count, 1);
}

#[test]
fn generate_controller_fails_outside_rust_project() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["generate", "controller", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no Rust project found"));
}

#[test]
fn generate_controller_fails_inside_non_oxgen_rust_project() {
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
        .args(["generate", "controller", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no oxgen project found"));
}

#[test]
fn generate_controller_rejects_invalid_controller_name() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");

    Command::cargo_bin("oxgen")
        .unwrap()
        .current_dir(&project)
        .args(["generate", "controller", "user profile"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid name"));
}
