use oxgen::cli::parser::parse_args;
use oxgen::core::result::OxgenResult;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> OxgenResult<()> {
    let command = parse_args(std::env::args().skip(1).collect())?;
    command.execute()
}
