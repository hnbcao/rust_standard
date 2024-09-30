pub(crate) mod configs;
mod assets;
mod core;
mod controller;

use actix::System;
use actix_web::{get, web, App, HttpServer, Responder};
use tokio::runtime::{Builder, Runtime};
use crate::configs::AppConfig;
use crate::controller::start_web_service;
use crate::core::context::Context;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}


fn main() {
    let worker = num_cpus::get().max(2);
    let rt = System::with_tokio_rt(|| build_rt(worker, "main").expect("init runtime fail"));
    rt.block_on(async {
        let config = AppConfig::load().expect("can not load application configuration.");
        let ctx = Context::new(config).expect("can not inital application context.").into();
        start_web_service(ctx).await.expect("web service start fail.")
    });
    // HttpServer::new(|| {
    //     App::new().service(greet)
    // })
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await
}

fn build_rt(worker: usize, name: impl Into<String>) -> std::io::Result<Runtime> {
    let name = name.into();
    Builder::new_multi_thread()
        .worker_threads(worker)
        .enable_all()
        .thread_name(name)
        .build()
}