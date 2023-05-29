use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/suraj")]
async fn suraj() -> impl Responder {
    HttpResponse::Ok().body("Suraj Vijayan")
}

#[get("/monish")]
async fn monish() -> impl Responder {
    HttpResponse::Ok().body("Monish Vijayan")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("vijayans").service(suraj).service(monish))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
