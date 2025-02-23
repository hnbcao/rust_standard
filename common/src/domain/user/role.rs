use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, EnumIter};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}