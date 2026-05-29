use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct {{capitalized_resource_name}} {
    pub id: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}
