use actix_web::{HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("open") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}