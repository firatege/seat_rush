use actix_http::body::BoxBody;
use actix_web::http::StatusCode;
use actix_multipart::{LimitExceeded, MultipartError};
use actix_web::http::header::ToStrError;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use std::num::ParseIntError;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors};
use crate::utils::responser::BodyBuilder;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Authorization error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Request validation error: {0}")]
    Validation(String),

    #[error("Internal Server Error: {0}")]
    Internal(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    Conflict(String),

    #[error("JWT Error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Database Error: {0}")]
    DbError(#[from] sea_orm::DbErr),


    #[error("Multipart Error: {0}")]
    MultipartError(#[from] MultipartError),

    #[error("Multipart Limit Exceeded: {0}")]
    LimitExceeded(#[from] LimitExceeded),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Parse int error: {0}")]
    HeaderToStrError(#[from] ToStrError),

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl From<ValidationError> for GatewayError {
    fn from(value: ValidationError) -> Self {
        GatewayError::Validation(value.to_string())
    }
}

impl From<ValidationErrors> for GatewayError {
    fn from(value: ValidationErrors) -> Self {
        GatewayError::Validation(value.to_string())
    }
}


impl ResponseError for GatewayError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        // ensure the response body carries the same HTTP status as the error
        let body = BodyBuilder::from(self.to_string()).status(self.status_code());
        error!("{:?}", body);
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl GatewayError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            GatewayError::Auth(_) => StatusCode::UNAUTHORIZED,
            GatewayError::Forbidden(_) => StatusCode::FORBIDDEN,
            GatewayError::NotFound(_) => StatusCode::NOT_FOUND,
            GatewayError::Validation(_) => StatusCode::BAD_REQUEST,
            GatewayError::Conflict(_) => StatusCode::CONFLICT,

            GatewayError::JWTError(_) => StatusCode::UNAUTHORIZED,
            GatewayError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GatewayError::MultipartError(_) => StatusCode::BAD_REQUEST,
            GatewayError::LimitExceeded(_) => StatusCode::PAYLOAD_TOO_LARGE,
            GatewayError::ReqwestError(_) => StatusCode::BAD_GATEWAY,
            GatewayError::HeaderToStrError(_) => StatusCode::BAD_REQUEST,
            GatewayError::ParseIntError(_) => StatusCode::BAD_REQUEST,

            GatewayError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
