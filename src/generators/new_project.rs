use include_dir::{Dir, DirEntry, include_dir};
use inquire::Select;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::error::OxgenError;
use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

static MOCK_PROJECT_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/project/mock");

static MONGODB_PROJECT_TEMPLATES: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project/mongodb");

const BUILT_IN_LIBRARY: &[&str] = &["test"];
const CONFUSING_PACKAGE_NAMES: &[&str] = &["std", "core", "alloc", "proc_macro"];
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

pub struct NewProjectGenerator {
    name: String,
    force: bool,
    dry_run: bool,
    database: String,
}

impl NewProjectGenerator {
    pub fn new(name: String, force: bool, dry_run: bool, database: String) -> Self {
        Self {
            name,
            force,
            dry_run,
            database,
        }
    }

    fn project_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    fn validate_package_name(&self) -> OxgenResult<()> {
        if self.name.trim().is_empty() {
            return Err(OxgenError::InvalidPackageName(self.name.clone()));
        }

        let Some(first_char) = self.name.chars().next() else {
            return Err(OxgenError::InvalidPackageName(self.name.clone()));
        };

        if !first_char.is_ascii_alphabetic() {
            return Err(OxgenError::InvalidPackageName(self.name.clone()));
        }

        let is_valid = self
            .name
            .chars()
            .all(|char| char.is_ascii_alphanumeric() || char == '-' || char == '_');

        if !is_valid {
            return Err(OxgenError::InvalidPackageName(self.name.clone()));
        }

        if BUILT_IN_LIBRARY.contains(&self.name.as_str()) {
            return Err(OxgenError::RustBuiltInPackageName(self.name.clone()));
        }

        if CONFUSING_PACKAGE_NAMES.contains(&self.name.as_str()) {
            return Err(OxgenError::ConfusingPackageName(self.name.clone()));
        }

        if RUST_KEYWORDS.contains(&self.name.as_str()) {
            return Err(OxgenError::RustKeywordPackageName(self.name.clone()));
        }
        Ok(())
    }

    fn crate_name(&self) -> String {
        self.name.replace('-', "_")
    }

    fn collect_template_files(&self, project_templates: &Dir<'_>) -> OxgenResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        Self::collect_template_files_recursive(project_templates, &mut files);

        Ok(files)
    }

    fn collect_template_files_recursive(dir: &Dir<'_>, files: &mut Vec<PathBuf>) {
        for entry in dir.entries() {
            match entry {
                DirEntry::Dir(child_dir) => {
                    Self::collect_template_files_recursive(child_dir, files);
                }
                DirEntry::File(file) => {
                    files.push(file.path().to_path_buf());
                }
            }
        }
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
        templates_dir: &Dir<'_>,
        project_path: &Path,
        template_path: &Path,
    ) -> OxgenResult<()> {
        let file = templates_dir
            .get_file(template_path)
            .ok_or_else(|| OxgenError::InvalidTemplatePath(template_path.display().to_string()))?;

        let output_path = self.output_path_from_template_path(template_path);
        let final_output_path = project_path.join(output_path);

        if let Some(parent) = final_output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        match file.contents_utf8() {
            Some(content) => {
                let rendered_content = self.render_template(content);
                fs::write(final_output_path, rendered_content)?;
            }
            None => {
                fs::write(final_output_path, file.contents())?;
            }
        }

        Ok(())
    }

    fn print_dry_run(&self, templates_dir: &Dir<'_>, project_path: &Path) -> OxgenResult<()> {
        println!("dry run: project `{}` would be created", self.name);
        println!();

        for template_path in self.collect_template_files(templates_dir)? {
            let output_path = self.output_path_from_template_path(&template_path);
            println!("create {}", project_path.join(output_path).display());
        }

        Ok(())
    }

    fn format_generated_project(&self, project_path: &Path) -> OxgenResult<()> {
        let output = std::process::Command::new("cargo")
            .arg("fmt")
            .current_dir(project_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            return Err(OxgenError::CargoFmtFailed(stderr.to_string()));
        }

        Ok(())
    }

    fn init_git_repository(&self, project_path: &Path) -> OxgenResult<()> {
        let output = std::process::Command::new("git")
            .arg("init")
            .current_dir(project_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            return Err(OxgenError::GitInitFailed(stderr.trim().to_string()));
        }

        Ok(())
    }

    fn git_available(&self) -> bool {
        if let Err(error) = std::process::Command::new("git").arg("--version").output()
            && error.kind() == std::io::ErrorKind::NotFound
        {
            false
        } else {
            true
        }
    }

    fn ask_db_engine(&self) -> OxgenResult<&Dir<'_>> {
        let options = vec![
            "Use the mock database engine",
            "Use the MongoDB database engine",
        ];

        let choice = Select::new("Select a database engine:", options).prompt();

        match choice {
            Ok("Use the mock database engine") => Ok(&MOCK_PROJECT_TEMPLATES),
            Ok("Use the MongoDB database engine") => Ok(&MONGODB_PROJECT_TEMPLATES),
            Ok(_) => Err(OxgenError::UnknownDatabase),
            Err(error) => Err(OxgenError::InquireMenuInteractionFailed(error.to_string())),
        }
    }
}

impl Generator for NewProjectGenerator {
    fn generate(&self) -> OxgenResult<()> {
        // if self.database is Some and is "mongo" or "mock" we use project_templates
        // if self.database is None we ask db engine
        let templates_dir: &Dir<'_> = match self.database.as_str() {
            "mongo" | "mongodb" => &MONGODB_PROJECT_TEMPLATES,
            "mock" => &MOCK_PROJECT_TEMPLATES,
            "none" => self.ask_db_engine()?,
            _ => return Err(OxgenError::UnknownDatabase),
        };

        self.validate_package_name()?;

        let project_path = self.project_path();

        if self.dry_run {
            return self.print_dry_run(templates_dir, &project_path);
        }

        if project_path.exists() {
            if !self.force {
                return Err(OxgenError::ProjectAlreadyExists(
                    project_path.display().to_string(),
                ));
            }

            fs::remove_dir_all(&project_path)?;
        }

        fs::create_dir_all(&project_path)?;

        for template_path in self.collect_template_files(templates_dir)? {
            self.write_template_file(templates_dir, &project_path, &template_path)?;
        }

        if self.git_available() {
            self.init_git_repository(&project_path)?;
        }

        self.format_generated_project(&project_path)?;

        println!("created project `{}`", self.name);
        println!();
        println!("next steps:");
        println!("  cd {}", self.name);
        if templates_dir == &MONGODB_PROJECT_TEMPLATES {
            println!("  change MONGO_URI in .env")
        }
        println!("  cargo run");

        Ok(())
    }
}
