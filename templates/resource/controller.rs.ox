use axum::extract::Path;

use crate::common::response::{ApiResponse, AppResult};

use super::{
    dto::{
        {{capitalized_resource_name}}DeleteResponse,
        {{capitalized_resource_name}}ListResponse,
        {{capitalized_resource_name}}Response,
    },
    model::{{capitalized_resource_name}},
    service,
};

fn mock_{{resource_name}}_list() -> Vec<{{capitalized_resource_name}}> {
    vec![
        {{capitalized_resource_name}} {
            id: String::from("{{resource_name}}_1"),
            created_at: 1710000000,
            updated_at: None,
        },
        {{capitalized_resource_name}} {
            id: String::from("{{resource_name}}_2"),
            created_at: 1710000001,
            updated_at: None,
        },
    ]
}

pub async fn get_all_{{resource_name}}_handler() -> AppResult<{{capitalized_resource_name}}ListResponse> {
    let {{resource_name}}_list = mock_{{resource_name}}_list();

    let response = service::get_all_{{resource_name}}({{resource_name}}_list).await?;

    Ok(ApiResponse::success(response))
}

pub async fn get_{{resource_name}}_handler(
    Path(id): Path<String>,
) -> AppResult<{{capitalized_resource_name}}Response> {
    let {{resource_name}}_list = mock_{{resource_name}}_list();

    let response = service::get_{{resource_name}}({{resource_name}}_list, id).await?;

    Ok(ApiResponse::success(response))
}

pub async fn create_{{resource_name}}_handler() -> AppResult<{{capitalized_resource_name}}ListResponse> {
    let {{resource_name}}_list = mock_{{resource_name}}_list();

    let response = service::create_{{resource_name}}({{resource_name}}_list).await?;

    Ok(ApiResponse::success(response))
}

pub async fn update_{{resource_name}}_handler(
    Path(id): Path<String>,
) -> AppResult<{{capitalized_resource_name}}Response> {
    let {{resource_name}}_list = mock_{{resource_name}}_list();

    let response = service::update_{{resource_name}}({{resource_name}}_list, id).await?;

    Ok(ApiResponse::success(response))
}

pub async fn delete_{{resource_name}}_handler(
    Path(id): Path<String>,
) -> AppResult<{{capitalized_resource_name}}DeleteResponse> {
    let {{resource_name}}_list = mock_{{resource_name}}_list();

    let response = service::delete_{{resource_name}}({{resource_name}}_list, id).await?;

    Ok(ApiResponse::success(response))
}
