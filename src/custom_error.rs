use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub struct AppError(StatusCode, String);

impl AppError {
    pub fn new(code: StatusCode, msg: impl Into<String>) -> Self {
        Self(code, msg.into())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error - {} with message {}", self.0, self.1)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.0).body(self.1.to_owned())
    }
    fn status_code(&self) -> StatusCode {
        self.0
    }
}
