use std::any::{Any, TypeId};
use std::sync::Arc;
use std::time::Duration;

use salvo::{Depot, handler, Listener, Router, Server, Service};
use salvo::conn::TcpListener;

use crate::core::context::Context;
use crate::core::errors::AppResult;
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

    let webapi = Router::with_path(&path).push(web::router());
    let openapi = Router::with_path(&path).push(openapi::router());

    let web = Service::new(Router::new().push(webapi).push(openapi)).hoop(ContextInject { context: ctx.clone() });
    // .hoop(TraceLogger)
    // .catcher(SalvoErrorCatcher::catcher());
    salvo::http::request::set_global_secure_max_size(1024 * 1024);
    server.serve(web).await;
    Ok(())
}

pub struct ContextInject<T: Send + Sync + 'static> {
    pub context: Arc<T>,
}

#[handler]
impl<T: Send + Sync + 'static> ContextInject<T> {
    async fn handle(&self, depot: &mut Depot) {
        insert_arc(depot, self.context.clone())
    }
}

/// 从上下文中获取Arc<T>信息
pub fn insert_arc<T: Any + Send + Sync>(depot: &mut Depot, t: Arc<T>) {
    depot.insert(&type_key::<Arc<T>>(), t);
}

/// 设置泛型类，注意使用Arc等容器时，可能每次获取都不一样
#[inline]
fn type_key<T: 'static>() -> String {
    format!("{:?}", TypeId::of::<T>())
}