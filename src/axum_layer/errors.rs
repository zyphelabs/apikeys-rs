use std::fmt;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;

use crate::errors::{ApiKeyLimiterError, ApiKeyStorageError};

#[derive(Debug)]
pub enum ApiKeyLayerError {
    MissingApiKey,
    InvalidApiKey,
    ApiKeyNotFound,
    DomainNotAllowed,
    LimiterError(ApiKeyLimiterError),
    StorageError(ApiKeyStorageError),
    UnexpectedError,
}

impl fmt::Display for ApiKeyLayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiKeyLayerError::MissingApiKey => write!(f, "x-api-key header is not set"),
            ApiKeyLayerError::InvalidApiKey => write!(f, "The provided API key is not valid"),
            ApiKeyLayerError::ApiKeyNotFound => write!(f, "The provided API key was not found"),
            ApiKeyLayerError::DomainNotAllowed => {
                write!(f, "The provided API key is not allowed for this domain")
            }
            ApiKeyLayerError::LimiterError(e) => write!(f, "Limiter error: {}", e),
            ApiKeyLayerError::UnexpectedError => write!(f, "Unexpected error"),
            ApiKeyLayerError::StorageError(e) => write!(f, "Storage error: {}", e),
        }
    }
}

impl ApiKeyLayerError {
    pub fn to_message_type(&self) -> String {
        match self {
            ApiKeyLayerError::MissingApiKey => "MissingApiKey".to_string(),
            ApiKeyLayerError::InvalidApiKey => "InvalidApiKey".to_string(),
            ApiKeyLayerError::ApiKeyNotFound => "ApiKeyNotFound".to_string(),
            ApiKeyLayerError::DomainNotAllowed => "DomainNotAllowed".to_string(),
            ApiKeyLayerError::LimiterError(e) => e.to_message_type(),
            ApiKeyLayerError::UnexpectedError => "UnexpectedError".to_string(),
            ApiKeyLayerError::StorageError(e) => e.to_message_type(),
        }
    }
}

impl std::error::Error for ApiKeyLayerError {}

impl From<ApiKeyLayerError> for ApiKeyErrorResponse {
    fn from(error: ApiKeyLayerError) -> Self {
        ApiKeyErrorResponse { message: error.to_string(), _type: error.to_message_type() }
    }
}

impl IntoResponse for ApiKeyLayerError {
    fn into_response(self) -> Response<Body> {
        match self {
            ApiKeyLayerError::MissingApiKey => {
                (StatusCode::UNAUTHORIZED, Json::<ApiKeyErrorResponse>(self.into())).into_response()
            }
            ApiKeyLayerError::InvalidApiKey => {
                (StatusCode::UNAUTHORIZED, Json::<ApiKeyErrorResponse>(self.into())).into_response()
            }
            ApiKeyLayerError::ApiKeyNotFound => {
                (StatusCode::UNAUTHORIZED, Json::<ApiKeyErrorResponse>(self.into())).into_response()
            }
            ApiKeyLayerError::DomainNotAllowed => {
                (StatusCode::UNAUTHORIZED, Json::<ApiKeyErrorResponse>(self.into())).into_response()
            }
            ApiKeyLayerError::LimiterError(e) => (
                StatusCode::UNAUTHORIZED,
                Json::<ApiKeyErrorResponse>(ApiKeyErrorResponse { message: e.to_string(), _type: e.to_message_type() }),
            )
                .into_response(),
            ApiKeyLayerError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json::<ApiKeyErrorResponse>(self.into())).into_response()
            }
            ApiKeyLayerError::StorageError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::<ApiKeyErrorResponse>(ApiKeyErrorResponse { message: e.to_string(), _type: e.to_message_type() }),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiKeyErrorResponse {
    message: String,
    #[serde(rename = "type")]
    _type: String,
}
