use include_dir::Dir;

use crate::core::error::OxgenError;
use crate::core::file_writer::FileWriter;
use crate::core::generator::{Generator, ResourceGenerator, resource_templates};
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::project_detector::ensure_oxgen_project_root;
use crate::core::result::OxgenResult;
use crate::core::template::TemplateRenderer;

pub struct ModelGenerator {
    name: Name,
    context: GeneratorContext,
}

impl ModelGenerator {
    pub fn new(name: Name, context: GeneratorContext) -> Self {
        Self { name, context }
    }

    fn load_template(templates_dir: &'static Dir<'static>) -> OxgenResult<&'static str> {
        let template_path = "model.rs.ox";

        let file = templates_dir
            .get_file(template_path)
            .ok_or_else(|| OxgenError::TemplateNotFound(template_path.to_string()))?;

        file.contents_utf8()
            .ok_or_else(|| OxgenError::InvalidTemplatePath(template_path.to_string()))
    }
}

impl Generator for ModelGenerator {
    fn generate(&self) -> OxgenResult<()> {
        ensure_oxgen_project_root()?;

        let resource_generator = ResourceGenerator::new(&self.name, self.context);

        resource_generator.ensure_module_structure("model")?;

        let model_path = resource_generator.resource_file_path("model");

        let templates_dir = resource_templates(self.context.database);

        let template = Self::load_template(templates_dir)?;

        let renderer = TemplateRenderer {
            name: self.name.clone(),
            collection: None,
        };

        let content = renderer.render_template(template);

        let writer = FileWriter::new(self.context.force, self.context.dry_run);

        writer.write_file(model_path, &content)?;

        Ok(())
    }
}
