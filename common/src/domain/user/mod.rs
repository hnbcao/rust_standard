use sea_orm::{DbConn, DbErr, EntityTrait};

use crate::domain::user::user::{Entity as UserEntity, Model as UserModel};

pub mod user;
pub mod role;
pub mod user_role;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<UserModel>, DbErr> {
        UserEntity::find_by_id(id).one(db).await
    }
}