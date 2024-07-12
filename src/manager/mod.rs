use async_trait::async_trait;

use crate::{
    errors::{ApiKeyLimiterError, ApiKeyManagerError, ApiKeyStorageError},
    traits::{ApiKeyLimiter, ApiKeyManager, ApiKeyStorage},
    types::ApiKey,
};

#[derive(Clone)]
pub struct KeyManager<S, L>
where
    S: ApiKeyStorage + Send + Sync,
    L: ApiKeyLimiter + Send + Sync,
{
    storage: S,
    limiter: L,
}

impl<S, L> KeyManager<S, L>
where
    S: ApiKeyStorage + Send + Sync,
    L: ApiKeyLimiter + Send + Sync,
{
    pub fn new(storage: S, limiter: L) -> Self {
        KeyManager { storage, limiter }
    }
}

#[async_trait]
impl<S, L> ApiKeyManager for KeyManager<S, L>
where
    S: ApiKeyStorage + Send + Sync,
    L: ApiKeyLimiter + Send + Sync,
{
    async fn get_key(&self, key: &str) -> Result<ApiKey, ApiKeyManagerError> {
        let api_key = self.storage.retrieve_api_key(key).await?;

        Ok(api_key)
    }

    async fn use_key(&self, key: &str) -> Result<(), ApiKeyManagerError> {
        let api_key = self.get_key(key).await?;

        self.limiter.use_key(&api_key).await?;

        Ok(())
    }
}

impl From<ApiKeyLimiterError> for ApiKeyManagerError {
    fn from(error: ApiKeyLimiterError) -> Self {
        ApiKeyManagerError::LimiterError(error)
    }
}

impl From<ApiKeyStorageError> for ApiKeyManagerError {
    fn from(error: ApiKeyStorageError) -> Self {
        ApiKeyManagerError::StorageError(error)
    }
}
