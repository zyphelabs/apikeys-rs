pub mod axum_layer;
pub mod errors;
pub mod limiters;
pub mod manager;
#[cfg(test)]
mod mock;
pub mod storage;
pub mod traits;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock::mock_api_key::get_mock_api_key,
        storage::{memory_storage::HashMapStorage, mongodb_storage::MongoDBStorage},
        traits::ApiKeyStorage,
    };

    #[tokio::test]
    async fn it_can_store_an_api_key() {
        let mut storage = HashMapStorage::new();

        let key = "test_key";

        let api_key = get_mock_api_key(Some(key.to_string()));

        let result = storage.store_api_key(key, &api_key).await;

        match result {
            Ok(storage_key) => assert_eq!(key, storage_key, "The stored key should match the key that was passed in"),
            Err(e) => assert_eq!(true, false, "The key should have been stored {}", e),
        }
    }

    #[tokio::test]
    async fn it_can_rietrieve_an_api_key() {
        let mut storage = HashMapStorage::new();

        let key = "test_key";

        let api_key = get_mock_api_key(Some(key.to_string()));

        let result = storage.store_api_key(key, &api_key).await;

        match result {
            Ok(storage_key) => assert_eq!(key, storage_key, "The stored key should match the key that was passed in"),
            Err(e) => assert_eq!(true, false, "The key should have been stored {}", e),
        }

        let result = storage.retrieve_api_key(key).await;

        match result {
            Ok(retrieved_api_key) => assert_eq!(
                api_key.key, retrieved_api_key.key,
                "The retrieved key should match the key that was passed in"
            ),
            Err(e) => match e {
                errors::ApiKeyStorageError::KeyNotFound => {
                    assert_eq!(true, false, "The key should have been found {}", e)
                }
                _ => assert_eq!(true, false, "The key should have been found {}", e),
            },
        }
    }

    #[tokio::test]
    async fn it_can_store_an_api_key_using_mongodb_storage() {
        dotenv::dotenv().ok();
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let db_name = std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set");

        let mut storage = MongoDBStorage::new(&uri, &db_name, None)
            .await
            .expect("Failed to create MongoDBStorage");

        let key = "test_key";

        let api_key = get_mock_api_key(Some(key.to_string()));

        let _ = storage.delete_api_key(key).await;

        let result = storage.store_api_key(key, &api_key).await;

        match result {
            Ok(storage_key) => assert_eq!(key, storage_key, "The stored key should match the key that was passed in"),
            Err(e) => assert_eq!(true, false, "The key should have been stored {}", e),
        }
    }
}
