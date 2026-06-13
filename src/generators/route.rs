use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{Dir, include_dir};

use crate::core::error::OxgenError;
use crate::core::file_writer::FileWriter;
use crate::core::generator::Generator;
use crate::core::naming::Name;
use crate::core::project_detector::ensure_oxgen_project_root;
use crate::core::result::OxgenResult;
use crate::core::template::TemplateRenderer;

static RESOURCE_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/resource/mock");

pub struct RouteGenerator {
    name: Name,
    force: bool,
    dry_run: bool,
}

impl RouteGenerator {
    pub fn new(name: Name, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }

    fn routes_directory_path(&self) -> PathBuf {
        PathBuf::from("src").join("routes")
    }

    fn route_path(&self, name: &Name) -> PathBuf {
        self.routes_directory_path()
            .join(format!("{}.rs", name.snake))
    }

    fn routes_mod_path(&self) -> PathBuf {
        self.routes_directory_path().join("mod.rs")
    }

    fn main_path(&self) -> PathBuf {
        PathBuf::from("src").join("main.rs")
    }

    fn cargo_toml_path(&self) -> PathBuf {
        PathBuf::from("Cargo.toml")
    }

    fn load_template(&self) -> OxgenResult<&'static str> {
        let template_path = "route.rs.ox";

        let file = RESOURCE_TEMPLATES
            .get_file(template_path)
            .ok_or_else(|| OxgenError::TemplateNotFound(template_path.to_string()))?;

        file.contents_utf8()
            .ok_or_else(|| OxgenError::InvalidTemplatePath(template_path.to_string()))
    }

    fn ensure_routes_directory(&self, routes_directory_path: &Path) -> OxgenResult<()> {
        if routes_directory_path.exists() {
            return Ok(());
        }

        if self.dry_run {
            println!("[CREATE] {}", routes_directory_path.display());
            return Ok(());
        }

        fs::create_dir_all(routes_directory_path)?;

        Ok(())
    }

    fn ensure_routes_mod_file(&self, name: &Name) -> OxgenResult<()> {
        let routes_mod_path = self.routes_mod_path();
        let route_declaration = format!("pub mod {};", name.snake);

        if self.dry_run {
            if !routes_mod_path.exists() {
                println!("[CREATE] {}", routes_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    route_declaration,
                    routes_mod_path.display()
                );
                return Ok(());
            }

            let content = fs::read_to_string(&routes_mod_path)?;

            if !content.lines().any(|line| line.trim() == route_declaration) {
                println!("[UPDATE] {}", routes_mod_path.display());
                println!(
                    "[ADD] `{}` to {}",
                    route_declaration,
                    routes_mod_path.display()
                );
            }

            return Ok(());
        }

        if let Some(parent) = routes_mod_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = if routes_mod_path.exists() {
            fs::read_to_string(&routes_mod_path)?
        } else {
            String::new()
        };

        if content.lines().any(|line| line.trim() == route_declaration) {
            return Ok(());
        }

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(&route_declaration);
        content.push('\n');

        fs::write(routes_mod_path, content)?;

        Ok(())
    }

    fn read_crate_name(&self) -> OxgenResult<String> {
        let cargo_toml_path = self.cargo_toml_path();
        let content = fs::read_to_string(&cargo_toml_path)?;

        let crate_name = content
            .lines()
            .map(str::trim)
            .find_map(|line| {
                if !line.starts_with("name") {
                    return None;
                }

                let (_, value) = line.split_once('=')?;
                let value = value.trim().trim_matches('"');

                if value.is_empty() {
                    return None;
                }

                Some(value.replace('-', "_"))
            })
            .ok_or_else(|| {
                OxgenError::InvalidPackageName(
                    "unable to read package name from Cargo.toml".to_string(),
                )
            })?;

        Ok(crate_name)
    }

    fn render_main_with_route_import(&self, content: &str, name: &Name) -> OxgenResult<String> {
        let grouped_route_import = format!("{}::{}_routes", name.snake, name.snake);
        let direct_route_import = format!("routes::{}::{}_routes", name.snake, name.snake);

        if content.contains(&grouped_route_import) || content.contains(&direct_route_import) {
            return Ok(content.to_string());
        }

        let nested_route_import = format!("        {}::{}_routes,\n", name.snake, name.snake);

        if content.contains("    routes::{") {
            if content.contains("        health::health_routes,\n") {
                return Ok(content.replace(
                    "        health::health_routes,\n",
                    &format!("        health::health_routes,\n{}", nested_route_import),
                ));
            }

            let routes_block_position =
                content
                    .find("    routes::{")
                    .and_then(|routes_block_start| {
                        content[routes_block_start..].find("    },").map(
                            |relative_routes_block_end| {
                                (routes_block_start, relative_routes_block_end)
                            },
                        )
                    });

            if let Some((routes_block_start, relative_routes_block_end)) = routes_block_position {
                let routes_block_end = routes_block_start + relative_routes_block_end;
                let mut updated = content.to_string();
                updated.insert_str(routes_block_end, &nested_route_import);
                return Ok(updated);
            }
        }

        if content.contains("    routes::health::health_routes,\n") {
            return Ok(content.replace(
                "    routes::health::health_routes,\n",
                &format!(
                    "    routes::{{\n        health::health_routes,\n{}    }},\n",
                    nested_route_import
                ),
            ));
        }

        let crate_name = self.read_crate_name()?;
        let standalone_import = format!(
            "use {}::routes::{}::{}_routes;\n",
            crate_name, name.snake, name.snake
        );

        Ok(format!("{}{}", standalone_import, content))
    }

    fn render_main_with_route_merge(&self, content: &str, name: &Name) -> String {
        let route_merge = format!("        .merge({}_routes())", name.snake);

        if content.contains(&route_merge) {
            return content.to_string();
        }

        if content.contains("        .merge(health_routes)\n") {
            return content.replace(
                "        .merge(health_routes)\n",
                &format!("        .merge(health_routes)\n{}\n", route_merge),
            );
        }

        if content.contains("        .with_state(app_state.clone())") {
            return content.replace(
                "        .with_state(app_state.clone())",
                &format!("{}\n        .with_state(app_state.clone())", route_merge),
            );
        }

        content.to_string()
    }

    fn ensure_main_file_uses_route(&self, name: &Name) -> OxgenResult<()> {
        let main_path = self.main_path();

        if !main_path.exists() {
            return Err(OxgenError::ProjectNotFound);
        }

        let content = fs::read_to_string(&main_path)?;
        let with_import = self.render_main_with_route_import(&content, name)?;
        let updated = self.render_main_with_route_merge(&with_import, name);

        if updated == content {
            return Ok(());
        }

        if self.dry_run {
            println!("[UPDATE] {}", main_path.display());
            println!("[ADD] route import `{}::{}_routes`", name.snake, name.snake);
            println!("[MERGE] `{}_routes()` into app router", name.snake);
            return Ok(());
        }

        fs::write(main_path, updated)?;

        Ok(())
    }
}

impl Generator for RouteGenerator {
    fn generate(&self) -> OxgenResult<()> {
        let routes_directory_path = self.routes_directory_path();
        let route_path = self.route_path(&self.name);

        ensure_oxgen_project_root()?;

        if route_path.exists() && !self.force {
            return Err(OxgenError::FileAlreadyExists(
                route_path.display().to_string(),
            ));
        }

        let template = self.load_template()?;
        let renderer = TemplateRenderer {
            name: self.name.clone(),
            collection: None,
        };
        let content = renderer.render_template(template);

        self.ensure_routes_directory(&routes_directory_path)?;

        let writer = FileWriter::new(self.force, self.dry_run);
        writer.write_file(&route_path, &content)?;

        self.ensure_routes_mod_file(&self.name)?;
        self.ensure_main_file_uses_route(&self.name)?;

        Ok(())
    }
}
