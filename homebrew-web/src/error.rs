use actix_multipart::MultipartError;
use actix_web::{body::Body, error, http::StatusCode, BaseHttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Failed to process the request: {}", reason)]
    BadRequest { reason: Cow<'static, str> },

    #[display(fmt = "An internal error occurred: {}", reason)]
    InternalError { reason: Cow<'static, str> },
}

impl ApiError {
    pub fn error<S>(reason: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::BadRequest {
            reason: reason.into(),
        }
    }

    pub fn internal_error<S>(reason: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::InternalError {
            reason: reason.into(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::BadRequest { .. } => "Bad Request".to_string(),
            Self::InternalError { .. } => "Internal Error".to_string(),
        }
    }

    pub fn reason(&self) -> String {
        match self {
            Self::BadRequest { reason } => reason.to_string(),
            Self::InternalError { reason } => reason.to_string(),
        }
    }
}

impl<'a> error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> BaseHttpResponse<Body> {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.reason(),
            error: self.name(),
        };
        HttpResponseBuilder::new(self.status_code())
            .json(error_response)
            .into()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        let message = err.to_string();
        match err {
            sqlx::Error::RowNotFound => ApiError::error(message),
            _ => ApiError::internal_error(message),
        }
    }
}

impl From<MultipartError> for ApiError {
    fn from(err: MultipartError) -> Self {
        let message = err.to_string();
        ApiError::error(message)
    }
}
