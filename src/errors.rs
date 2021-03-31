use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::Error;
use std::fmt::Debug;
use validator::ValidationErrors;

#[derive(Debug, Serialize, Display)]
pub enum CustomError {
    #[display(fmt = "Bad Gateway {}", _0)]
    IntegrationError(String),

    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    ValidationError(String),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::IntegrationError(ref message) => HttpResponse::BadGateway().json(message),
            CustomError::ValidationError(ref message) => HttpResponse::BadRequest().json(message),
            _ => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    }
}

impl From<Error> for CustomError {
    fn from(error: Error) -> CustomError {
        CustomError::IntegrationError(error.to_string())
    }
}

impl From<ValidationErrors> for CustomError {
    fn from(error: ValidationErrors) -> CustomError {
        CustomError::ValidationError(error.to_string())
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError::IntegrationError(error.to_string())
    }
}
