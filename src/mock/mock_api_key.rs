use crate::types;

pub fn get_mock_api_key(key: Option<String>) -> types::ApiKey {
    types::ApiKey {
        key: key.unwrap_or("test_key".to_string()),
        limits: types::ApiKeyLimits {
            max_reads_per_minute: types::ApiKeyLimit::Limited(100),
            max_writes_per_minute: types::ApiKeyLimit::Limited(100),
        },
        restrictions: types::ApiKeyRestrictions { allowed_domains: vec!["example.com".to_string()] },
        status: types::ApiKeyStatus::Active,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}
