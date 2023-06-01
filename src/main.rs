use std::sync::Mutex;
mod custom_error;
mod custom_middleware;
mod extractors;
mod nesting;
mod tls;

use actix_cors::Cors;
use actix_web::{
    dev::Service, get, http::StatusCode, middleware::Logger, post, web, App, HttpResponse,
    HttpServer, Responder, Result,
};
use custom_error::AppError;
use custom_middleware::Authorization;
use nesting::nesting;
use tls::get_tls_config;

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

#[get("/error")]
async fn throw_error() -> Result<String, AppError> {
    Err(AppError::new(StatusCode::BAD_GATEWAY, "Bad gateway"))
}

struct AppState {
    app_name: String,
    count: Mutex<i32>,
}

#[get("/app_name")]
async fn app_name(data: web::Data<AppState>) -> String {
    format!("Hello {}!", &data.app_name)
}

#[get("/counter")]
async fn counter(data: web::Data<AppState>) -> String {
    let mut counter = data.count.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web Project"),
        count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Authorization)
            .wrap(Cors::default().allow_any_origin())
            .wrap(Logger::new("%t %r %s %b B %D ms"))
            .wrap_fn(|req, srv| {
                println!("Hello i'm under the water");
                srv.call(req)
            })
            .app_data(app_state.clone())
            .service(extractors::extractors())
            .service(nesting())
            .service(counter)
            .service(app_name)
            .service(hello)
            .service(echo)
            .service(throw_error)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/vijayans").service(suraj).service(monish))
    })
    .bind_rustls("0.0.0.0:5000", get_tls_config())?
    .bind("0.0.0.0:5001")?
    .run()
    .await
}
