use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "Failed to process the request: {}", field)]
    Unprocessable { field: String },

    #[display(fmt = "An internal error occurred: {}", field)]
    InternalError { field: String },
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ApiError::Unprocessable { .. } => StatusCode::BAD_REQUEST,
            ApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        let message = err.to_string();
        match err {
            sqlx::Error::RowNotFound => ApiError::Unprocessable { field: message },
            _ => ApiError::InternalError { field: message },
        }
    }
}
