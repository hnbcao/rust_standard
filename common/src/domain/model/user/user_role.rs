use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, EntityTrait, EnumIter, RelationDef, RelationTrait};
use crate::domain::model::user::{role, user};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user_id: String,
    pub role_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
                .into(),
            Relation::Role => Entity::belongs_to(role::Entity)
                .from(Column::RoleId)
                .to(role::Column::Id)
                .into(),
        }
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}