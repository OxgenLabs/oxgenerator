use oxgen::core::cli::parse_cli;
use oxgen::core::error::OxgeneratorError;
use oxgen::core::result::OxgeneratorResult;

fn main() -> OxgeneratorResult<(), OxgeneratorError> {
    parse_cli()
}

