use std::time::Instant;

use salvo::{Depot, FlowCtrl, Handler, Request, Response};
use salvo::http::mime;
use tracing::{Instrument, Level};
use uuid::Uuid;

use crate::core::salvo::{REQUEST_ID_NAME, TRACE_USER_OR_APP_NAME};

/// 分布式日志追踪
#[derive(Debug)]
pub struct TraceLogger;

#[async_trait::async_trait]
impl Handler for TraceLogger {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let trace = Uuid::new_v4().as_simple().to_string();
        let span = tracing::span!(Level::INFO,
            "tfp-web",
            trace=%trace,
            remote_addr = %req.remote_addr().to_string(),
            version = ?req.version(),
            method = %req.method(),
            path = %req.uri(),);
        depot.insert(REQUEST_ID_NAME, trace);
        // 使用span.enter();有时候不会和span一起输出
        let url = format!("{}{}", req.remote_addr(), req.uri().path());
        if req.content_type().map_or(false, |f| f.subtype().eq(&mime::JSON)) {
            let body: serde_json::Value = req.parse_json().await.unwrap_or(serde_json::Value::Null);
            span.in_scope(|| tracing::info!("{} queries: {:?} body: {:?}", url, req.queries(), body.to_string()));
        } else {
            span.in_scope(|| tracing::info!("{} queries: {:?} ", url, req.queries()));
        }
        async move {
            let now = Instant::now();
            ctrl.call_next(req, depot, res).await;
            let duration = now.elapsed();

            if let Ok(u) = depot.get::<String>(TRACE_USER_OR_APP_NAME) {
                tracing::info!("{} {} {:?}", u, url, duration);
            } else {
                tracing::info!("{} {:?}", url, duration);
            }
        }
            .instrument(span)
            .await
    }
}

#[cfg(test)]
mod tests {
    use salvo::{Depot, Router};
    use salvo::prelude::endpoint;
    use salvo::Request;
    use salvo::test::ResponseExt;
    use salvo::test::TestClient;
    use tracing::instrument;

    use crate::core::errors::AppResult;
    use crate::core::salvo::api_result::ResponseResult;
    use crate::core::salvo::logger::TraceLogger;

    #[tokio::test]
    async fn test_log() {
        tracing_subscriber::fmt::init();

        #[endpoint]
        #[instrument]
        async fn hello() -> AppResult<ResponseResult<'static, &'static str>> {
            tracing::info!("ssssssssssssssssssssssss");
            Ok(ResponseResult::ok("hello"))
            // Err(TagFlowError::ApiRequestParamStr("err"))
        }

        #[endpoint]
        #[instrument]
        async fn hello2(depot: &Depot) -> AppResult<ResponseResult<'static, &'static str>> {
            tracing::info!("ssssssssssssssssssssssss");
            Ok(ResponseResult::ok("hello"))
            // Err(TagFlowError::ApiRequestParamStr("err"))
        }

        let router = Router::new()
            .hoop(TraceLogger)
            .push(Router::with_path("hello").get(hello))
            .push(Router::with_path("hello2").get(hello2));

        let s = TestClient::
        //get("http://127.0.0.1:5801/hello?sss=22&eee=23")
        get("http://127.0.0.1:5801/hello2?sss=22&eee=23")
            .add_header("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjdXJyZW50VGltZU1pbGxpcyI6IjE3MjIyMTkwMTAyNjEiLCJhY2Nlc3NDb2RlIjoicWIucWJlZS5hbGwiLCJuZWVkUmVmcmVzaCI6dHJ1ZSwiZXhwIjoxNzIyODIzODEwLCJ1c2VySWQiOiIwMDE0NWYzZjhlYzQ0MGFiYTIxMjYwOGNlMzFjZGQyMiIsImFjY291bnRLZXkiOiIwMDE0NWYzZjhlYzQ0MGFiYTIxMjYwOGNlMzFjZGQyMiJ9.MmB3DeH02uVgzctq6R06v2ej3wYqjUlQ1oWGWgjmevg.ZTU4NTBmYjdhNjliNGY2OWEzZWFhNDY2NWJiYTg3MTU=", true)
            .send(router)
            .await
            .take_string()
            .await
            .unwrap();
        println!("{}", s);
        // assert!(logs_contain("duration"));
    }
}
