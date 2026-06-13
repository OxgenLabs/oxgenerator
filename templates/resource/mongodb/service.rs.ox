use crate::common::error::AppError;

use super::{
    dto::{
        {{capitalized_resource_name}}ListResponse,
        {{capitalized_resource_name}}Response,
        {{capitalized_resource_name}}DeleteResponse,
    },
    model::{{capitalized_resource_name}},
};

pub async fn get_all_{{resource_name}}(
    {{resource_name}}_list: Vec<{{capitalized_resource_name}}>
) -> Result<{{capitalized_resource_name}}ListResponse, AppError> {
    Ok({{capitalized_resource_name}}ListResponse::from({{resource_name}}_list))
}

pub async fn get_{{resource_name}}(
    {{resource_name}}_list: Vec<{{capitalized_resource_name}}>,
    id: String,
) -> Result<{{capitalized_resource_name}}Response, AppError> {
    let {{resource_name}} = {{resource_name}}_list
        .into_iter()
        .find(|{{resource_name}}| {{resource_name}}.id == id)
        .ok_or_else(|| AppError::not_found("{{capitalized_resource_name}} not found"))?;

    Ok({{capitalized_resource_name}}Response::from({{resource_name}}))
}

pub async fn create_{{resource_name}}(
    mut {{resource_name}}_list: Vec<{{capitalized_resource_name}}>
) -> Result<{{capitalized_resource_name}}ListResponse, AppError> {
    {{resource_name}}_list.push({{capitalized_resource_name}} {
        id: String::from("{{resource_name}}_1"),
        created_at: 1710000000,
        updated_at: None,
    });

    Ok({{capitalized_resource_name}}ListResponse::from({{resource_name}}_list))
}

pub async fn update_{{resource_name}}(
    mut {{resource_name}}_list: Vec<{{capitalized_resource_name}}>,
    id: String,
) -> Result<{{capitalized_resource_name}}Response, AppError> {
    let {{resource_name}} = {{resource_name}}_list
        .iter_mut()
        .find(|{{resource_name}}| {{resource_name}}.id == id)
        .ok_or_else(|| AppError::not_found("{{capitalized_resource_name}} not found"))?;

    {{resource_name}}.updated_at = Some(1710000000);

    Ok({{capitalized_resource_name}}Response::from({{resource_name}}.clone()))
}

pub async fn delete_{{resource_name}}(
    mut {{resource_name}}_list: Vec<{{capitalized_resource_name}}>,
    id: String,
) -> Result<{{capitalized_resource_name}}DeleteResponse, AppError> {
    let initial_len = {{resource_name}}_list.len();

    {{resource_name}}_list.retain(|{{resource_name}}| {{resource_name}}.id != id);

    if {{resource_name}}_list.len() == initial_len {
        return Err(AppError::not_found("{{capitalized_resource_name}} not found"));
    }

    Ok({{capitalized_resource_name}}DeleteResponse { id })
}
