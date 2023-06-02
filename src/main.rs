use std::{str::FromStr, sync::Mutex};
mod custom_error;
mod custom_middleware;
mod extractors;
mod from_db;
mod nesting;
mod tls;

use actix_cors::Cors;
use actix_web::{
    dev::Service, get, http::StatusCode, middleware::Logger, post, web, App, HttpResponse,
    HttpServer, Responder, Result,
};
use custom_error::AppError;
use custom_middleware::Authorization;
use dotenvy::{dotenv, var as get_env};
use from_db::fetch_from_db;
use nesting::nesting;
use stargate_grpc::{client, AuthToken, StargateClient};
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let _app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web Project"),
        count: Mutex::new(0),
    });

    let astra_ui = get_env("ASTRA_URI").expect("ASTRA_URI not found");
    let bearer_token = get_env("BEARER_TOKEN").expect("BEARER_TOKEN not found");

    let astra_client = StargateClient::builder()
        .uri(astra_ui)
        .expect("Invalid ASTRA_URI")
        .auth_token(AuthToken::from_str(&bearer_token).expect("Invalid BEARER_TOKEN"))
        .tls(Some(
            client::default_tls_config().expect("Couldn't load TLS config"),
        ))
        .connect()
        .await
        .expect("Couldn't connect to stargate");

    log::info!("ASTRA DB connection SUCCESS");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(astra_client.clone()))
            // .app_data(app_state.clone())
            .wrap(Authorization)
            .wrap(Cors::default().allow_any_origin())
            .wrap(Logger::new("%t %r %s %b B %D ms"))
            .wrap_fn(|req, srv| {
                println!("Hello i'm under the water");
                srv.call(req)
            })
            .service(fetch_from_db)
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
    .bind_rustls("0.0.0.0:5000", get_tls_config())
    .expect("Unable to bind with TLS at 0.0.0.0:5000")
    .bind("0.0.0.0:5001")
    .expect("Unable to bind at 0.0.0.0:5001")
    .run()
    .await
}
