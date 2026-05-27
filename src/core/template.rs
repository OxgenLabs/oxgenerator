use crate::core::naming::Name;
use crate::core::result::OxgenResult;

pub struct TemplateContext {
    pub name: Name,
}

impl TemplateContext {
    pub fn new(raw_name: &str) -> OxgenResult<Self> {
        Ok(Self {
            name: Name::new(raw_name)?,
        })
    }
}

pub fn render_template(content: &str, context: &TemplateContext) -> String {
    content
        .replace("{{name}}", &context.name.raw)
        .replace("{{snake_name}}", &context.name.snake)
        .replace("{{pascal_name}}", &context.name.pascal)
        .replace("{{kebab_name}}", &context.name.kebab)
}
