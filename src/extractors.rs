use actix_web::{
    get,
    web::{Json, Path, Query},
    Result,
};
use actix_web::{HttpRequest, Scope};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ExtractorsKind {
    no: i32,
    string: String,
}

#[get("/path/{no}/{string}")]
async fn path_service(path: Path<ExtractorsKind>) -> Result<String> {
    Ok(format!("path: {} with {}", path.no, path.string))
}

#[get("/query")]
async fn query_service(query: Query<ExtractorsKind>) -> Result<String> {
    Ok(format!("query: {} with {}", query.no, query.string))
}

#[get("/body")]
async fn body_service(body: Json<ExtractorsKind>) -> Result<String> {
    Ok(format!("body: {} with {}", body.no, body.string))
}

#[derive(Serialize)]
struct HeaderResponse {
    auth: String,
    custom: String,
}

#[get("/header")]
async fn header_service(req: HttpRequest) -> Json<HeaderResponse> {
    let auth_token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let custom_header = req.headers().get("x-custom").unwrap().to_str().unwrap();
    Json(HeaderResponse {
        auth: auth_token.to_owned(),
        custom: custom_header.to_owned(),
    })
}

pub fn extractors() -> Scope {
    actix_web::web::scope("/extractors")
        .service(path_service)
        .service(query_service)
        .service(body_service)
        .service(header_service)
}
