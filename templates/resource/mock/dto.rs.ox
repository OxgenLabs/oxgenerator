use serde::Serialize;

use super::model::{{capitalized_resource_name}};

#[derive(Debug, Serialize)]
pub struct {{capitalized_resource_name}}Response {
    pub id: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl From<{{capitalized_resource_name}}> for {{capitalized_resource_name}}Response {
    fn from(value: {{capitalized_resource_name}}) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct {{capitalized_resource_name}}ListResponse {
    pub {{resource_name}}_list: Vec<{{capitalized_resource_name}}Response>,
    pub total: usize,
}

impl From<Vec<{{capitalized_resource_name}}>> for {{capitalized_resource_name}}ListResponse {
    fn from(values: Vec<{{capitalized_resource_name}}>) -> Self {
        let total = values.len();

        Self {
            {{resource_name}}_list: values
                .into_iter()
                .map({{capitalized_resource_name}}Response::from)
                .collect(),
            total,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct {{capitalized_resource_name}}DeleteResponse {
    pub id: String,
}
