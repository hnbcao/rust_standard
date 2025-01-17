use std::any::Any;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use crate::assets::{AssetsOpenapi, AssetsStatic};
use crate::configs;
use crate::core::context::Context;
use crate::core::errors::AppResult;
use crate::core::salvo::context_inject::ContextInject;
use crate::core::salvo::logger::TraceLogger;
use crate::core::salvo::HEADER_APP_TOKEN;
use crate::core::shutdown;
use crate::core::version::Version;
use salvo::conn::TcpListener;
use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use salvo::oapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme};
use salvo::oapi::{Contact, Info, OpenApi, SecurityRequirement, SecurityScheme, Server as OpenApiServer};
use salvo::serve_static::static_embed;
use salvo::{Listener, Router, Server, Service};
use salvo_compression::Compression;
use crate::core::rapidoc::RapiDoc;

mod openapi;
mod web;

pub async fn start_web_service(ctx: Arc<Context>) -> AppResult<()> {
    let config = &ctx.config;
    let path = config.path();
    let context = ctx.clone();
    let server = Server::new(TcpListener::new(context.config.server.to_string()).bind().await);
    let handle = server.handle();
    shutdown::push(async move { handle.stop_graceful(Duration::from_secs(30)) }).await;

    let static_api = add_static(Router::new());
    let (mut webapi, webdoc) = merge_router(&ctx.version, || web::router());
    let (mut openapi, opendoc) = merge_router(&ctx.version, || openapi::router());

    // 内部添加cors，开发环境可开放，其他环境则严格设置
    if let Some(cors) = add_cors(&config.openapi) {
        webapi = webapi.hoop(cors.clone());
        openapi = openapi.hoop(cors);
    }

    // 合并所有请求路由
    let mut all_routers = if path.is_empty() { Router::new() } else { Router::with_path(&path) };

    // 返回内容有512kb时开启压缩
    let compression = Compression::new().disable_all();
    all_routers = all_routers.push(static_api.hoop(compression.clone()));
    all_routers = add_web_doc(webdoc, &config.openapi, all_routers);
    all_routers = add_open_doc(opendoc, &config.openapi, all_routers);
    all_routers = all_routers.push(webapi.hoop(compression.clone()));
    all_routers = all_routers.push(openapi.hoop(compression.clone()));

    let web = Service::new(all_routers).hoop(ContextInject { context: ctx.clone() }).hoop(TraceLogger);

    salvo::http::request::set_global_secure_max_size(1024 * 1024);
    server.serve(web).await;
    Ok(())
}

/// 合并路由
fn merge_router<F>(version: &Version, f: F) -> (Router, OpenApi)
where
    F: Fn() -> Router,
{
    let router = f();
    let doc = create_openapi(version).merge_router(&router);
    (router, doc)
}

/// 创建cors
fn add_cors(openapi_config: &configs::OpenApi) -> Option<CorsHandler> {
    if openapi_config.cors_origin.is_empty() {
        return None;
    }
    let origin = match openapi_config.cors_origin.contains(&String::from("*")) {
        true => AllowOrigin::any(),
        _ => (&openapi_config.cors_origin).into(),
    };

    let cors = Cors::new()
        .allow_origin(origin)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();
    Some(cors)
}

/// 创建开放文档
fn create_openapi(v: &Version) -> OpenApi {
    let info = Info::new("WEB模板应用", v.to_string())
        .description(v.description())
        .contact(Contact::new().name("WEB模板应用").email("admin@admin.com.cn"));
    OpenApi::with_info(info)
}

/// 添加静态资源
fn add_static(ar: Router) -> Router {
    ar.push(Router::with_path("openapi-doc/<*path>").get(static_embed::<AssetsOpenapi>()))
        .push(Router::with_path("static/<*path>").get(static_embed::<AssetsStatic>()))
}

/// 添加内部Openapi Doc
fn add_web_doc(doc: OpenApi, config: &configs::OpenApi, all_routers: Router) -> Router {
    if !config.enable_web_doc {
        return all_routers;
    }
    let server_doc = doc
        .servers([OpenApiServer::new(config.server.as_str())])
        .add_security_scheme("Qbee", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer).bearer_format("Bear")))
        .security(vec![SecurityRequirement::new("Qbee", ["edit:items", "read:items"])]);

    all_routers.push(server_doc.into_router("server-api-doc/openapi.json")).push(
        RapiDoc::new("server-api-doc/openapi.json")
            .title("TagFlow Openapi")
            .lib_url("openapi-doc/rapidoc-min.js")
            .into_router("/doc.html"),
    )
}

/// 添加对外Openapi Doc
fn add_open_doc(oo: OpenApi, config: &configs::OpenApi, all_routers: Router) -> Router {
    if !config.enable_open_doc {
        return all_routers;
    }
    let external_doc = oo
        .servers([OpenApiServer::new(config.server.as_str())])
        .add_security_scheme("App-Token", SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(HEADER_APP_TOKEN))))
        .security(vec![SecurityRequirement::new("App-Token", ["edit:items", "read:items"])]);

    all_routers.push(external_doc.into_router("external-api-doc/openapi.json")).push(
        RapiDoc::new("external-api-doc/openapi.json")
            .title("TagFlow Openapi")
            .lib_url("openapi-doc/rapidoc-min.js")
            .into_router("open-doc.html"),
    )
}
