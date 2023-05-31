use actix_web::{web, HttpResponse, Scope};

pub fn nesting() -> Scope {
    web::scope("/nesting")
        .route(
            "/nested",
            web::get().to(|| async { HttpResponse::Ok().body("nested outisde") }),
        )
        .route(
            "/nested-1",
            web::get().to(|| async { HttpResponse::Ok().body("nested outisde 1") }),
        )
}
