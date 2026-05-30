use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    modules::{{resource_name}}::controller::{
        create_{{resource_name}}_handler,
        delete_{{resource_name}}_handler,
        get_{{resource_name}}_handler,
        get_all_{{resource_name}}_handler,
        update_{{resource_name}}_handler,
    },
    state::AppState,
};

pub fn {{resource_name}}_routes() -> Router<AppState> {
    let routes = Router::new()
        .route("/", get(get_all_{{resource_name}}_handler))
        .route("/{id}", get(get_{{resource_name}}_handler))
        .route("/", post(create_{{resource_name}}_handler))
        .route("/{id}", patch(update_{{resource_name}}_handler))
        .route("/{id}", delete(delete_{{resource_name}}_handler));

    Router::new().nest("/{{resource_name}}", routes)
}
