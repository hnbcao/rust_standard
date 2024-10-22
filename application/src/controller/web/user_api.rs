// use salvo::{Depot, Router};
// use salvo::oapi::endpoint;
// use crate::controller::web::hello;
// use crate::core::errors::AppResult;
//
// /// 查询用户信息
// #[endpoint(tags("元数据-点位"))]
// async fn get(depot: &mut Depot, query_param: JsonBody<AttrIdsOpenQuery>) -> AppResult<ResponseResult<'static, HashMap<i64, MixAttrOpenVo>>> {
//
// }
//
// pub(crate) fn router() -> Router {
//     Router::new().get(hello)
// }