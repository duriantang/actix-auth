use actix_web::{error::ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
