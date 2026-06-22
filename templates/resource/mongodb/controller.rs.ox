use axum::{
    Json,
    extract::{Path, State},
};
use mongodb::bson::{Document, oid::ObjectId};

use crate::{
    common::{
        error::AppError,
        response::{ApiResponse, AppResult},
    },
    state::AppState,
};

use super::{
    dto::{
        {{capitalized_resource_name}}DeleteResponse,
        {{capitalized_resource_name}}ListResponse,
        {{capitalized_resource_name}}Response,
    },
    model::{{capitalized_resource_name}},
    service,
};

pub async fn get_all_{{resource_name}}_handler(
    State(state): State<AppState>,
) -> AppResult<{{capitalized_resource_name}}ListResponse> {
    let {{resource_name}}_collection = state
        .database
        .collection::<{{capitalized_resource_name}}>("{{collection_name}}");

    let response =
        service::get_all_{{resource_name}}({{resource_name}}_collection).await?;

    Ok(ApiResponse::success(response))
}

pub async fn get_{{resource_name}}_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<{{capitalized_resource_name}}Response> {
    let id = ObjectId::parse_str(&id).map_err(|_| AppError::bad_request("Invalid {{resource_name}} id"))?;

    let {{resource_name}}_collection = state
        .database
        .collection::<{{capitalized_resource_name}}>("{{collection_name}}");

    let response =
        service::get_{{resource_name}}({{resource_name}}_collection, id).await?;

    Ok(ApiResponse::success(response))
}

pub async fn create_{{resource_name}}_handler(
    State(state): State<AppState>,
    Json({{resource_name}}): Json<{{capitalized_resource_name}}>,
) -> AppResult<{{capitalized_resource_name}}Response> {
    let {{resource_name}}_collection = state
        .database
        .collection::<{{capitalized_resource_name}}>("{{collection_name}}");

    let response = service::create_{{resource_name}}(
        {{resource_name}}_collection,
        {{resource_name}},
    )
    .await?;

    Ok(ApiResponse::success(response))
}

pub async fn update_{{resource_name}}_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(update): Json<Document>,
) -> AppResult<{{capitalized_resource_name}}Response> {
    let id = ObjectId::parse_str(&id).map_err(|_| AppError::bad_request("Invalid {{resource_name}} id"))?;

    let {{resource_name}}_collection = state
        .database
        .collection::<{{capitalized_resource_name}}>("{{collection_name}}");

    let response = service::update_{{resource_name}}(
        {{resource_name}}_collection,
        id,
        update,
    )
    .await?;

    Ok(ApiResponse::success(response))
}

pub async fn delete_{{resource_name}}_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<{{capitalized_resource_name}}DeleteResponse> {
    let id = ObjectId::parse_str(&id).map_err(|_| AppError::bad_request("Invalid {{resource_name}} id"))?;

    let {{resource_name}}_collection = state
        .database
        .collection::<{{capitalized_resource_name}}>("{{collection_name}}");

    let response = service::delete_{{resource_name}}(
        {{resource_name}}_collection,
        id,
    )
    .await?;

    Ok(ApiResponse::success(response))
}
