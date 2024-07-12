use std::fmt;

#[derive(Debug)]
pub enum ApiKeyStorageError {
    KeyNotFound,
    KeyAlreadyExists,
    SerializationError(String),
    StorageError(String),
}

impl fmt::Display for ApiKeyStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiKeyStorageError::KeyNotFound => write!(f, "Key not found"),
            ApiKeyStorageError::KeyAlreadyExists => write!(f, "Key already exists"),
            ApiKeyStorageError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            ApiKeyStorageError::StorageError(e) => write!(f, "Storage error: {}", e),
        }
    }
}

impl ApiKeyStorageError {
    pub fn to_message_type(&self) -> String {
        match self {
            ApiKeyStorageError::KeyNotFound => "KeyNotFound".to_string(),
            ApiKeyStorageError::KeyAlreadyExists => "KeyAlreadyExists".to_string(),
            ApiKeyStorageError::SerializationError(_) => "SerializationError".to_string(),
            ApiKeyStorageError::StorageError(_) => "StorageError".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ApiKeyManagerError {
    StorageError(ApiKeyStorageError),
    LimiterError(ApiKeyLimiterError),
    Other(String),
}

#[derive(Debug)]
pub enum ApiKeyLimiterError {
    RateLimitExceeded,
    Other(String),
}

impl fmt::Display for ApiKeyLimiterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiKeyLimiterError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ApiKeyLimiterError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl ApiKeyLimiterError {
    pub fn to_message_type(&self) -> String {
        match self {
            ApiKeyLimiterError::RateLimitExceeded => "RateLimitExceeded".to_string(),
            ApiKeyLimiterError::Other(_) => "ApiLimiter::Other".to_string(),
        }
    }
}
