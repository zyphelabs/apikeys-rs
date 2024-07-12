use std::task::{Context, Poll};

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use futures_util::future::BoxFuture;
use http::HeaderMap;
use tower::{Layer, Service};

pub mod errors;
use tracing::error;

use self::errors::ApiKeyLayerError;
use crate::{errors::ApiKeyManagerError, traits::ApiKeyManager};

#[derive(Clone)]
pub struct ApiKeyLayer<T>
where
    T: ApiKeyManager + Send + Sync + Clone,
{
    manager: T,
}

impl<S, T> Layer<S> for ApiKeyLayer<T>
where
    T: ApiKeyManager + Send + Sync + Clone,
{
    type Service = ApiKeyMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiKeyMiddleware { inner, manager: self.manager.clone() }
    }
}

#[derive(Clone)]
pub struct ApiKeyMiddleware<S, T>
where
    T: ApiKeyManager + Send + Sync + Clone,
{
    inner: S,
    manager: T,
}

impl<S, T> Service<Request> for ApiKeyMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    T: ApiKeyManager + Send + Sync + Clone + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let headers = request.headers().clone();
        // let origin = extract_header(header::ORIGIN.as_str(), &headers);

        let x_api_key = match extract_header("x-api-key", &headers) {
            Some(key) => key,
            None => {
                return Box::pin(async move {
                    let response = errors::ApiKeyLayerError::MissingApiKey.into_response();
                    Ok(response)
                });
            }
        };

        let manager = self.manager.clone();
        let future = self.inner.call(request);
        let verification_future = verify_api_key(manager, x_api_key);
        Box::pin(async move {
            match verification_future.await {
                Ok(true) => {
                    let response: Response = future.await?;
                    Ok(response)
                }
                Ok(false) => {
                    let response = errors::ApiKeyLayerError::InvalidApiKey.into_response();
                    Ok(response)
                }
                Err(e) => {
                    let response = e.into_response();
                    Ok(response)
                }
            }
        })
    }
}

impl<T> ApiKeyLayer<T>
where
    T: ApiKeyManager + Send + Sync + Clone,
{
    pub fn new(manager: T) -> Self
    where
        T: ApiKeyManager + Send + Sync + Clone,
    {
        Self { manager }
    }
}

fn extract_header(key: &str, headers: &HeaderMap) -> Option<String> {
    match headers.get(key) {
        Some(key) => match key.to_str() {
            Ok(key) => Some(key.to_string()),
            Err(_) => None,
        },
        None => None,
    }
}

async fn verify_api_key(
    manager: impl ApiKeyManager + Send + Sync,
    key: String,
) -> Result<bool, errors::ApiKeyLayerError> {
    match manager.use_key(key.as_str()).await {
        Ok(key) => key,
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(true)
}

impl From<ApiKeyManagerError> for ApiKeyLayerError {
    fn from(error: ApiKeyManagerError) -> Self {
        match error {
            ApiKeyManagerError::LimiterError(e) => ApiKeyLayerError::LimiterError(e),
            e => {
                error!("{e:?}");
                ApiKeyLayerError::InvalidApiKey
            }
        }
    }
}
