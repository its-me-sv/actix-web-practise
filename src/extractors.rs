use actix_web::Scope;
use actix_web::{
    get,
    web::{Json, Path, Query},
    Result,
};
use serde::Deserialize;

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

pub fn extractors() -> Scope {
    actix_web::web::scope("/extractors")
        .service(path_service)
        .service(query_service)
        .service(body_service)
}
