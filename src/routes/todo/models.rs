use super::db::Model;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crudcrate::{CRUDResource, ToCreateModel, ToUpdateModel};
use sea_orm::{
    ActiveValue, Condition, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, Order,
    QueryOrder, QuerySelect, entity::prelude::*,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    ToSchema, Serialize, Deserialize, FromQueryResult, ToUpdateModel, ToCreateModel, Clone,
)]
#[active_model = "super::db::ActiveModel"]
pub struct Todo {
    // This will generate the UUID on create and not generate update or create models in documentation
    #[crudcrate(create_model = false, update_model = false, on_create = Uuid::new_v4())]
    id: Uuid,
    title: String,
    #[crudcrate(on_create = false)]
    completed: bool,
    // Update the last_updated field on create and update
    #[crudcrate(create_model = false, update_model = false, on_create = chrono::Utc::now(), on_update = chrono::Utc::now())]
    last_updated: DateTime<Utc>,
    // This is a field not in the database model that we can generate in our get one/all functions below
    #[crudcrate(
        non_db_attr = true,
        default = None,
    )]
    user_readable_title: Option<String>,
}

impl From<Model> for Todo {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            title: model.title.clone(),
            completed: model.completed,
            last_updated: model.last_updated,
            user_readable_title: format!("Todo: {}", model.title).into(), // We can define the non-db field here during From<Model>
        }
    }
}

#[async_trait]
impl CRUDResource for Todo {
    type EntityType = super::db::Entity;
    type ColumnType = super::db::Column;
    type ModelType = super::db::Model;
    type ActiveModelType = super::db::ActiveModel;
    type ApiModel = Todo;
    type CreateModel = TodoCreate;
    type UpdateModel = TodoUpdate;

    const ID_COLUMN: Self::ColumnType = super::db::Column::Id;
    const RESOURCE_NAME_PLURAL: &'static str = "todos";
    const RESOURCE_NAME_SINGULAR: &'static str = "todo";
    const RESOURCE_DESCRIPTION: &'static str = "A simple todo list item.";

    async fn get_all(
        db: &DatabaseConnection,
        condition: Condition,
        order_column: Self::ColumnType,
        order_direction: Order,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<Self::ApiModel>, DbErr> {
        let models = Self::EntityType::find()
            .filter(condition)
            .order_by(order_column, order_direction)
            .offset(offset)
            .limit(limit)
            .all(db)
            .await?;
        Ok(models.into_iter().map(Self::ApiModel::from).collect())
    }

    async fn get_one(db: &DatabaseConnection, id: Uuid) -> Result<Self::ApiModel, DbErr> {
        let model =
            Self::EntityType::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!(
                    "{} not found",
                    Self::RESOURCE_NAME_SINGULAR
                )))?;
        Ok(Self::ApiModel::from(model))
    }

    async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        update_data: Self::UpdateModel,
    ) -> Result<Self::ApiModel, DbErr> {
        let existing: Self::ActiveModelType = Self::EntityType::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "{} not found",
                Self::RESOURCE_NAME_PLURAL
            )))?
            .into();

        let updated_model = update_data.merge_into_activemodel(existing);
        let updated = updated_model.update(db).await?;
        Ok(Self::ApiModel::from(updated))
    }

    fn sortable_columns() -> Vec<(&'static str, Self::ColumnType)> {
        vec![
            ("id", Self::ColumnType::Id),
            ("title", Self::ColumnType::Title),
            ("completed", Self::ColumnType::Completed),
            ("last_updated", Self::ColumnType::LastUpdated),
        ]
    }

    fn filterable_columns() -> Vec<(&'static str, Self::ColumnType)> {
        vec![
            ("title", Self::ColumnType::Title),
            ("completed", Self::ColumnType::Completed),
        ]
    }
}
