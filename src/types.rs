use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ApiKeyLimit {
    Limited(u32),
    Unlimited,
}

impl fmt::Display for ApiKeyLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiKeyLimit::Limited(limit) => write!(f, "{}", limit),
            ApiKeyLimit::Unlimited => write!(f, "unlimited"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyLimits {
    pub max_reads_per_minute: ApiKeyLimit,
    pub max_writes_per_minute: ApiKeyLimit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyRestrictions {
    pub allowed_domains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ApiKeyStatus {
    Active,
    Inactive,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub key: String,
    pub limits: ApiKeyLimits,
    pub restrictions: ApiKeyRestrictions,
    pub status: ApiKeyStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
