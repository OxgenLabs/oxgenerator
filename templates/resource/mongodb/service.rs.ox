use crate::common::error::AppError;

use mongodb::{
    Collection,
    bson::{Document, doc, oid::ObjectId},
    options::ReturnDocument,
};

use super::{
    dto::{
        {{capitalized_resource_name}}DeleteResponse,
        {{capitalized_resource_name}}ListResponse,
        {{capitalized_resource_name}}Response,
    },
    model::{{capitalized_resource_name}},
};

pub async fn get_all_{{resource_name}}(
    {{resource_name}}_collection: Collection<{{capitalized_resource_name}}>,
) -> Result<{{capitalized_resource_name}}ListResponse, AppError> {
    let mut cursor = {{resource_name}}_collection.find(doc! {}).await?;
    let mut results = Vec::new();

    while cursor.advance().await? {
        let {{resource_name}} = cursor.deserialize_current()?;
        results.push({{resource_name}});
    }

    Ok({{capitalized_resource_name}}ListResponse::from(results))
}

pub async fn get_{{resource_name}}(
    {{resource_name}}_collection: Collection<{{capitalized_resource_name}}>,
    id: ObjectId,
) -> Result<{{capitalized_resource_name}}Response, AppError> {
    let {{resource_name}} = {{resource_name}}_collection
        .find_one(doc! {
            "_id": id,
        })
        .await?
        .ok_or_else(|| AppError::not_found("{{capitalized_resource_name}} not found"))?;

    Ok({{capitalized_resource_name}}Response::from(
        {{resource_name}},
    ))
}

pub async fn create_{{resource_name}}(
    {{resource_name}}_collection: Collection<{{capitalized_resource_name}}>,
    {{resource_name}}: {{capitalized_resource_name}},
) -> Result<{{capitalized_resource_name}}Response, AppError> {
    let id = {{resource_name}}.id;

    {{resource_name}}_collection
        .insert_one(&{{resource_name}})
        .await?;

    let created_{{resource_name}} = {{resource_name}}_collection
        .find_one(doc! {
            "_id": id,
        })
        .await?
        .ok_or_else(|| AppError::not_found("{{capitalized_resource_name}} not found after creation"))?;

    Ok({{capitalized_resource_name}}Response::from(
        created_{{resource_name}},
    ))
}

pub async fn update_{{resource_name}}(
    {{resource_name}}_collection: Collection<{{capitalized_resource_name}}>,
    id: ObjectId,
    update: Document,
) -> Result<{{capitalized_resource_name}}Response, AppError> {
    let updated_{{resource_name}} = {{resource_name}}_collection
        .find_one_and_update(
            doc! {
                "_id": id,
            },
            doc! {
                "$set": update,
            },
        )
        .return_document(ReturnDocument::After)
        .await?
        .ok_or_else(|| AppError::not_found("{{capitalized_resource_name}} not found"))?;

    Ok({{capitalized_resource_name}}Response::from(
        updated_{{resource_name}},
    ))
}

pub async fn delete_{{resource_name}}(
    {{resource_name}}_collection: Collection<{{capitalized_resource_name}}>,
    id: ObjectId,
) -> Result<{{capitalized_resource_name}}DeleteResponse, AppError> {
    let result = {{resource_name}}_collection
        .delete_one(doc! {
            "_id": id,
        })
        .await?;

    if result.deleted_count == 0 {
        return Err(AppError::not_found(
            "{{capitalized_resource_name}} not found",
        ));
    }

    Ok({{capitalized_resource_name}}DeleteResponse { id })
}
