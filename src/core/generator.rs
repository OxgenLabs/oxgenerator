use include_dir::{Dir, include_dir};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::database::DatabaseEngine;
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::result::OxgenResult;

static MOCK_RESOURCE_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/resource/mock");

static MONGODB_RESOURCE_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/resource/mongodb");

pub fn resource_templates(database: DatabaseEngine) -> &'static Dir<'static> {
    match database {
        DatabaseEngine::Mock => &MOCK_RESOURCE_TEMPLATES,
        DatabaseEngine::MongoDb => &MONGODB_RESOURCE_TEMPLATES,
    }
}

pub trait Generator {
    fn generate(&self) -> OxgenResult<()>;
}

pub struct ResourceGenerator<'a> {
    name: &'a Name,
    context: GeneratorContext,
}

impl<'a> ResourceGenerator<'a> {
    pub fn new(name: &'a Name, context: GeneratorContext) -> Self {
        Self { name, context }
    }

    pub fn module_directory_path(&self) -> PathBuf {
        PathBuf::from("src").join("modules").join(&self.name.snake)
    }

    pub fn resource_file_path(&self, file_name: &str) -> PathBuf {
        self.module_directory_path().join(format!("{file_name}.rs"))
    }

    pub fn root_modules_mod_path(&self) -> PathBuf {
        PathBuf::from("src").join("modules").join("mod.rs")
    }

    pub fn resource_module_mod_path(&self) -> PathBuf {
        self.module_directory_path().join("mod.rs")
    }

    pub fn ensure_module_structure(&self, resource_part: &str) -> OxgenResult<()> {
        let module_directory = self.module_directory_path();

        self.ensure_directory(&module_directory)?;

        let root_declaration = format!("pub mod {};", self.name.snake);

        self.ensure_module_declaration(&self.root_modules_mod_path(), &root_declaration)?;

        let resource_declaration = format!("pub mod {resource_part};");

        self.ensure_module_declaration(&self.resource_module_mod_path(), &resource_declaration)?;

        Ok(())
    }

    fn ensure_directory(&self, path: &Path) -> OxgenResult<()> {
        if path.exists() {
            return Ok(());
        }

        if self.context.dry_run {
            println!("[CREATE] {}", path.display());
            return Ok(());
        }

        fs::create_dir_all(path)?;

        Ok(())
    }

    fn ensure_module_declaration(&self, path: &Path, declaration: &str) -> OxgenResult<()> {
        if self.context.dry_run {
            self.print_module_declaration_change(path, declaration)?;
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };

        if content.lines().any(|line| line.trim() == declaration) {
            return Ok(());
        }

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(declaration);
        content.push('\n');

        fs::write(path, content)?;

        Ok(())
    }

    fn print_module_declaration_change(&self, path: &Path, declaration: &str) -> OxgenResult<()> {
        if !path.exists() {
            println!("[CREATE] {}", path.display());
            println!("[ADD] `{declaration}` to {}", path.display());
            return Ok(());
        }

        let content = fs::read_to_string(path)?;

        if !content.lines().any(|line| line.trim() == declaration) {
            println!("[UPDATE] {}", path.display());
            println!("[ADD] `{declaration}` to {}", path.display());
        }

        Ok(())
    }
}
