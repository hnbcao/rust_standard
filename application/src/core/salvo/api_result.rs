use salvo::{Response, Scribe};
use salvo::http::{HeaderValue, StatusCode};
use salvo::http::header::CONTENT_TYPE;
use salvo::hyper::http;
use salvo::oapi::{Components, EndpointOutRegister, oapi, Object, Operation, Ref, RefOr, Schema, ToSchema};
use serde::{Deserialize, Serialize};

use crate::core::errors::AppResult;
use crate::core::salvo::{DEFAULT_JSON, to_string_schema};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResponseResult<'a, T> {
    /// 错误码
    #[serde(default)]
    pub code: &'a str,
    /// 错误消息
    #[serde(default)]
    pub message: &'a str,
    /// 跟踪ID，发生错误时可根据该ID排查(为了兼容以前的格式)
    #[serde(default, rename = "traceId")]
    pub trace_id: Option<&'a str>,
    /// 数据对象（错误时为null）
    #[serde(default)]
    pub data: Option<T>,
}

impl<'a> ResponseResult<'_, bool> {
    pub fn err(trace_id: &'a str, message: &'a str) -> ResponseResult<'a, bool> {
        ResponseResult {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            message,
            trace_id: Some(trace_id),
            data: None,
        }
    }

    pub fn to_string(&self) -> AppResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<T> ResponseResult<'static, T>
where
    T: Serialize + ToSchema,
{
    pub fn ok(t: T) -> ResponseResult<'static, T> {
        ResponseResult {
            code: StatusCode::OK.as_str(),
            message: StatusCode::OK.as_str(),
            trace_id: None,
            data: Some(t),
        }
    }
}

impl<T> From<T> for ResponseResult<'static, T>
where
    T: Serialize + ToSchema,
{
    fn from(value: T) -> Self {
        ResponseResult::ok(value)
    }
}

impl<T> ToSchema for ResponseResult<'_, T>
where
    T: Serialize + ToSchema,
{
    fn to_schema(components: &mut Components) -> RefOr<Schema> {
        let symbol = std::any::type_name::<Self>().replace("::", ".");
        let status_code = http::StatusCode::OK;
        let schema = Object::new()
            .required("code")
            .property("code", to_string_schema(status_code.as_str()))
            .required("message")
            .property("message", to_string_schema(status_code.as_str()))
            .required("traceId")
            .property("traceId", String::to_schema(components))
            .required("data")
            .property("data", T::to_schema(components));
        components.schemas.insert(symbol.clone(), schema);
        RefOr::Ref(Ref::new(format!("#/components/schemas/{}", symbol)))
    }
}

impl<T> EndpointOutRegister for ResponseResult<'_, T>
where
    T: Serialize + ToSchema,
{
    fn register(components: &mut Components, operation: &mut Operation) {
        operation.responses.insert(
            StatusCode::OK.as_str(),
            oapi::Response::new("OK").add_content("application/json", Self::to_schema(components)),
        );
    }
}

impl<T> Scribe for ResponseResult<'static, T>
where
    T: Serialize + Send + ToSchema,
{
    #[inline]
    fn render(self, res: &mut Response) {
        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        match serde_json::to_vec(&self) {
            Ok(bytes) => {
                res.write_body(bytes).ok();
            }
            Err(e) => {
                tracing::error!(error = ?e, "JsonContent write error");
                res.render(DEFAULT_JSON);
            }
        }
    }
}