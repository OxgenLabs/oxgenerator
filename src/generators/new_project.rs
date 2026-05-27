use std::fs;
use std::path::{Path, PathBuf};

use crate::core::error::OxgenError;
use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct NewProjectGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl NewProjectGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }

    fn project_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    fn template_root(&self) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/project")
    }

    fn validate_project_name(&self) -> OxgenResult<()> {
        if self.name.trim().is_empty() {
            return Err(OxgenError::InvalidProjectName(self.name.clone()));
        }

        let Some(first_char) = self.name.chars().next() else {
            return Err(OxgenError::InvalidProjectName(self.name.clone()));
        };

        if !first_char.is_ascii_alphabetic() {
            return Err(OxgenError::InvalidProjectName(self.name.clone()));
        }

        let is_valid = self
            .name
            .chars()
            .all(|char| char.is_ascii_alphanumeric() || char == '-' || char == '_');

        if !is_valid {
            return Err(OxgenError::InvalidProjectName(self.name.clone()));
        }

        Ok(())
    }

    fn crate_name(&self) -> String {
        self.name.replace('-', "_")
    }

    fn collect_template_files(&self) -> OxgenResult<Vec<(PathBuf, PathBuf)>> {
        let template_root = self.template_root();
        let mut files = Vec::new();

        self.collect_template_files_recursive(&template_root, &template_root, &mut files)?;

        Ok(files)
    }

    fn collect_template_files_recursive(
        &self,
        template_root: &Path,
        current_dir: &Path,
        files: &mut Vec<(PathBuf, PathBuf)>,
    ) -> OxgenResult<()> {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_template_files_recursive(template_root, &path, files)?;
                continue;
            }

            if !path.is_file() {
                continue;
            }

            let relative_path = path
                .strip_prefix(template_root)
                .map_err(|_| OxgenError::InvalidTemplatePath(path.display().to_string()))?
                .to_path_buf();

            let output_path = self.output_path_from_template_path(&relative_path);

            files.push((path, output_path));
        }

        Ok(())
    }

    fn output_path_from_template_path(&self, template_path: &Path) -> PathBuf {
        let path_as_string = template_path.to_string_lossy();

        if let Some(path_without_extension) = path_as_string.strip_suffix(".ox") {
            return PathBuf::from(path_without_extension);
        }

        template_path.to_path_buf()
    }

    fn render_template(&self, content: &str) -> String {
        content
            .replace("{{project_name}}", &self.name)
            .replace("{{crate_name}}", &self.crate_name())
            .replace("__PROJECT_NAME__", &self.name)
            .replace("__CRATE_NAME__", &self.crate_name())
    }

    fn write_template_file(
        &self,
        project_path: &Path,
        template_path: &Path,
        output_path: &Path,
    ) -> OxgenResult<()> {
        let content = fs::read_to_string(template_path)?;
        let rendered_content = self.render_template(&content);
        let final_output_path = project_path.join(output_path);

        if let Some(parent) = final_output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(final_output_path, rendered_content)?;

        Ok(())
    }

    fn print_dry_run(&self, project_path: &Path) -> OxgenResult<()> {
        println!("dry run: project `{}` would be created", self.name);
        println!();

        for (_, output_path) in self.collect_template_files()? {
            println!("create {}", project_path.join(output_path).display());
        }

        Ok(())
    }
}

impl Generator for NewProjectGenerator {
    fn generate(&self) -> OxgenResult<()> {
        self.validate_project_name()?;

        let project_path = self.project_path();
        let template_root = self.template_root();

        if !template_root.exists() {
            return Err(OxgenError::TemplateDirectoryNotFound(
                template_root.display().to_string(),
            ));
        }

        if self.dry_run {
            return self.print_dry_run(&project_path);
        }

        if project_path.exists() {
            if !self.force {
                let dotenv_path = project_path.join(".env.example");
                let toml_path = project_path.join("Cargo.toml");
                let src_path = project_path.join("src/");
                if dotenv_path.exists() || toml_path.exists() || src_path.exists() {
                    return Err(OxgenError::ProjectAlreadyExists(
                        project_path.display().to_string(),
                    ));
                }
            }

            fs::remove_dir_all(&project_path)?;
        }

        fs::create_dir_all(&project_path)?;

        for (template_path, output_path) in self.collect_template_files()? {
            self.write_template_file(&project_path, &template_path, &output_path)?;
        }

        println!("created project `{}`", self.name);
        println!();
        println!("next steps:");
        println!("  cd {}", self.name);
        println!("  cargo run");

        Ok(())
    }
}
