use async_trait::async_trait;

use crate::{
    errors::{ApiKeyLimiterError, ApiKeyManagerError, ApiKeyStorageError},
    types::ApiKey,
};

#[async_trait]
pub trait ApiKeyStorage {
    async fn store_api_key(&mut self, key: &str, value: &ApiKey) -> Result<String, ApiKeyStorageError>;
    async fn retrieve_api_key(&self, key: &str) -> Result<ApiKey, ApiKeyStorageError>;
    async fn delete_api_key(&mut self, key: &str) -> Result<bool, ApiKeyStorageError>;
}

#[async_trait]
pub trait ApiKeyLimiter {
    async fn use_key(&self, api_key: &ApiKey) -> Result<(), ApiKeyLimiterError>;
}

#[async_trait]
pub trait ApiKeyManager {
    async fn get_key(&self, key: &str) -> Result<ApiKey, ApiKeyManagerError>;
    async fn use_key(&self, key: &str) -> Result<(), ApiKeyManagerError>;
}
