use salvo::http::StatusCode;
use salvo::oapi::{Components, Object, Ref, RefOr, Schema, schema, ToSchema};

use crate::core::errors::AppError;

mod error_handler;
pub mod api_result;
pub mod context_inject;
pub mod logger;

pub const TRACE_USER_OR_APP_NAME: &str = "USER-APP-IDENT";

/// 外部服务认证标识（HEADER）
pub const HEADER_APP_TOKEN: &str = "APP-TOKEN";

pub const REQUEST_ID_NAME: &str = "x-request-id";
pub const TRACE_ID_DEFAULT: &str = "unknown";
pub const DEFAULT_JSON: &str = r#"{"code":"500","message":null,"traceId":null,"data":null}"#;


/// 401展示的结构体
pub fn schema_401(components: &mut Components) -> Schema {
    Schema::from(
        Object::new()
            .property("code", to_string_schema(StatusCode::UNAUTHORIZED.as_str()))
            .required("code")
            .required("message")
            .property("message", to_string_schema(StatusCode::UNAUTHORIZED.canonical_reason().unwrap_or("unknown reason")))
            .required("traceId")
            .property("traceId", String::to_schema(components))
            .required("data")
            .property("data", schema::empty()),
    )
}

/// 500展示的结构体
pub fn schema_500(components: &mut Components) -> Schema {
    Schema::from(
        Object::new()
            .property("code", to_string_schema(StatusCode::INTERNAL_SERVER_ERROR.as_str()))
            .required("code")
            .required("message")
            .property("message", String::to_schema(components))
            .required("traceId")
            .property("traceId", String::to_schema(components))
            .required("data")
            .property("data", schema::empty()),
    )
}

/// 401结构体
fn to_401_schema(components: &mut Components) -> RefOr<Schema> {
    let mut symbol = std::any::type_name::<AppError>().replace("::", ".");
    // 需要添加后缀，不然会覆盖正常的展示
    symbol.push_str("401");
    let schema = schema_401(components);
    components.schemas.insert(symbol.clone(), schema);
    RefOr::Ref(Ref::new(format!("#/components/schemas/{}", symbol)))
}

/// 转换string类型
pub fn to_string_schema(str: impl Into<String>) -> Schema {
    Schema::Object(Object::new().default_value(serde_json::Value::String(str.into())))
}