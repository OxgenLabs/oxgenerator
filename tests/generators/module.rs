use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use oxgen::core::error::OxgenError;
use oxgen::core::generator::Generator;
use oxgen::generators::module::ModuleGenerator;
use tempfile::tempdir;

static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

struct CurrentDirGuard {
    previous_dir: PathBuf,
}

impl CurrentDirGuard {
    fn change_to(path: &Path) -> Self {
        let previous_dir = env::current_dir().unwrap();
        env::set_current_dir(path).unwrap();

        Self { previous_dir }
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        env::set_current_dir(&self.previous_dir).unwrap();
    }
}

fn create_minimal_oxgen_project() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();
    let project = temp_dir.path();

    fs::create_dir_all(project.join(".oxgen")).unwrap();
    fs::create_dir_all(project.join("src/modules")).unwrap();
    fs::create_dir_all(project.join("src/routes")).unwrap();

    fs::write(
        project.join(".oxgen/config.toml"),
        r#"generator = "oxgen"
"#,
    )
    .unwrap();

    fs::write(
        project.join("Cargo.toml"),
        r#"[package]
name = "test-api"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    fs::write(project.join("src/modules/mod.rs"), "pub mod health;\n").unwrap();
    fs::write(project.join("src/routes/mod.rs"), "pub mod health;\n").unwrap();

    fs::write(
        project.join("src/main.rs"),
        r#"use test_api::{
    routes::health::health_routes,
};

use axum::Router;

async fn bootstrap() {
    let health_routes = health_routes();

    let app = Router::new()
        .merge(health_routes)
        .with_state(());
}
"#,
    )
    .unwrap();

    temp_dir
}

fn generate_module(name: &str, force: bool, dry_run: bool) -> Result<(), OxgenError> {
    ModuleGenerator::new(name.to_string(), force, dry_run).generate()
}

fn count_lines_equal(content: &str, expected: &str) -> usize {
    content
        .lines()
        .filter(|line| line.trim() == expected)
        .count()
}

fn count_occurrences(content: &str, pattern: &str) -> usize {
    content.matches(pattern).count()
}

#[test]
fn generate_module_creates_all_expected_files() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    assert!(temp_dir.path().join("src/modules/user").is_dir());
    assert!(temp_dir.path().join("src/modules/user/model.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user/dto.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user/service.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user/controller.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user/mod.rs").is_file());
    assert!(temp_dir.path().join("src/routes/user.rs").is_file());
}

#[test]
fn generate_module_updates_modules_mod_with_resource_module() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/modules/mod.rs")).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod health;"));
    assert!(content.lines().any(|line| line.trim() == "pub mod user;"));
}

#[test]
fn generate_module_updates_resource_mod_with_all_module_parts() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/modules/user/mod.rs")).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod model;"));
    assert!(content.lines().any(|line| line.trim() == "pub mod dto;"));
    assert!(content.lines().any(|line| line.trim() == "pub mod service;"));
    assert!(content.lines().any(|line| line.trim() == "pub mod controller;"));
}

#[test]
fn generate_module_updates_routes_mod_file() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/routes/mod.rs")).unwrap();

    assert!(content.lines().any(|line| line.trim() == "pub mod health;"));
    assert!(content.lines().any(|line| line.trim() == "pub mod user;"));
}

#[test]
fn generate_module_updates_main_file_with_route_import_and_merge() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/main.rs")).unwrap();

    assert!(content.contains("health::health_routes,"));
    assert!(content.contains("user::user_routes,"));
    assert!(content.contains(".merge(health_routes)"));
    assert!(content.contains(".merge(user_routes())"));
}

#[test]
fn generate_module_replaces_placeholders_in_generated_files() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let generated_files = [
        temp_dir.path().join("src/modules/user/model.rs"),
        temp_dir.path().join("src/modules/user/dto.rs"),
        temp_dir.path().join("src/modules/user/service.rs"),
        temp_dir.path().join("src/modules/user/controller.rs"),
        temp_dir.path().join("src/routes/user.rs"),
    ];

    for file in generated_files {
        let content = fs::read_to_string(file).unwrap();

        assert!(!content.contains("{{name}}"));
        assert!(!content.contains("{{resource_name}}"));
        assert!(!content.contains("{{capitalized_resource_name}}"));
    }
}

#[test]
fn generate_module_generates_expected_model_content() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/modules/user/model.rs")).unwrap();

    assert!(content.contains("use serde::{Deserialize, Serialize};"));
    assert!(content.contains("#[derive(Serialize, Deserialize, Clone)]"));
    assert!(content.contains("pub struct User"));
    assert!(content.contains("pub id: String"));
    assert!(content.contains("pub created_at: i64"));
    assert!(content.contains("pub updated_at: Option"));
}

#[test]
fn generate_module_generates_expected_dto_content() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/modules/user/dto.rs")).unwrap();

    assert!(content.contains("use serde::Serialize;"));
    assert!(content.contains("use super::model::User;"));
    assert!(content.contains("pub struct UserResponse"));
    assert!(content.contains("pub struct UserListResponse"));
    assert!(content.contains("pub struct UserDeleteResponse"));
    assert!(content.contains("impl From"));
}

#[test]
fn generate_module_generates_expected_service_content() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/modules/user/service.rs")).unwrap();

    assert!(content.contains("use crate::common::error::AppError;"));
    assert!(content.contains("UserDeleteResponse"));
    assert!(content.contains("UserListResponse"));
    assert!(content.contains("UserResponse"));
    assert!(content.contains("model::User"));
    assert!(content.contains("pub async fn get_all_user"));
    assert!(content.contains("pub async fn get_user"));
    assert!(content.contains("pub async fn create_user"));
    assert!(content.contains("pub async fn update_user"));
    assert!(content.contains("pub async fn delete_user"));
    assert!(content.contains("AppError::not_found(\"User not found\")"));
}

#[test]
fn generate_module_generates_expected_controller_content() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content =
        fs::read_to_string(temp_dir.path().join("src/modules/user/controller.rs")).unwrap();

    assert!(content.contains("use axum::extract::Path;"));
    assert!(content.contains("use crate::common::response::{ApiResponse, AppResult};"));
    assert!(content.contains("UserDeleteResponse"));
    assert!(content.contains("UserListResponse"));
    assert!(content.contains("UserResponse"));
    assert!(content.contains("model::User"));
    assert!(content.contains("service"));
    assert!(content.contains("fn mock_user_list() -> Vec"));
    assert!(content.contains("id: String::from(\"user_1\")"));
    assert!(content.contains("id: String::from(\"user_2\")"));
    assert!(content.contains("pub async fn get_all_user_handler"));
    assert!(content.contains("pub async fn get_user_handler"));
    assert!(content.contains("pub async fn create_user_handler"));
    assert!(content.contains("pub async fn update_user_handler"));
    assert!(content.contains("pub async fn delete_user_handler"));
    assert!(content.contains("service::get_all_user"));
    assert!(content.contains("service::get_user"));
    assert!(content.contains("service::create_user"));
    assert!(content.contains("service::update_user"));
    assert!(content.contains("service::delete_user"));
    assert!(content.contains("Ok(ApiResponse::success(response))"));
}

#[test]
fn generate_module_generates_expected_route_content() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let content = fs::read_to_string(temp_dir.path().join("src/routes/user.rs")).unwrap();

    assert!(content.contains("pub fn user_routes()"));
    assert!(content.contains("modules::user::controller"));
    assert!(content.contains("create_user_handler"));
    assert!(content.contains("delete_user_handler"));
    assert!(content.contains("get_user_handler"));
    assert!(content.contains("get_all_user_handler"));
    assert!(content.contains("update_user_handler"));
    assert!(content.contains(r#".nest("/user", routes)"#));
}

#[test]
fn generate_module_uses_snake_case_for_files_and_pascal_case_for_types() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user-profile", false, false).unwrap();

    assert!(temp_dir.path().join("src/modules/user_profile/model.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user_profile/dto.rs").is_file());
    assert!(temp_dir.path().join("src/modules/user_profile/service.rs").is_file());
    assert!(temp_dir
        .path()
        .join("src/modules/user_profile/controller.rs")
        .is_file());
    assert!(temp_dir.path().join("src/routes/user_profile.rs").is_file());

    let model_content =
        fs::read_to_string(temp_dir.path().join("src/modules/user_profile/model.rs")).unwrap();
    let dto_content =
        fs::read_to_string(temp_dir.path().join("src/modules/user_profile/dto.rs")).unwrap();
    let service_content =
        fs::read_to_string(temp_dir.path().join("src/modules/user_profile/service.rs")).unwrap();
    let controller_content =
        fs::read_to_string(temp_dir.path().join("src/modules/user_profile/controller.rs")).unwrap();
    let route_content =
        fs::read_to_string(temp_dir.path().join("src/routes/user_profile.rs")).unwrap();
    let modules_mod_content = fs::read_to_string(temp_dir.path().join("src/modules/mod.rs")).unwrap();
    let routes_mod_content = fs::read_to_string(temp_dir.path().join("src/routes/mod.rs")).unwrap();
    let main_content = fs::read_to_string(temp_dir.path().join("src/main.rs")).unwrap();

    assert!(model_content.contains("pub struct UserProfile"));

    assert!(dto_content.contains("pub struct UserProfileResponse"));
    assert!(dto_content.contains("pub struct UserProfileListResponse"));
    assert!(dto_content.contains("pub struct UserProfileDeleteResponse"));

    assert!(service_content.contains("model::UserProfile"));
    assert!(service_content.contains("pub async fn get_all_user_profile"));
    assert!(service_content.contains("pub async fn get_user_profile"));
    assert!(service_content.contains("pub async fn create_user_profile"));
    assert!(service_content.contains("pub async fn update_user_profile"));
    assert!(service_content.contains("pub async fn delete_user_profile"));
    assert!(service_content.contains("AppError::not_found(\"UserProfile not found\")"));

    assert!(controller_content.contains("model::UserProfile"));
    assert!(controller_content.contains("fn mock_user_profile_list() -> Vec"));
    assert!(controller_content.contains("id: String::from(\"user_profile_1\")"));
    assert!(controller_content.contains("id: String::from(\"user_profile_2\")"));
    assert!(controller_content.contains("pub async fn get_all_user_profile_handler"));
    assert!(controller_content.contains("pub async fn get_user_profile_handler"));
    assert!(controller_content.contains("pub async fn create_user_profile_handler"));
    assert!(controller_content.contains("pub async fn update_user_profile_handler"));
    assert!(controller_content.contains("pub async fn delete_user_profile_handler"));

    assert!(route_content.contains("pub fn user_profile_routes()"));
    assert!(route_content.contains("modules::user_profile::controller"));
    assert!(route_content.contains("create_user_profile_handler"));
    assert!(route_content.contains("delete_user_profile_handler"));
    assert!(route_content.contains("get_user_profile_handler"));
    assert!(route_content.contains("get_all_user_profile_handler"));
    assert!(route_content.contains("update_user_profile_handler"));
    assert!(route_content.contains(r#".nest("/user_profile", routes)"#));

    assert!(modules_mod_content
        .lines()
        .any(|line| line.trim() == "pub mod user_profile;"));
    assert!(routes_mod_content
        .lines()
        .any(|line| line.trim() == "pub mod user_profile;"));
    assert!(main_content.contains("user_profile::user_profile_routes,"));
    assert!(main_content.contains(".merge(user_profile_routes())"));
}

#[test]
fn generate_module_fails_if_module_already_exists_without_force() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();

    let result = generate_module("user", false, false);

    assert!(matches!(result, Err(OxgenError::FileAlreadyExists(path)) if path.ends_with("src/modules/user/model.rs")));
}

#[test]
fn generate_module_overwrites_existing_files_with_force() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    fs::create_dir_all("src/modules/user").unwrap();

    fs::write("src/modules/user/model.rs", "old model").unwrap();
    fs::write("src/modules/user/dto.rs", "old dto").unwrap();
    fs::write("src/modules/user/service.rs", "old service").unwrap();
    fs::write("src/modules/user/controller.rs", "old controller").unwrap();
    fs::write("src/routes/user.rs", "old route").unwrap();

    generate_module("user", true, false).unwrap();

    let model_content = fs::read_to_string("src/modules/user/model.rs").unwrap();
    let dto_content = fs::read_to_string("src/modules/user/dto.rs").unwrap();
    let service_content = fs::read_to_string("src/modules/user/service.rs").unwrap();
    let controller_content = fs::read_to_string("src/modules/user/controller.rs").unwrap();
    let route_content = fs::read_to_string("src/routes/user.rs").unwrap();

    assert!(!model_content.contains("old model"));
    assert!(!dto_content.contains("old dto"));
    assert!(!service_content.contains("old service"));
    assert!(!controller_content.contains("old controller"));
    assert!(!route_content.contains("old route"));

    assert!(model_content.contains("pub struct User"));
    assert!(dto_content.contains("pub struct UserResponse"));
    assert!(service_content.contains("pub async fn get_all_user"));
    assert!(controller_content.contains("pub async fn get_all_user_handler"));
    assert!(route_content.contains("pub fn user_routes()"));
}

#[test]
fn generate_module_dry_run_creates_nothing_and_does_not_update_existing_files() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    let modules_mod_before = fs::read_to_string("src/modules/mod.rs").unwrap();
    let routes_mod_before = fs::read_to_string("src/routes/mod.rs").unwrap();
    let main_before = fs::read_to_string("src/main.rs").unwrap();

    generate_module("user", false, true).unwrap();

    assert!(!Path::new("src/modules/user").exists());
    assert!(!Path::new("src/modules/user/model.rs").exists());
    assert!(!Path::new("src/modules/user/dto.rs").exists());
    assert!(!Path::new("src/modules/user/service.rs").exists());
    assert!(!Path::new("src/modules/user/controller.rs").exists());
    assert!(!Path::new("src/routes/user.rs").exists());

    let modules_mod_after = fs::read_to_string("src/modules/mod.rs").unwrap();
    let routes_mod_after = fs::read_to_string("src/routes/mod.rs").unwrap();
    let main_after = fs::read_to_string("src/main.rs").unwrap();

    assert_eq!(modules_mod_before, modules_mod_after);
    assert_eq!(routes_mod_before, routes_mod_after);
    assert_eq!(main_before, main_after);
}

#[test]
fn generate_module_force_does_not_duplicate_module_declarations() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();
    generate_module("user", true, false).unwrap();
    generate_module("user", true, false).unwrap();

    let modules_mod_content = fs::read_to_string("src/modules/mod.rs").unwrap();
    let resource_mod_content = fs::read_to_string("src/modules/user/mod.rs").unwrap();

    assert_eq!(count_lines_equal(&modules_mod_content, "pub mod user;"), 1);
    assert_eq!(count_lines_equal(&resource_mod_content, "pub mod model;"), 1);
    assert_eq!(count_lines_equal(&resource_mod_content, "pub mod dto;"), 1);
    assert_eq!(count_lines_equal(&resource_mod_content, "pub mod service;"), 1);
    assert_eq!(
        count_lines_equal(&resource_mod_content, "pub mod controller;"),
        1
    );
}

#[test]
fn generate_module_force_does_not_duplicate_route_declaration_or_main_merge() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    generate_module("user", false, false).unwrap();
    generate_module("user", true, false).unwrap();
    generate_module("user", true, false).unwrap();

    let routes_mod_content = fs::read_to_string("src/routes/mod.rs").unwrap();
    let main_content = fs::read_to_string("src/main.rs").unwrap();

    assert_eq!(count_lines_equal(&routes_mod_content, "pub mod user;"), 1);
    assert_eq!(count_occurrences(&main_content, "user::user_routes"), 1);
    assert_eq!(count_occurrences(&main_content, ".merge(user_routes())"), 1);
}

#[test]
fn generate_module_fails_outside_rust_project() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = tempdir().unwrap();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    let result = generate_module("user", false, false);

    assert!(matches!(result, Err(OxgenError::ProjectNotFound)));
}

#[test]
fn generate_module_fails_inside_non_oxgen_rust_project() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = tempdir().unwrap();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    fs::create_dir_all("src").unwrap();

    fs::write(
        "Cargo.toml",
        r#"[package]
name = "plain-rust-api"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    fs::write("src/main.rs", "fn main() {}\n").unwrap();

    let result = generate_module("user", false, false);

    assert!(matches!(result, Err(OxgenError::OxgenProjectNotFound)));
}

#[test]
fn generate_module_rejects_invalid_module_name() {
    let _lock = CURRENT_DIR_LOCK.lock().unwrap();
    let temp_dir = create_minimal_oxgen_project();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    let result = generate_module("user profile", false, false);

    assert!(matches!(result, Err(OxgenError::InvalidName(name)) if name == "user profile"));
}
