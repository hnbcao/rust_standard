mod user_api;

use actix_web::{HttpResponse, web as atx_web};

pub fn config(cfg: &mut atx_web::ServiceConfig) {
    cfg.service(
        atx_web::resource("/app")
            .route(atx_web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(atx_web::head().to(HttpResponse::MethodNotAllowed)),
    );
}