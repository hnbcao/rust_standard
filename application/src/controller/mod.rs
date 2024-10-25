use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

use salvo::{Listener, Router, Server, Service};
use salvo::conn::TcpListener;

use crate::core::context::Context;
use crate::core::errors::AppResult;
use crate::core::salvo::context_inject::ContextInject;
use crate::core::salvo::logger::TraceLogger;
use crate::core::shutdown;

mod web;
mod openapi;

pub async fn start_web_service(ctx: Arc<Context>) -> AppResult<()> {
    let config = &ctx.config;
    let path = config.path();
    let context = ctx.clone();
    let server = Server::new(TcpListener::new(context.config.server.to_string()).bind().await);
    let handle = server.handle();
    shutdown::push(async move { handle.stop_graceful(Duration::from_secs(30)) }).await;

    let (webapi, openapi) = if path.is_empty() {
        (Router::new().push(web::router()), Router::new().push(openapi::router()))
    } else {
        (Router::with_path(&path).push(web::router()), Router::with_path(&path).push(openapi::router()))
    };

    let web = Service::new(Router::new()
        .push(webapi)
        .push(openapi))
        .hoop(ContextInject { context: ctx.clone() })
        .hoop(TraceLogger);

    salvo::http::request::set_global_secure_max_size(1024 * 1024);
    server.serve(web).await;
    Ok(())
}