use tokio::runtime::{Builder, Runtime};

use crate::configs::AppConfig;
use crate::controller::start_web_service;
use crate::core::context::Context;
use crate::core::opentelemetry::init_tracing_subscriber;
use crate::core::shutdown;

pub(crate) mod configs;
mod assets;
mod core;
mod controller;
mod service;

fn main() {
    let worker = num_cpus::get().max(2);
    let rt = build_rt(worker, "main").expect("init runtime fail");
    rt.block_on(async {
        init_tracing_subscriber("http://10.73.13.61:4317").expect("can not init logger.");
        let config = AppConfig::load().expect("can not load application configuration.");
        let ctx = Context::new(config).await.expect("can not init application context.");
        ctx.run_database_migration().await.expect("can not process database migrations.");
        // ctx.start().await.expect("can not start application.");
        ctx.add_cluster_event_hook().await;
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

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use opentelemetry::global;
    use tokio::time::sleep;
    use tracing::Instrument;
    use tracing_core::Level;
    use uuid::Uuid;

    use crate::build_rt;
    use crate::core::opentelemetry::init_tracing_subscriber;

    #[test]
    pub fn test_logger() {
        let worker = num_cpus::get().max(2);
        let rt = build_rt(worker, "main").expect("init runtime fail");
        rt.block_on(async {
            let _guard = init_tracing_subscriber("http://10.73.13.61:4317").expect("can not init logger.");
            log(1);
            log2().await;
            log3();
            log4();
            log5_2().await;
            sleep(Duration::from_secs(10)).await;
        });
    }

    async fn log2() {
        let trace = Uuid::new_v4().as_simple().to_string();
        let span = tracing::span!(Level::INFO, "test-log2", trace=%trace);
        async move {
            log(2);
        }.instrument(span).await;
    }

    fn log3() {
        let trace = Uuid::new_v4().as_simple().to_string();
        let span = tracing::span!(Level::INFO, "test-log3", trace=%trace);
        let _enter = span.enter();
        log(3);
    }

    #[tracing::instrument]
    fn log4() {
        log(4);
    }

    fn log(id: i64) {
        tracing::info!("this is info log {}.",id);
        tracing::warn!("this is warn log {}.",id);
        tracing::debug!("this is debug log {}.",id);
        tracing::error!("this is error log {}.",id);
    }

    async fn log5_2() {
        let trace = Uuid::new_v4().as_simple().to_string();
        let span = tracing::span!(Level::INFO, "test-log52", trace=%trace);
        async move {
            tracing::info!("this is info log {}.",52);
            log5_3(53);
        }.instrument(span).await;
    }

    fn log5_3(id: i64) {
        let trace = Uuid::new_v4().as_simple().to_string();
        let span = tracing::span!(Level::INFO, "test-log53", trace=%trace);
        let _enter = span.enter();
        tracing::info!("this is info log {}.",id);
        log5_4(54);
    }


    #[tracing::instrument]
    fn log5_4(id: i64) {
        tracing::info!("this is info log {}.",id);
    }
}