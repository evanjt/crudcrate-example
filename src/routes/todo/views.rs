use super::models::{Todo, TodoCreate, TodoUpdate};
use crudcrate::{CRUDResource, crud_handlers};
use sea_orm::DatabaseConnection;
use utoipa_axum::{router::OpenApiRouter, routes};

// Build the CRUD handlers for the Todo model to be used in the router.
// The macro will generate the following functions:
// - get_one_handler at GET /todo/{id}
// - get_all_handler at GET /todo
// - create_one_handler at POST /todo
// - update_one_handler at PUT /todo/{id}
// - delete_one_handler at DELETE /todo/{id}
// - delete_many_handler at DELETE /todo/batch

crud_handlers!(Todo, TodoUpdate, TodoCreate);

pub fn router(db: &DatabaseConnection) -> OpenApiRouter
where
    Todo: CRUDResource,
{
    OpenApiRouter::new()
        .routes(routes!(get_one_handler))
        .routes(routes!(get_all_handler))
        .routes(routes!(create_one_handler))
        .routes(routes!(update_one_handler))
        .routes(routes!(delete_one_handler))
        .routes(routes!(delete_many_handler))
        .with_state(db.clone())
}
