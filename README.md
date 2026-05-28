# oxgenerator

A small, fast and opinionated Rust project generator for building clean API with a structured architecture from the first command.

`oxgen` is designed to remove the repetitive work of creating the same folders, files and naming conventions again and again. It gives you a consistent starting point for Rust projects and provides generators for common building blocks such as resources, controllers, services and models.

> Status: early development. The project is currently focused on the `new` command and the foundation of the generator system.

## What is oxgen?

`oxgen` is a CLI tool that helps you scaffold Rust projects with a predictable structure.

Instead of starting every project from an empty folder, manually creating modules, wiring files together and repeating the same architecture decisions, `oxgen` gives you a ready-to-edit project layout.

The goal is not to hide Rust behind a framework. The goal is to make project creation faster while keeping the generated code explicit, readable and easy to modify.

## Features

- Create a new Rust project from embedded templates.
- Generate common application files:
  - resources
  - controllers
  - services
  - models
- Support short command aliases.
- Support `--dry-run` to preview generated files.
- Support `--force` to recreate an existing target.
- Keep generated code simple and explicit.
- Use embedded templates through `include_dir`, so the binary can carry its project templates.

## Installation

### From source

```bash
git clone https://github.com/OxgenLabs/oxgenerator.git
cd oxgenerator
cargo install --path .
```

Then check that the binary is available:

```bash
oxgen --version
```

You should see something like:

```bash
oxgen 0.1.0
```

## Quick start

Create a new project:

```bash
oxgen new my-api
```

Then enter the generated project:

```bash
cd my-api
cargo run
```

Preview what would be created without writing anything:

```bash
oxgen new my-api --dry-run
```

Replace an existing folder:

```bash
oxgen new my-api --force
```

## Usage

```bash
oxgen <command> [options]
```

### Commands

```bash
oxgen new <project-name>
```

Create a new project from the embedded project template.

```bash
oxgen generate <generator> <name>
```

Generate a new file or group of files inside an existing project.

The short alias is also available:

```bash
oxgen g <generator> <name>
```

### Available generators

```bash
oxgen generate resource user
oxgen generate controller user
oxgen generate service user
oxgen generate model user
```

Short aliases:

```bash
oxgen g resource user
oxgen g res user

oxgen g controller user
oxgen g ctrl user

oxgen g service user
oxgen g svc user

oxgen g model user
```

### Options

```bash
--dry-run
```

Print what would be generated without writing files.

```bash
--force
```

Overwrite the target when it already exists.

```bash
-h, --help
```

Display help.

```bash
-v, --version
```

Display the current version.

## Template system

`oxgen` embeds its templates directly into the binary.

This means the generator can read files from the internal template directory and write them into the target project without requiring the user to manually copy template assets.

Templates can use placeholders such as:

```text
{{project_name}}
{{crate_name}}
__PROJECT_NAME__
__CRATE_NAME__
```

These placeholders are replaced during generation.

## Design goals

### Explicit over magical

Generated code should be easy to read and easy to delete. `oxgen` should not create a project that feels impossible to understand without the generator.

### Consistent architecture

The main value of the tool is consistency. The same command should always generate the same kind of structure, naming and file organization.

### Rust-first

`oxgen` is written in Rust and targets Rust projects first. It follows Rust naming rules, crate naming expectations and module organization.

### Small core, useful commands

The project should stay lightweight. The generator should focus on commands that save time without becoming a large framework.

## Roadmap


`oxgen` is still in early development. The roadmap below tracks the features planned for the CLI and the generated Rust projects.

- [ ] Create the `new` command to create a Rust package
    - [x] use .ox template file
    - [x] create `--dry-run` flag so users can preview every generated file
    - [x] create `--force` flag so users can force create the package
    - [x] generate a `.gitignore` adapted to Rust projects
    - [x] generate a `.env.example` file
    - [x] generate a `README.md` inside created projects
    - [x] automatically initialize a git repository when creating a new package
    - [ ] add better error messages for invalid project names, missing arguments and existing folders
    - [x] Add tests for the `new` generator
    - [x] Add tests for project name validation
- [x] Improve the CLI help output
- [ ] Create the `generate` command
    - [ ]  for generating a full resource module
    - [ ]  for generating controllers
    - [ ]  for generating services
    - [ ]  for generating models
    - [ ]  for generating repositories
    - [ ]  for generating routes
    - [ ]  for generating DTOs or payload structs
    - [ ]  for generating error files per resource
    - [ ]  create aliases such as `g`, `res`, `ctrl` and `svc`
    - [x] create `--dry-run` flag so users can preview generated file
    - [x] create `--force` flag so users can force create the file
- [ ] Support databases
- [ ] Investigate whether project dependencies should use fixed versions or the latest compatible versions
- [x] Add tests for the CLI parser
- [x] Add tests for template rendering
- [ ] Add tests for resource, controller, service, model and repository generators
- [] Add CI
    - [ ] for formatting
    - [ ] for linting
    - [x] for test
    - [x] for release
- [ ] Add installation instructions for Linux, macOS and Windows
- [ ] Add documentation for the generated project architecture
- [ ] Add documentation explaining each generated layer and its responsibility
- [ ] Add examples showing how to create and extend a generated project

## Development

Clone the repository:

```bash
git clone https://github.com/OxgenLabs/oxgenerator.git
cd oxgenerator
```

Run the project:

```bash
cargo run -- new demo-api
```

Run a dry-run generation:

```bash
cargo run -- new demo-api --dry-run
```

Run checks:

```bash
cargo fmt
cargo clippy
cargo test
```

Build the binary:

```bash
cargo build --release
```

## Contributing

Contributions are welcome, especially around:

- project template improvements;
- generator behavior;
- CLI help output;
- tests;
- documentation;
- release automation.

Before opening a pull request, please make sure the project is formatted and passes checks:

```bash
cargo fmt
cargo clippy
cargo test
```

## License

This project is distributed under the license provided in the repository.

See [`LICENSE`](./LICENSE) for details.
