use actix_web::{
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "forbidden")]
    Forbidden,
    #[display(fmt = "not found")]
    NotFound,
    #[display(fmt = "conflict")]
    Conflict,
    #[display(fmt = "internal server error")]
    InternalServerError,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
pub struct ErrorToResponse {
    pub error: String,
}

#[cfg(not(tarpaulin_include))]
impl ResponseError for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header((header::CONTENT_TYPE, "application/json; charset=UTF-8"))
            .body(
                serde_json::to_string(&ErrorToResponse {
                    error: self.to_string(),
                })
                .unwrap(),
            )
    }

    #[cfg(not(tarpaulin_include))]
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::Conflict => StatusCode::CONFLICT,
            ServiceError::Forbidden => StatusCode::FORBIDDEN
        }
    }
}

// impl From<ValidationErrors> for ServiceError {
//     #[cfg(not(tarpaulin_include))]
//     fn from(_: ValidationErrors) -> ServiceError {
//         ServiceError::NotAnEmail
//     }
// }

#[cfg(not(tarpaulin_include))]
impl From<sqlx::Error> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: sqlx::Error) -> Self {
        use sqlx::error::Error;
        use std::borrow::Cow;
        if let Error::Database(err) = e {
            if err.code() == Some(Cow::from("23505")) {
                return ServiceError::Conflict;
            }
        }
        ServiceError::InternalServerError
    }
}

#[cfg(not(tarpaulin_include))]
pub type ServiceResult<V> = std::result::Result<V, ServiceError>;

