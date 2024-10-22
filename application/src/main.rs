use tokio::runtime::{Builder, Runtime};

use crate::configs::AppConfig;
use crate::controller::start_web_service;
use crate::core::context::Context;
use crate::core::shutdown;

pub(crate) mod configs;
mod assets;
mod core;
mod controller;

fn main() {
    let worker = num_cpus::get().max(2);
    let rt = build_rt(worker, "main").expect("init runtime fail");
    rt.block_on(async {
        let config = AppConfig::load().expect("can not load application configuration.");
        let ctx = Context::new(config).await.expect("can not init application context.");
        ctx.run_database_migration().await.expect("can not process database migrations.");
        // ctx.start().await.expect("can not start application.");
        start_web_service(ctx.into()).await.expect("web service start fail.");
        // 等到所有任务优雅关闭
        shutdown::completed().await;
    });
}

fn build_rt(worker: usize, name: impl Into<String>) -> std::io::Result<Runtime> {
    let name = name.into();
    Builder::new_multi_thread()
        .worker_threads(worker)
        .enable_all()
        .thread_name(name)
        .build()
}