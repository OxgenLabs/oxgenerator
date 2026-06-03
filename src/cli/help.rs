pub fn print_help() {
    println!(
        r#"Oxgen - Rust project generator

USAGE:
    oxgen <COMMAND> [OPTIONS]

COMMANDS:
    new <name>              Create a new Rust API project
    generate <type> <name>  Generate a module inside the current project
    g <type> <name>         Alias for generate
    help                    Print this help message
    version                 Print the Oxgen version
    update                  Update Oxgen version

GENERATOR TYPES:
    module                  Generate a complete module structure
    controller              Generate a controller file
    service                 Generate a service file
    model                   Generate a model file
    dto                     Generate a dto file
    route                   Generate a route file

OPTIONS:
    -h, --help              Print this help message
    -v, --version           Print the Oxgen version
"#
    );
}
