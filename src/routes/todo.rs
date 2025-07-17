use chrono::{DateTime, Utc};
use crudcrate::{CRUDResource, EntityToModels};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, EntityToModels)]
#[sea_orm(table_name = "todo")]
#[crudcrate(description = "Manages a list of todo items", generate_router)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    // This will generate the UUID for `id` on create and not include them in generated update or create models
    #[crudcrate(primary_key, sortable, create_model = false, update_model = false, on_create = Uuid::new_v4())]
    pub id: Uuid,
    // This will be included in sort and filter queries in the generated API
    #[crudcrate(sortable, filterable)]
    pub title: String,
    // This will be included in the filter queries and will optional on create but set to false by default
    #[crudcrate(filterable, on_create = false)]
    pub completed: bool,
    // Updates the `last_updated` field when created and updated with Utc::now() function, and doesn't include it in the create or update models
    #[crudcrate(sortable, create_model = false, update_model = false, on_create = chrono::Utc::now(), on_update = chrono::Utc::now())]
    pub last_updated: DateTime<Utc>,
    // Non-database field for additional metadata that doesn't get stored in the DB
    // Use sea_orm(ignore) to exclude from DB mapping and crudcrate(non_db_attr) for API generation
    #[sea_orm(ignore)]
    #[crudcrate(non_db_attr = true, default = vec![])]
    pub tags: Vec<String>,
}

// Sea-orm required relation fields
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
