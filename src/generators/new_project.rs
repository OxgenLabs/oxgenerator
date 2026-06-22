use include_dir::{Dir, DirEntry, include_dir};
use inquire::Select;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::database::DatabaseEngine;
use crate::core::error::OxgenError;
use crate::core::generator::Generator;
use crate::core::naming::Name;
use crate::core::result::OxgenResult;
use crate::core::template::TemplateRenderer;

static MOCK_PROJECT_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project/mock");

static MONGODB_PROJECT_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project/mongodb");

fn project_templates(database: DatabaseEngine) -> &'static Dir<'static> {
    match database {
        DatabaseEngine::Mock => &MOCK_PROJECT_TEMPLATES,
        DatabaseEngine::MongoDb => &MONGODB_PROJECT_TEMPLATES,
    }
}

pub struct NewProjectGenerator {
    name: Name,
    force: bool,
    dry_run: bool,
    database: Option<DatabaseEngine>,
}

impl NewProjectGenerator {
    pub fn new(name: Name, force: bool, dry_run: bool, database: Option<DatabaseEngine>) -> Self {
        Self {
            name,
            force,
            dry_run,
            database,
        }
    }

    fn project_path(&self) -> PathBuf {
        PathBuf::from(&self.name.raw)
    }

    fn collect_template_files(&self, project_templates: &Dir<'_>) -> Vec<PathBuf> {
        let mut files = Vec::new();

        Self::collect_template_files_recursive(project_templates, &mut files);

        files
    }

    fn collect_template_files_recursive(directory: &Dir<'_>, files: &mut Vec<PathBuf>) {
        for entry in directory.entries() {
            match entry {
                DirEntry::Dir(child_directory) => {
                    Self::collect_template_files_recursive(child_directory, files);
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
                let renderer = TemplateRenderer {
                    name: self.name.clone(),
                    collection: None,
                };

                let rendered_content = renderer.render_template(content);

                fs::write(final_output_path, rendered_content)?;
            }
            None => {
                fs::write(final_output_path, file.contents())?;
            }
        }

        Ok(())
    }

    fn print_dry_run(&self, templates_dir: &Dir<'_>, project_path: &Path) {
        println!("dry run: project `{}` would be created", self.name.raw);
        println!();

        for template_path in self.collect_template_files(templates_dir) {
            let output_path = self.output_path_from_template_path(&template_path);

            println!("create {}", project_path.join(output_path).display());
        }
    }

    fn format_generated_project(&self, project_path: &Path) -> OxgenResult<()> {
        let output = std::process::Command::new("cargo")
            .arg("fmt")
            .current_dir(project_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            return Err(OxgenError::CargoFmtFailed(stderr.trim().to_string()));
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
        match std::process::Command::new("git").arg("--version").output() {
            Ok(_) => true,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(_) => true,
        }
    }

    fn ask_db_engine(&self) -> OxgenResult<DatabaseEngine> {
        let options = vec![
            "Use the mock database engine",
            "Use the MongoDB database engine",
        ];

        let choice = Select::new("Select a database engine:", options)
            .prompt()
            .map_err(|error| OxgenError::InquireMenuInteractionFailed(error.to_string()))?;

        match choice {
            "Use the mock database engine" => Ok(DatabaseEngine::Mock),
            "Use the MongoDB database engine" => Ok(DatabaseEngine::MongoDb),
            _ => Err(OxgenError::UnknownDatabase),
        }
    }

    fn selected_database(&self) -> OxgenResult<DatabaseEngine> {
        match self.database {
            Some(database) => Ok(database),
            None => self.ask_db_engine(),
        }
    }
}

impl Generator for NewProjectGenerator {
    fn generate(&self) -> OxgenResult<()> {
        let database = self.selected_database()?;
        let templates_dir = project_templates(database);
        let project_path = self.project_path();

        if self.dry_run {
            self.print_dry_run(templates_dir, &project_path);

            return Ok(());
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

        for template_path in self.collect_template_files(templates_dir) {
            self.write_template_file(templates_dir, &project_path, &template_path)?;
        }

        if self.git_available() {
            self.init_git_repository(&project_path)?;
        }

        self.format_generated_project(&project_path)?;

        println!("created project `{}`", self.name.raw);
        println!();
        println!("next steps:");
        println!("  cd {}", self.name.raw);

        if database == DatabaseEngine::MongoDb {
            println!("  change MONGO_URI in .env");
        }

        println!("  cargo run");

        Ok(())
    }
}
