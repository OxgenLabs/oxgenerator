use std::fs;
use std::path::Path;

use assert_cmd::Command;
use oxgen::core::error::OxgenError;
use oxgen::core::generator::Generator;
use oxgen::core::naming::Name;
use oxgen::generators::route::RouteGenerator;
use tempfile::tempdir;

use crate::common::current_dir::CurrentDirGuard;

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

fn count_occurrences(content: &str, pattern: &str) -> usize {
    content.matches(pattern).count()
}

#[test]
fn route_generator_creates_route_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    generator.generate().unwrap();

    let route_path = project.join("src/routes/user.rs");

    assert!(route_path.exists());

    let content = fs::read_to_string(route_path).unwrap();

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
fn route_generator_replaces_template_placeholders() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("pricing").unwrap(), false, false);

    generator.generate().unwrap();

    let content = fs::read_to_string(project.join("src/routes/pricing.rs")).unwrap();

    assert!(content.contains("pub fn pricing_routes()"));
    assert!(content.contains("modules::pricing::controller"));
    assert!(content.contains("create_pricing_handler"));
    assert!(content.contains("delete_pricing_handler"));
    assert!(content.contains("get_pricing_handler"));
    assert!(content.contains("get_all_pricing_handler"));
    assert!(content.contains("update_pricing_handler"));
    assert!(content.contains(r#".nest("/pricing", routes)"#));

    assert!(!content.contains("{{name}}"));
    assert!(!content.contains("{{resource_name}}"));
    assert!(!content.contains("{{capitalized_resource_name}}"));
}

#[test]
fn route_generator_updates_routes_mod_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    generator.generate().unwrap();

    let content = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();

    assert!(content.contains("pub mod health;"));
    assert!(content.contains("pub mod user;"));
}

#[test]
fn route_generator_updates_main_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    generator.generate().unwrap();

    let content = fs::read_to_string(project.join("src/main.rs")).unwrap();

    assert!(content.contains("routes::{"));
    assert!(content.contains("health::health_routes,"));
    assert!(content.contains("user::user_routes,"));
    assert!(content.contains(".merge(health_routes)"));
    assert!(content.contains(".merge(user_routes())"));
}

#[test]
fn route_generator_returns_file_already_exists_without_touching_main_or_routes_mod() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    fs::write(project.join("src/routes/user.rs"), "existing route").unwrap();

    let main_before = fs::read_to_string(project.join("src/main.rs")).unwrap();
    let routes_mod_before = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    let result = generator.generate();

    assert!(matches!(
        result,
        Err(OxgenError::FileAlreadyExists(path)) if Path::new(&path) == Path::new("src/routes/user.rs")
    ));

    let main_after = fs::read_to_string(project.join("src/main.rs")).unwrap();
    let routes_mod_after = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();
    let route_content = fs::read_to_string(project.join("src/routes/user.rs")).unwrap();

    assert_eq!(main_before, main_after);
    assert_eq!(routes_mod_before, routes_mod_after);
    assert_eq!(route_content, "existing route");
}

#[test]
fn route_generator_force_overwrites_existing_route_file() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    fs::write(project.join("src/routes/user.rs"), "existing route").unwrap();

    let generator = RouteGenerator::new(Name::new("user").unwrap(), true, false);

    generator.generate().unwrap();

    let content = fs::read_to_string(project.join("src/routes/user.rs")).unwrap();

    assert_ne!(content, "existing route");
    assert!(content.contains("pub fn user_routes()"));
}

#[test]
fn route_generator_force_does_not_duplicate_main_import_or_merge() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    generator.generate().unwrap();

    let forced_generator = RouteGenerator::new(Name::new("user").unwrap(), true, false);

    forced_generator.generate().unwrap();
    forced_generator.generate().unwrap();

    let main_content = fs::read_to_string(project.join("src/main.rs")).unwrap();
    let routes_mod_content = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();

    assert_eq!(count_occurrences(&main_content, "user::user_routes"), 1);
    assert_eq!(count_occurrences(&main_content, ".merge(user_routes())"), 1);
    assert_eq!(count_occurrences(&routes_mod_content, "pub mod user;"), 1);
}

#[test]
fn route_generator_dry_run_does_not_create_or_update_files() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let main_before = fs::read_to_string(project.join("src/main.rs")).unwrap();
    let routes_mod_before = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, true);

    generator.generate().unwrap();

    assert!(!project.join("src/routes/user.rs").exists());

    let main_after = fs::read_to_string(project.join("src/main.rs")).unwrap();
    let routes_mod_after = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();

    assert_eq!(main_before, main_after);
    assert_eq!(routes_mod_before, routes_mod_after);
}

#[test]
fn route_generator_supports_kebab_case_resource_name() {
    let temp_dir = create_oxgen_project();
    let project = temp_dir.path().join("test-api");
    let _guard = CurrentDirGuard::change_to(&project);

    let generator = RouteGenerator::new(Name::new("user-profile").unwrap(), false, false);

    generator.generate().unwrap();

    let route_path = project.join("src/routes/user_profile.rs");
    let route_content = fs::read_to_string(route_path).unwrap();
    let routes_mod_content = fs::read_to_string(project.join("src/routes/mod.rs")).unwrap();
    let main_content = fs::read_to_string(project.join("src/main.rs")).unwrap();

    assert!(route_content.contains("pub fn user_profile_routes()"));
    assert!(route_content.contains("modules::user_profile::controller"));
    assert!(route_content.contains("create_user_profile_handler"));
    assert!(route_content.contains("delete_user_profile_handler"));
    assert!(route_content.contains("get_user_profile_handler"));
    assert!(route_content.contains("get_all_user_profile_handler"));
    assert!(route_content.contains("update_user_profile_handler"));
    assert!(route_content.contains(r#".nest("/user_profile", routes)"#));

    assert!(routes_mod_content.contains("pub mod user_profile;"));
    assert!(main_content.contains("user_profile::user_profile_routes,"));
    assert!(main_content.contains(".merge(user_profile_routes())"));
}

#[test]
fn route_generator_fails_outside_rust_project() {
    let temp_dir = tempfile::tempdir().unwrap();
    let _guard = CurrentDirGuard::change_to(temp_dir.path());

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    let result = generator.generate();

    assert!(matches!(result, Err(OxgenError::ProjectNotFound)));
}

#[test]
fn route_generator_fails_outside_oxgen_project() {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    fs::create_dir_all(root.join("src/routes")).unwrap();

    fs::write(
        root.join("Cargo.toml"),
        r#"[package]
name = "test-app"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    fs::write(
        root.join("src/routes/mod.rs"),
        r#"pub mod health;
"#,
    )
    .unwrap();

    fs::write(
        root.join("src/main.rs"),
        r#"use test_app::{
    routes::health::health_routes,
    state::{AppState, SecretStore},
};

fn main() {}
"#,
    )
    .unwrap();

    let _guard = CurrentDirGuard::change_to(root);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    let result = generator.generate();

    assert!(matches!(result, Err(OxgenError::OxgenProjectNotFound)));
}

#[test]
fn route_generator_returns_project_not_found_when_main_file_is_missing() {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    fs::create_dir_all(root.join(".oxgen")).unwrap();
    fs::create_dir_all(root.join("src/routes")).unwrap();

    fs::write(
        root.join(".oxgen/config.toml"),
        r#"generator = "oxgen"
database = "mock"
"#,
    )
    .unwrap();

    fs::write(
        root.join("Cargo.toml"),
        r#"[package]
name = "test-app"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    fs::write(
        root.join("src/routes/mod.rs"),
        r#"pub mod health;
"#,
    )
    .unwrap();

    let _guard = CurrentDirGuard::change_to(root);

    let generator = RouteGenerator::new(Name::new("user").unwrap(), false, false);

    let result = generator.generate();

    assert!(matches!(result, Err(OxgenError::ProjectNotFound)));
}
