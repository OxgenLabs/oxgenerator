use std::fmt;
use std::io;

#[derive(Debug)]
pub enum OxgenError {
    InvalidCommand(String),
    MissingArgument(String),
    UnknownCommand(String),
    UnknownGenerator(String),
    InvalidName(String),
    ProjectAlreadyExists(String),
    ProjectNotFound,
    FileAlreadyExists(String),
    TemplateNotFound(String),
    Io(String),
    InvalidProjectName(String),
    InvalidTemplatePath(String),
    TemplateDirectoryNotFound(String),
}

impl fmt::Display for OxgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OxgenError::InvalidCommand(message) => {
                write!(f, "Invalid command: {}", message)
            }
            OxgenError::MissingArgument(argument) => {
                write!(f, "Missing argument: {}", argument)
            }
            OxgenError::UnknownCommand(command) => {
                write!(f, "Unknown command: {}", command)
            }
            OxgenError::UnknownGenerator(generator) => {
                write!(f, "Unknown generator: {}", generator)
            }
            OxgenError::InvalidName(name) => {
                write!(f, "Invalid name: {}", name)
            }
            OxgenError::ProjectAlreadyExists(path) => {
                write!(
                    f,
                    "Project already exists: `{}/`. Use `--force` to overwrite it, but be careful: this will delete all existing content in `{}/`.",
                    path, path
                )
            }
            OxgenError::ProjectNotFound => {
                write!(f, "No Rust project found in the current directory")
            }
            OxgenError::FileAlreadyExists(path) => {
                write!(f, "File already exists: {}", path)
            }
            OxgenError::TemplateNotFound(path) => {
                write!(f, "Template not found: {}", path)
            }
            OxgenError::InvalidProjectName(name) => {
                write!(f, "Invalid project name: {}", name)
            }
            OxgenError::InvalidTemplatePath(path) => {
                write!(f, "Invalid template path: {}", path)
            }
            OxgenError::TemplateDirectoryNotFound(path) => {
                write!(f, "Template directory not found: {}", path)
            }
            OxgenError::Io(message) => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl From<io::Error> for OxgenError {
    fn from(error: io::Error) -> Self {
        OxgenError::Io(error.to_string())
    }
}
