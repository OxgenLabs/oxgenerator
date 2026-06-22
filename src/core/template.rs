use crate::core::naming::Name;

pub struct TemplateRenderer {
    pub name: Name,
    pub collection: Option<String>,
}

impl TemplateRenderer {
    pub fn render_template(&self, content: &str) -> String {
        let mut rendered = content
            .replace("{{crate_name}}", &self.name.snake)
            .replace("{{project_name}}", &self.name.raw)
            .replace("{{name}}", &self.name.snake)
            .replace("{{resource_name}}", &self.name.snake)
            .replace("{{capitalized_resource_name}}", &self.name.pascal);

        if let Some(collection) = &self.collection {
            rendered = rendered.replace("{{collection_name}}", collection);
        } else {
            rendered = rendered.replace("{{collection_name}}", &self.name.raw);
        }

        rendered
    }
}
