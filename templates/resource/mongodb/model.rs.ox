use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct {{capitalized_resource_name}} {
 #[serde(rename = "_id")]
    pub id: ObjectId,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}
