use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

#[derive(Debug, Clone)]
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

fn validate_name(input: &str) -> OxgenResult<()> {
    if input.trim().is_empty() {
        return Err(OxgenError::InvalidName(input.to_string()));
    }

    if input.chars().any(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '-')) {
        return Err(OxgenError::InvalidName(input.to_string()));
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
