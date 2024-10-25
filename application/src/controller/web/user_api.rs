use salvo::{
    Depot,
    oapi::{endpoint, extract::PathParam}, Router, Writer,
};

use crate::core::errors::AppResult;
use crate::core::salvo::api_result::ResponseResult;
use crate::core::salvo::context_inject::obtain_context;
use crate::service::user_service::{UserService, UserVo};

/// 查询用户信息
#[endpoint(tags("用户管理"), parameters(("id", description = "用户ID")))]
async fn get_ins(depot: &mut Depot, id: PathParam<i64>) -> AppResult<ResponseResult<'static, UserVo>> {
    let ctx = obtain_context(depot)?;
    tracing::info!("this is info log in controller.");
    tracing::debug!("this is debug log in controller.");
    tracing::event!(tracing::Level::INFO,label=2,"this is event log in controller.");
    let user = UserService::find_by_id(ctx, id.into_inner()).await?;
    Ok(ResponseResult::ok(user))
}

/// 查询用户信息2
#[endpoint(tags("用户管理"), parameters(("id", description = "用户ID")))]
async fn get_spawn(depot: &mut Depot, id: PathParam<i64>) -> AppResult<ResponseResult<'static, UserVo>> {
    let ctx = obtain_context(depot)?;
    tracing::info!("this is info log in controller.");
    tracing::debug!("this is debug log in controller.");
    tracing::event!(tracing::Level::INFO,label=2,"this is event log in controller.");
    let user = UserService::find_by_id(ctx, id.into_inner()).await?;
    Ok(ResponseResult::ok(user))
}

pub(crate) fn router() -> Router {
    Router::with_path("user")
        .push(Router::with_path("ins/<id:num>").get(get_ins))
        .push(Router::with_path("spawn/<id:num>").get(get_spawn))
}