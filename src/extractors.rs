use actix_web::Scope;
use actix_web::{get, web::Path, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParams {
    no: i32,
    string: String,
}

#[get("/path/{no}/{string}")]
async fn path_service(path: Path<PathParams>) -> Result<String> {
    Ok(format!("{} with {}", path.no, path.string))
}

pub fn extractors() -> Scope {
    actix_web::web::scope("/extractors").service(path_service)
}
