# Apikeys-rs

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/apikeys-rs)](https://crates.io/crates/apikeys-rs)

## Description

Apikeys-rs is a comprehensive toolkit designed to streamline API key management for your backend services. It offers an intuitive abstraction layer for API key storage and a versatile key-based rate limiter, ensuring seamless integration with your preferred storage solutions.

The library comes with a ready-to-use Axum layer and includes built-in storage options and limiters.

### Api Key Storage
- [x] Memory Storage
- [x] MongoDB Storage

### Rate Limiter
- [x] Redis Limiter

### Todo
- [ ] Increase test coverage
- [ ] Add more storage interfaces
- [ ] Add more limiters
- [ ] Implement a weight system to make api calls have their own usage strategy, limits and computational cost

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

```
cargo add apikeys-rs
```

## Basic Usage

### Initialize storage
```rust
use apikeys_rs::{
    storage::{mongodb_storage::MongoDBStorage},
    traits::ApiKeyStorage,
    types::{ApiKey, ApiKeyLimits, ApiKeyLimit, ApiKeyRestrictions, ApiKeyStatus}
};

dotenv::dotenv().ok();
let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
let db_name = std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set");

let mut storage = MongoDBStorage::new(&uri, &db_name, None)
    .await
    .expect("Failed to create MongoDBStorage");
```

### Store a key
```rust
// [...] imports

let key = "test_key";

let api_key_config = ApiKey {
    key: key.unwrap_or("test_key".to_string()),
    limits: ApiKeyLimits {
        max_reads_per_minute: ApiKeyLimit::Limited(100),
        max_writes_per_minute: ApiKeyLimit::Limited(100),
    },
    restrictions: ApiKeyRestrictions { allowed_domains: vec!["example.com".to_string()] },
    status: ApiKeyStatus::Active,
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};

let result = storage.store_api_key(key, &api_key_config).await;
```

### Retrieve a key
```rust
// [...] imports

let key = "test_key";

let result = storage.retrieve_api_key(key).await;

match result {
    Ok(retrieved_api_key) => {
        /* [...]  your code here */
    },
    Err(e) => {
        /* [...]  your code here */
    },
}
```

### Redis Limiter
```rust
use apikeys_rs::{
    limiters::{RedisLimiter}
};

let redis_uri = "redis_connection_string";

let redis_limiter = RedisLimiter::new(redis_uri).expect("Unable to create redis limiter");

let api_key = /* [...] get api key from storage (see above) */;

let result = redis_limiter.use_key(&key);

match result {
    /* [...] your code here */
}

```

## Axum Layer Usage

```rust
// [...] imports

// Create the API Key Storage
let api_key_storage = MongoDBStorage::new(
    std::env::var("MONGODB_URI").unwrap().as_str(),
    std::env::var("MONGODB_DB_NAME").unwrap().as_str(),
    None,
)
.await
.expect("Could not create MongoDBStorage");

// Create the API Key Limiter
let api_key_limiter = RedisLimiter::new(redis_connection_string.clone().as_str())
    .await
    .expect("Could not create RedisLimiter");

// Create the API Key Manager
let api_key_manager = KeyManager::new(api_key_storage, api_key_limiter);

// Create the API Key Axum Layer
let api_key_layer = ApiKeyLayer::new(api_key_manager);

let app = Router::new()
    .route("/verify", post(verify))
    // Configure Axum to use the Api Key layer
    .layer(api_key_layer)
    .with_state(state);

```

## Contributing

Feel free to open issues and send PRs. We will evaluate them together in the comment section.

## License

This project is licensed under the [MIT License](LICENSE).
