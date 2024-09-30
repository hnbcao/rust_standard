use std::sync::Arc;
use actix_web::{App, HttpServer};
use crate::core::context::Context;
use crate::core::errors::AppResult;
use actix_web::web as atx_web;

mod web;
mod openapi;

pub async fn start_web_service(ctx: Arc<Context>) -> std::io::Result<()> {
    let config = &ctx.config;
    let addrs = config.addrs();
    let path = config.path();
    let ctx = ctx.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .service(atx_web::scope(&path)
                .configure(web::config)
                .service(atx_web::scope("/open").configure(openapi::config)))
            .route("/healthz", atx_web::get().to(|| async { "hello world" }))
    }).bind(addrs)?.run().await
}