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
    OxgenProjectNotFound,
    FileAlreadyExists(String),
    TemplateNotFound(String),
    Io(String),
    InvalidPackageName(String),
    InvalidTemplatePath(String),
    TemplateDirectoryNotFound(String),
    CargoFmtFailed(String),
    ConfusingPackageName(String),
    RustKeywordPackageName(String),
    RustBuiltInPackageName(String),
    GitInitFailed(String),
    InquireMenuInteractionFailed(String),
    CollectionUnsupportedGenerator(String),
    CollectionRequiresMongoDb,
    UnknownDatabase,
}

impl fmt::Display for OxgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OxgenError::InvalidCommand(message) => {
                write!(
                    f,
                    "invalid command\n\nhelp: {}\n\nRun `oxgen --help` to see the available commands.",
                    message
                )
            }

            OxgenError::MissingArgument(argument) => {
                write!(
                    f,
                    "missing required argument `{}`\n\nhelp: provide the missing argument and try again.\n\nexample: `oxgen new my-package`",
                    argument
                )
            }

            OxgenError::UnknownCommand(command) => {
                write!(
                    f,
                    "unknown command `{}`\n\nhelp: this command is not supported by oxgen.\n\nRun `oxgen --help` to see the available commands.",
                    command
                )
            }

            OxgenError::UnknownGenerator(generator) => {
                write!(
                    f,
                    "unknown generator `{}`\n\nhelp: this generator does not exist.\n\nAvailable generators should match the resources supported by the project template.",
                    generator
                )
            }

            OxgenError::InvalidName(name) => {
                write!(
                    f,
                    "invalid name `{}`\n\nhelp: names should use lowercase letters, numbers, `-` or `_`, and should not be empty.",
                    name
                )
            }

            OxgenError::ProjectAlreadyExists(path) => {
                write!(
                    f,
                    "project directory already exists `{}`\n\nhelp: choose another package name or use `--force` to overwrite the existing directory.\n\nwarning: `--force` will delete all existing content in `{}`.",
                    path, path
                )
            }

            OxgenError::ProjectNotFound => {
                write!(
                    f,
                    "no Rust project found in the current directory\n\nhelp: run this command inside a Rust project, or create a new one first with `oxgen new <package-name>`."
                )
            }

            OxgenError::OxgenProjectNotFound => {
                write!(
                    f,
                    "no oxgen project found in the current directory\n\nhelp: run this command at the root of a project created with `oxgen new <package-name>`.\n\nexpected: `.oxgen/config.toml`"
                )
            }

            OxgenError::FileAlreadyExists(path) => {
                write!(
                    f,
                    "file already exists `{}`\n\nhelp: remove the existing file, choose another name, or use a force option if the command supports it.",
                    path
                )
            }

            OxgenError::TemplateNotFound(path) => {
                write!(
                    f,
                    "template file not found `{}`\n\nhelp: make sure the `.ox` template file exists and that the path is correct.",
                    path
                )
            }

            OxgenError::InvalidPackageName(name) => {
                write!(
                    f,
                    "invalid package name `{}`\n\nhelp: package names must start with a letter and should only contain lowercase letters, numbers, `-` or `_`.\n\nexample: `my-api`, `auth_service`, `oxgen-demo`",
                    name
                )
            }

            OxgenError::InvalidTemplatePath(path) => {
                write!(
                    f,
                    "invalid template path `{}`\n\nhelp: provide a valid path to an `.ox` template file or template directory.",
                    path
                )
            }

            OxgenError::TemplateDirectoryNotFound(path) => {
                write!(
                    f,
                    "template directory not found `{}`\n\nhelp: make sure the template directory exists before running the command.",
                    path
                )
            }

            OxgenError::CargoFmtFailed(path) => {
                write!(
                    f,
                    "failed to format generated Rust files in `{}`\n\nhelp: make sure `rustfmt` is installed and that the generated files contain valid Rust code.\n\ntry: `rustup component add rustfmt`",
                    path
                )
            }

            OxgenError::ConfusingPackageName(name) => {
                write!(
                    f,
                    "confusing package name `{}`\n\nhelp: this name may be confused with a Rust standard library crate or module.\n\nChoose a more specific name, for example `{}_app`, `{}_service` or `my_{}`.",
                    name, name, name, name
                )
            }

            OxgenError::RustKeywordPackageName(name) => {
                write!(
                    f,
                    "invalid package name `{}`\n\nhelp: `{}` is a Rust keyword and cannot be used as a package name.\n\nChoose a different name, for example `{}_app` or `my_{}`.",
                    name, name, name, name
                )
            }

            OxgenError::RustBuiltInPackageName(name) => {
                write!(
                    f,
                    "invalid package name `{}`\n\nhelp: `{}` conflicts with a Rust built-in crate or standard library module.\n\nChoose a more specific name, for example `{}_utils`, `{}_app` or `my_{}`.",
                    name, name, name, name, name
                )
            }

            OxgenError::Io(message) => {
                write!(
                    f,
                    "I/O operation failed\n\nhelp: check file permissions, paths, and whether the target directory exists.\n\ndetails: {}",
                    message
                )
            }
            OxgenError::GitInitFailed(err) => {
                write!(
                    f,
                    "failed to initialize git repository\n\nhelp: make sure Git is installed and available in your PATH.\n\ntry: `git --version`\n\ndetails: {}",
                    err
                )
            }
            OxgenError::InquireMenuInteractionFailed(err) => {
                write!(
                    f,
                    "failed using menu\n\nhelp: make sure to choose an option.\n\ndetails: {}",
                    err
                )
            }
            OxgenError::UnknownDatabase => {
                write!(
                    f,
                    "unknown database engine\n\nhelp: choose one of: `mock`, `mongo`.\n\nRun `oxgen help` to see usage and options."
                )
            }
            OxgenError::CollectionUnsupportedGenerator(generator) => {
                write!(
                    f,
                    "the --collection option is not supported by the `{generator}` generator"
                )
            }
            OxgenError::CollectionRequiresMongoDb => {
                write!(
                    f,
                    "the --collection option can only be used with a MongoDB project"
                )
            }
        }
    }
}

impl From<io::Error> for OxgenError {
    fn from(error: io::Error) -> Self {
        OxgenError::Io(error.to_string())
    }
}
