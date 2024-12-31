use salvo::{Depot, oapi, Request, Response, Writer};
use salvo::http::StatusCode;
use salvo::oapi::{Components, EndpointOutRegister,  Operation, Ref, RefOr, Schema,  ToSchema};
use salvo::prelude::Text;
use sea_orm::prelude::async_trait;

use crate::core::errors::AppError;
use crate::core::salvo::{DEFAULT_JSON, REQUEST_ID_NAME, schema_500, to_401_schema, TRACE_ID_DEFAULT};
use crate::core::salvo::api_result::ResponseResult;

/// 全局异常处理
#[async_trait::async_trait]
impl Writer for AppError {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        tracing::error!("queries: {:?}, body: {:?}, msg: {}", req.queries(), req.body(), self);
        render_error_json(self, depot, res);
    }
}

fn render_error_json(error: AppError, depot: &mut Depot, res: &mut Response) {
    let id = depot
        .get::<String>(REQUEST_ID_NAME)
        .ok()
        .map_or(TRACE_ID_DEFAULT, |id| id.as_str());
    if let Ok(str) = ResponseResult::err(id, error.to_string().as_str()).to_string() {
        res.render(Text::Json(str));
    } else {
        res.render(Text::Json(DEFAULT_JSON));
    }
}

/// 500结构体
impl ToSchema for AppError {
    fn to_schema(components: &mut Components) -> RefOr<Schema> {
        let symbol = std::any::type_name::<Self>().replace("::", ".");
        let schema = schema_500(components);
        components.schemas.insert(symbol.clone(), schema);
        RefOr::Ref(Ref::new(format!("#/components/schemas/{}", symbol)))
    }
}

impl EndpointOutRegister for AppError {
    fn register(components: &mut Components, operation: &mut Operation) {
        operation.responses.insert(
            StatusCode::UNAUTHORIZED.as_str(),
            oapi::Response::new("Unauthorized").add_content("application/json", to_401_schema(components)),
        );
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Bad request or Internal Server Error").add_content("application/json", AppError::to_schema(components)),
        );
    }
}
