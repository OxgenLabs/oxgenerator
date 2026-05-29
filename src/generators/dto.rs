use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{Dir, include_dir};

use crate::core::error::OxgenError;
use crate::core::file_writer::FileWriter;
use crate::core::generator::Generator;
use crate::core::naming::Name;
use crate::core::project_detector::ensure_oxgen_project_root;
use crate::core::result::OxgenResult;

static RESOURCE_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/resource");

pub struct DtoGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl DtoGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }

    fn load_template(path: &str) -> OxgenResult<&'static str> {
        let template = RESOURCE_TEMPLATES
            .get_file(path)
            .ok_or_else(|| OxgenError::TemplateNotFound(path.to_string()))?;

        template
            .contents_utf8()
            .ok_or_else(|| OxgenError::TemplateNotFound(path.to_string()))
    }

    fn render_template(template: &str, name: &Name) -> String {
        template
            .replace("{{resource_name}}", &name.snake)
            .replace("{{capitalized_resource_name}}", &name.pascal)
    }

    fn ensure_resource_mod_file(path: &Path, dry_run: bool) -> OxgenResult<()> {
        let module_declaration = "pub mod dto;";

        let current_content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };

        if current_content
            .lines()
            .any(|line| line.trim() == module_declaration)
        {
            return Ok(());
        }

        let mut updated_content = current_content;

        if !updated_content.is_empty() && !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }

        updated_content.push_str(module_declaration);
        updated_content.push('\n');

        let file_exists = path.exists();

        if dry_run {
            if file_exists {
                println!("[UPDATE] {}", path.display());
            } else {
                println!("[CREATE] {}", path.display());
            }

            println!("[ADD] `{}` to {}", module_declaration, path.display());

            return Ok(());
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, updated_content)?;

        Ok(())
    }

    fn ensure_modules_mod_file(path: &Path, resource_name: &str, dry_run: bool) -> OxgenResult<()> {
        let module_declaration = format!("pub mod {};", resource_name);

        let current_content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };

        if current_content
            .lines()
            .any(|line| line.trim() == module_declaration)
        {
            return Ok(());
        }

        let mut updated_content = current_content;

        if !updated_content.is_empty() && !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }

        updated_content.push_str(&module_declaration);
        updated_content.push('\n');

        let file_exists = path.exists();

        if dry_run {
            if file_exists {
                println!("[UPDATE] {}", path.display());
            } else {
                println!("[CREATE] {}", path.display());
            }

            println!("[ADD] `{}` to {}", module_declaration, path.display());

            return Ok(());
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, updated_content)?;

        Ok(())
    }
}

impl Generator for DtoGenerator {
    fn generate(&self) -> OxgenResult<()> {
        let name = Name::new(&self.name)?;
        let writer = FileWriter::new(self.force, self.dry_run);

        let resource_dir = PathBuf::from("src").join("modules").join(&name.snake);

        let dto_path = resource_dir.join("dto.rs");
        let resource_mod_path = resource_dir.join("mod.rs");
        let modules_mod_path = PathBuf::from("src").join("modules").join("mod.rs");

        ensure_oxgen_project_root()?;
        writer.create_dir(&resource_dir)?;
        Self::ensure_modules_mod_file(&modules_mod_path, &name.snake, self.dry_run)?;
        Self::ensure_resource_mod_file(&resource_mod_path, self.dry_run)?;

        let template = Self::load_template("dto.rs.ox")?;
        let content = Self::render_template(template, &name);

        writer.write_file(dto_path, &content)?;

        Ok(())
    }
}
