use crate::core::naming::Name;

pub fn render_template(content: &str, name: &Name) -> String {
    content
        .replace("{{crate_name}}", &name.snake)
        .replace("{{project_name}}", &name.raw)
        .replace("{{name}}", &name.snake)
        .replace("{{resource_name}}", &name.snake)
        .replace("{{capitalized_resource_name}}", &name.pascal)
}
