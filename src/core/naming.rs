use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

const BUILT_IN_LIBRARY: &[&str] = &["test"];
const CONFUSING_PACKAGE_NAMES: &[&str] = &["std", "core", "alloc", "proc_macro"];
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub raw: String,
    pub snake: String,
    pub pascal: String,
    pub kebab: String,
}

impl Name {
    pub fn new(input: &str) -> OxgenResult<Self> {
        validate_name(input)?;

        let words = split_words(input);

        let snake = words.join("_");
        let kebab = words.join("-");
        let pascal = words
            .iter()
            .map(|word| capitalize(word))
            .collect::<Vec<String>>()
            .join("");

        Ok(Self {
            raw: input.to_string(),
            snake,
            pascal,
            kebab,
        })
    }
}

pub fn validate_name(input: &str) -> OxgenResult<()> {
    if input.trim().is_empty() {
        return Err(OxgenError::InvalidName(input.to_string()));
    }

    let Some(first_char) = input.chars().next() else {
        return Err(OxgenError::InvalidName(input.to_string()));
    };

    if !first_char.is_ascii_alphabetic() {
        return Err(OxgenError::InvalidName(input.to_string()));
    }

    let is_valid = input
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || char == '-' || char == '_');

    if !is_valid {
        return Err(OxgenError::InvalidName(input.to_string()));
    }

    if BUILT_IN_LIBRARY.contains(&input) {
        return Err(OxgenError::RustBuiltInPackageName(input.to_string()));
    }

    if CONFUSING_PACKAGE_NAMES.contains(&input) {
        return Err(OxgenError::ConfusingPackageName(input.to_string()));
    }

    if RUST_KEYWORDS.contains(&input) {
        return Err(OxgenError::RustKeywordPackageName(input.to_string()));
    }
    Ok(())
}

fn split_words(input: &str) -> Vec<String> {
    input
        .replace('-', "_")
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| part.to_lowercase())
        .collect()
}

fn capitalize(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(first) => {
            let first = first.to_uppercase().collect::<String>();
            let rest = chars.collect::<String>();
            format!("{}{}", first, rest)
        }
        None => String::new(),
    }
}
