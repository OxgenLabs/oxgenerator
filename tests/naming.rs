use oxgen::core::error::OxgenError;
use oxgen::core::naming::Name;

#[test]
fn name_accepts_simple_lowercase_name() {
    let name = Name::new("user").unwrap();

    assert_eq!(name.raw, "user");
    assert_eq!(name.snake, "user");
    assert_eq!(name.kebab, "user");
    assert_eq!(name.pascal, "User");
}

#[test]
fn name_accepts_kebab_case_name() {
    let name = Name::new("user-profile").unwrap();

    assert_eq!(name.raw, "user-profile");
    assert_eq!(name.snake, "user_profile");
    assert_eq!(name.kebab, "user-profile");
    assert_eq!(name.pascal, "UserProfile");
}

#[test]
fn name_accepts_snake_case_name() {
    let name = Name::new("user_profile").unwrap();

    assert_eq!(name.raw, "user_profile");
    assert_eq!(name.snake, "user_profile");
    assert_eq!(name.kebab, "user-profile");
    assert_eq!(name.pascal, "UserProfile");
}

#[test]
fn name_accepts_name_with_numbers() {
    let name = Name::new("user2-profile").unwrap();

    assert_eq!(name.raw, "user2-profile");
    assert_eq!(name.snake, "user2_profile");
    assert_eq!(name.kebab, "user2-profile");
    assert_eq!(name.pascal, "User2Profile");
}

#[test]
fn name_rejects_empty_name() {
    let result = Name::new("");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_blank_name() {
    let result = Name::new("   ");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_pascal_case_name() {
    let result = Name::new("UserProfile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_camel_case_name() {
    let result = Name::new("userProfile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_starting_with_number() {
    let result = Name::new("1user");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_starting_with_dash() {
    let result = Name::new("-user");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_starting_with_underscore() {
    let result = Name::new("_user");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_with_space() {
    let result = Name::new("user profile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_with_slash() {
    let result = Name::new("user/profile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_with_backslash() {
    let result = Name::new(r"user\profile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_with_dot() {
    let result = Name::new("user.profile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_name_with_special_character() {
    let result = Name::new("user@profile");

    assert!(matches!(result, Err(OxgenError::InvalidName(_))));
}

#[test]
fn name_rejects_rust_builtin_library_name() {
    let result = Name::new("test");

    assert!(matches!(result, Err(OxgenError::RustBuiltInPackageName(_))));
}

#[test]
fn name_rejects_confusing_package_name_std() {
    let result = Name::new("std");

    assert!(matches!(result, Err(OxgenError::ConfusingPackageName(_))));
}

#[test]
fn name_rejects_confusing_package_name_core() {
    let result = Name::new("core");

    assert!(matches!(result, Err(OxgenError::ConfusingPackageName(_))));
}

#[test]
fn name_rejects_confusing_package_name_alloc() {
    let result = Name::new("alloc");

    assert!(matches!(result, Err(OxgenError::ConfusingPackageName(_))));
}

#[test]
fn name_rejects_confusing_package_name_proc_macro() {
    let result = Name::new("proc_macro");

    assert!(matches!(result, Err(OxgenError::ConfusingPackageName(_))));
}

#[test]
fn name_rejects_rust_keyword_async() {
    let result = Name::new("async");

    assert!(matches!(result, Err(OxgenError::RustKeywordPackageName(_))));
}

#[test]
fn name_rejects_rust_keyword_fn() {
    let result = Name::new("fn");

    assert!(matches!(result, Err(OxgenError::RustKeywordPackageName(_))));
}

#[test]
fn name_rejects_rust_keyword_struct() {
    let result = Name::new("struct");

    assert!(matches!(result, Err(OxgenError::RustKeywordPackageName(_))));
}

#[test]
fn name_rejects_rust_keyword_self_lowercase() {
    let result = Name::new("self");

    assert!(matches!(result, Err(OxgenError::RustKeywordPackageName(_))));
}
