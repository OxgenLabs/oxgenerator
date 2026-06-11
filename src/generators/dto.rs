use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{Dir, include_dir};

use crate::core::error::OxgenError;
use crate::core::file_writer::FileWriter;
use crate::core::generator::Generator;
use crate::core::naming::Name;
use crate::core::project_detector::ensure_oxgen_project_root;
use crate::core::result::OxgenResult;
use crate::core::template::render_template;

static RESOURCE_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/resource");

pub struct DtoGenerator {
    name: Name,
    force: bool,
    dry_run: bool,
}

impl DtoGenerator {
    pub fn new(name: Name, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }

    fn module_path(&self, name: &Name) -> PathBuf {
        PathBuf::from("src").join("modules").join(&name.snake)
    }

    fn dto_path(&self, name: &Name) -> PathBuf {
        self.module_path(name).join("dto.rs")
    }

    fn root_modules_mod_path(&self) -> PathBuf {
        PathBuf::from("src").join("modules").join("mod.rs")
    }

    fn resource_module_mod_path(&self, name: &Name) -> PathBuf {
        self.module_path(name).join("mod.rs")
    }

    fn load_template(&self) -> OxgenResult<&'static str> {
        let template_path = "dto.rs.ox";

        let file = RESOURCE_TEMPLATES
            .get_file(template_path)
            .ok_or_else(|| OxgenError::TemplateNotFound(template_path.to_string()))?;

        file.contents_utf8()
            .ok_or_else(|| OxgenError::InvalidTemplatePath(template_path.to_string()))
    }

    fn ensure_module_directory(&self, module_path: &Path) -> OxgenResult<()> {
        if module_path.exists() {
            return Ok(());
        }

        if self.dry_run {
            println!("[CREATE] {}", module_path.display());
            return Ok(());
        }

        fs::create_dir_all(module_path)?;

        Ok(())
    }

    fn ensure_root_modules_mod_file(&self, name: &Name) -> OxgenResult<()> {
        let modules_mod_path = self.root_modules_mod_path();
        let module_declaration = format!("pub mod {};", name.snake);

        if self.dry_run {
            if !modules_mod_path.exists() {
                println!("[CREATE] {}", modules_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    module_declaration,
                    modules_mod_path.display()
                );
                return Ok(());
            }

            let content = fs::read_to_string(&modules_mod_path)?;

            if !content
                .lines()
                .any(|line| line.trim() == module_declaration)
            {
                println!("[UPDATE] {}", modules_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    module_declaration,
                    modules_mod_path.display()
                );
            }

            return Ok(());
        }

        if let Some(parent) = modules_mod_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = if modules_mod_path.exists() {
            fs::read_to_string(&modules_mod_path)?
        } else {
            String::new()
        };

        if content
            .lines()
            .any(|line| line.trim() == module_declaration)
        {
            return Ok(());
        }

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(&module_declaration);
        content.push('\n');

        fs::write(modules_mod_path, content)?;

        Ok(())
    }

    fn ensure_resource_module_mod_file(&self, name: &Name) -> OxgenResult<()> {
        let module_mod_path = self.resource_module_mod_path(name);
        let dto_declaration = "pub mod dto;";

        if self.dry_run {
            if !module_mod_path.exists() {
                println!("[CREATE] {}", module_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    dto_declaration,
                    module_mod_path.display()
                );
                return Ok(());
            }

            let content = fs::read_to_string(&module_mod_path)?;

            if !content.lines().any(|line| line.trim() == dto_declaration) {
                println!("[UPDATE] {}", module_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    dto_declaration,
                    module_mod_path.display()
                );
            }

            return Ok(());
        }

        if let Some(parent) = module_mod_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = if module_mod_path.exists() {
            fs::read_to_string(&module_mod_path)?
        } else {
            String::new()
        };

        if content.lines().any(|line| line.trim() == dto_declaration) {
            return Ok(());
        }

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(dto_declaration);
        content.push('\n');

        fs::write(module_mod_path, content)?;

        Ok(())
    }
}

impl Generator for DtoGenerator {
    fn generate(&self) -> OxgenResult<()> {
        let module_path = self.module_path(&self.name);
        let dto_path = self.dto_path(&self.name);

        ensure_oxgen_project_root()?;
        self.ensure_module_directory(&module_path)?;
        self.ensure_root_modules_mod_file(&self.name)?;
        self.ensure_resource_module_mod_file(&self.name)?;

        let template = self.load_template()?;
        let content = render_template(template, &self.name);

        let writer = FileWriter::new(self.force, self.dry_run);
        writer.write_file(dto_path, &content)?;

        Ok(())
    }
}
