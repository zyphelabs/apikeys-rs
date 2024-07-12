use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisError};

use crate::{
    errors::ApiKeyLimiterError,
    traits::ApiKeyLimiter,
    types::{ApiKey, ApiKeyLimit},
};

#[derive(Clone)]
pub struct RedisLimiter {
    redis_client: Client,
}

impl RedisLimiter {
    pub async fn new(uri: &str) -> Result<Self, RedisError> {
        tracing::debug!("Creating redis client from uri: {}", uri);
        let redis_client = Client::open(uri)?;
        Ok(Self { redis_client })
    }
}

#[async_trait]
impl ApiKeyLimiter for RedisLimiter {
    async fn use_key(&self, api_key: &ApiKey) -> Result<(), ApiKeyLimiterError> {
        match api_key.limits.max_reads_per_minute {
            ApiKeyLimit::Limited(max_reads_per_minute) => {
                let mut connection = self.redis_client.get_async_connection().await?;

                let key = format!("{}_read_count", api_key.key);

                match connection.exists(&key).await? {
                    true => {}
                    false => {
                        let _: Result<String, RedisError> = connection.set(&key, 0).await;

                        let _: Result<String, RedisError> = connection.expire(&key, 60).await;
                    }
                }

                let result: i32 = connection.incr(&key, 1).await?;

                if result > max_reads_per_minute as i32 {
                    return Err(ApiKeyLimiterError::RateLimitExceeded);
                }
            }
            ApiKeyLimit::Unlimited => {}
        }

        Ok(())
    }
}

impl From<RedisError> for ApiKeyLimiterError {
    fn from(error: RedisError) -> Self {
        ApiKeyLimiterError::Other(error.to_string())
    }
}
