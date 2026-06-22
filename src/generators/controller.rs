use include_dir::Dir;

use crate::core::error::OxgenError;
use crate::core::file_writer::FileWriter;
use crate::core::generator::{Generator, ResourceGenerator, resource_templates};
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::project_detector::ensure_oxgen_project_root;
use crate::core::result::OxgenResult;
use crate::core::template::TemplateRenderer;

pub struct ControllerGenerator {
    name: Name,
    context: GeneratorContext,
    collection: Option<String>,
}

impl ControllerGenerator {
    pub fn new(name: Name, context: GeneratorContext, collection: Option<String>) -> Self {
        Self {
            name,
            context,
            collection,
        }
    }

    fn load_template(&self, templates_dir: &'static Dir<'static>) -> OxgenResult<&'static str> {
        let template_path = "controller.rs.ox";

        let file = templates_dir
            .get_file(template_path)
            .ok_or_else(|| OxgenError::TemplateNotFound(template_path.to_string()))?;

        file.contents_utf8()
            .ok_or_else(|| OxgenError::InvalidTemplatePath(template_path.to_string()))
    }

    fn validate_collection(&self) -> OxgenResult<()> {
        if self.collection.is_some() && !self.context.database.supports_collection() {
            return Err(OxgenError::CollectionRequiresMongoDb);
        }

        Ok(())
    }
}

impl Generator for ControllerGenerator {
    fn generate(&self) -> OxgenResult<()> {
        ensure_oxgen_project_root()?;
        self.validate_collection()?;

        let resource_generator = ResourceGenerator::new(&self.name, self.context);

        resource_generator.ensure_module_structure("controller")?;

        let controller_path = resource_generator.resource_file_path("controller");

        let templates_dir = resource_templates(self.context.database);

        let template = self.load_template(templates_dir)?;

        let renderer = TemplateRenderer {
            name: self.name.clone(),
            collection: self.collection.clone(),
        };

        let content = renderer.render_template(template);

        let writer = FileWriter::new(self.context.force, self.context.dry_run);

        writer.write_file(controller_path, &content)?;

        Ok(())
    }
}
