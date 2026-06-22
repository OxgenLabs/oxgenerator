pub fn print_help() {
    println!(
        r#"Oxgen - Rust API project generator

USAGE:
    oxgen <COMMAND> [OPTIONS]
    oxgen generate <GENERATOR> <NAME> [OPTIONS]
    oxgen g <GENERATOR> <NAME> [OPTIONS]

COMMANDS:
    new <NAME>                  Create a new Rust API project
    generate <GENERATOR> <NAME> Generate a resource in the current Oxgen project
    g <GENERATOR> <NAME>        Alias for generate
    help                        Print this help message
    version                     Print the installed Oxgen version
    update                      Update Oxgen to the latest available version

GENERATORS:
    module, mod                 Generate a complete resource module
    controller, ctrl            Generate a controller
    service, svc                Generate a service
    model                       Generate a model
    dto                         Generate DTOs
    route                       Generate routes

OPTIONS:
    --database <ENGINE>         Select the database engine
                                Supported values: mock, mongo, mongodb
                                Opens an interactive selection when omitted
    --force                     Replace an existing project directory
    --dry-run                   Preview generated files without writing them

GENERATE OPTIONS:
    --force                     Overwrite files that already exist
    --dry-run                   Preview changes without writing files
    --collection <NAME>         Set the MongoDB collection name
                                Available for module and controller generators

GLOBAL OPTIONS:
    -h, --help                  Print this help message
    -v, --version               Print the installed Oxgen version
"#
    );
}
