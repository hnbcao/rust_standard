use std::sync::Arc;

use salvo::oapi::ToSchema;
use serde::Serialize;

use common::domain::user::user::Model as User;
use common::domain::user::UserRepository;

use crate::core::context::Context;
use crate::core::errors::{AppError, AppResult};

pub struct UserService;

impl UserService {
    #[tracing::instrument(skip(ctx))]
    pub(crate) async fn find_by_id(ctx: &Arc<Context>, user_id: i64) -> AppResult<UserVo> {
        let user = UserRepository::find_by_id(&ctx.db, user_id).await?;
        match user {
            None => Err(AppError::ApiRequestParam(format!("can not find user with id: {}", user_id))),
            Some(user) => Ok(user.into())
        }
    }

    #[tracing::instrument(skip(ctx))]
    pub(crate) async fn find_by_id_spawn(ctx: &Arc<Context>, user_id: i64) -> AppResult<UserVo> {
        let span = tracing::info_span!("info-span");
        let _enter = span.enter();
        let user = UserRepository::find_by_id(&ctx.db, user_id).await?;
        match user {
            None => Err(AppError::ApiRequestParam(format!("can not find user with id: {}", user_id))),
            Some(user) => Ok(user.into())
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserVo {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: String,
}

impl From<User> for UserVo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            phone: value.phone,
        }
    }
}