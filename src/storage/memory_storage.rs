use std::collections::HashMap;

use async_trait::async_trait;

use crate::{errors::ApiKeyStorageError, traits::ApiKeyStorage, types::ApiKey};

#[derive(Clone, Default)]
pub struct HashMapStorage {
    map: HashMap<String, ApiKey>,
}

impl HashMapStorage {
    pub fn new() -> Self {
        Self { map: HashMap::default() }
    }
}

#[async_trait]
impl ApiKeyStorage for HashMapStorage {
    async fn store_api_key(&mut self, key: &str, value: &ApiKey) -> Result<String, ApiKeyStorageError> {
        self.map.insert(key.to_string(), value.clone());

        Ok(key.to_string())
    }

    async fn retrieve_api_key(&self, key: &str) -> Result<ApiKey, ApiKeyStorageError> {
        match self.map.get(key).cloned() {
            Some(api_key) => Ok(api_key),
            None => Err(ApiKeyStorageError::KeyNotFound),
        }
    }

    async fn delete_api_key(&mut self, key: &str) -> Result<bool, ApiKeyStorageError> {
        Ok(self.map.remove(key).is_some())
    }
}
