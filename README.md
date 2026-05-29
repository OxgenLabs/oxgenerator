[![Test](https://github.com/OxgeneratorLabs/oxgenerator/actions/workflows/test.yml/badge.svg)](https://github.com/OxgeneratorLabs/oxgenerator/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/oxgen.svg)](https://crates.io/crates/oxgenerator)

`oxgen` is a small Rust CLI that scaffolds clean **API** projects with a structured architecture from the first command.

It helps you avoid repeating the same setup work: folders, modules, naming conventions, starter files and project layout decisions.

![demo](etc/demo.gif)

```bash
cargo install oxgen
oxgen new my-api
cd my-api
cargo run
```

> Status: early development. The project currently focuses on project creation and the foundation of the generator system.

## What is oxgen?

`oxgen` is a CLI tool that helps you scaffold Rust projects with a predictable structure.

Instead of starting every project from an empty folder, manually creating modules, wiring files together and repeating the same architecture decisions, `oxgen` gives you a ready-to-edit project layout.

The goal is not to hide Rust behind a framework. The goal is to make project creation faster while keeping the generated code explicit, readable and easy to modify.

## Why oxgen?

Starting a Rust API project often involves the same setup work again and again: project structure, configuration files, naming conventions, module organization and starter documentation.

`oxgen` removes that repetitive setup while keeping the generated code simple and transparent.

It is designed for developers who want:

- a fast starting point for Rust API projects;
- explicit generated code instead of framework magic;
- consistent project structure across multiple projects;
- a small CLI that focuses on useful scaffolding commands.

## Features

### Available now

- Create a new Rust project from embedded templates.
- Preview generated files with `--dry-run`.
- Recreate an existing target with `--force`.
- Generate a Rust-friendly `.gitignore` file.
- Generate a `.env.example` file.
- Generate a starter `README.md` inside created projects.
- Automatically initialize a Git repository when creating a new package.
- Validate project names with clear error messages.
- Keep generated code simple and explicit.
- Use embedded templates through `include_dir`, so the binary can carry its project templates.

### Planned

- Generate resources.
- Generate controllers.
- Generate services.
- Generate models.
- Generate routes.
- Generate DTOs or payload structs.
- Generate error files per resource.
- Support short aliases such as `g`, `res`, `ctrl` and `svc`.
- Support databases.

## Installation

### From crates.io

If the crate is published on crates.io, install it with:

```bash
cargo install oxgen
```

Then check that the binary is available:

```bash
oxgen --version
```

You should see something like:

```bash
oxgen 0.1.0
```

### From source

```bash
git clone https://github.com/OxgeneratorLabs/oxgenerator.git
cd oxgenerator
cargo install --path .
```

Then check that the binary is available:

```bash
oxgen --version
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

This creates a ready-to-edit Rust project with a predictable structure, starter configuration files and a Rust-oriented layout.

Example generated structure:

```text
my-api/
├── Cargo.toml
├── .env.example
├── .git
├── .gitignore
├── README.md
└── src
    ├── common
    │   ├── error.rs
    │   ├── mod.rs
    │   └── response.rs
    ├── lib.rs
    ├── main.rs
    ├── middleware
    │   └── mod.rs
    ├── modules
    │   ├── health
    │   │   ├── controllers.rs
    │   │   ├── dto.rs
    │   │   ├── mod.rs
    │   │   └── services.rs
    │   └── mod.rs
    ├── routes
    │   ├── health.rs
    │   └── mod.rs
    └── state.rs
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

Create a new project from the embedded project template:

```bash
oxgen new <project-name>
```

Generate a new file or group of files inside an existing project:

```bash
oxgen generate <generator> <name>
```

> Note: the generator system is part of the roadmap and may not be fully available yet depending on the current version.

### Planned generators

```bash
oxgen generate resource user
oxgen generate controller user
oxgen generate service user
oxgen generate model user
oxgen generate dto user
```

### Options

Preview what would be generated without writing files:

```bash
--dry-run
```

Overwrite the target when it already exists:

```bash
--force
```

Display help:

```bash
-h, --help
```

Display the current version:

```bash
-v, --version
```

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

`oxgen` is still in early development. The roadmap below tracks the main features planned for the CLI and the generated Rust projects.

- [x] Create a new Rust package with `oxgen new`.
    - [x] Support `.ox` template files.
    - [x] Support `--dry-run` for previewing generated files.
    - [x] Support `--force` for recreating existing targets.
    - [x] Generate a `.gitignore` adapted to Rust projects.
    - [x] Generate a `.env.example` file.
    - [x] Generate a `README.md` inside created projects.
    - [x] Automatically initialize a Git repository when creating a new package.
    - [x] Add better error messages for invalid project names, missing arguments and existing folders.
    - [x] Improve the CLI help output.
    - [x] Add tests for the `new` generator.
    - [x] Add tests for project name validation.
    - [x] Add tests for the CLI parser.
    - [x] Add tests for template rendering.
- [ ] Complete the `generate` command.
    - [ ] Generate full resource modules.
    - [ ] Generate controllers.
    - [ ] Generate services.
    - [x] Generate models.
    - [ ] Generate routes.
    - [x] Generate DTOs or payload structs.
    - [ ] Generate error files per resource.
    - [ ] Add aliases such as `g`, `res`, `ctrl` and `svc`.
    - [ ] Add tests for resource, controller, service, model and repository generators.
- [ ] Add CI for formatting.
- [ ] Add CI for linting.
- [x] Add CI for tests.
- [x] Add CI for releases.
- [ ] Add installation instructions for Linux, macOS and Windows.
- [ ] Add documentation for the generated project architecture.
- [ ] Add documentation explaining each generated layer and its responsibility.
- [ ] Add examples showing how to create and extend a generated project.
- [ ] Investigate whether project dependencies should use fixed versions or the latest compatible versions.
- [ ] Support databases.

## Development

Clone the repository:

```bash
git clone https://github.com/OxgeneratorLabs/oxgenerator.git
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

Good first contribution ideas:

- improve installation instructions for Linux, macOS and Windows;
- document the generated project structure;
- add examples for generated projects;
- improve CLI error messages;
- add tests for future generators.

Before opening a pull request, please make sure the project is formatted and passes checks:

```bash
cargo fmt
cargo clippy
cargo test
```

## License

This project is distributed under the license provided in the repository.

See [`LICENSE`](./LICENSE) for details.
