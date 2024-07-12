use async_trait::async_trait;
use bson::doc;
use mongodb::{options::ClientOptions, Client, Database};

use crate::{errors::ApiKeyStorageError, traits::ApiKeyStorage, types::ApiKey};

#[derive(Clone)]
pub struct MongoDBStorage {
    db: Database,
    collection_name: String,
}

impl MongoDBStorage {
    pub async fn new(uri: &str, db_name: &str, collection_name: Option<String>) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);

        Ok(Self {
            db,
            collection_name: match collection_name {
                Some(name) => name,
                None => "api_keys".to_string(),
            },
        })
    }
}

#[async_trait]
impl ApiKeyStorage for MongoDBStorage {
    async fn store_api_key(&mut self, key: &str, value: &ApiKey) -> Result<String, ApiKeyStorageError> {
        let collection = self.db.collection::<ApiKey>(self.collection_name.as_str());

        let filter = doc! { "key": key };

        match collection.find_one(filter, None).await {
            Ok(result) => {
                if result.is_some() {
                    return Err(ApiKeyStorageError::KeyAlreadyExists);
                }
            }
            Err(e) => return Err(ApiKeyStorageError::StorageError(e.to_string())),
        }

        match collection.insert_one(value, None).await {
            Ok(result) => result,
            Err(e) => return Err(ApiKeyStorageError::StorageError(e.to_string())),
        };

        Ok(key.to_string())
    }

    async fn retrieve_api_key(&self, key: &str) -> Result<ApiKey, ApiKeyStorageError> {
        let collection = self.db.collection::<ApiKey>(self.collection_name.as_str());

        let filter = doc! { "key": key };

        let result = collection.find_one(filter, None).await;

        let api_key = match result {
            Ok(result) => match result {
                Some(doc) => doc,
                None => return Err(ApiKeyStorageError::KeyNotFound),
            },
            Err(e) => return Err(ApiKeyStorageError::StorageError(e.to_string())),
        };

        Ok(api_key)
    }

    async fn delete_api_key(&mut self, key: &str) -> Result<bool, ApiKeyStorageError> {
        let collection = self.db.collection::<ApiKey>(self.collection_name.as_str());

        let filter = doc! { "key": key };

        let result = collection.delete_one(filter, None).await;

        match result {
            Ok(result) => Ok(result.deleted_count > 0),
            Err(e) => return Err(ApiKeyStorageError::StorageError(e.to_string())),
        }
    }
}
